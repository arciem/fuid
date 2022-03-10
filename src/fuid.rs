use std::{fmt, str::FromStr};
use uuid::Uuid;
use super::base62;
#[cfg(feature = "serde")]
use serde::{de, Deserialize, Deserializer, Serialize};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Fuid(pub u128);

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

    pub fn as_u128(&self) -> u128 {
        self.0
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

impl FromStr for Fuid {
    type Err = base62::DecodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Fuid::with_string(s)
    }
}

impl TryFrom<&str> for Fuid {
    type Error = base62::DecodeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Fuid::with_string(value)
    }
}

impl From<Fuid> for String {
    fn from(f: Fuid) -> Self {
        base62::encode(f.0)
    }
}

impl From<u128> for Fuid {
    fn from(i: u128) -> Self {
        Self::with_int(i)
    }
}

impl From<Fuid> for u128 {
    fn from(f: Fuid) -> Self {
        f.0
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
