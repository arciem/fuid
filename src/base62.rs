import_stdlib!();

pub use self::DecodeError::*;

// Based on code from https://github.com/fbernier/base62

#[derive(Debug)]
pub enum DecodeError {
    InvalidBase62Byte(char, usize),
    ArithmeticOverflow
}

impl Error for DecodeError {
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

const BASE: u128 = 62;
const ALPHABET: [u8; BASE as usize] = [
    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9',
    b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J',
    b'K', b'L', b'M', b'N', b'O', b'P', b'Q', b'R', b'S', b'T',
    b'U', b'V', b'W', b'X', b'Y', b'Z', b'a', b'b', b'c', b'd',
    b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n',
    b'o', b'p', b'q', b'r', b's', b't', b'u', b'v', b'w', b'x',
    b'y', b'z'
    ];

pub fn encode(mut num: u128) -> String {
    if num == 0 {
        return "0".to_owned();
    }
    let mut bytes = Vec::new();

    while num > 0 {
        bytes.push(ALPHABET[(num % BASE) as usize]);
        num /= BASE
    }
    bytes.reverse();

    String::from_utf8(bytes).unwrap()
}

pub fn decode(string: &str) -> Result<u128, DecodeError> {
    let mut result = 0;

    for (i, c) in string.as_bytes().iter().rev().enumerate() {
        let num = BASE.pow(i as u32);
        match ALPHABET.binary_search(c) {
            Ok(v) => {
                match (v as u128).checked_mul(num) {
                    Some(z) => result += z,
                    None => return Err(ArithmeticOverflow),
                }
            }
            Err(_e) => {
                return Err(InvalidBase62Byte(c.to_owned() as char, string.len() - i));
            }
        }
    }

    Ok(result)
}


#[cfg(test)]
mod tests {
    use super::*;
    import_stdlib!();

    #[test]
    fn test_encode() {
        assert_eq!(encode(852751187393), "F0ob4rZ");
    }

    #[test]
    fn test_decode() -> Result<(), Box<dyn Error>> {
        assert_eq!(decode("F0ob4rZ")?, 852751187393);
        Ok(())
    }

    #[test]
    fn test_decode_invalid_char() {
        assert!(decode("ds{Z455f").is_err());
        assert_eq!(format!("{}", decode("ds{Z455f").unwrap_err()), "InvalidBase62Byte('{', 3)");
    }

    #[test]
    fn test_decode_long_string() {
        assert!(decode("dsZ455fzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz\
                                zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz")
            .is_err());
    }
}
