# Simple Pass Gen 
Simple Pass Gen is a simple password generator library (source code can be found ![here](https://github.com/goto-eof/simple-pass-gen)).

# How to use it?
In order to use it first of all add simple-pass-gen to your Cargo.toml file:
```
simple-pass-gen = "0.1.8"
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
or if you want to generate an easy to remember password
```
pub fn generate_mnemonic_password(password_length: i32) 
-> Result<String, SimplePassGenError>
```

# Example
```
use simple_pass_gen::generate_mnemonic_password;
use simple_pass_gen::generate_password;

fn main() {
    let result = generate_mnemonic_password(5).unwrap();
    println!("{}", result);
    
    let result2 = generate_password(5, true, true, true, false).unwrap();
    println!("{}", result2);
}
```

p.s. this is my first library on crates.io (: