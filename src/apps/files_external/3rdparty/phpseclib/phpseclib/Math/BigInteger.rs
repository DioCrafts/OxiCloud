use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, BitAnd, BitOr, BitXor, Mul, Not, Sub};
use num_bigint::{BigInt, BigUint, Sign, ToBigInt};
use num_traits::{One, Zero, ToPrimitive, Num};
use rand::{Rng, thread_rng};
use rand::distributions::Uniform;

// Reduction constants
const MONTGOMERY: u8 = 0;
const BARRETT: u8 = 1;
const POWEROF2: u8 = 2;
const CLASSIC: u8 = 3;
const NONE: u8 = 4;

// Array constants
const VALUE: usize = 0;
const SIGN: usize = 1;

// Cache constants
const VARIABLE: usize = 0;
const DATA: usize = 1;

/// Mode constants
const MODE_INTERNAL: u8 = 1;
const MODE_GMP: u8 = 3;
const MODE_BCMATH: u8 = 2;

/// Karatsuba Cutoff
const KARATSUBA_CUTOFF: usize = 25;

#[derive(Clone)]
pub struct BigInteger {
    /// Holds the BigInteger's value
    value: BigInt,
    
    /// Precision
    precision: i64,
    
    /// Precision Bitmask
    bitmask: Option<BigInt>,
}

impl BigInteger {
    /// Create a new BigInteger from various sources
    pub fn new<T: Into<BigInt>>(value: T) -> Self {
        BigInteger {
            value: value.into(),
            precision: -1,
            bitmask: None,
        }
    }

    /// Create a BigInteger from string representation with optional base
    pub fn from_str(s: &str, base: i32) -> Result<Self, String> {
        // Handle special case for binary strings
        if base == 2 || base == -2 {
            let is_negative = base < 0 || (base > 0 && s.starts_with('-'));
            let s = if s.starts_with('-') { &s[1..] } else { s };
            
            // Filter out any non-binary digits
            let filtered: String = s.chars()
                .filter(|c| *c == '0' || *c == '1')
                .collect();
            
            // Pad to multiple of 4 (for hex conversion)
            let padded = format!("{:0>width$}", filtered, width = (filtered.len() + 3) & !3);
            
            // Convert to hex
            let mut hex = String::new();
            for chunk in padded.as_bytes().chunks(4) {
                let chunk_str = std::str::from_utf8(chunk).unwrap_or("0000");
                let bin_val = u8::from_str_radix(chunk_str, 2).unwrap_or(0);
                hex.push_str(&format!("{:x}", bin_val));
            }
            
            let mut result = if is_negative {
                BigInteger::from_str(&format!("-0x{}", hex), 16)?
            } else {
                BigInteger::from_str(&format!("0x{}", hex), 16)?
            };
            
            if is_negative && base < 0 {
                let one = BigInteger::new(1);
                result = result.add(&one);
            }
            
            return Ok(result);
        }
        
        // Handle other bases
        let radix = base.abs() as u32;
        let is_negative = base < 0 || (base > 0 && s.starts_with('-'));
        let s = if s.starts_with('-') { &s[1..] } else { s };
        
        // Strip 0x for hex
        let s = if (radix == 16) && (s.starts_with("0x") || s.starts_with("0X")) {
            &s[2..]
        } else {
            s
        };
        
        // Convert string to BigInt
        let result = match BigInt::from_str_radix(s, radix) {
            Ok(n) => {
                if is_negative { -n } else { n }
            },
            Err(_) => return Err(format!("Invalid number in base {}", base)),
        };
        
        // For negative input in twos complement format
        let mut bigint = BigInteger::new(result);
        if base < 0 && is_negative {
            let one = BigInteger::new(1);
            bigint = bigint.add(&one);
        }
        
        Ok(bigint)
    }

    /// Convert to bytes (base-256)
    pub fn to_bytes(&self, twos_compliment: bool) -> Vec<u8> {
        if twos_compliment {
            let zero = BigInteger::new(0);
            let comparison = self.compare(&zero);
            
            if comparison == Ordering::Equal {
                if self.precision > 0 {
                    return vec![0; ((self.precision + 1) >> 3) as usize];
                } else {
                    return vec![];
                }
            }
            
            let mut temp = if comparison < Ordering::Equal {
                let one = BigInteger::new(1);
                self.add(&one)
            } else {
                self.clone()
            };
            
            let mut bytes = temp.to_bytes(false);
            
            if bytes.is_empty() {
                bytes = vec![0];
            }
            
            if bytes[0] & 0x80 != 0 {
                bytes.insert(0, 0);
            }
            
            if comparison < Ordering::Equal {
                bytes = bytes.iter().map(|b| !b).collect();
            }
            
            return bytes;
        }
        
        // Normal (non twos-compliment) mode
        if self.value.is_zero() {
            if self.precision > 0 {
                return vec![0; ((self.precision + 1) >> 3) as usize];
            }
            return vec![];
        }
        
        let mut bytes = self.value.to_signed_bytes_be();
        
        // Ensure the sign bit is properly handled
        if !self.value.is_negative() && !bytes.is_empty() && bytes[0] & 0x80 != 0 {
            bytes.insert(0, 0);
        }
        
        // Trim leading zeros unless precision requires them
        if self.precision <= 0 {
            let mut start = 0;
            while start < bytes.len() && bytes[start] == 0 {
                start += 1;
            }
            bytes = bytes[start..].to_vec();
        } else {
            // Pad or trim to the required precision
            let target_len = ((self.precision + 7) >> 3) as usize;
            if bytes.len() < target_len {
                let padding = vec![0; target_len - bytes.len()];
                bytes = [padding, bytes].concat();
            } else if bytes.len() > target_len {
                bytes = bytes[bytes.len() - target_len..].to_vec();
            }
        }
        
        bytes
    }

    /// Convert to hexadecimal string
    pub fn to_hex(&self, twos_compliment: bool) -> String {
        let bytes = self.to_bytes(twos_compliment);
        let mut result = String::new();
        
        for byte in bytes {
            result.push_str(&format!("{:02x}", byte));
        }
        
        result
    }

    /// Convert to bit string (binary)
    pub fn to_bits(&self, twos_compliment: bool) -> String {
        let hex = self.to_hex(twos_compliment);
        let mut bits = String::new();
        
        for (i, c) in hex.chars().enumerate() {
            let nibble = u8::from_str_radix(&c.to_string(), 16).unwrap_or(0);
            let bit_chunk = format!("{:04b}", nibble);
            bits.push_str(&bit_chunk);
        }
        
        // Remove leading zeros unless precision requires them
        if self.precision <= 0 {
            let mut i = 0;
            while i < bits.len() && &bits[i..i+1] == "0" {
                i += 1;
            }
            bits = bits[i..].to_string();
        } else {
            // Trim or pad to the required precision
            if bits.len() > self.precision as usize {
                bits = bits[bits.len() - self.precision as usize..].to_string();
            } else if bits.len() < self.precision as usize {
                bits = format!("{:0>width$}", bits, width = self.precision as usize);
            }
        }
        
        // Handle special case for twos_compliment with positive number
        if twos_compliment && self.compare(&BigInteger::new(0)) > Ordering::Equal && self.precision <= 0 && !bits.is_empty() {
            bits = format!("0{}", bits);
        }
        
        bits
    }

    /// Convert to decimal string (base 10)
    pub fn to_string(&self) -> String {
        self.value.to_string()
    }

    /// Set precision for bit operations
    pub fn set_precision(&mut self, bits: i64) {
        self.precision = bits;
        
        // Create bitmask with appropriate precision
        if bits > 0 {
            let mut mask = BigInt::from(1);
            mask = mask << bits;
            mask = mask - BigInt::from(1);
            self.bitmask = Some(mask);
        } else {
            self.bitmask = None;
        }
        
        // Apply mask to current value if precision is set
        if let Some(ref mask) = self.bitmask {
            self.value = &self.value & mask;
        }
    }

    /// Compare with another BigInteger
    pub fn compare(&self, other: &BigInteger) -> Ordering {
        self.value.cmp(&other.value)
    }

    /// Test equality with another BigInteger
    pub fn equals(&self, other: &BigInteger) -> bool {
        self.value == other.value
    }

    /// Add another BigInteger
    pub fn add(&self, y: &BigInteger) -> BigInteger {
        BigInteger::new(&self.value + &y.value)
    }

    /// Subtract another BigInteger
    pub fn subtract(&self, y: &BigInteger) -> BigInteger {
        BigInteger::new(&self.value - &y.value)
    }

    /// Multiply by another BigInteger
    pub fn multiply(&self, y: &BigInteger) -> BigInteger {
        BigInteger::new(&self.value * &y.value)
    }

    /// Divide by another BigInteger
    pub fn divide(&self, y: &BigInteger) -> (BigInteger, BigInteger) {
        let (quot, rem) = self.value.div_rem(&y.value);
        
        // Ensure the remainder is positive
        let (quot_final, rem_final) = if rem < BigInt::zero() {
            let y_abs = y.value.abs();
            (quot - BigInt::one(), rem + y_abs)
        } else {
            (quot, rem)
        };
        
        (BigInteger::new(quot_final), BigInteger::new(rem_final))
    }

    /// Modular exponentiation (a^b mod n)
    pub fn mod_pow(&self, e: &BigInteger, n: &BigInteger) -> BigInteger {
        let zero = BigInteger::new(0);
        
        // Handle negative exponent
        if e.compare(&zero) < Ordering::Equal {
            let e_pos = BigInteger::new(e.value.abs());
            let inverse = self.mod_inverse(n);
            
            match inverse {
                Some(inv) => inv.mod_pow(&e_pos, n),
                None => return zero, // Cannot compute modular inverse
            }
        } else if n.compare(&zero) < Ordering::Equal {
            // Modulus must be positive
            return self.mod_pow(e, &BigInteger::new(n.value.abs()));
        } else if self.compare(&zero) < Ordering::Equal || self.compare(n) >= Ordering::Equal {
            // Normalize base to be within modulus range
            let (_, remainder) = self.divide(n);
            return remainder.mod_pow(e, n);
        } else if e.value.is_zero() {
            // x^0 mod n = 1 (for n > 1)
            return BigInteger::new(1);
        } else if e.value == BigInt::one() {
            // x^1 mod n = x mod n
            let (_, remainder) = self.divide(n);
            return remainder;
        } else {
            // Use built-in modpow for efficiency
            let result = self.value.modpow(&e.value, &n.value);
            return BigInteger::new(result);
        }
    }

    /// Modular inverse (1/a mod n)
    pub fn mod_inverse(&self, n: &BigInteger) -> Option<BigInteger> {
        let zero = BigInteger::new(0);
        let n_abs = BigInteger::new(n.value.abs());
        
        // Handle negative inputs
        let a = if self.compare(&zero) < Ordering::Equal {
            let temp = self.abs();
            let temp_inv = temp.mod_inverse(&n_abs)?;
            BigInteger::new(n_abs.value - temp_inv.value)
        } else {
            self.clone()
        };
        
        // Extended Euclidean Algorithm
        let (gcd, x, _) = a.extended_gcd(&n_abs);
        
        if !gcd.equals(&BigInteger::new(1)) {
            return None; // No modular inverse exists
        }
        
        // Normalize the result to be positive
        let result = if x.compare(&zero) < Ordering::Equal {
            BigInteger::new(&x.value + &n_abs.value)
        } else {
            x
        };
        
        Some(result)
    }

    /// Extended Greatest Common Divisor
    pub fn extended_gcd(&self, n: &BigInteger) -> (BigInteger, BigInteger, BigInteger) {
        let mut a = self.abs();
        let mut b = n.abs();
        let mut x0 = BigInteger::new(1);
        let mut y0 = BigInteger::new(0);
        let mut x1 = BigInteger::new(0);
        let mut y1 = BigInteger::new(1);
        
        while !b.value.is_zero() {
            let (q, r) = a.divide(&b);
            a = b;
            b = r;
            
            let x2 = x0.subtract(&q.multiply(&x1));
            let y2 = y0.subtract(&q.multiply(&y1));
            
            x0 = x1;
            y0 = y1;
            x1 = x2;
            y1 = y2;
        }
        
        (a, x0, y0)
    }

    /// Greatest Common Divisor
    pub fn gcd(&self, n: &BigInteger) -> BigInteger {
        let (gcd, _, _) = self.extended_gcd(n);
        gcd
    }

    /// Absolute value
    pub fn abs(&self) -> BigInteger {
        BigInteger::new(self.value.abs())
    }

    /// Bitwise AND
    pub fn bitwise_and(&self, y: &BigInteger) -> BigInteger {
        BigInteger::new(&self.value & &y.value)
    }

    /// Bitwise OR
    pub fn bitwise_or(&self, y: &BigInteger) -> BigInteger {
        BigInteger::new(&self.value | &y.value)
    }

    /// Bitwise XOR
    pub fn bitwise_xor(&self, y: &BigInteger) -> BigInteger {
        BigInteger::new(&self.value ^ &y.value)
    }

    /// Bitwise NOT
    pub fn bitwise_not(&self) -> BigInteger {
        let result = !&self.value;
        
        // Apply precision if set
        let mut bigint = BigInteger::new(result);
        if self.precision > 0 {
            bigint.set_precision(self.precision);
        }
        
        bigint
    }

    /// Bitwise right shift
    pub fn bitwise_right_shift(&self, shift: u32) -> BigInteger {
        BigInteger::new(&self.value >> shift)
    }

    /// Bitwise left shift
    pub fn bitwise_left_shift(&self, shift: u32) -> BigInteger {
        BigInteger::new(&self.value << shift)
    }

    /// Left rotation
    pub fn bitwise_left_rotate(&self, shift: i64) -> BigInteger {
        let mut precision = self.precision;
        
        // Determine bit length if precision not set
        if precision <= 0 {
            precision = self.value.bits() as i64;
        }
        
        // Normalize shift value
        let mut shift = if shift < 0 {
            (shift + precision) % precision
        } else {
            shift % precision
        };
        
        if shift == 0 {
            return self.clone();
        }
        
        // Create bitmask for the target precision
        let mask = BigInt::from(1) << precision;
        let mask = &mask - BigInt::from(1);
        
        // Perform rotation by combining left and right shifts
        let left = &self.value << shift;
        let right = &self.value >> (precision - shift);
        
        BigInteger::new((left | right) & mask)
    }

    /// Right rotation
    pub fn bitwise_right_rotate(&self, shift: i64) -> BigInteger {
        self.bitwise_left_rotate(-shift)
    }

    /// Generate a random number within range
    pub fn random(min: Option<&BigInteger>, max: Option<&BigInteger>) -> BigInteger {
        let min = min.unwrap_or(&BigInteger::new(0));
        let max = max.unwrap_or(&BigInteger::new(BigInt::from(0x7FFFFFFF)));
        
        if min.equals(max) {
            return min.clone();
        }
        
        let mut rng = thread_rng();
        let range = if max.compare(min) < Ordering::Equal {
            &max.value..=&min.value
        } else {
            &min.value..=&max.value
        };
        
        // Generate random bits
        let range_bigint = &max.value - &min.value + BigInt::from(1);
        let bits_needed = range_bigint.bits() as usize;
        let bytes_needed = (bits_needed + 7) / 8;
        
        let mut bytes = vec![0u8; bytes_needed];
        rng.fill(&mut bytes[..]);
        
        // Ensure the random number is within range
        let mut random_bigint = BigInt::from_bytes_be(Sign::Plus, &bytes);
        random_bigint = random_bigint % range_bigint;
        random_bigint = random_bigint + &min.value;
        
        BigInteger::new(random_bigint)
    }

    /// Generate a random prime number
    pub fn random_prime(min: Option<&BigInteger>, max: Option<&BigInteger>, timeout: Option<u64>) -> Option<BigInteger> {
        let min = min.unwrap_or(&BigInteger::new(0));
        let max = max.unwrap_or(&BigInteger::new(BigInt::from(0x7FFFFFFF)));
        
        let start_time = std::time::Instant::now();
        let timeout_duration = timeout.map(std::time::Duration::from_secs);
        
        // Generate a random odd number in the range
        let mut x = BigInteger::random(Some(min), Some(max));
        if x.value.is_even() {
            x.value += BigInt::from(1);
        }
        
        // If we exceed max, start from min
        if x.compare(max) > Ordering::Equal {
            x = min.clone();
            if x.value.is_even() {
                x.value += BigInt::from(1);
            }
        }
        
        let initial_x = x.clone();
        let two = BigInteger::new(2);
        
        loop {
            // Check timeout
            if let Some(duration) = timeout_duration {
                if start_time.elapsed() > duration {
                    return None;
                }
            }
            
            // Check primality
            if x.is_prime(None) {
                return Some(x);
            }
            
            // Move to next odd number
            x = x.add(&two);
            
            // If we exceed max, wrap around to min
            if x.compare(max) > Ordering::Equal {
                x = min.clone();
                if x.equals(&two) {
                    return Some(x); // 2 is prime
                }
                if x.value.is_even() {
                    x.value += BigInt::from(1);
                }
            }
            
            // If we've checked all numbers in range
            if x.equals(&initial_x) {
                return None;
            }
        }
    }

    /// Check if a number is prime
    pub fn is_prime(&self, certainty: Option<u32>) -> bool {
        // Small number optimization
        if self.value <= BigInt::from(1) {
            return false;
        }
        if self.value == BigInt::from(2) || self.value == BigInt::from(3) {
            return true;
        }
        if self.value.is_even() {
            return false;
        }
        
        // Determine test iterations based on bit length
        let certainty = certainty.unwrap_or_else(|| {
            let bit_length = self.value.bits() as usize;
            if bit_length >= 163 { 2 }
            else if bit_length >= 106 { 3 }
            else if bit_length >= 81 { 4 }
            else if bit_length >= 68 { 5 }
            else if bit_length >= 56 { 6 }
            else if bit_length >= 50 { 7 }
            else if bit_length >= 43 { 8 }
            else if bit_length >= 37 { 9 }
            else if bit_length >= 31 { 12 }
            else if bit_length >= 25 { 15 }
            else if bit_length >= 18 { 18 }
            else { 27 }
        });
        
        // Trial division by small primes
        let small_primes = [
            3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 
            73, 79, 83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 
            157, 163, 167, 173, 179, 181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 
            239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307, 311, 313, 317
        ];
        
        for &p in &small_primes {
            if self.value == BigInt::from(p) {
                return true;
            }
            if &self.value % p == BigInt::zero() {
                return false;
            }
        }
        
        // Miller-Rabin primality test
        let n_minus_one = &self.value - BigInt::one();
        
        // Find r and d such that n-1 = 2^r * d where d is odd
        let mut r = 0;
        let mut d = n_minus_one.clone();
        
        while &d & BigInt::one() == BigInt::zero() {
            d >>= 1;
            r += 1;
        }
        
        'witness_loop: for _ in 0..certainty {
            // Generate random witness a such that 2 <= a <= n-2
            let mut rng = thread_rng();
            let a = loop {
                let a = rng.gen_range(BigInt::from(2)..&n_minus_one);
                if a >= BigInt::from(2) && a < n_minus_one {
                    break a;
                }
            };
            
            // Compute a^d mod n
            let mut x = a.modpow(&d, &self.value);
            
            if x == BigInt::one() || x == n_minus_one {
                continue 'witness_loop;
            }
            
            for _ in 0..r-1 {
                x = (&x * &x) % &self.value;
                if x == n_minus_one {
                    continue 'witness_loop;
                }
            }
            
            return false; // Definitely composite
        }
        
        true // Probably prime
    }
}

impl fmt::Display for BigInteger {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl fmt::Debug for BigInteger {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BigInteger({})", self.to_string())
    }
}

impl Add for BigInteger {
    type Output = BigInteger;
    
    fn add(self, other: BigInteger) -> BigInteger {
        self.add(&other)
    }
}

impl Add for &BigInteger {
    type Output = BigInteger;
    
    fn add(self, other: &BigInteger) -> BigInteger {
        self.add(other)
    }
}

impl Sub for BigInteger {
    type Output = BigInteger;
    
    fn sub(self, other: BigInteger) -> BigInteger {
        self.subtract(&other)
    }
}

impl Sub for &BigInteger {
    type Output = BigInteger;
    
    fn sub(self, other: &BigInteger) -> BigInteger {
        self.subtract(other)
    }
}

impl Mul for BigInteger {
    type Output = BigInteger;
    
    fn mul(self, other: BigInteger) -> BigInteger {
        self.multiply(&other)
    }
}

impl Mul for &BigInteger {
    type Output = BigInteger;
    
    fn mul(self, other: &BigInteger) -> BigInteger {
        self.multiply(other)
    }
}

impl BitAnd for BigInteger {
    type Output = BigInteger;
    
    fn bitand(self, other: BigInteger) -> BigInteger {
        self.bitwise_and(&other)
    }
}

impl BitAnd for &BigInteger {
    type Output = BigInteger;
    
    fn bitand(self, other: &BigInteger) -> BigInteger {
        self.bitwise_and(other)
    }
}

impl BitOr for BigInteger {
    type Output = BigInteger;
    
    fn bitor(self, other: BigInteger) -> BigInteger {
        self.bitwise_or(&other)
    }
}

impl BitOr for &BigInteger {
    type Output = BigInteger;
    
    fn bitor(self, other: &BigInteger) -> BigInteger {
        self.bitwise_or(other)
    }
}

impl BitXor for BigInteger {
    type Output = BigInteger;
    
    fn bitxor(self, other: BigInteger) -> BigInteger {
        self.bitwise_xor(&other)
    }
}

impl BitXor for &BigInteger {
    type Output = BigInteger;
    
    fn bitxor(self, other: &BigInteger) -> BigInteger {
        self.bitwise_xor(other)
    }
}

impl Not for BigInteger {
    type Output = BigInteger;
    
    fn not(self) -> BigInteger {
        self.bitwise_not()
    }
}

impl Not for &BigInteger {
    type Output = BigInteger;
    
    fn not(self) -> BigInteger {
        self.bitwise_not()
    }
}