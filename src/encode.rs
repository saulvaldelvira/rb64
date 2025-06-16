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

#[cfg(feature = "std")]
#[doc(inline)]
pub use encoder::Base64Encoder;

#[cfg(feature = "std")]
mod encoder {
    use std::io::{BufRead, Read};

    use super::encode_chunk;

    /// This is a Base 64 Encoder Reader
    ///
    /// It takes a [reader](Read) and converts it's
    /// output to Base 64.
    pub struct Base64Encoder<T: ?Sized + BufRead> {
        chunk: [u8; 4],
        offset: usize,
        reader: T,
    }

    impl<T: BufRead> Base64Encoder<T> {
        /// Creates a [Base64Encoder] with a given [reader](Read)
        pub fn new(reader: T) -> Self {
            Self {
                reader,
                offset: 4,
                chunk: [0; 4],
            }
        }
    }

    impl<T: ?Sized + BufRead> Read for Base64Encoder<T> {
        /// Read the next chunk of data into buf.
        ///
        /// When the reader has finished, returns 0.
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            let mut count = 0;
            while self.offset < 4 {
                buf[count] = self.chunk[self.offset];
                self.offset += 1;
                count += 1;
                if count == buf.len() {
                    return Ok(count)
                }
            }

            let mut slice = [0; 3];

            #[inline(always)]
            fn fill_chunk<R: BufRead + ?Sized>(r: &mut R, slice: &mut [u8; 3]) -> std::io::Result<usize> {
                let mut len = 0;
                while len < 3 {
                    let iter = r.read(&mut slice[len..])?;
                    len += iter;
                    if len == 3 || iter == 0 { break }
                }
                Ok(len)
            }

            while count < buf.len() / 4 {
                let len = fill_chunk(&mut self.reader, &mut slice)?;
                if len == 0 { return Ok(count) }

                let chunk = encode_chunk(&slice[..len]);
                for c in chunk {
                    buf[count] = c as u8;
                    count += 1;
                }
            }

            let rem = buf.len() % 4;
            if rem > 0 {
                let len = fill_chunk(&mut self.reader, &mut slice)?;
                if len == 0 { return Ok(count) }

                let slice = &mut slice[..len];
                self.reader.read_exact(slice)?;

                for (i, c) in encode_chunk(slice).iter().enumerate() {
                    self.chunk[i] = *c as u8;
                }
                self.offset = 0;
                buf[count..(count + rem)].copy_from_slice(&self.chunk[..rem]);
                self.offset += rem;
                count += rem;
            }

            Ok(count)
        }
    }
}

