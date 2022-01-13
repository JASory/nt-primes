//extern crate number-theory;
pub use number_theory::traits::NumberTheory;
pub use number_theory::arithmetic::mpz::Mpz;
pub use number_theory::arithmetic::sign::Sign;

pub mod traits;

pub use crate::traits::NTPrimality;


