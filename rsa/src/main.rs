extern crate num;

use num::bigint::ToBigInt;
use num::pow;
use num::Integer;
use num::ToPrimitive;

fn main() {
    // let k = 8;
    let p = 13;
    let q = 17;
    let n = p * q;
    let fin = (p - 1) * (q - 1);
    let e: usize = 7;
    let d = euclid_ext(fin, e as i32);
    let d = d as usize;

    let encoded = encode("こんにちは".to_string(), (e, n as usize));
    println!("{:?}", "こんにちは".as_bytes());
    println!("encoded");
    let decoded = decode(encoded, (d, n as usize));
    println!("decoded = {}", decoded);
}

fn encode(msg: String, pubkey: (usize, usize)) -> Vec<u8> {
    let vec: Vec<u8> = msg.into_bytes();
    let mut e = vec![];
    for v in vec {
        let v = v.to_bigint().unwrap();
        let n = pubkey.1.to_bigint().unwrap();
        let v = pow(v, pubkey.0).mod_floor(&n);
        let v: u8 = v.to_u8().unwrap();
        e.push(v);
    }
    e
}

fn decode(enc: Vec<u8>, private_key: (usize, usize)) -> String {
    let mut d = vec![];
    for v in enc {
        let v = v.to_bigint().unwrap();
        println!("orig v = {}", v);
        let n = private_key.1.to_bigint().unwrap();
        let v = pow(v, private_key.0);
        println!("pow {}", v);
        let v = v.mod_floor(&n);
        println!("mod {}", v);
        let v: u8 = v.to_u8().unwrap();
        d.push(v);
    }
    String::from_utf8(d).unwrap()
}

fn euclid_ext(a: i32, b: i32) -> i32 {
    let mut x1 = 1;
    let mut y1 = 0;
    let mut z1 = a;
    let mut x2 = 0;
    let mut y2 = 1;
    let mut z2 = b;

    while z2 != 1 {
        if z2 == 0 {
            break;
        }
        let q = (z1 - (z1 % z2)) / z2;
        x1 = x1 - (q * x2);
        y1 = y1 - (q * y2);
        z1 = z1 - (q * z2);

        let t = x1;
        x1 = x2;
        x2 = t;

        let t = y1;
        y1 = y2;
        y2 = t;

        let t = z1;
        z1 = z2;
        z2 = t;
    }

    if y2 < 0 {
        y2 + a
    } else {
        y2
    }
}
