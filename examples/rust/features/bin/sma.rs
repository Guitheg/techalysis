use technicalysis::errors::TechnicalysisError;
use technicalysis::features::sma::sma;

fn main() -> Result<(), TechnicalysisError> {
    let data = [10.0, 11.0, 12.0, 13.0, 12.5, 12.0];
    let sma3 = sma(&data, 3)?;
    println!("{sma3:?}");
    Ok(())
}
