use std::io::{stdout, Write};

const DIGITS_LEN: usize = 33;

fn main() {
    let mut digits = [0; DIGITS_LEN];
    digits[0] = 1;
    let mut i: u64 = 0;
    println!("checking last {} digits", DIGITS_LEN);

    let mut output = stdout();

    loop {
        if check_pow2_mod_100(i) && is_only_pow2(digits) {
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

// TODO: investigate further on this, if it is better, etc
// Warning: code autogenerated by ./src/generatelookup.py since I don't know how to use macros yet
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

fn check_pow2_mod_100(n: u64) -> bool {
    // values smaller than this will never be returned to again
    if n <= 3 {
        return false;
    }

    let n: u64 = (n - 3) % 100;
    let n: usize = usize::try_from(n).unwrap();

    LOOKUP_ARRAY[n]
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
