//! Generate and parrse Friendly Universal Identifiers (FUIDs)
//!
//! Here is an example of a FUID:
//!
//! ```text
//! 6fTiplVKIi6bJFe8rTXPcu
//! ```
//!
//! FUIDs are wrapped 128-bit unsigned integers, which gives them the same
//! resolution as a UUID. FUIDs are serialized to Base62, which is a sequence of
//! digits and alphanumerics. This makes them easier to handle than normal UUID
//! encoding, yet when generated randomly they use a UUID generation algorithm
//! and are therefore isomorphich with UUIDs. One advantage of using FUIDs is
//! that they can be converted from and to short strings that can stand in as
//! human-readable identifiers for testing purposes, but can be full-length
//! random numbers for production purposes.
//!
//! # Getting Started

//! Add the following to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies.fuid]
//! version = "1.0.0"
//! features = [
//!     "serde",                # Serde support
//! ]
//! ```
//!
//! When you want a random FUID, you can generate one:
//!
//! ```
//! # fn main() {
//! # {
//! use fuid::Fuid;
//!
//! let id = Fuid::new();
//! # }
//! # }
//! ```
//!
//! You can convert short strings to and from FUIDs. FUID-compatible strings can
//! include numerals and upper and lower case English letters.
//!
//! ```
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! # {
//! use fuid::Fuid;
//!
//! let s = "A";
//! let id: Fuid = s.try_into()?; // Not all strings are valid FUIDs.
//! let s2: String = id.into();
//! assert_eq!(s, s2);
//! # Ok(())
//! # }
//! # }
//! ```
//!
//! You can convert unsigned integers to and from FUIDs.
//!
//! ```
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! # {
//! use fuid::Fuid;
//!
//! let s: u128 = 10;
//! let id: Fuid = s.into();
//! let s2: u128 = id.into();
//! assert_eq!(s, s2);
//! # Ok(())
//! # }
//! # }
//! ```
//!
//!

#![warn(rust_2018_idioms)]

mod base62;
mod fuid;

pub use crate::base62::*;
pub use crate::fuid::*;

#[cfg(test)]
mod tests {
    use crate::fuid::Fuid;

    #[test]
    fn test_fuid() -> Result<(), Box<dyn std::error::Error>> {
        let a = "6fTiplVKIi6bJFe8rTXPcu";
        let b = "5z1JeaxqBJ4Y3pEXh2B8Sj";

        let fa = Fuid::with_string(a)?;
        let fb = Fuid::with_string(b)?;

        assert_eq!(fa.to_string(), a);
        assert_eq!(fb.to_string(), b);

        assert_ne!(Fuid::new(), fa);
        assert_ne!(Fuid::new(), fb);

        assert!(Fuid::with_string("ab!").is_err());

        Ok(())
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serde() -> Result<(), Box<dyn std::error::Error>> {
        use serde_json::{to_string, from_str};

        let a = Fuid::new();
        let b = to_string(&a)?;
        println!("{}", b);
        let c: Fuid = from_str(&b)?;
        assert_eq!(a, c);

        Ok(())
    }
}
