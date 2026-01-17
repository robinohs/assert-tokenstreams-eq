use std::{
    io::Write,
    process::{Command, Stdio},
    string::FromUtf8Error,
};

use pretty_assertions::assert_eq;
use thiserror::Error;

#[derive(Error, Debug)]
enum RustfmtError {
    #[error("Could not create child process for rustfmt.")]
    Io(#[from] std::io::Error),
    #[error("Could not convert output bytes to UTF-8.")]
    Utf8(#[from] FromUtf8Error),
    #[error("Could not read from rustfmt child process.")]
    Output,
    #[error("Could not access stdin of rustfmt child process.")]
    Stdin,
    #[error("Input is not valid Rust code.\n{0}")]
    InvalidRust(String),
}

/// # Panics
///
/// This function will panic if:
/// - The token streams are not equal.
/// - Either token stream is not valid Rust code.
/// - `rustfmt` is not installed or is configured incorrectly.
/// - It was not possible to create a child process to run `rustfmt`.
/// - The output of `rustfmt` could not be converted to UTF-8.
pub fn compare_tokenstreams(
    first_tokenstream: &impl std::fmt::Display,
    second_tokenstream: &impl std::fmt::Display,
) {
    let first_formatted = match apply_rustfmt(first_tokenstream) {
        Ok(tokens) => tokens,
        Err(e) => panic!("Error formatting first token stream: {e}"),
    };
    let second_formatted = match apply_rustfmt(second_tokenstream) {
        Ok(tokens) => tokens,
        Err(e) => panic!("Error formatting second token stream: {e}"),
    };
    assert_eq!(first_formatted, second_formatted);
}

fn apply_rustfmt(tokens: &impl std::fmt::Display) -> Result<String, RustfmtError> {
    let mut process = Command::new("rustfmt")
        .arg("--")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let Some(stdin) = process.stdin.as_mut() else {
        return Err(RustfmtError::Stdin);
    };

    write!(stdin, "{tokens}")?;

    let output = process
        .wait_with_output()
        .map_err(|_| RustfmtError::Output)?;

    if output.status.success() {
        let fmt_tokens = String::from_utf8(output.stdout)?;
        Ok(fmt_tokens)
    } else {
        let err = String::from_utf8(output.stderr)?;
        Err(RustfmtError::InvalidRust(err))
    }
}

#[macro_export]
macro_rules! assert_tokenstreams_eq {
    ($left:expr, $right:expr) => {{
        $crate::compare_tokenstreams($left, $right);
    }};
}

#[cfg(test)]
mod tests {
    use quote::quote;

    use crate::assert_tokenstreams_eq;

    #[test]
    fn test_compare_tokenstreams_equal() {
        let first = quote! {
            fn test(a: String, b: String) {
                return a;
            }
        };
        let second = quote! {
            fn test(a: String, b: String) {
                return a;
            }
        };
        assert_tokenstreams_eq!(&first, &second);
    }

    #[test]
    #[should_panic(expected = "(left == right)")]
    fn test_compare_tokenstreams_unequal() {
        let first = quote! {
            fn test(a: String, b: String) {
                let test = 2;
                return a;
            }
        };
        let second = quote! {
            fn test2(a: String, b: String) {
                let test = 2;
                return b;
            }
        };
        assert_tokenstreams_eq!(&first, &second);
    }

    #[test]
    #[should_panic(expected = "Input is not valid Rust code.")]
    fn test_compare_tokenstreams_invalid_rust_code() {
        let first = quote! {
            async fn test(a: String, b: String) {
                let test = async {
                    a + b
                }
                return test;
            }
        };
        let second = quote! {
            async fn test(a: String, b: String) {
                return a;
            }
        };
        assert_tokenstreams_eq!(&first, &second);
    }

    #[test]
    #[should_panic(expected = "(left == right)")]
    fn test_compare_tokenstreams_unequal_async() {
        let first = quote! {
            async fn test(a: String, b: String) {
                let test = async {
                    a + b
                }.await;
                return test;
            }
        };
        let second = quote! {
            async fn test(a: String, b: String) {
                return a;
            }
        };
        assert_tokenstreams_eq!(&first, &second);
    }
}
