#![cfg(feature = "serde")]

use oci_digest::Digest;

const DIGEST: &str = "sha256:2c26b46b68ffc68ff99b453c1d30413413422d706483bfa0f98a5e886266e7ae";

#[test]
fn encode() {
    let digest: Digest = DIGEST.parse().unwrap();
    let json = format!(r#""{}""#, DIGEST);

    assert_eq!(json, serde_json::to_string(&digest).unwrap());
}

#[test]
fn decode() {
    let digest: Digest = DIGEST.parse().unwrap();
    let json = format!(r#""{}""#, DIGEST);

    assert_eq!(digest, serde_json::from_str(&json).unwrap());
}
