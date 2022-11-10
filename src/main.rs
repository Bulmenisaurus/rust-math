use std::io::{stdout, Write};

const DIGITS_LEN: usize = 34;

fn main() {
    let mut digits = [0; DIGITS_LEN];
    digits[0] = 1;
    let mut i: u64 = 0;
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
