use number_theory::traits::NumberTheory;

pub trait NTPrimality: NumberTheory + Sized{

 fn is_standard(&self) -> bool;/*
  /// Check if n is a sophie germain 
 fn sophie_safe(&self) -> Option<Self>;
 
 fn prime_form
 /// Finds if number is prime, then returns either zero if no prime found at the distance or the prime
 fn distance_prime(&self, delta: Self) -> Option<Self>;
 /// Returns nearest greater prime from self. Self is not necessarily a prime
 fn nearest_greater(&self) -> Option<Self>;
  /// Returns nearest lesser prime from self. Self is not necessarily a prime
 fn nearest_lesser(&self) -> Option<Self>;
 /// Evaluates if self is prime and it's greater and lesser primes are the same distance
 fn is_balanced(&self) -> bool;
 /// Is stronger than the arithmetic mean 
 fn is_nt_strong(&self) -> bool;
 */
}




impl NTPrimality for u64{

 fn is_standard(&self) -> bool{
  	self.is_prime()
  }

}


