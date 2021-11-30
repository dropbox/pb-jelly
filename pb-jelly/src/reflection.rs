use crate::erased;

/// A mutable borrow of a field, or `Empty` if the field does not have a value.
#[non_exhaustive]
pub enum FieldMut<'a> {
    /// Mutable borrow of the field.
    Value(&'a mut dyn Reflection),
    // TODO: Would be nice to support this, but some more thought would need to be put into what
    // the API for it looks like.
    // Repeated(&'a mut Vec<dyn Reflection>),
    /// An empty oneof variant.
    Empty,
}

pub trait Reflection: erased::Message {
    /// Returns the name of the field that is set inside a oneof group.
    ///
    /// If no field is set, returns None.
    ///
    /// # Panics
    /// If the name given is not the name of a oneof field in this message.
    fn which_one_of(&self, _oneof_name: &str) -> Option<&'static str> {
        unimplemented!("proto reflection is not implemented for this type")
    }

    /// Returns a mutable borrow of the given field.
    ///
    /// If the field given is part of a oneof group, and that oneof group is not currently set to
    /// this variant, this will mutate the current message, setting the oneof group to the variant
    /// given by this field name with the default value.
    ///
    /// # Panics
    /// If the name given is not the name of a field in this message.
    fn get_field_mut(&mut self, _field_name: &str) -> FieldMut<'_> {
        unimplemented!("proto reflection is not implemented for this type")
    }
}
