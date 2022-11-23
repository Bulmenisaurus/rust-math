//TODO: use this in the implementation of src/main.rs
// this can be used in two ways:
// 1) Skipping already computed values. For example, once we look for 35 digits, we can skip
//    the first 1364988100834 values of two, since we know all values prior to that do not
//    satisfy 34 digits and can't possibly satisfy 35 digits. (need to be careful here, in
//    case the previous value is equal to this one, so need to compute until the exponent-1?)
//
// 2) When computing, this can be used to generate multiple values, each some distance from the
//    previous for multithreading. For example, once we have computed the first X results, start
//    4 threads:  X, X + 1B, X + 2B, X+3B, and let them each compute 1B values.

fn main() {
    let mut digits = [0; DIGITS_LEN];
    digits[0] = 1;
    println!("{:?}", pow2_digits_slow(digits, 1364988100834))
}

const DIGITS_LEN: usize = 10;

// square a number, using the naive multiplication algorithm
fn pow_2_digits(digits: [u8; DIGITS_LEN]) -> [u8; DIGITS_LEN] {
    let mut new_digits: [u8; DIGITS_LEN] = [0; DIGITS_LEN];

    // distribute one number to the digits of the other, eg 34^2 = (34)(34) = 34(30) + 34(4)
    for i in 0..DIGITS_LEN {
        let digits_multiplied = mul_digits(digits, digits[i]);
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
fn mul_digits(digits: [u8; DIGITS_LEN], n: u8) -> [u8; DIGITS_LEN] {
    let mut new_digits: [u8; DIGITS_LEN] = [0; DIGITS_LEN];

    digits.iter().enumerate().for_each(|(i, &item)| {
        // guaranteed to fit in, as item <= 9 and 9*item <= 18 < 127
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
fn pow2_digits_slow(base: [u8; DIGITS_LEN], exponent: u64) -> [u8; DIGITS_LEN] {
    if exponent == 0 {
        return base;
    }

    if exponent % 2 == 0 {
        // 2^{2x} = (2^{x})^{2}
        // recursed_value = 2^{x}, so we need to return square(recursed_value)
        let recursed_value = pow2_digits_slow(base, exponent / 2);

        return pow_2_digits(recursed_value);
    } else {
        // 2^{2x + 1} = 2(2^{x})^{2}
        // we need to first square, then multiply by two

        let recursed_value = pow2_digits_slow(base, (exponent - 1) / 2);

        return mul_digits(pow_2_digits(recursed_value), 2);
    }
}
