use techalib::{errors::TechalibError, indicators::ema::ema};

fn main() -> Result<(), TechalibError> {
    let prices = vec![10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0, 18.0, 19.0];
    let ema5 = ema(&prices, 5, None)?.values;
    println!("{ema5:?}");

    Ok(())
}
