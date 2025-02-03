use alloc::string::String;
use core::fmt::Write;

use serde::{Deserialize, Serialize, Serializer};

use crate::Digest;

#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
impl Serialize for Digest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::Error;

        let mut buffer = String::new();

        write!(buffer, "{}:", self.algo()).map_err(S::Error::custom)?;

        for byte in self.as_ref() {
            write!(buffer, "{:02x}", byte).map_err(S::Error::custom)?;
        }

        buffer.serialize(serializer)
    }
}

struct Visitor;

impl<'de> serde::de::Visitor<'de> for Visitor {
    type Value = Digest;

    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(formatter, "a string containing a digest")
    }

    fn visit_borrowed_str<E: serde::de::Error>(self, value: &'de str) -> Result<Self::Value, E> {
        value.parse().map_err(E::custom)
    }

    fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<Self::Value, E> {
        value.parse().map_err(E::custom)
    }

    fn visit_string<E: serde::de::Error>(self, value: String) -> Result<Self::Value, E> {
        value.parse().map_err(E::custom)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
impl<'de> Deserialize<'de> for Digest {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_str(Visitor)
    }
}
