use rand::distributions::Uniform;
use rand::prelude::Distribution;
use structs::SimplePassGenError;

mod structs;

pub fn generate_password(
    password_length: i32,
    include_upper_case_charset: bool,
    include_lower_case_charset: bool,
    include_numbers_charset: bool,
    include_symbols_charset: bool,
) -> Result<String, SimplePassGenError> {
    if password_length <= 0 || password_length > 256 {
        return Err(SimplePassGenError::new("invalid password length"));
    }
    if !include_upper_case_charset
        && !include_lower_case_charset
        && !include_numbers_charset
        && !include_symbols_charset
    {
        return Err(SimplePassGenError::new(
            "at least one charset should be enabled",
        ));
    }

    let charset = build_final_charset(
        include_upper_case_charset,
        include_lower_case_charset,
        include_numbers_charset,
        include_symbols_charset,
    );

    let mut rng = rand::thread_rng();
    let die = Uniform::from(0..charset.len() - 2);
    let mut password = "".to_owned();
    for _n in 1..password_length + 1 {
        let throw = die.sample(&mut rng);
        let mut char = charset.chars();
        let char = char.nth(throw).unwrap();
        password = format!("{}{}", password, char);
    }
    return Ok(password);
}

fn build_final_charset(
    include_upper_case_charset: bool,
    include_lower_case_charset: bool,
    include_numbers_charset: bool,
    include_symbols_charset: bool,
) -> String {
    let low_case = "abcdefghijklmnopqrstuvxyz";
    let up_case = "ABCDEFGHIJKLMNOPQRSTUVXYZ";
    let numbers = "0123456789";
    let symbols = "!£$%&/()=?^*°:;.,";

    let mut all = "".to_owned();

    if include_upper_case_charset {
        all = format!("{}{}", all, up_case);
    }

    if include_lower_case_charset {
        all = format!("{}{}", all, low_case);
    }

    if include_numbers_charset {
        all = format!("{}{}", all, numbers);
    }

    if include_symbols_charset {
        all = format!("{}{}", all, symbols);
    }
    all
}

pub fn generate_mnemonic_password(password_length: i32) -> Result<String, SimplePassGenError> {
    if password_length <= 0 || password_length > 256 {
        return Err(SimplePassGenError::new("invalid password length"));
    }
    let consonants_lc = "bcdfghjklmnpqrstuvxyz";
    let vowels_lc = "aeiou";

    let mut password = "".to_owned();

    for i in 1..password_length + 1 {
        if i % 2 == 0 {
            let mut rng = rand::thread_rng();
            let die = Uniform::from(0..vowels_lc.len() - 1);
            let throw = die.sample(&mut rng);
            let mut char = vowels_lc.chars();
            let mut char = char.nth(throw).unwrap();
            if throw % 2 == 1 {
                char = char.to_ascii_uppercase();
            }

            password = format!("{}{}", password, char);
        } else {
            let mut rng = rand::thread_rng();
            let die = Uniform::from(0..consonants_lc.len() - 1);
            let throw = die.sample(&mut rng);
            let mut char = consonants_lc.chars();
            let mut char = char.nth(throw).unwrap();
            if throw % 2 == 1 {
                char = char.to_ascii_uppercase();
            }
            password = format!("{}{}", password, char);
        }
    }
    return Ok(password);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_pass_if_password_lenght_equals_five() {
        let result = generate_password(5, true, true, true, false);
        assert_eq!(result.unwrap().len(), 5);
    }

    #[test]
    fn should_fail_if_no_charset_selected() {
        let result = generate_password(5, false, false, false, false);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn should_fail_if_password_length_is_invalid() {
        let result = generate_password(0, false, false, false, false);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn should_fail_if_mnemonic_password_length_is_invalid() {
        let result = generate_mnemonic_password(0);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn should_fail_if_mnemonic_password_length_is_invalid_greater_than() {
        let result = generate_mnemonic_password(257);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn should_pass_if_menmonic_password_lenght_equals_five() {
        let result = generate_mnemonic_password(5);
        assert_eq!(result.unwrap().len(), 5);
    }
}
