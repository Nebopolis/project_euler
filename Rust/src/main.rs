extern crate primes;
extern crate num;
use primes::PrimeSet;
use num::{BigUint, Zero, One};
use num::bigint::ToBigUint;

static MANY_DIGIT: &'static str = "7316717653133062491922511967442657474235534919493496983520312774506326239578318016984801869478851843858615607891129494954595017379583319528532088055111254069874715852386305071569329096329522744304355766896648950445244523161731856403098711121722383113622298934233803081353362766142828064444866452387493035890729629049156044077239071381051585930796086670172427121883998797908792274921901699720888093776657273330010533678812202354218097512545405947522435258490771167055601360483958644670632441572215539753697817977846174064955149290862569321978468622482839722413756570560574902614079729686524145351004748216637048440319989000889524345065854122758866688116427171479924442928230863465674813919123162824586178664583591245665294765456828489128831426076900422421902267105562632111110937054421750694165896040807198403850962455444362981230987879927244284909188845801561660979191338754992005240636899125607176060588611646710940507754100225698315520005593572972571636269561882670428252483600823257530420752963450";

fn p1(max: u64) -> u64 {
    let sum = (1..max + 1).fold(0u64, |acc, i| {
        if i%3 == 0 || i%5 ==0 {
            acc + i
        } else {
            acc
        }
    });
    sum
}

fn p6(max: u64) -> u64 {
    let max = max + 1;
    let sum_of_squares = (1..max).fold(0u64, |acc, i| acc + (i * i));
    let sum = (1..max).fold(0u64, |acc, i| acc + i);
    let square_of_sums = sum * sum;
    square_of_sums - sum_of_squares
}

fn p7(max: usize) -> u64 {
    let max = max - 1;
    let mut pset = PrimeSet::new();
    let prime = pset.get(max);
    prime
}

fn p8(width: usize) -> BigUint {
    let nums: Vec<(char)> = MANY_DIGIT.chars().collect();

    let mut start = 0;
    let mut length = width;
    let max = nums.len();
    let mut high_total: BigUint = Zero::zero();
    while length < max {
        let curr_total = (start..length).fold(One::one(), |acc, i| acc * nums[i].to_digit(10).unwrap().to_biguint().unwrap());
        if curr_total > high_total {
            high_total = curr_total;
        }
        start += 1;
        length += 1;   
    }
    high_total
}


fn main() {
    println!("{}", p1(1000));
    println!("{}", p6(100));
    println!("{}", p7(10_001));
    println!("{}", p8(13));
}
