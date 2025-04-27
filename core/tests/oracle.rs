#[macro_export]
macro_rules! oracle_test {
    ($name: ident, $f: expr) => {
        paste::paste! {
            #[test]
            fn [<test_ $name _with_oracle>]() {
                use crate::tests::helper::{assert_vec_close, read_fixture};
                let (input, expected) = read_fixture(stringify!($name));
                let output = $f(&input);
                assert!(output.is_ok());
                assert_vec_close(&output.unwrap(), &expected);
            }
        }
    };
}
