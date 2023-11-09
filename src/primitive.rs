/*

  NTPrimality for primitive types 

*/

use crate::NTPrimality;
use number_theory::NumberTheory;


macro_rules! impl_ntprimality(
  ($($t:ty),* $(,)*) => {$(
  
impl NTPrimality for $t{
  
  
 fn is_standard(&self) -> bool{
  	self.is_prime()
  }
  
 fn sophie_safe(&self) -> Option<Self> {
 	if self.is_prime(){
 	 let safe = *self*2 +1;
 	 if 2.exp_residue(&(*self*2), &safe)==1{
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
     return Some(0)
 }
 // fn form_prime(&self, scalar: Self, delta: Self) -> bool
 fn nearest_greater(&self) -> Option<Self> {  // if 
    let mut x = self.clone();
    loop {
      if x == <$t>::MAX {return None}
      x+=1;
      if x.is_prime(){
        return Some(x)
      }
    }
 }
 
 fn nearest_lesser(&self) -> Option<Self> {
 let mut x = self.clone();
    loop {
      if x ==0 {return None}
      x-=1;
      if x.is_prime(){
        return Some(x)
      }
    }
 }
 
 fn nearest(&self) -> Self {// nearest prime within interval 0;2^64 to self
     let mut iter = 0;
     
     loop {
       iter+=1;
       
       if iter < <$t>::MAX-*self {
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
//     return 0 // this should never return 
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
     if self < &2 {//     If less than 2 return false
        return false
     }
     
     if self > &(<$t>::MAX-2) {   // no primes are in the interval 2^64-57;2^64 so we only need to check lower numbers
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
    )*}
);

impl_ntprimality!(u8,u16,u32,u64,usize,u128, i8,i16,i32,i64,i128,isize);
