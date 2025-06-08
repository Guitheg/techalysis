#[macro_export]
macro_rules! expect_err_overflow_or_ok_with {
    ($expr:expr, $ok_block:expr) => {{
        match $expr {
            Ok(ok) => $ok_block(ok),
            Err(e) => match e {
                TechalysisError::Overflow(_, _) => {}
                _ => panic!(
                    "Expected Ok(_) or {:?}, but got: {:?}",
                    stringify!($err_variant),
                    e
                ),
            },
        }
    }};
}
