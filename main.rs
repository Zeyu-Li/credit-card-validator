/**
 * Credit Card Validator
 * A simple program to validate if a credit card is valid
 * using Luhn's algorithm and institution check
 */

/**
 * Credit card validator given unsigned 64 int
 * @param string_card_number: String
 * string version of the credit card number
 * @return: bool
 * if it is a valid credit card number
 */
fn validate_card(card_number: u64) -> bool {
    let max: u64 = 9999999999999999;
    // check length (<10e17)
    if card_number > max {
        return false;
    }

    // TODO: check if institution/issuer numbers are valid
    // if substring 0-6 in valid issuer list

    // check digit using the Luhn algorithm (see https://en.wikipedia.org/wiki/Luhn_algorithm)
    // gets digits (floor division)
    let luhn_digits = card_number / 10;

    // space for 0s
    let check_digits = format!("{:0<16}", luhn_digits.to_string().chars().rev().collect::<String>());
    // let check_digits = luhn_digits.to_string().chars().rev().collect::<String>();
    let size = check_digits.len();

    let mut sum: u32 = (card_number % (10 as u64)) as u32;

    for i in 0..size {
        let digit = check_digits.chars().nth(i).unwrap() as u32 - '0' as u32;

        // debug
        // println!("{} {}", i, digit);

        // every even number is added
        if i % 2 == 1 {
            sum += digit;
        } else {
            let new_sum = digit * 2;
            // if new sum is greater than 9 then sum the digits
            // if new_sum > 9 {
            //     sum += (check_digits.chars().nth(0).unwrap() as u32 - '0' as u32) + (check_digits.chars().nth(1).unwrap() as u32 - '0' as u32) 
            // } else {
            //     sum += new_sum;
            // }
            sum += new_sum / 10;
            sum += new_sum % 10;
        }
    }

    // get last digit
    let last = sum % 10;

    // println!("Sum is {}", sum);
    // println!("Last is {}", last);

    // if last is 0 then it is fine
    if last == 0 {
        return true;
    }
    
    // otherwise check if 10-last is the check digit
    return (card_number % (10 as u64)) as u32 == 0;
}

/**
 * Credit card validator given string
 * @param string_card_number: String
 * string version of the credit card number
 * @return: bool
 * if it is a valid credit card number
 */
fn validate_card_str(string_card_number: &str) -> bool {
    let card_number = string_card_number.parse::<u64>().unwrap();
    let max: u64 = 9999999999999999;
    // check length (<10e17)
    if card_number > max {
        return false;
    }

    // check digit using the Luhn algorithm (see https://en.wikipedia.org/wiki/Luhn_algorithm)
    // gets digits (floor division)
    let luhn_digits = card_number / 10;

    let check_digits = format!("{:0<16}", luhn_digits.to_string().chars().rev().collect::<String>());
    let size = check_digits.len();

    let mut sum: u32 = (card_number % (10 as u64)) as u32;

    for i in 0..size {
        let digit = check_digits.chars().nth(i).unwrap() as u32 - '0' as u32;

        // debug
        // println!("{} {}", i, digit);

        // every even number is added
        if i % 2 == 1 {
            sum += digit;
        } else {
            let new_sum = digit * 2;
            // if new sum is greater than 9 then sum the digits
            // if new_sum > 9 {
            //     sum += (check_digits.chars().nth(0).unwrap() as u32 - '0' as u32) + (check_digits.chars().nth(1).unwrap() as u32 - '0' as u32) 
            // } else {
            //     sum += new_sum;
            // }
            sum += new_sum / 10;
            sum += new_sum % 10;
        }
    }

    // get last digit
    let last = sum % 10;

    // println!("Sum is {}", sum);
    // println!("Last is {}", last);

    // if last is 0 then it is fine
    if last == 0 {
        return true;
    }
    
    // otherwise check if 10-last is the check digit
    return (card_number % (10 as u64)) as u32 == 0;
}

fn main() {
    let card_number_1: u64 = 4538300023446815;
    let card_number_2: u64 = 4538300013446812;
    let card_number_3: &str = "4538300023446815";

    let valid_state_1 = if validate_card(card_number_1) {"valid"} else {"invalid"};
    let valid_state_2 = if validate_card(card_number_2) {"valid"} else {"invalid"};
    let valid_state_3 = if validate_card_str(card_number_3) {"valid"} else {"invalid"};

    println!("The card {} is {}", card_number_1, valid_state_1);
    println!("The card {} is {}", card_number_2, valid_state_2);
    println!("The card {} is {}", card_number_3, valid_state_3);
}
