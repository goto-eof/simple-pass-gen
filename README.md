# Simple Pass Gen 
Simple Pass Gen is a Rust library for password generation.

# How to use it?
In order to use it first of all add simple-pass-gen to your toml file:
```
simple-pass-gen = "0.1.5"
```

Then call one of two available functions:
```
pub fn generate_password(
    password_length: i32,
    include_upper_case: bool,
    include_lower_case: bool,
    include_numbers: bool,
    include_symbols: bool,
) -> Result<String, SimplePassGenError>;
```
or 
```
pub fn generate_mnemonic_password(password_length: i32) 
-> Result<String, SimplePassGenError>
```

# Example
```
use simple_pass_gen::generate_mnemonic_password;
use simple_pass_gen::generate_password;

fn main() {
    println!("{}", generate_mnemonic_password(5).unwrap());

    println!("{}", generate_password(5, true, true, true, false).unwrap());
}
```

p.s. this is my first library on crates.io :)