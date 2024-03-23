#[cfg(feature = "std")]
pub mod with_std {
    pub use std::{fmt, str::FromStr};

    pub use std::string::String;
    pub use std::vec::Vec;
    pub use std::error::Error;
    pub use std::string::ToString;
    pub use std::borrow::ToOwned;
    pub use std::boxed::Box;
    pub use std::format;
}

#[cfg(not(feature = "std"))]
pub mod without_std {
    extern crate alloc;

    pub use core::fmt::{self};
    pub type String = alloc::string::String;
    pub type Vec<T> = alloc::vec::Vec<T>;
    pub type Box<T> = alloc::boxed::Box<T>;
    pub use alloc::string::ToString;
    pub use alloc::str::FromStr;
    pub use alloc::borrow::ToOwned;
    pub use alloc::format;

    pub trait Error: fmt::Debug + fmt::Display { }

    impl<T> From<T> for Box<dyn Error>
    where
        T: Error + 'static,
    {
        fn from(err: T) -> Box<dyn Error> {
            Box::new(err)
        }
    }
}

macro_rules! import_stdlib {
    () => {
        #[cfg(feature = "std")]
        pub use $crate::stdlib::with_std::*;
        #[cfg(not(feature = "std"))]
        pub use $crate::stdlib::without_std::*;
    };
}
