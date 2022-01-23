A library for recreational mathematics providing efficient checks for primes of various forms.  Relies on number-theory for fast primality. Only works for u64 or Mpz. Due to the specialized weighting that the core library [number-theory](https://crates.io/crates/number-theory) uses to ensure that error rate is less than 2^-64 primality checks are slower than competitors for values between 2^64 and 2^512 but several times faster outside that range. 