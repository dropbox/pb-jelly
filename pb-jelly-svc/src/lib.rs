#![feature(never_type, nll, generators, try_blocks)]

#[macro_use]
extern crate dbx_async;
#[macro_use]
extern crate do_catch;
#[macro_use]
extern crate events;

use std::io;

use context::Context;
use dbx_collections::rc::RcCell;
use events::extract_error_context;
use futures::{
    Future,
    Stream,
};
use pb::Message;
use std::sync::{
    Arc,
    Mutex,
};

pub trait Transport {
    type Error;

    fn unary_unary<Request, Response>(
        &mut self,
        ctx: Context,
        route: &'static str,
        req: Request,
    ) -> Box<dyn Future<Item = Response, Error = Self::Error>>
    where
        Request: Message + Send + 'static,
        Response: Message + Default + Send + 'static;

    fn unary_streaming<Request, Response>(
        &mut self,
        ctx: Context,
        route: &'static str,
        req: Request,
    ) -> Box<dyn Stream<Item = Response, Error = Self::Error>>
    where
        Request: Message + Send + 'static,
        Response: Message + Default + Send + 'static;
}

pub trait RawTransport {
    type Error: From<io::Error> + Send + 'static;

    fn raw_unary_unary(
        &mut self,
        ctx: Context,
        route: &'static str,
        req: Vec<u8>,
    ) -> Box<dyn Future<Item = Vec<u8>, Error = Self::Error>>;

    fn raw_unary_streaming(
        &mut self,
        ctx: Context,
        route: &'static str,
        req: Vec<u8>,
    ) -> Box<dyn Stream<Item = Vec<u8>, Error = Self::Error>>;
}

pub trait Dispatch<S> {
    type Error;

    fn dispatch_unary(service: &mut S, route: &str, req: &[u8])
        -> Box<dyn Future<Item = Vec<u8>, Error = Self::Error>>;
    fn dispatch_stream(
        service: &mut S,
        route: &str,
        req: &[u8],
    ) -> Box<dyn Stream<Item = Vec<u8>, Error = Self::Error>>;
}

impl<T: RawTransport> Transport for T
where
    T::Error: From<io::Error>,
{
    type Error = T::Error;

    fn unary_unary<Request, Response>(
        &mut self,
        ctx: Context,
        route: &'static str,
        req: Request,
    ) -> Box<dyn Future<Item = Response, Error = Self::Error>>
    where
        Request: Message + Send + 'static,
        Response: Message + Default + Send + 'static,
    {
        let req_blob = req.serialize_to_vec();
        Box::new(
            self.raw_unary_unary(ctx, route, req_blob)
                .and_then(|resp: Vec<u8>| Response::deserialize_from_slice(&resp).map_err(|e| e.into())),
        )
    }

    fn unary_streaming<Request, Response>(
        &mut self,
        ctx: Context,
        route: &'static str,
        req: Request,
    ) -> Box<dyn Stream<Item = Response, Error = Self::Error>>
    where
        Request: Message + Send + 'static,
        Response: Message + Default + Send + 'static,
    {
        let req_blob = req.serialize_to_vec();
        let res: Box<dyn Stream<Item = Response, Error = Self::Error>> = Box::new(
            self.raw_unary_streaming(ctx, route, req_blob)
                .and_then(|resp: Vec<u8>| Response::deserialize_from_slice(&resp).map_err(|e| e.into())),
        );
        res
    }
}

impl<T: Transport> Transport for Arc<Mutex<T>> {
    type Error = T::Error;

    fn unary_unary<Request, Response>(
        &mut self,
        ctx: Context,
        route: &'static str,
        req: Request,
    ) -> Box<dyn Future<Item = Response, Error = Self::Error>>
    where
        Request: Message + Send + 'static,
        Response: Message + Default + Send + 'static,
    {
        let mut inner = self.lock().expect("Transport lock poisoned?");
        inner.unary_unary(ctx, route, req)
    }

    fn unary_streaming<Request, Response>(
        &mut self,
        ctx: Context,
        route: &'static str,
        req: Request,
    ) -> Box<dyn Stream<Item = Response, Error = Self::Error>>
    where
        Request: Message + Send + 'static,
        Response: Message + Default + Send + 'static,
    {
        let mut inner = self.lock().expect("Transport lock poisoned?");
        inner.unary_streaming(ctx, route, req)
    }
}

impl<T: Transport> Transport for Option<T> {
    type Error = T::Error;

    fn unary_unary<Request, Response>(
        &mut self,
        ctx: Context,
        route: &'static str,
        req: Request,
    ) -> Box<dyn Future<Item = Response, Error = Self::Error>>
    where
        Request: Message + Send + 'static,
        Response: Message + Default + Send + 'static,
    {
        let inner = self.as_mut().expect("Transport disappeared?");
        inner.unary_unary(ctx, route, req)
    }

    fn unary_streaming<Request, Response>(
        &mut self,
        ctx: Context,
        route: &'static str,
        req: Request,
    ) -> Box<dyn Stream<Item = Response, Error = Self::Error>>
    where
        Request: Message + Send + 'static,
        Response: Message + Default + Send + 'static,
    {
        let inner = self.as_mut().expect("Transport disappeared?");
        inner.unary_streaming(ctx, route, req)
    }
}

impl<T: Transport> Transport for RcCell<T> {
    type Error = T::Error;

    fn unary_unary<Request, Response>(
        &mut self,
        ctx: Context,
        route: &'static str,
        req: Request,
    ) -> Box<dyn Future<Item = Response, Error = Self::Error>>
    where
        Request: Message + Send + 'static,
        Response: Message + Default + Send + 'static,
    {
        self.borrow_mut().unary_unary(ctx, route, req)
    }

    fn unary_streaming<Request, Response>(
        &mut self,
        ctx: Context,
        route: &'static str,
        req: Request,
    ) -> Box<dyn Stream<Item = Response, Error = Self::Error>>
    where
        Request: Message + Send + 'static,
        Response: Message + Default + Send + 'static,
    {
        self.borrow_mut().unary_streaming(ctx, route, req)
    }
}

pub trait UnaryResponder: 'static {
    /// Respond to the request.
    /// * The Ok() variant of `result` should be an encoded protobuf message
    /// * The Err() variant should be an error message suitable for logging
    fn respond(self, result: Result<Vec<u8>, String>);

    /// Mark a request as canceled.
    /// * Note, `method_name` is a temporary hack to make sure we log the same information in the case of cancel.
    ///   In the future, tprt (or some more generic layer) should handle this for us.
    fn cancel(self, method_name: Option<&str>);
}

pub trait StreamResponder: 'static {
    /// Send a single response. This can be called multiple times.
    fn item(&self, item: Vec<u8>);

    /// Finish the stream with an error. No more responses can be sent.
    fn error(self, error: String);

    /// Finish the stream successfully. No more responses can be sent.
    fn complete(self);

    /// Mark a request as canceled.
    /// * Note, `method_name` is a temporary hack to make sure we log the same information in the case of cancel.
    ///   In the future, tprt (or some more generic layer) should handle this for us.
    fn cancel(self, method_name: Option<&str>);
}

pub trait Responder: 'static {
    type UnaryResponder: UnaryResponder;
    type StreamResponder: StreamResponder;

    fn bundle(self) -> ResponderBundle<Self::UnaryResponder, Self::StreamResponder>;
}

#[derive(Debug)]
pub enum ResponderBundle<U: UnaryResponder, S: StreamResponder> {
    Unary(U),
    Stream(S),
}

impl<U: UnaryResponder, S: StreamResponder> Responder for ResponderBundle<U, S> {
    type UnaryResponder = U;
    type StreamResponder = S;

    fn bundle(self) -> Self {
        self
    }
}

impl<U: UnaryResponder, S: StreamResponder> ResponderBundle<U, S> {
    pub fn cancel(self, method_name: Option<&str>) {
        match self {
            ResponderBundle::Unary(u) => u.cancel(method_name),
            ResponderBundle::Stream(u) => u.cancel(method_name),
        }
    }

    pub fn error(self, error: String) {
        match self {
            ResponderBundle::Unary(u) => u.respond(Err(error)),
            ResponderBundle::Stream(u) => u.error(error),
        }
    }
}

#[derive(Debug)]
pub struct Request<R: Responder> {
    pub method_name: String,
    pub method_req: Vec<u8>,
    pub responder: R,
}

pub trait ServerRequestHandler<Service>: Dispatch<Service> {
    fn handle_request<R: Responder>(
        service: &mut Service,
        service_request: Request<R>,
    ) -> Box<dyn Future<Item = (), Error = !>>;
}

impl<Service, Dispatcher: Dispatch<Service>> ServerRequestHandler<Service> for Dispatcher
where
    Dispatcher::Error: Into<anyhow::Error> + From<io::Error> + 'static,
{
    fn handle_request<R: Responder>(
        service: &mut Service,
        service_request: Request<R>,
    ) -> Box<dyn Future<Item = (), Error = !>> {
        let Request {
            method_name,
            method_req,
            responder,
        } = service_request;
        match responder.bundle() {
            ResponderBundle::Unary(responder) => {
                let fut = Dispatcher::dispatch_unary(service, &method_name, &method_req);
                let in_progress_request = InProgressUnaryRequest {
                    method_name,
                    responder: Some(responder),
                };
                async_boxed!({
                    in_progress_request.respond(dbx_await!(fut));
                    Ok(())
                })
            },
            ResponderBundle::Stream(responder) => {
                let stream = Dispatcher::dispatch_stream(service, &method_name, &method_req);
                let mut in_progress_request = InProgressStreamRequest {
                    method_name,
                    responder: Some(responder),
                };
                async_boxed!({
                    let r: Result<(), Dispatcher::Error> = do_catch!({
                        for_stream!((item in stream) {
                            in_progress_request.item(item);
                        });
                    });

                    match r {
                        Ok(()) => in_progress_request.done(),
                        Err(e) => in_progress_request.error(e),
                    }

                    Ok(())
                })
            },
        }
    }
}

struct InProgressUnaryRequest<U: UnaryResponder> {
    method_name: String,
    responder: Option<U>, // None means that we've responded already
}

impl<U: UnaryResponder> Drop for InProgressUnaryRequest<U> {
    fn drop(&mut self) {
        if let Some(responder) = self.responder.take() {
            responder.cancel(Some(&self.method_name));
        }
    }
}

impl<U: UnaryResponder> InProgressUnaryRequest<U> {
    fn respond<E: Into<anyhow::Error>>(mut self, result: Result<Vec<u8>, E>) {
        self.responder
            .take()
            .expect("Complete called more than once?")
            .respond(result.map_err(|err| {
                let err: anyhow::Error = err.into();
                info!("Error while responding to unary request.",
                      err => err,
                      method_name => self.method_name
                );
                extract_error_context(&err)
            }));
    }
}

struct InProgressStreamRequest<S: StreamResponder> {
    method_name: String,
    responder: Option<S>, // None means that we've responded already
}

impl<S: StreamResponder> Drop for InProgressStreamRequest<S> {
    fn drop(&mut self) {
        if let Some(responder) = self.responder.take() {
            responder.cancel(Some(&self.method_name));
        }
    }
}

impl<S: StreamResponder> InProgressStreamRequest<S> {
    fn item(&mut self, item: Vec<u8>) {
        self.responder.as_mut().expect("Item called after complete?").item(item);
    }

    fn error<E: Into<anyhow::Error>>(mut self, err: E) {
        let err: anyhow::Error = err.into();
        info!("Error while responding to stream request.",
              err => err,
              method_name => self.method_name
        );
        self.responder
            .take()
            .expect("Stream completed more than once?")
            .error(extract_error_context(&err));
    }

    fn done(mut self) {
        self.responder
            .take()
            .expect("Stream completed more than once?")
            .complete();
    }
}
