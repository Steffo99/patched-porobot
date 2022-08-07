#[cfg(not(feature = "telegram"))]
fn main() {
    println!("The `telegram` feature was not included on compilation, therefore this binary is not available.")
}

#[cfg(feature = "telegram")]
fn main() {
    println!("Hello telegram world!")
}
