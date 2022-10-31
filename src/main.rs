use std::cmp;

fn main() {
    let mut digits: Vec<u8> = vec![1];
    let mut i: u64 = 0;
    let digits_len: usize = 20;
    println!("checking last {} digits", digits_len);

    loop {
        if is_only_pow2(digits.clone()) {
            println!("found 2^{}", i);
            // not 128
            if i > 7 {
                break;
            }
        }

        if i % 10_000_000 == 0 {
            println!("Scanned {}", i);
        }
        
        digits = mul_2_digits(digits.clone(), digits_len);
        i += 1;
    }
}

// digits are stored back to front, least significant in the front
fn mul_2_digits(digits: Vec<u8>, max_length: usize) -> Vec<u8> {
    let mut new_digits: Vec<u8> = vec![0; cmp::min(digits.len(), max_length)];

    
    digits.iter().enumerate().for_each(|(i, item)| {
        if i >= max_length {
            return;
        }

        let new_digit_value = item * 2;
        let digit = new_digit_value % 10;
        let carry = (new_digit_value - digit) / 10;

        new_digits[i] += digit;

        // clamp to 20
        if carry == 0 || i + 1 == max_length {
            return;
        }

        if new_digits.len() < i+2{
            new_digits.push(0);
        }
        new_digits[i + 1] = carry;
    });

    new_digits
}

fn is_only_pow2(digits: Vec<u8>) -> bool {
    digits.iter().all(|&m| { 
         m == 1u8 || m == 2u8 || m == 4u8 || m == 8u8
    })
}