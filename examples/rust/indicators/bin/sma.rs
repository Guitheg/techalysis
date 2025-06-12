use techalib::errors::TechalibError;
use techalib::indicators::sma::sma;

fn main() -> Result<(), TechalibError> {
    let data = [10.0, 11.0, 12.0, 13.0, 12.5, 12.0];
    let sma3 = sma(&data, 3)?.values;
    println!("{sma3:?}");
    Ok(())
}
