use number_theory::traits::NumberTheory;
use number_theory::Mpz;
use std::cmp::*;
/*
   An important note for permitting faster evaluation across integer limits we note that the largest 64-bit prime is 2^64-59 
   and the smallest prime greater than 2^64 is 2^64 + 13
*/


pub trait NTPrimality: NumberTheory + Sized{
  /// Checks if a number is a standard prime
 fn is_standard(&self) -> bool;
  /// Check if n is a sophie germain, returns Safe prime if true
  /**
  
     ```
     
     let sophie = 1481u64;
     let safe = sophie.unwrap();
     assert_eq!(safe,2963)
  */
 fn sophie_safe(&self) -> Option<Self>;
  /// Finds if number is prime, then returns either zero if no prime found at the distance or the prime. Returns None if self is not prime. 
  /**
 
    ```
    
    let start = 3u64;
    let twin = start.delta(2).unwrap();
    
    assert_eq!(5, twin); 
 */
  
 fn delta(&self, delta:Self) -> Option<Self>;
 /// Returns nearest prime to self. Only operates within bounds of datatype. Biases toward larger primes. 
 fn nearest(&self) -> Self;
 /// Returns nearest greater prime from self. Self is not necessarily a prime, will return None if value can't be found in range of the integer type. 
 /**
 
    ```
    
    let point = Mpz::u_from_string("45431654658454164674896465").unwrap();
    let prime = point.nearest_greater().unwrap();
    
    assert_eq!("45431654658454164674896481", prime.to_string())
 */
 fn nearest_greater(&self) -> Option<Self>;
  /// Returns nearest lesser prime from self. Self is not necessarily a prime
 fn nearest_lesser(&self) -> Option<Self>;
  /// Number-theorectic strength. -1 for weak primes, 0 for balanced, 1 for strong. Returns None if it is not possible to evaluate in the datatype bounds. 
 fn nt_strength(&self) -> Option<i8>;
 /// Prime numbers of the form xn + k
 /**
 
    ```
    
    let pythagorean = 17657u64;
    
    assert_eq!(true, pythagorean.is_form(4u64,1u64))
 */
 fn is_form(&self, scalar: Self, addend: Self) -> bool;
 /// Returns true if self-2 and self+2 are not prime, but self is.
 fn is_isolated(&self) -> bool;
}




impl NTPrimality for u64{

 fn is_standard(&self) -> bool{
  	self.is_prime()
  }
  
 fn sophie_safe(&self) -> Option<Self> {
 	if self.is_prime(){
 	 let safe = *self*2 +1;
 	 if 2u64.mod_pow(&(*self*2), &safe)==1{
 	   return Some(safe)
 	 }
 	}
 	return None
 }
 
 fn delta(&self, delta: Self) -> Option<Self> {
     if self.is_prime() == false {return None}
     if (self + delta).is_prime() {return Some(self + delta)}
     if delta > *self { return None}
     if (self -delta).is_prime(){return Some(self - delta)}
     return Some(0u64)
 }
 // fn form_prime(&self, scalar: Self, delta: Self) -> bool
 fn nearest_greater(&self) -> Option<Self> {  // if 
    let mut x = self.clone();
    loop {
      x+=1;
      if x == u64::MAX {return None}
      if x.is_prime(){
        return Some(x)
      }
    }
 }
 
 fn nearest_lesser(&self) -> Option<Self> {
 let mut x = self.clone();
    loop {
      if x ==0u64 {return None}
      x-=1;
      if x.is_prime(){
        return Some(x)
      }
    }
 }
 
 fn nearest(&self) -> Self {// nearest prime within interval 0;2^64 to self
     let mut iter = 0u64;
     
     loop {
       iter+=1;
       
       if iter < u64::MAX-*self {
         if (*self+iter).is_prime(){
           return *self + iter
         }
       }
       
       if iter < *self{
         if (*self -iter).is_prime(){
           return *self - iter
         }
       }
     }
     return 0u64 // this should never return 
 }
    // Returns -1 if weak 0 if balanced and 1 if strong and None if unable to evaluate due to out of bounds
 fn nt_strength(&self) -> Option<i8>{
      if self.is_prime() == false {return None}
      let upper = self.nearest_greater();
      let lower = self.nearest_lesser();

      match (upper,lower){
        
        (None, _) => return None,
        (_, None) => return None,
        (Some(x), Some(y)) => {
         let upper_delta = x -*self;
         let lower_delta = *self -y;
         
          if upper_delta > lower_delta {
            return Some(-1i8)
          } 
          if upper_delta == lower_delta {
            return Some(0i8)
          } 
          return Some(1i8)
          }
      
      }
 }
 
 fn is_form(&self, scalar: Self, addend: Self) -> bool {
      if addend >= *self {
        return false
      }
      if (*self-addend)%scalar != 0 {
       return false
      }
      
      return self.is_prime()
 }
 
 fn is_isolated(&self) -> bool {
     if self < &2u64 {//     If less than 2 return false
        return false
     }
     
     if self > &(u64::MAX-57) {   // no primes are in the interval 2^64-57;2^64 so we only need to check lower numbers
       return false
     } 
     
     if self.is_prime() == false{// evaluate if prime is 
        return false
     }
     

    if (*self -2).is_prime() == false && (*self +2).is_prime() == false {
       return true
    }
    
    return false
 } 
 
 }
 
 impl NTPrimality for Mpz{
 
 fn is_standard(&self) -> bool {
     self.probable_prime()
 }
 
 fn sophie_safe(&self) -> Option<Self>{
     self.is_sophie()
 }
 
 fn delta(&self, delta: Self) -> Option<Self> {
    if self.is_standard() == false{
      return None
    }
    
    if self.addition(delta.clone()).is_standard(){
      return Some(self.addition(delta.clone()))
    }
    if self.u_cmp(&delta) == Ordering::Less {
      return None
    }
    if self.subtraction(delta.clone()).is_standard(){
      return Some(self.subtraction(delta))
    }
    
    return Some(Mpz::zero())
 }
 
 fn nearest_greater(&self) -> Option<Self>{
     let mut x = self.clone();
     
     loop {
       x.successor();
       if x.is_standard(){
         return Some(x)
       }
     }
     
     return None  // this will never return
 }
 
 fn nearest_lesser(&self) -> Option<Self>{
     let mut x = self.clone();
     if x == Mpz::from_u64(2){
       return None
     }
     loop {
       x.mut_subtraction(Mpz::one());
       if x.is_standard(){
         return Some(x)
       }
     }
     
     return None
 }
 
 fn nearest(&self) -> Self{
     let mut iter = Mpz::zero();
     
    loop {
    
    iter.successor();
    
    if self.addition(iter.clone()).is_standard(){
       return self.addition(iter)
    }
    
    if self.u_cmp(&iter) == Ordering::Greater{
      if self.subtraction(iter.clone()).is_standard(){
       return self.subtraction(iter)
      }
    }
    
    }
    
    return Mpz::zero() // this should never return
 }
 
 
 fn nt_strength(&self) -> Option<i8> {
 
    if self.is_standard() == false {return None}
    
    let upper = self.nearest_greater();
    let lower = self.nearest_lesser();
    
    match (upper, lower){
    
    (None, _) => return None, 
    (_, None) => return None, 
    (Some(x), Some(y)) => {
    
     let upper_delta = x.subtraction(self.clone());
     let lower_delta = self.subtraction(y);
     
     if upper_delta.u_cmp(&lower_delta) == Ordering::Greater{
         return Some(-1i8)
     }
     
     if upper_delta.u_cmp(&lower_delta) == Ordering::Equal{
         return Some(0i8)
     }
     
     return Some(1i8)
    
    }
    
    }
 
 }
 
 fn is_form(&self, scalar: Self, addend: Self) -> bool {
     if addend.u_cmp(self) == Ordering::Greater{
        return false
     }
     
     if self.subtraction(addend).euclidean(&scalar).1 !=Mpz::zero(){
        return false
     }
     
     return self.is_standard()
 } 
 
 fn is_isolated(&self) -> bool{
    if self.is_standard() == false{
      return false
    }
    
    let two = Mpz::from_u64(2);
    
    if self.subtraction(two.clone()).is_standard() == false &&self.addition(two).is_standard() == false {
    return true
    }
    
    return false
 }
 
 }



