use std::io::{stdout, Write};

const DIGITS_LEN: usize = 36;
const SKIP_POW_2: u64 = 4572950197771;

fn main() {
    let mut digits = [0; DIGITS_LEN];
    digits[0] = 1;

    digits = exponentiate_by_squaring(digits, SKIP_POW_2);

    let mut i: u64 = SKIP_POW_2;
    println!("checking last {} digits", DIGITS_LEN);

    let mut output = stdout();

    loop {
        if is_only_pow2(digits) {
            println!("\nfound 2^{}", i);
            return;
        }

        if i % 1_000_000_000 == 0 {
            print!("\rScanned {} billion", i / 1_000_000_000);
            output.flush().unwrap();
        }

        digits = mul_2_digits(digits);
        i += 1;
    }
}

// square a number, using the naive multiplication algorithm
fn square_digits(digits: [u8; DIGITS_LEN]) -> [u8; DIGITS_LEN] {
    let mut new_digits: [u8; DIGITS_LEN] = [0; DIGITS_LEN];

    // distribute one number to the digits of the other, eg 34^2 = (34)(34) = 34(30) + 34(4)
    for i in 0..DIGITS_LEN {
        let digits_multiplied = mul_digits_by_digit(digits, digits[i]);
        // since multiplying by 10^n just means skipping n digits, we start adding the digits column by column at an offset
        for x in 0..DIGITS_LEN - i {
            let new_digit_value = new_digits[x + i] + digits_multiplied[x];
            let digit = new_digit_value % 10;
            let carry = (new_digit_value - digit) / 10;

            new_digits[x + i] = digit;

            if x + i + 1 != DIGITS_LEN {
                new_digits[x + i + 1] += carry;
            }
        }
    }

    return new_digits;
}

// digits are stored back to front, least significant in the front
fn mul_digits_by_digit(digits: [u8; DIGITS_LEN], n: u8) -> [u8; DIGITS_LEN] {
    let mut new_digits: [u8; DIGITS_LEN] = [0; DIGITS_LEN];

    digits.iter().enumerate().for_each(|(i, &item)| {
        // guaranteed to fit in, as item <= 9, n <= 9, and <=9 * <=9 = <= 81 < u8::MAX = 127
        let new_digit_value = item * n;
        let digit = new_digit_value % 10;
        let carry = (new_digit_value - digit) / 10;

        new_digits[i] += digit;

        // if currently at the DIGITS_LENth digit, don't set next (as it is out of bounds)
        // otherwise, set the next digit to the carry otherwise
        if i + 1 != DIGITS_LEN {
            new_digits[i + 1] = carry;
        }
    });

    new_digits
}

// computes base^exponent mod modulus, in O(log(exponent)) multiplications
// https://en.wikipedia.org/wiki/Exponentiation_by_squaring
// recurse log_2(exponent) times
fn exponentiate_by_squaring(base: [u8; DIGITS_LEN], exponent: u64) -> [u8; DIGITS_LEN] {
    if exponent == 0 {
        return base;
    }

    if exponent % 2 == 0 {
        // 2^{2x} = (2^{x})^{2}
        // recursed_value = 2^{x}, so we need to return square(recursed_value)
        let recursed_value = exponentiate_by_squaring(base, exponent / 2);

        return square_digits(recursed_value);
    } else {
        // 2^{2x + 1} = 2(2^{x})^{2}
        // we need to first square, then multiply by two

        let recursed_value = exponentiate_by_squaring(base, (exponent - 1) / 2);

        return mul_digits_by_digit(square_digits(recursed_value), 2);
    }
}

// digits are stored back to front, least significant in the front
fn mul_2_digits(digits: [u8; DIGITS_LEN]) -> [u8; DIGITS_LEN] {
    let mut new_digits: [u8; DIGITS_LEN] = [0; DIGITS_LEN];

    digits.iter().enumerate().for_each(|(i, &item)| {
        // guaranteed to fit in, as item <= 9 and 2*item <= 18 < 127
        let new_digit_value = item * 2;
        let digit = new_digit_value % 10;
        let carry = (new_digit_value - digit) / 10;

        new_digits[i] += digit;

        // if currently at the DIGITS_LENth digit, don't set next (as it is out of bounds)
        // otherwise, set the next digit to the carry otherwise
        if i + 1 != DIGITS_LEN {
            new_digits[i + 1] = carry;
        }
    });

    new_digits
}

fn is_only_pow2(digits: [u8; DIGITS_LEN]) -> bool {
    digits.iter().all(|&m| m == 1 || m == 2 || m == 4 || m == 8)
}
