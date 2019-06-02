# oci-spec
oci-spec is a library for Serializer/Deserialiser [oci-runtime-spec](https://github.com/opencontainers/runtime-spec) and [oci-image-spec](https://github.com/opencontainers/image-spec) config.

# Usage
```
extern crate oci_spec;

use oci_spec::runtime::Spec;

fn main() {
    let spec = match Spec::load("config.json") {
        Ok(s) => s,
        Err(e) => panic!("{}", e),
    }
    println!("oci-runtime-spec version is {}", spec.version);
}
```

# License
oci-spec is under the MIT or Apache-2.0 license.
