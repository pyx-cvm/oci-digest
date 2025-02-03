extern crate std;

use std::string::ToString;

use oci_digest::{Digest, Error};

// Test default to sha256 when no algorithm is specified.
#[test]
fn compat() {
    const LOWER: &str = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";
    const BYTES: [u8; 32] = [
        0xe3, 0xb0, 0xc4, 0x42, 0x98, 0xfc, 0x1c, 0x14, 0x9a, 0xfb, 0xf4, 0xc8, 0x99, 0x6f, 0xb9,
        0x24, 0x27, 0xae, 0x41, 0xe4, 0x64, 0x9b, 0x93, 0x4c, 0xa4, 0x95, 0x99, 0x1b, 0x78, 0x52,
        0xb8, 0x55,
    ];

    let digest = Digest::Sha256(BYTES);

    assert_eq!(digest, LOWER.parse().unwrap());
    assert_eq!(digest.to_string().split_once(':'), Some(("sha256", LOWER)));
}

#[test]
fn sha256() {
    const LOWER: &str = "sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";
    const BYTES: [u8; 32] = [
        0xe3, 0xb0, 0xc4, 0x42, 0x98, 0xfc, 0x1c, 0x14, 0x9a, 0xfb, 0xf4, 0xc8, 0x99, 0x6f, 0xb9,
        0x24, 0x27, 0xae, 0x41, 0xe4, 0x64, 0x9b, 0x93, 0x4c, 0xa4, 0x95, 0x99, 0x1b, 0x78, 0x52,
        0xb8, 0x55,
    ];

    let digest = Digest::Sha256(BYTES);

    assert_eq!(digest, LOWER.parse().unwrap());
    assert_eq!(digest.to_string(), LOWER);
}

#[test]
fn sha384() {
    const LOWER: &str = "sha384:38b060a751ac96384cd9327eb1b1e36a21fdb71114be07434c0cc7bf63f6e1da274edebfe76f65fbd51ad2f14898b95b";
    const BYTES: [u8; 48] = [
        0x38, 0xb0, 0x60, 0xa7, 0x51, 0xac, 0x96, 0x38, 0x4c, 0xd9, 0x32, 0x7e, 0xb1, 0xb1, 0xe3,
        0x6a, 0x21, 0xfd, 0xb7, 0x11, 0x14, 0xbe, 0x07, 0x43, 0x4c, 0x0c, 0xc7, 0xbf, 0x63, 0xf6,
        0xe1, 0xda, 0x27, 0x4e, 0xde, 0xbf, 0xe7, 0x6f, 0x65, 0xfb, 0xd5, 0x1a, 0xd2, 0xf1, 0x48,
        0x98, 0xb9, 0x5b,
    ];

    let digest = Digest::Sha384(BYTES);

    assert_eq!(digest, LOWER.parse().unwrap());
    assert_eq!(digest.to_string(), LOWER);
}

#[test]
fn sha512() {
    const LOWER: &str = "sha512:cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e";
    const BYTES: [u8; 64] = [
        0xcf, 0x83, 0xe1, 0x35, 0x7e, 0xef, 0xb8, 0xbd, 0xf1, 0x54, 0x28, 0x50, 0xd6, 0x6d, 0x80,
        0x07, 0xd6, 0x20, 0xe4, 0x05, 0x0b, 0x57, 0x15, 0xdc, 0x83, 0xf4, 0xa9, 0x21, 0xd3, 0x6c,
        0xe9, 0xce, 0x47, 0xd0, 0xd1, 0x3c, 0x5d, 0x85, 0xf2, 0xb0, 0xff, 0x83, 0x18, 0xd2, 0x87,
        0x7e, 0xec, 0x2f, 0x63, 0xb9, 0x31, 0xbd, 0x47, 0x41, 0x7a, 0x81, 0xa5, 0x38, 0x32, 0x7a,
        0xf9, 0x27, 0xda, 0x3e,
    ];

    let digest = Digest::Sha512(BYTES);

    assert_eq!(digest, LOWER.parse().unwrap());
    assert_eq!(digest.to_string(), LOWER);
}

#[test]
fn bad_algorithm() {
    const BAD: &str = "sha255:0000000000000000000000000000000000000000000000000000000000000000";
    assert_eq!(Err(Error::Algorithm), BAD.parse::<Digest>());
}

#[test]
fn bad_length() {
    const BAD: &str = "sha512:0000000000000000000000000000000000000000000000000000000000000000";
    assert_eq!(Err(Error::Length), BAD.parse::<Digest>());
}

#[test]
fn bad_character() {
    const BAD: &str = "sha256:XX00000000000000000000000000000000000000000000000000000000000000";
    assert_eq!(Err(Error::Character), BAD.parse::<Digest>());
}

#[test]
fn bad_uppercase_algo() {
    const BAD: &str = "SHA256:E3B0C44298FC1C149AFBF4C8996FB92427AE41E4649B934CA495991B7852B855";
    assert_eq!(Err(Error::Algorithm), BAD.parse::<Digest>());
}

#[test]
fn bad_uppercase_hash() {
    const BAD: &str = "sha256:E3B0C44298FC1C149AFBF4C8996FB92427AE41E4649B934CA495991B7852B855";
    assert_eq!(Err(Error::Character), BAD.parse::<Digest>());
}
