#[derive(Debug)]
pub enum TestError {
    Aieaie,
    MinusFour,
}

fn result_test() -> Result<u32, TestError> {
    Err(TestError::Aieaie)
}

fn process() -> Result<(), TestError> {
    let x = result_test()?;
    println!("Value: {x:?}");
    Ok(())
}

fn main() {
    let result = process();
    println!("Value: {result:?}");
}
