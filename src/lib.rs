//! Generate and parse Friendly Universal Identifiers (FUIDs)
//!
//! Here is an example of a FUID:
//!
//! ```text
//! 6fTiplVKIi6bJFe8rTXPcu
//! ```
//!
//! FUIDs are wrapped 128-bit unsigned integers, which gives them the same
//! resolution as UUIDs. FUIDs are serialized to Base62, which is a sequence of
//! digits and letters. This makes them shorter and easier to handle than normal
//! UUID encoding, yet when generated randomly they use a UUID generation
//! algorithm and are therefore isomorphic with UUIDs. One advantage of using
//! FUIDs is that they can be converted from and to short strings that can stand
//! in as human-readable identifiers for testing purposes, but can be
//! full-length random numbers for production purposes.
//!
//! # Getting Started
//!
//! Add the following to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies.fuid]
//! version = "1.2.1"
//! features = ["serde"]        # Optional: Enable Serde support
//! ```
//!
//! # `no_std` Support
//!
//! `fuid` supports `no_std` environments. In the `no_std` environment, the
//! `alloc` crate will be used.
//!
//! ```toml
//! [dependencies.fuid]
//! version = "1.2.1"
//! default-features = false    # Disable default features
//! features = ["serde"]        # Optional: Enable Serde support with no_std
//! ```
//!
//! # Usage
//!
//! When you want a random FUID, you can generate one:
//!
//! ```
//! # fn main() {
//! # {
//! use fuid::Fuid;
//!
//! for _ in 0..10 {
//!     let id = Fuid::new();
//!     println!("{}", id);
//! }
//! # }
//! # }
//! ```
//!
//! Example output:
//!
//! ```text
//! 2OgGZdF0XpQ7nuoC99CDxX
//! 4YX6PmtlWBkhMEyyc6G7Oz
//! 7ZxST7AIFUpG9EgnD9CgI7
//! 4XCFe4KvZjQBfb4sH6Ybqd
//! 5tmyQuvYZyJ6vOWCQLEqkB
//! 3RcWxQsqs8mWIpn1RQIVJc
//! 7MlBR5pJakurAhNRr4LwfZ
//! 6WKpT00vD4BJ3vZpEdZaZw
//! 4SxascPNNmBN1jFQYCtWpr
//! 3k9FL4LZe71geQdbOyCvz3
//! ```
//!
//! You can convert short strings to and from FUIDs. FUID-compatible strings may
//! include numerals and upper and lower case English letters.
//!
//! ```
//! # fn main() {
//! # {
//! # use std::str::FromStr;
//! use fuid::Fuid;
//!
//! let s = "A";
//! let id = Fuid::from_str(s).unwrap(); // Not all strings are valid FUIDs.
//! let s2: String = id.into();
//! assert_eq!(s2, s);
//! let id2: Fuid = s.try_into().unwrap();
//! let s3: String = id2.into();
//! assert_eq!(s3, s);
//! # }
//! # }
//! ```
//!
//! You can convert unsigned integers to and from FUIDs.
//!
//! ```
//! # fn main() {
//! # {
//! use fuid::Fuid;
//!
//! let n: u128 = 10;
//! let id: Fuid = n.into();
//! let n2: u128 = id.into();
//! assert_eq!(n, n2);
//! # }
//! # }
//! ```
//!
//! You can use the `fuid!` macro to easily convert literals into FUIDs.
//!
//! ```
//! # fn main() {
//! # {
//! use fuid::fuid;
//!
//! let a = fuid!("A");
//! assert_eq!(String::from(a), "A");
//!
//! let b = fuid!(1);
//! assert_eq!(b.as_u128(), 1);
//! # }
//! # }
//! ```
//!
//! You can convert UUIDs to and from FUIDs.
//!
//! ```
//! # fn main() {
//! # {
//! # use std::str::FromStr;
//! use fuid::Fuid;
//! use uuid::Uuid;
//!
//! let f = "3k9FL4LZe71geQdbOyCvz3";
//! let u = "7b06fb9f-cb59-4c6d-a38c-028d27193acd";
//! let fuid = Fuid::from_str(f).unwrap();
//! let uuid = Uuid::from_str(u).unwrap();
//! assert_eq!(fuid, uuid.into());
//! assert_eq!(uuid, fuid.into());
//! # }
//! # }
//! ```

#![cfg_attr(not(feature = "std"), no_std)]
// #![warn(clippy::std_instead_of_core, clippy::alloc_instead_of_core, clippy::std_instead_of_alloc)]
// #![no_std]

#![warn(rust_2018_idioms)]

#[macro_use]
mod stdlib;
mod base62;
mod fuid;

pub use crate::base62::*;
pub use crate::fuid::*;

#[cfg(test)]
mod tests {
    use super::{fuid, Fuid};
    import_stdlib!();

    #[test]
    #[cfg(feature = "std")]
    fn test_std() {
        // panic!("Compiled with std");
    }

    #[test]
    #[cfg(not(feature = "std"))]
    fn test_no_std() {
        // panic!("Compiled with std");
    }

    #[test]
    fn test_fuid() -> Result<(), Box<dyn Error>> {
        let a = "6fTiplVKIi6bJFe8rTXPcu";
        let b = "5z1JeaxqBJ4Y3pEXh2B8Sj";

        let fa = Fuid::with_str(a)?;
        let fb = Fuid::with_str(b)?;

        assert_eq!(fa.to_string(), a);
        assert_eq!(fb.to_string(), b);

        assert_ne!(Fuid::new(), fa);
        assert_ne!(Fuid::new(), fb);

        assert!(Fuid::with_str("ab!").is_err());

        let _: Fuid = "A".into();
        let _: Fuid = "A".to_string().into();

        Ok(())
    }

    #[test]
    fn test_macro() -> Result<(), Box<dyn Error>> {
        let a = fuid!("A");
        assert_eq!(String::from(a), "A");

        let b = fuid!(1);
        assert_eq!(b.as_u128(), 1);

        Ok(())
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serde() -> Result<(), Box<dyn Error>> {
        use serde_json::{to_string, from_str};

        let a = Fuid::new();
        let b = to_string(&a)?;
        let c: Fuid = from_str(&b)?;
        assert_eq!(a, c);

        Ok(())
    }
}
