# assert-tokenstreams-eq

A utility for comparing token streams in tests. It applies `rustfmt` to the
token streams to ensure consistent formatting and leverages `pretty_assertions`
to clearly visualize differences when the assertion fails.

<p align="">
    <a href="https://rustup.rs"><img alt="MSRV" src="https://img.shields.io/badge/Rust-1.83.0%2B-orange.svg"></a>
    <a href="https://img.shields.io/crates/l/assert-tokenstreams-eq"><img alt="License: MIT OR Apache-2.0" src="https://img.shields.io/crates/l/assert-tokenstreams-eq"></a>
</p>

## Usage

Add this to your `Cargo.toml`:

```toml
[dev-dependencies]
assert-tokenstreams-eq = "0.1"
quote = "1.0"
```

Then use the macro in your tests:

```rust
use assert_tokenstreams_eq::assert_tokenstreams_eq;
use quote::quote;

#[test]
fn test_code_generation() {
    let expected = quote! {
        fn test(a: String, b: String) {
            return a;
        }
    };

    let actual = quote! {
        fn   test  ( a :String,b:String){return a ;}
    };

    assert_tokenstreams_eq!(&expected, &actual);
}
```

## How It Works

1. Both token streams are formatted using `rustfmt`. This handles differences in
   whitespace, indentation, and equivalent syntax.
2. The formatted strings are compared.
3. If they differ, `pretty_assertions` prints a colorful diff showing exactly
   where the mismatch occurred.

## Prerequisites

This crate relies on `rustfmt` being installed and available in your path.

```sh
rustup component add rustfmt
```
