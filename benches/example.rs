use criterion::{criterion_group, criterion_main, Criterion};
use std::io::{stdout, Write};

const DIGITS_LEN: usize = 40;
// how many iterations of the algorithm to run
const BENCHMARK_LOOP_LENGTH: usize = 1_000_000;

fn is_only_pow2(digits: [u8; DIGITS_LEN]) -> bool {
    digits.iter().all(|&m| m == 1 || m == 2 || m == 4 || m == 8)
}

fn basic_multiplication(bench: &mut Criterion) {
    let mut digits = [0; DIGITS_LEN];
    digits[0] = 1;
    let mut i: u64 = 0;

    let mut output = stdout();

    bench.bench_function("basic", |b| {
        b.iter(|| {
            for _ in 0..BENCHMARK_LOOP_LENGTH {
                if is_only_pow2(digits) {
                    println!("\nfound 2^{}", i);
                    return;
                }

                if i & 1_000_000_000 == 0 {
                    print!("\rScanned {} billion", i / 1_000_000_000);
                    output.flush().unwrap();
                }

                digits = basic_multiplication_mul_2_digits(digits);
                i += 1;
            }
        })
    });
}

fn basic_multiplication_mul_2_digits(digits: [u8; DIGITS_LEN]) -> [u8; DIGITS_LEN] {
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

fn simplified_basic_multiplication(bench: &mut Criterion) {
    let mut digits = [0; DIGITS_LEN];
    digits[0] = 1;
    let mut i: u64 = 0;

    let mut output = stdout();

    bench.bench_function("simplified", |b| {
        b.iter(|| {
            for _ in 0..BENCHMARK_LOOP_LENGTH {
                if is_only_pow2(digits) {
                    println!("\nfound 2^{}", i);
                    return;
                }

                if i & 1_000_000_000 == 0 {
                    print!("\rScanned {} billion", i / 1_000_000_000);
                    output.flush().unwrap();
                }

                digits = simplified_multiplication_mul_2_digits(digits);
                i += 1;
            }
        })
    });
}

fn simplified_multiplication_mul_2_digits(digits: [u8; DIGITS_LEN]) -> [u8; DIGITS_LEN] {
    let mut new_digits: [u8; DIGITS_LEN] = [0; DIGITS_LEN];

    digits.iter().enumerate().for_each(|(i, &item)| {
        // guaranteed to fit in, as item <= 9 and 2*item <= 18 < 127
        new_digits[i] += (item * 2) % 10;

        // if currently at the DIGITS_LENth digit, don't set next (as it is out of bounds)
        // otherwise, set the next digit to the carry otherwise
        if i + 1 != DIGITS_LEN {
            new_digits[i + 1] = (item >= 5).into();
        }
    });

    new_digits
}

criterion_group!(
    benches,
    basic_multiplication,
    simplified_basic_multiplication
);
criterion_main!(benches);
