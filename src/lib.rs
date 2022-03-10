#![warn(rust_2018_idioms)]

mod base62;
pub mod fuid;

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
