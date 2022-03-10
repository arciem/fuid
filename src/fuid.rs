use std::{fmt, str::FromStr};
use uuid::Uuid;
use super::base62;
#[cfg(feature = "serde")]
use serde::{de, Deserialize, Deserializer, Serialize};

/// A Friendly Universal Identifier (FUID).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Fuid(u128);

impl Fuid {
    /// Creates a new, random FUID.
    pub fn new() -> Fuid {
        Fuid(Uuid::new_v4().as_u128())
    }

    /// Creates a new FUID from the given string. FUID-compatible strings may
    /// include numerals and upper and lower case English letters.
    pub fn with_str(s: &str) -> Result<Fuid, base62::DecodeError> {
        match base62::decode(s) {
            Ok(n) => Ok(Fuid(n)),
            Err(e) => Err(e),
        }
    }

    /// Creates a new FUID from the given u128.
    pub fn with_u128(i: u128) -> Fuid {
        Self(i)
    }

    /// Returns the wrapped u128 value.
    pub fn as_u128(&self) -> u128 {
        self.0
    }

    /// Returns the Base62 encoding of the wrapped value.
    pub fn to_string(&self) -> String {
        base62::encode(self.0)
    }
}

impl fmt::Display for Fuid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl fmt::Debug for Fuid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Fuid")
            .field(&self.to_string())
            .finish()
    }
}

impl FromStr for Fuid {
    type Err = base62::DecodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Fuid::with_str(s)
    }
}

impl TryFrom<&str> for Fuid {
    type Error = base62::DecodeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Fuid::with_str(value)
    }
}

impl From<Fuid> for String {
    fn from(f: Fuid) -> Self {
        f.to_string()
    }
}

impl From<u128> for Fuid {
    fn from(i: u128) -> Self {
        Self::with_u128(i)
    }
}

impl From<Fuid> for u128 {
    fn from(f: Fuid) -> Self {
        f.as_u128()
    }
}

impl From<Uuid> for Fuid {
    fn from(u: Uuid) -> Self {
        u.as_u128().into()
    }
}

impl From<Fuid> for Uuid {
    fn from(f: Fuid) -> Self {
        Uuid::from_u128(f.as_u128())
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Fuid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Fuid::with_str(&s).map_err(de::Error::custom)
    }
}

#[cfg(feature = "serde")]
impl Serialize for Fuid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
