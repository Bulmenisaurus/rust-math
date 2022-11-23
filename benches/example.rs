use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::io::{stdout, Write};

const DIGITS_LEN: usize = 40;
// how many iterations of the algorithm to run
const BENCHMARK_LOOP_LENGTH: usize = 50_000_000;

fn is_only_pow2(digits: [u8; DIGITS_LEN]) -> bool {
    digits.iter().all(|&m| m == 1 || m == 2 || m == 4 || m == 8)
}

fn basic_multiplication(bench: &mut Criterion) {
    let mut output = stdout();

    bench.bench_function("basic", |b| {
        b.iter(|| {
            let mut digits = [0; DIGITS_LEN];
            digits[0] = 1;
            let mut i: u64 = 0;

            for _ in 0..BENCHMARK_LOOP_LENGTH {
                if is_only_pow2(digits) {
                    println!("\nfound 2^{}", i);
                    return;
                }

                if i & 1_000_000_000 == 0 {
                    // print!("\rScanned {} billion", i / 1_000_000_000);
                    output.flush().unwrap();
                }

                digits = basic_multiplication_mul_2_digits(digits);
                i += 1;
            }
            black_box(digits[0]);
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

// lookup table for mod(2x, 10)
const TIMES_2_MOD_10: [u8; 10] = [0, 2, 4, 6, 8, 0, 2, 4, 6, 8];

// lookup table for (2x - mod(2x, 10))/10
const TIMES_2_MOD_10_CARRY: [u8; 10] = [0, 0, 0, 0, 0, 1, 1, 1, 1, 1];

fn lookup_multiplication(bench: &mut Criterion) {
    let mut output = stdout();

    bench.bench_function("lookup", |b| {
        b.iter(|| {
            let mut digits = [0; DIGITS_LEN];
            digits[0] = 1;
            let mut i: u64 = 0;

            for _ in 0..BENCHMARK_LOOP_LENGTH {
                if is_only_pow2(digits) {
                    println!("\nfound 2^{}", i);
                    return;
                }

                if i & 1_000_000_000 == 0 {
                    // print!("\rScanned {} billion", i / 1_000_000_000);
                    output.flush().unwrap();
                }

                digits = lookup_multiplication_mul_2_digits(digits);
                i += 1;
            }
            black_box(digits[0]);
        })
    });
}

fn lookup_multiplication_mul_2_digits(digits: [u8; DIGITS_LEN]) -> [u8; DIGITS_LEN] {
    let mut new_digits: [u8; DIGITS_LEN] = [0; DIGITS_LEN];

    digits.iter().enumerate().for_each(|(i, &item)| {
        // guaranteed to fit in, as item <= 9 and 2*item <= 18 < 127

        let new_digit: u8 = TIMES_2_MOD_10[usize::from(item)];
        new_digits[i] += new_digit;

        // if currently at the DIGITS_LENth digit, don't set next (as it is out of bounds)
        // otherwise, set the next digit to the carry otherwise
        if i + 1 != DIGITS_LEN {
            new_digits[i + 1] = TIMES_2_MOD_10_CARRY[usize::from(item)];
        }
    });

    new_digits
}

const LOOKUP_ARRAY: [bool; 100] = [
    false, // 8
    false, // 16
    false, // 32
    false, // 64
    true,  // 128
    false, // 256
    false, // 512
    false, // 24
    false, // 48
    false, // 96
    false, // 192
    false, // 384
    false, // 768
    false, // 536
    false, // 72
    true,  // 144
    true,  // 288
    false, // 576
    false, // 152
    false, // 304
    false, // 608
    false, // 216
    false, // 432
    false, // 864
    false, // 728
    false, // 456
    false, // 912
    true,  // 824
    false, // 648
    false, // 296
    false, // 592
    true,  // 184
    false, // 368
    false, // 736
    false, // 472
    false, // 944
    true,  // 888
    false, // 776
    false, // 552
    false, // 104
    false, // 208
    false, // 416
    false, // 832
    false, // 664
    false, // 328
    false, // 656
    false, // 312
    false, // 624
    true,  // 248
    false, // 496
    false, // 992
    false, // 984
    false, // 968
    false, // 936
    false, // 872
    false, // 744
    true,  // 488
    false, // 976
    false, // 952
    false, // 904
    false, // 808
    false, // 616
    false, // 232
    false, // 464
    false, // 928
    false, // 856
    false, // 712
    true,  // 424
    true,  // 848
    false, // 696
    false, // 392
    false, // 784
    false, // 568
    false, // 136
    false, // 272
    false, // 544
    false, // 88
    false, // 176
    false, // 352
    false, // 704
    false, // 408
    false, // 816
    false, // 632
    false, // 264
    false, // 528
    false, // 56
    true,  // 112
    true,  // 224
    true,  // 448
    false, // 896
    false, // 792
    false, // 584
    false, // 168
    false, // 336
    false, // 672
    false, // 344
    false, // 688
    false, // 376
    false, // 752
    false, // 504
];

fn modular_multiplication(bench: &mut Criterion) {
    let mut output = stdout();

    bench.bench_function("modular", |b| {
        b.iter(|| {
            let mut digits = [0; DIGITS_LEN];
            digits[0] = 1;
            let mut i: u64 = 0;

            for _ in 0..(BENCHMARK_LOOP_LENGTH / 100) {
                for x in 0..100 {
                    if LOOKUP_ARRAY[x] && is_only_pow2(digits) {
                        println!("\nfound 2^{}", i);
                        return;
                    }

                    digits = modular_multiplication_mul_2_digits(digits);
                    i += 1;
                }

                if i & 1_000_000_000 == 0 {
                    // print!("\rScanned {} billion", i / 1_000_000_000);
                    output.flush().unwrap();
                }
            }
            black_box(digits[0]);
        })
    });
}

fn modular_multiplication_mul_2_digits(digits: [u8; DIGITS_LEN]) -> [u8; DIGITS_LEN] {
    basic_multiplication_mul_2_digits(digits)
}

fn hybrid_multiplication(bench: &mut Criterion) {
    let mut output = stdout();

    bench.bench_function("hybrid", |b| {
        b.iter(|| {
            let mut digits = [0; DIGITS_LEN];
            digits[0] = 1;
            let mut i: u64 = 0;

            for _ in 0..(BENCHMARK_LOOP_LENGTH / 100) {
                for x in 0..100 {
                    if LOOKUP_ARRAY[x] && is_only_pow2(digits) {
                        println!("\nfound 2^{}", i);
                        return;
                    }

                    digits = hybrid_multiplication_mul_2_digits(digits);
                    i += 1;
                }

                if i & 1_000_000_000 == 0 {
                    // print!("\rScanned {} billion", i / 1_000_000_000);
                    output.flush().unwrap();
                }
            }
            black_box(digits[0]);
        })
    });
}

fn hybrid_multiplication_mul_2_digits(digits: [u8; DIGITS_LEN]) -> [u8; DIGITS_LEN] {
    let mut new_digits: [u8; DIGITS_LEN] = [0; DIGITS_LEN];

    digits.iter().enumerate().for_each(|(i, &item)| {
        // guaranteed to fit in, as item <= 9 and 2*item <= 18 < 127

        let new_digit: u8 = TIMES_2_MOD_10[usize::from(item)];
        new_digits[i] += new_digit;

        // if currently at the DIGITS_LENth digit, don't set next (as it is out of bounds)
        // otherwise, set the next digit to the carry otherwise
        if i + 1 != DIGITS_LEN {
            new_digits[i + 1] = TIMES_2_MOD_10_CARRY[usize::from(item)];
        }
    });

    new_digits
}

criterion_group!(
    benches,
    basic_multiplication,
    lookup_multiplication,
    modular_multiplication,
    hybrid_multiplication
);
criterion_main!(benches);
