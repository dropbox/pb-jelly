//! Blanket-implemented erased typings for the types used throughout the serialization portion of
//! this crate, to allow for implementation of the proto message field reflection API.
use std::io::{
    Cursor,
    Result,
};

use crate::descriptor::MessageDescriptor;
use crate::Message as ConcreteMessage;

pub trait Message {
    fn erased_descriptor(&self) -> Option<MessageDescriptor>;
    fn erased_compute_size(&self) -> usize;
    fn erased_compute_grpc_slices_size(&self) -> usize;
    fn erased_serialize(&self) -> Vec<u8>;
    fn erased_deserialize(&mut self, r: &[u8]) -> Result<()>;
}

impl<T> Message for T
where
    T: ConcreteMessage,
{
    fn erased_descriptor(&self) -> Option<MessageDescriptor> {
        T::DESCRIPTOR
    }

    fn erased_compute_size(&self) -> usize {
        self.compute_size()
    }

    fn erased_compute_grpc_slices_size(&self) -> usize {
        self.compute_grpc_slices_size()
    }

    fn erased_serialize(&self) -> Vec<u8> {
        self.serialize_to_vec()
    }

    fn erased_deserialize(&mut self, r: &[u8]) -> Result<()> {
        self.deserialize(&mut Cursor::new(r))
    }
}
