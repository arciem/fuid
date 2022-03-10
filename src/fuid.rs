use std::fmt;
use uuid::Uuid;
use super::base62;
#[cfg(feature = "serde")]
use serde::{de, Deserialize, Deserializer, Serialize};

/// Friendly Universal Identifier (FUID)
///
/// FUIDs are wrapped 128-bit unsigned integers, which gives them the same
/// resolution as a UUID. Fuids are serialized to Base62, which is a sequence of
/// digits and alphanumerics. This makes them easier to handle than normal UUID
/// encoding, yet when generated randomly they use a UUID generation algorithm
/// and are therefore isomorphich with UUIDs. One advantage of using FUIDs is
/// that they can be converted from and to short strings that can stand in as
/// human-readable identifiers for testing purposes, but can be full-length
/// random numbers for production purposes.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Fuid(u128);

impl Fuid {
    pub fn new() -> Fuid {
        Fuid(Uuid::new_v4().as_u128())
    }

    pub fn with_string(s: &str) -> Result<Fuid, base62::DecodeError> {
        match base62::decode(s) {
            Ok(n) => Ok(Fuid(n)),
            Err(e) => Err(e),
        }
    }

    pub fn with_int(i: u128) -> Fuid {
        Self(i)
    }
}

impl fmt::Display for Fuid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", base62::encode(self.0))
    }
}

impl fmt::Debug for Fuid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Fuid")
            .field(&base62::encode(self.0))
            .finish()
    }
}

impl TryFrom<&str> for Fuid {
    type Error = base62::DecodeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Fuid::with_string(value)
    }
}

impl From<u128> for Fuid {
    fn from(i: u128) -> Self {
        Self::with_int(i)
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Fuid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Fuid::with_string(&s).map_err(de::Error::custom)
    }
}

#[cfg(feature = "serde")]
impl Serialize for Fuid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&base62::encode(self.0))
    }
}
