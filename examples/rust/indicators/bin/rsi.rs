use technicalysis::{errors::TechnicalysisError, indicators::rsi};

fn main() -> Result<(), TechnicalysisError> {
    let prices = vec![10.0, 11.0, 10.0, 10.1, 14.0, 15.0, 16.0, 17.0, 18.0, 19.0];
    let rsi4 = rsi(&prices, 4)?;
    println!("{rsi4:?}");

    Ok(())
}
