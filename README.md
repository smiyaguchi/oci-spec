# oci-spec
[![oci-spec at crates.io](https://img.shields.io/crates/v/oci-spec.svg)](https://crates.io/crates/oci-spec)
[![oci-spec at docs.rs](https://docs.rs/oci-spec/badge.svg)](https://docs.rs/oci-spec)

oci-spec is a library for Serialize/Deserialize [oci-runtime-spec](https://github.com/opencontainers/runtime-spec) and [oci-image-spec](https://github.com/opencontainers/image-spec) config.

# Usage
## Deserialize oci-runtime-spec
Use `load` of `Spec` to deserialize oci-runtime-spec config.

If it is a required item in oci-runtime-spec but there is no value in JSON, an error will occur.
Also if there is no JSON value in the optional item in oci-runtime-spec, None will be set.

For example, `ociVersion` is a required item in oci-runtime-spec, but if `ociVersion` does not exist in JSON, an error occurs.

Although `root` is an object type and internally has a required `path`, no error occurs even if there is no value in JSON.
However, if `root` has a value in JSON and the value of the required item held internally is not in JSON, an error occurs.

Example:
```rust
extern crate oci_spec;

use oci_spec::runtime::Spec;

fn main() {
    // Deserialize oci runtime specification config.
    let spec = match Spec::load("config.json") {
        Ok(s) => s,
        Err(e) => panic!("{}", e),
    }
    println!("oci-runtime-spec version is {}", spec.version);
}
```

## Serialize oci-runtime-spec
Use `save` of `Spec` to serialize oci runtime spec config.

If the type is Option and the value is None, it is not serialized.

Example:
```rust
extern crate oci_spec;

use oci_spec::runtime::Spec;

fn main() {
    // Deserialize oci runtime specification config.
    let spec = match Spec::load("config.json") {
        Ok(s) => s,
        Err(e) => panic!("{}", e),
    }
    
    // Serialize oci runtime specification config.
    match Spec::save(&spec, "config.json") {
        Ok(_) => {},
        Err(e) => panic!("{}", e),
    }
}
```

# License
oci-spec is under the MIT or Apache-2.0 license.
