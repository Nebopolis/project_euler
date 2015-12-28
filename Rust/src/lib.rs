#![allow(dead_code)]
#![allow(unused_imports)]
extern crate primes;
extern crate num;
use primes::PrimeSet;
//use num::{BigUint, Zero, One};
//use num::bigint::ToBigUint;


/// ### [Multiples of 3 and 5](https://projecteuler.net/problem=1)
/// If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
/// Find the sum of all the multiples of 3 or 5 below 1000.
///
/// # Examples
/// ```
/// use euler::p1;
/// assert_eq!(233_168, p1(1_000));
/// ```
pub fn p1(max: u64) -> u64 {
    (1..max).fold(0u64, |acc, i| {
        if i%3 == 0 || i%5 == 0 {
            acc + i
        } else {
            acc
        }
    })
}

/// ### [Even Fibonacci numbers](https://projecteuler.net/problem=2)
/// Each new term in the Fibonacci sequence is generated by adding the previous two terms. By starting with 1 and 2, the first 10 terms will be: `1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...`
/// By considering the terms in the Fibonacci sequence whose values do not exceed four million, find the sum of the even-valued terms.
///
/// # Examples
/// ```
/// use euler::p2;
/// assert_eq!(4_613_732, p2(4_000_000));
/// ```
pub fn p2(max: u64) -> u64 {
    unimplemented!();
}

/// ### [Largest prime factor](https://projecteuler.net/problem=3)
/// The prime factors of 13195 are 5, 7, 13 and 29.
/// What is the largest prime factor of the number 600851475143?
///
/// # Examples
/// ```
/// use euler::p3;
/// assert_eq!(6857, p3(600_851_475_143));
/// ```
pub fn p3(max: u64) -> u64 {
    unimplemented!();
}

/// ### [Largest palindrome product](https://projecteuler.net/problem=4)
/// A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
/// Find the largest palindrome made from the product of two 3-digit numbers.
///
/// # Examples
/// ```
/// use euler::p4;
/// assert_eq!(906_609, p4(999));
/// ```
pub fn p4(max: u64) -> u64 {
    unimplemented!();
}

/// ### [Smallest multiple](https://projecteuler.net/problem=5)
/// 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
/// What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
///
/// # Examples
/// ```
/// use euler::p5;
/// assert_eq!(232_792_560, p5(20));
/// ```
pub fn p5(max: u64) -> u64 {
    unimplemented!();
}

/// ### [Sum square difference](https://projecteuler.net/problem=6)
/// The sum of the squares of the first ten natural numbers is, `12 + 22 + ... + 102 = 385`.
/// The square of the sum of the first ten natural numbers is, `(1 + 2 + ... + 10)2 = 552 = 3025`.
/// Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is 3025 − 385 = 2640.
/// Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.
///
/// # Examples
/// ```
/// use euler::p6;
/// assert_eq!(25_164_150, p6(100));
/// ```
pub fn p6(max: u64) -> u64 {
    let max = max + 1;
    let sum_of_squares = (1..max).fold(0u64, |acc, i| acc + (i * i));
    let sum = (1..max).fold(0u64, |acc, i| acc + i);
    let square_of_sums = sum * sum;
    square_of_sums - sum_of_squares
}


/// ### [10001st prime](https://projecteuler.net/problem=7)
/// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
/// What is the 10,001st prime number?
///
/// # Examples
/// ```
/// use euler::p7;
/// assert_eq!(104743, p7(10_001));
/// ```
pub fn p7(max: usize) -> u64 {
    let max = max - 1;
    let mut pset = PrimeSet::new();
    let prime = pset.get(max);
    prime
}

/// ### [Largest product in a series](https://projecteuler.net/problem=8)
/// The four adjacent digits in the 1000-digit number that have the greatest product are 9 × 9 × 8 × 9 = 5832.
/// Find the thirteen adjacent digits in the 1000-digit number that have the greatest product. What is the value of this product?
/// 
/// ```text
/// 73167176531330624919225119674426574742355349194934
/// 96983520312774506326239578318016984801869478851843
/// 85861560789112949495459501737958331952853208805511
/// 12540698747158523863050715693290963295227443043557
/// 66896648950445244523161731856403098711121722383113
/// 62229893423380308135336276614282806444486645238749
/// 30358907296290491560440772390713810515859307960866
/// 70172427121883998797908792274921901699720888093776
/// 65727333001053367881220235421809751254540594752243
/// 52584907711670556013604839586446706324415722155397
/// 53697817977846174064955149290862569321978468622482
/// 83972241375657056057490261407972968652414535100474
/// 82166370484403199890008895243450658541227588666881
/// 16427171479924442928230863465674813919123162824586
/// 17866458359124566529476545682848912883142607690042
/// 24219022671055626321111109370544217506941658960408
/// 07198403850962455444362981230987879927244284909188
/// 84580156166097919133875499200524063689912560717606
/// 05886116467109405077541002256983155200055935729725
/// 71636269561882670428252483600823257530420752963450
/// ```
/// 
/// # Examples
/// ```
/// use euler::p8;
/// assert_eq!(23_514_624_000, p8(13));
/// ```
pub fn p8(width: usize) -> u64 {
    static MANY_DIGIT: &'static str = "7316717653133062491922511967442657474235534919493496983520312774506326239578318016984801869478851843858615607891129494954595017379583319528532088055111254069874715852386305071569329096329522744304355766896648950445244523161731856403098711121722383113622298934233803081353362766142828064444866452387493035890729629049156044077239071381051585930796086670172427121883998797908792274921901699720888093776657273330010533678812202354218097512545405947522435258490771167055601360483958644670632441572215539753697817977846174064955149290862569321978468622482839722413756570560574902614079729686524145351004748216637048440319989000889524345065854122758866688116427171479924442928230863465674813919123162824586178664583591245665294765456828489128831426076900422421902267105562632111110937054421750694165896040807198403850962455444362981230987879927244284909188845801561660979191338754992005240636899125607176060588611646710940507754100225698315520005593572972571636269561882670428252483600823257530420752963450";

    let nums: Vec<(char)> = MANY_DIGIT.chars().collect();

    let mut start = 0;
    let mut length = width;
    let max = nums.len();
    let mut high_total: u64 = 0;
    while length < max {
        let curr_total = (start..length).fold(1, |acc, i| acc * nums[i].to_digit(10).unwrap() as u64);
        if curr_total > high_total {
            high_total = curr_total;
        }
        start += 1;
        length += 1;   
    }
    high_total
}