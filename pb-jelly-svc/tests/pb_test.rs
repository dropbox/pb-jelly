use context::Context;
use futures::{
    future,
    Future,
    Stream,
};
use pb::Message;
use pb_service::Transport;
use proto_pbtest::servicepb::{
    InpMessage,
    OutMessage,
};
use proto_pbtest_svc::servicepb::{
    TestServiceClient,
    TestServiceDispatcher,
    TestServiceService,
};
use std::io::{
    Error,
    ErrorKind,
};

struct Dummy(bool);

impl Transport for Dummy {
    type Error = Error;

    fn unary_unary<Request, Response>(
        &mut self,
        _ctx: Context,
        _route: &'static str,
        _req: Request,
    ) -> Box<dyn Future<Item = Response, Error = Self::Error>>
    where
        Request: Message + Send + 'static,
        Response: Message + Default + Send + 'static,
    {
        if self.0 {
            Box::new(future::ok(Response::default()))
        } else {
            Box::new(future::err(ErrorKind::InvalidInput.into()))
        }
    }

    fn unary_streaming<Request, Response>(
        &mut self,
        _ctx: Context,
        _route: &'static str,
        _req: Request,
    ) -> Box<dyn Stream<Item = Response, Error = Self::Error>>
    where
        Request: Message + Send + 'static,
        Response: Message + Default + Send + 'static,
    {
        panic!("Streaming not supported");
    }
}

#[test]
fn client_test() {
    let mut client = TestServiceClient::new(Dummy(true));
    assert_eq!(
        OutMessage::default(),
        client.TestRPC(InpMessage::default()).wait().unwrap()
    );
    let mut client = TestServiceClient::new(Dummy(false));
    assert_eq!(
        ErrorKind::InvalidInput,
        client.TestRPC(InpMessage::default()).wait().err().unwrap().kind()
    );
}

#[test]
fn dispatcher_test() {
    let mut client = TestServiceClient::new(Dummy(true));
    let mut dispatcher = TestServiceDispatcher::new(&mut client);
    assert_eq!(
        OutMessage::default().serialize_to_vec(),
        dispatcher
            .dispatch_unary("/pbtest.TestService/TestRPC", &InpMessage::default().serialize_to_vec(),)
            .wait()
            .unwrap()
    );
    assert_eq!(
        ErrorKind::NotFound,
        dispatcher.dispatch_unary("blargh", &[]).wait().err().unwrap().kind()
    );
}
