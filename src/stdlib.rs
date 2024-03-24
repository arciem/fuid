#[cfg(feature = "std")]
pub mod with_std {
    pub use std::{fmt, str::FromStr};

    pub use std::string::String;
    pub use std::vec::Vec;
    pub use std::error::Error;
    pub use std::string::ToString;
    pub use std::borrow::ToOwned;
}

#[cfg(not(feature = "std"))]
pub mod without_std {
    extern crate alloc;

    pub use core::fmt::{self};
    pub use alloc::string::{String, ToString};
    pub use alloc::vec::Vec;
    pub use alloc::boxed::Box;
    pub use alloc::str::FromStr;
    pub use alloc::borrow::ToOwned;
    #[allow(unused_imports)]
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
        #[allow(unused_imports)]
        use $crate::stdlib::with_std::*;
        #[cfg(not(feature = "std"))]
        #[allow(unused_imports)]
        use $crate::stdlib::without_std::*;
    };
}
