use rb64::*;

#[test]
fn hello_world() {
    let msg = b"Hello world!";
    let enc = encode(msg);
    let dec = decode(&enc).unwrap();
    assert_eq!(msg, &*dec);
}
