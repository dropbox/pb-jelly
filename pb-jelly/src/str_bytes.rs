use bytes::Bytes;
use std::{
    convert::TryFrom,
    fmt,
    ops::Deref,
};

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct StrBytes {
    bytes: Option<Bytes>,
}

impl StrBytes {
    pub fn new() -> StrBytes {
        StrBytes { bytes: None }
    }

    /// Length of the underlying byte store
    pub fn len(&self) -> usize {
        match self.bytes.as_ref() {
            None => 0,
            Some(bytes) => bytes.len(),
        }
    }

    pub fn bytes(&self) -> Option<&Bytes> {
        self.bytes.as_ref()
    }

    pub fn into_bytes(self) -> Bytes {
        match self.bytes {
            None => Bytes::new(),
            Some(bytes) => bytes,
        }
    }

    pub fn replace_bytes(&mut self, bytes: Bytes) -> Result<Option<Bytes>, std::str::Utf8Error> {
        std::str::from_utf8(&bytes)?;

        Ok(self.bytes.replace(bytes))
    }
}

impl Deref for StrBytes {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match self.bytes.as_ref() {
            // SAFTEY: We validate on creation the underlying bytes are valid UTF8
            Some(bytes) => unsafe { std::str::from_utf8_unchecked(&bytes) },
            None => "",
        }
    }
}

impl TryFrom<Bytes> for StrBytes {
    type Error = std::str::Utf8Error;

    fn try_from(bytes: Bytes) -> Result<StrBytes, Self::Error> {
        std::str::from_utf8(&bytes)?;

        Ok(StrBytes { bytes: Some(bytes) })
    }
}

impl TryFrom<&[u8]> for StrBytes {
    type Error = std::str::Utf8Error;

    fn try_from(slice: &[u8]) -> Result<StrBytes, Self::Error> {
        std::str::from_utf8(slice)?;

        Ok(StrBytes {
            bytes: Some(Bytes::copy_from_slice(slice)),
        })
    }
}

impl From<&str> for StrBytes {
    fn from(string: &str) -> StrBytes {
        StrBytes {
            bytes: Some(Bytes::copy_from_slice(string.as_bytes())),
        }
    }
}

impl From<String> for StrBytes {
    fn from(string: String) -> StrBytes {
        StrBytes {
            bytes: Some(Bytes::from(string)),
        }
    }
}

impl fmt::Display for StrBytes {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&**self, f)
    }
}
