use number_theory::NumberTheory;


/// Extension of NumberTheory that supports checking of primes of certain form
pub trait NTPrimality: NumberTheory + Sized{
  /// Checks if a number is a standard prime
 fn is_standard(&self) -> bool;
  /// Check if n is a sophie germain, returns Safe prime if true
  /**
  
     use nt_primes::NTPrimality;
          
     let sophie = 1481u64;
     let safe = sophie.sophie_safe().unwrap();
     assert_eq!(safe,2963)
     
  */
 fn sophie_safe(&self) -> Option<Self>;
 
 ///Finds if number is prime, then returns either zero if no prime found at the distance or the prime. Returns None if self is not prime.  
 /**   
    
    use nt_primes::NTPrimality;
         
    let start = 3u64;
    let twin = start.delta(2).unwrap();
    
    assert_eq!(5, twin); 
    
 */
  
 fn delta(&self, delta:Self) -> Option<Self>;
 /// Returns nearest prime to self. Only operates within bounds of datatype. Biases toward larger primes. 
 fn nearest(&self) -> Self;
 /// Returns nearest greater prime from self. Self is not necessarily a prime, will return None if value can't be found in range of the integer type. 
 /**
 
     use number_theory::Mpz;
     use nt_primes::NTPrimality;
     
     let point =Mpz::from_string(
     "45431654658454164674896465").unwrap();
     let prime = point.nearest_greater().unwrap();
    
      assert_eq!("45431654658454164674896481", prime.to_string())
    
 */
 fn nearest_greater(&self) -> Option<Self>;
  /// Returns nearest lesser prime from self. Self is not necessarily a prime
 fn nearest_lesser(&self) -> Option<Self>;
  /** Number-theorectic strength. -1 for weak primes, 0 for balanced, 1 for strong. Returns None if it is not possible to evaluate in the datatype bounds. 
  */
 fn nt_strength(&self) -> Option<i8>;
 /// Prime numbers of the form xn + k
 /**
    
     use nt_primes::NTPrimality;
         
     let pythagorean = 17657u64;
    
     assert_eq!(true, pythagorean.is_form(4u64,1u64))
    
 */
 fn is_form(&self, scalar: Self, addend: Self) -> bool;
 /// Returns true if self-2 and self+2 are not prime, but self is.
 fn is_isolated(&self) -> bool;
}



