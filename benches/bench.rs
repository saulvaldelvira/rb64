#![feature(test)]
extern crate test;
use std::io::Read;
use std::u8;

use rb64::Base64Encoder;
use test::Bencher;

fn get_buffer_for_encode() -> Vec<u8> {
    let mut s = Vec::new();
    for _ in 0..1000 {
        for i in 0..u8::MAX {
            s.push(i);
        }
    }
    s
}

#[bench]
fn bench_encode(b: &mut Bencher) {
    let s = get_buffer_for_encode();
    b.iter(|| {
        rb64::encode(&s);
    });
}

#[bench]
fn bench_decode(b: &mut Bencher) {
    let s = get_buffer_for_encode();
    let enc = rb64::encode(&s);
    b.iter(|| {
        let d = rb64::decode(&enc).unwrap();
        for c in d.chunks(4) {
            println!("{c:?}");
        }
    })
}

#[bench]
fn bench_encode_reader(b: &mut Bencher) {
    let s = get_buffer_for_encode();
    b.iter(|| {
        let mut reader = Base64Encoder::new(&s[..]);
        let mut tmp = [0_u8; 4];

        while reader.read(&mut tmp).unwrap() > 0 {
            println!("{tmp:?}");
        }
    });
}
