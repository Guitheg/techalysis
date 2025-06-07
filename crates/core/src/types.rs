#[cfg(all(feature = "f32", not(feature = "f64")))]
pub type Float = f32;

#[cfg(not(all(feature = "f32", not(feature = "f64"))))]
pub type Float = f64;
