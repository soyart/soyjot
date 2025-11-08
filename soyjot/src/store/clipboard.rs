use serde::Deserialize;

use super::data::Data;
use super::error::StoreError;

pub const MEM: &str = "mem";
pub const PERSIST: &str = "persist";

/// Clipboard is our main entity.
/// Currently, it only has Data field and no metadata.
#[derive(Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct Clipboard(Data);

impl Clipboard {
    #[allow(dead_code)]
    pub fn new(_: &str) -> Self {
        Self(Data::new())
    }

    pub fn new_with_data<T>(_: &str, data: T) -> Self
    where
        T: Into<Data>,
    {
        Self(data.into())
    }

    pub fn is_implemented(&self) -> Result<(), StoreError> {
        Ok(())
    }
}

impl std::ops::Deref for Clipboard {
    type Target = [u8];
    fn deref(self: &Self) -> &Self::Target {
        self.0.as_ref()
    }
}

impl AsRef<Data> for Clipboard {
    fn as_ref(&self) -> &Data {
        &self.0
    }
}

impl AsRef<[u8]> for Clipboard {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl std::fmt::Debug for Clipboard {
    fn fmt(self: &Self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bytes: &[u8] = self.as_ref();

        if let Ok(string) = std::str::from_utf8(bytes) {
            write!(formatter, r#"{}"#, string)
        } else {
            write!(formatter, r#"{:?}"#, bytes)
        }
    }
}
