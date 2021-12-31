## Rust Password Generator

This library implements generation of random passwords with provided
requirements as described by  [AgileBits
1Password](https://discussions.agilebits.com/discussion/23842/how-random-are-the-generated-passwords)
in pure Rust. The algorithm is commonly used when generating website
passwords.

The library uses rand/OsRng for added randomness.

Sample example passwords this library may generate:

```text
0N[k9PhDqmmfaO`p_XHjVv`HTq|zsH4XiH8umjg9JAGJ#\Qm6lZ,28XF4{X?3sHj
7@90|0H7!4p\,c<!32:)0.9N
UlYuRtgqyWEivlXnLeBpZvIQ
Q795Im1VR5h363s48oZGaLDa
wpvbxlsc
```

> Since these are completely randomized, it's possible that they may generate passwords that don't comply with some custom password policies, such as ones that require both upper case AND lower case letters. If your particular use case needs a mix of casing, then you can either increase the number of characters in the password or check the output and regenerate if it fails a particular constraint, such as requiring both upper and lower case.

## Installation

```rust
[dependencies]
rust_password = { git = "https://github.com/antonmisa/rust_password" }
```

## Usage

```rust


pub fn main() {
  // Generate a password that is 64 characters long with 10 digits, 10 symbols,
  // allowing upper and lower case letters, disallowing repeat characters.
  let password: Generator = Generator::new(&None);
  match password.generate(64, 10, 10, false, false) {
    Ok(x) => x,
    Err(e) => log::error!("{}", e),
  };
}
```

## License

This code is licensed under the MIT license.
