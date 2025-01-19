use dbg_unreachable::unreachable;

use crate::prelude::String;

const TABLE: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '+', '/',
];

/// Encode the given byte array to a Base 64 String
///
/// # Example
/// ```
/// use rb64::encode;
///
/// let enc = encode(b"Hello world!");
/// assert_eq!(enc, "SGVsbG8gd29ybGQh");
/// ```
pub fn encode(bytes: &[u8]) -> String {
    let mut capacity = (bytes.len() / 3) * 4;
    if bytes.len() % 3 > 0 {
        capacity += 4;
    }
    let mut result = String::with_capacity(capacity);
    bytes.chunks(3).for_each(|chunk| {
        for c in encode_chunk(chunk) {
            if result.len() >= result.capacity() {
                unreachable!("The capacity will always be enough");
            }
            result.push(c);
        }
    });
    result
}

fn encode_chunk(bytes: &[u8]) -> [char; 4] {
    let mut buf = ['='; 4];
    macro_rules! set {
        ($i:expr, $e:expr) => {
            buf[$i] = TABLE[($e & 0b111111) as usize];
        };
    }

    set!(0, bytes[0] >> 2);
    if bytes.len() == 1 {
        set!(1, bytes[0] << 4);
        return buf;
    }
    set!(1, bytes[0] << 4 | bytes[1] >> 4);
    if bytes.len() == 2 {
        set!(2, bytes[1] << 2);
        return buf;
    }
    set!(2, bytes[1] << 2 | bytes[2] >> 6);
    set!(3, bytes[2]);
    buf
}

#[cfg(not(feature = "no-std"))]
#[doc(hidden)]
mod __encoder {
    use std::io::{Error, ErrorKind, Read};

    use super::encode_chunk;

    /// This is a Base 64 Encoder Reader
    ///
    /// It takes a [reader](Read) and converts it's
    /// output to Base 64.
    pub struct Base64Encoder<T: ?Sized + Read> {
        finished: bool,
        reader: T,
    }

    impl<T: Read> Base64Encoder<T> {
        /// Creates a [Base64Encoder] with a given [reader](Read)
        pub fn new(reader: T) -> Self {
            Self {
                reader,
                finished: false,
            }
        }
    }

    impl<T: ?Sized + Read> Read for Base64Encoder<T> {
        /// Read the next chunk of data into buf.
        ///
        /// When the reader has finished, returns 0.
        ///
        /// If the length of buf is less than 4, this
        /// function returns an Error.
        ///
        /// **TODO**: Fix this
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            if self.finished {
                return Ok(0);
            }
            if buf.len() < 4 {
                return Err(Error::new(ErrorKind::InvalidInput, "Buffer too small"));
            }
            let mut group = [0_u8; 3];
            let mut count = 0;

            while count < buf.len() / 4 && !self.finished {
                let n = self.reader.read(&mut group)?;
                if n == 0 {
                    self.finished = true;
                    break;
                }
                let chunk = encode_chunk(&group[0..n]);
                for i in 0..4 {
                    buf[count + i] = chunk[i] as u8;
                    if chunk[i] == '=' {
                        self.finished = true;
                    }
                }
                count += 4;
            }
            Ok(count)
        }
    }
}

#[cfg(not(feature = "no-std"))]
#[doc(inline)]
pub use __encoder::Base64Encoder;
