use crate::unreachable;

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
    if !bytes.len().is_multiple_of(3) {
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

#[cfg(feature = "std")]
#[doc(inline)]
pub use encoder::Base64Encoder;

#[cfg(feature = "std")]
mod encoder {
    use std::io::{self, BufRead, Bytes, Read};

    use super::encode_chunk;

    /// This is a Base 64 Encoder Reader
    ///
    /// It takes a [reader](Read) and converts it's
    /// output to Base 64.
    pub struct Base64Encoder<T: BufRead> {
        chunk: [u8; 4],
        offset: usize,
        reader: Bytes<T>,
    }

    impl<T: BufRead> Base64Encoder<T> {
        /// Creates a [Base64Encoder] with a given [reader](Read)
        pub fn new(reader: T) -> Self {
            Self {
                reader: reader.bytes(),
                offset: 4,
                chunk: [0; 4],
            }
        }

        fn try_read_exact(&mut self, slice: &mut [u8]) -> io::Result<std::result::Result<(), usize>> {
            let mut num = 0;
            macro_rules! next {
                () => {{
                    num += 1;
                    let Some(b) = self.reader.next() else {
                        return Ok(Err(num - 1))
                    };
                    b?
                }};
            }

            slice[0] = next!();
            slice[1] = next!();
            slice[2] = next!();

            Ok(Ok(()))
        }
    }

    impl<T: BufRead> Read for Base64Encoder<T> {
        /// Read the next chunk of data into buf.
        ///
        /// When the reader has finished, returns 0.
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            let mut count = 0;
            'main: while buf.len() - count > 0 {
                if self.offset == 4 {
                    let mut slice = [0; 3];
                    let len = match self.try_read_exact(&mut slice)? {
                        Ok(()) => 3,
                        Err(0) => break 'main,
                        Err(n) => n,
                    };
                    for (i, c) in encode_chunk(&slice[..len]).iter().enumerate() {
                        self.chunk[i] = *c as u8;
                    }
                    self.offset = 0;
                }
                while self.offset < 4 {
                    buf[count] = self.chunk[self.offset];
                    self.offset += 1;
                    count += 1;
                    if count == buf.len() {
                        break 'main
                    }
                }
            }
            Ok(count)
        }
    }
}

