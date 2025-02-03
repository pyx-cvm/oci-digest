# oci-digest

OCI Container Digests

This crate provides types for working with OCI container image digests.

## Features

- `std`: Enables standard library integration
- `serde`: Enables serde serialization/deserialization
- `hasher`: Enables hashing functionality

## Examples

Basic digest parsing:

```rust
use oci_digest::Digest;

let input = "sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

match input.parse().unwrap() {
    Digest::Sha256(..) => (),
    _ => unreachable!(),
}
```
