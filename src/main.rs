const DIGITS_LEN: usize = 25;

fn main() {
    let mut digits = [-1; DIGITS_LEN];
    digits[0] = 1;
    let mut i: u64 = 0;
    println!("checking last {} digits", DIGITS_LEN);

    loop {
        if is_only_pow2(digits) {
            println!("found 2^{}", i);
            // not 128
            if i > 7 {
                break;
            }
        }

        if i % 10_000_000 == 0 {
            println!("Scanned {}", i);
        }
        
        digits = mul_2_digits(digits);
        i += 1;
    }
}

// digits are stored back to front, least significant in the front
fn mul_2_digits(digits: [i8; DIGITS_LEN]) -> [i8; DIGITS_LEN] {
    let mut new_digits: [i8; DIGITS_LEN] = [-1; DIGITS_LEN];

    // manually set the first digit to 0, as loop sets next digit with carry calculations
    new_digits[0] = 0;

    
    digits.iter().enumerate().for_each(|(i, &item)| {
        // if the digit is unset, stop (0 * 2 = 0)
        if item == -1 {
            return;
        }

        // guaranteed to fit in, as item <= 9 and 2*item <= 18 < 127
        let new_digit_value = item * 2;
        let digit = new_digit_value % 10;
        let carry = (new_digit_value - digit) / 10;

        if new_digits[i] == -1 {
            new_digits[i] = digit;
        } else {
            new_digits[i] += digit;
        }


        let next_digit = carry;
        // if currently at the DIGITS_LENth digit, don't set next (as it is out of bounds)
        // otherwise, set the next digit to the carry otherwise
        if i + 1 != DIGITS_LEN && carry != 0 {
            new_digits[i + 1] = next_digit;
        }
    });

    new_digits
}

fn is_only_pow2(digits: [i8; DIGITS_LEN]) -> bool {
    digits.iter().all(|&m| { 
         m == 1 || m == 2|| m == 4|| m == 8 || m == -1 // unset
    })
}