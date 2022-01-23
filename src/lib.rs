//extern crate number-theory;

pub use number_theory::traits::NumberTheory;
pub use number_theory::arithmetic::mpz::Mpz;
pub use number_theory::arithmetic::sign::Sign;

pub mod traits;

pub use crate::traits::NTPrimality;
/*
fn main(){

 let start = 3u64;
    let twin = start.delta(2).unwrap();
    
    assert_eq!(5, twin)
   }
   */

// 2880504
