#![cfg(feature = "hasher")]

use std::unreachable;

use oci_digest::{Digest, Hasher};

#[test]
fn works() {
    let digest: Digest = "sha256:2c26b46b68ffc68ff99b453c1d30413413422d706483bfa0f98a5e886266e7ae"
        .parse()
        .unwrap();

    assert_eq!(digest, digest.hasher().chain("foo".as_bytes()).finish());
}

#[test]
fn default() {
    match Hasher::default() {
        Hasher::Sha256(_) => (),
        _ => unreachable!(),
    }
}
