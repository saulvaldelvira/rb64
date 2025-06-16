use std::io::{BufReader, Read};

use rb64::Base64Encoder;

#[test]
fn reader() {
    let buf = "abcdefg";
    let expected = "YWJjZGVmZw==";
    let mut reader = Base64Encoder::new(buf.as_bytes());

    let mut out = [0_u8; 5];
    let mut i = 0;
    while let Ok(n) = reader.read(&mut out) {
        if n == 0 {
            return;
        }
        assert_eq!(&expected.as_bytes()[i..i + n], &out[0..n]);
        i += n;
    }
}

#[test]
fn reader_huge() {
    let buf = b"a0bcdefghi".repeat(1000);
    let mut reader = Base64Encoder::new(BufReader::new(buf.as_slice()));

    let expected = rb64::encode(buf.as_slice());

    let mut out = Vec::new();
    reader.read_to_end(&mut out).unwrap();

    let out = String::from_utf8(out).unwrap();

    assert_eq!(expected.len(), out.len());
    assert_eq!(expected, &out[..]);
}
