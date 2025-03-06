# Human Friendly Ids

[![CI](https://github.com/JosiahBull/human-friendly-ids-rs/actions/workflows/ci.yaml/badge.svg)](https://github.com/JosiahBull/human-friendly-ids-rs/actions/workflows/ci.yaml)
[![Crates.io](https://img.shields.io/crates/v/human_friendly_ids)](https://crates.io/crates/human_friendly_ids)
[![Docs.rs](https://docs.rs/human_friendly_ids/badge.svg)](https://docs.rs/human_friendly_ids)

This Rust library is used for generating random human-friendly ids with a base-23 character set. The
ids are generated using a cryptographically secure random number generator.

The primary reason you would use this library is to generate visually unambiguous ids that are easy
to read, write, and speak over the phone. The library automatically corrects for common visual
confusions such as `1` vs `l`, `0` vs `O`, and `5` vs `S`, `rn` vs `m`, etc.

There is some excellent prior art in this space such as:

- <https://gajus.com/blog/avoiding-visually-ambiguous-characters-in-ids>
- <https://www.crockford.com/base32.html>
- <https://github.com/google/open-location-code>

Note that this library makes no attempt to avoid creating offensive or inappropriate ids. If you
need to avoid generating such ids, you should filter them out after generating them.

## Getting Started

```toml
[dependencies]
human_friendly_ids = "0.1.1"
```

### Usage

```rust
use human_friendly_ids::{UploadId, UploadIdDist};
use rand::{Rng, distr::Distribution, thread_rng};

let mut rng = thread_rng();
let dist = UploadIdDist::<12>;

let id = dist.sample(&mut rng);
println!("Generated ID: {}", id);
```

## Contribution

If you would like to contribute to this project, please open an issue or a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

Please observe the [Unicode-3.0](https://www.unicode.org/license.txt) license for the relevant code
included in this library.
