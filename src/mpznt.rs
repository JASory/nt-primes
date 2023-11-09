use crate::traits::NTPrimality;
use number_theory::Mpz;
use number_theory::NumberTheory;
use std::cmp::Ordering;

impl NTPrimality for Mpz{
 
 fn is_standard(&self) -> bool {
     self.is_prime()
 }
 
 fn sophie_safe(&self) -> Option<Self>{
     self.is_sophie()
 }
 
 fn delta(&self, delta: Self) -> Option<Self> {
    if !self.is_standard(){
      return None
    }
    
    if self.ref_addition(&delta).is_standard(){
      return Some(self.ref_addition(&delta))
    }
    if self.u_cmp(&delta) == Ordering::Less {
      return None
    }
    if self.ref_subtraction(&delta).is_standard(){
      return Some(self.ref_subtraction(&delta))
    }
    
    Some(Mpz::zero())
 }
 
 fn nearest_greater(&self) -> Option<Self>{
     let mut x = self.clone();
     
     loop {
       x.successor();
       if x.is_standard(){
         return Some(x)
       }
     }
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
 }
 
 fn nearest(&self) -> Self{
     let mut iter = Mpz::zero();
     
    loop {
    
    iter.successor();
    
    if self.ref_addition(&iter).is_standard(){
       return self.ref_addition(&iter)
    }
    
    if self.u_cmp(&iter) == Ordering::Greater && self.ref_subtraction(&iter).is_standard() {
      return self.ref_subtraction(&iter)
     }

    
    }
 }
 
 
 fn nt_strength(&self) -> Option<i8> {
 
    if !self.is_standard(){return None}
    
    let upper = self.nearest_greater();
    let lower = self.nearest_lesser();
    
    match (upper, lower){
    
    (None, _) => None, 
    (_, None) => None, 
    (Some(x), Some(y)) => {
    
     let upper_delta = x.ref_subtraction(self);
     let lower_delta = self.ref_subtraction(&y);
     
     if upper_delta.u_cmp(&lower_delta) == Ordering::Greater{
         return Some(-1i8)
     }
     
     if upper_delta.u_cmp(&lower_delta) == Ordering::Equal{
         return Some(0i8)
     }
     
     Some(1i8)
    
    }
    
    }
 
 }
 
 fn is_form(&self, scalar: Self, addend: Self) -> bool {
     if addend.u_cmp(self) == Ordering::Greater{
        return false
     }
     
     if self.ref_subtraction(&addend).ref_euclidean(&scalar).1 !=Mpz::zero(){
        return false
     }
    
     
     self.is_standard()
 } 
 
 fn is_isolated(&self) -> bool{
    if !self.is_standard(){
      return false
    }
    
    let two = Mpz::from_u64(2);
    
    if !self.ref_subtraction(&two).is_standard() && !self.ref_addition(&two).is_standard() {
    return true
    }
    
    false
 }
 
 }
