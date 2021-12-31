use rand::Rng;
use std::error::Error;
use std::fmt;
use std::num::TryFromIntError;
use std::any::Any;

#[derive(Debug, PartialEq)]
pub struct ExceedsTotalLengthError();

impl Error for ExceedsTotalLengthError {}

impl fmt::Display for ExceedsTotalLengthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", GeneratorErrorConst::ERR_EXCEEDS_TOTAL_LENGTH)
    }
}

#[derive(Debug, PartialEq)]
pub struct LettersExceedsAvailableError();

impl Error for LettersExceedsAvailableError {}

impl fmt::Display for LettersExceedsAvailableError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", GeneratorErrorConst::ERR_LETTERS_EXCEEDS_AVAILABLE)
    }
}

#[derive(Debug, PartialEq)]
pub struct DigitsExceedsAvailableError();

impl Error for DigitsExceedsAvailableError {}

impl fmt::Display for DigitsExceedsAvailableError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", GeneratorErrorConst::ERR_DIGITS_EXCEEDS_AVAILABLE)
    }
}

#[derive(Debug, PartialEq)]
pub struct SymbolsExceedsAvailableError();

impl Error for SymbolsExceedsAvailableError {}

impl fmt::Display for SymbolsExceedsAvailableError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", GeneratorErrorConst::ERR_SYMBOLS_EXCEEDS_AVAILABLE)
    }
}

pub trait Errorable {
	fn as_any(&self) -> &dyn Any;
}

#[derive(Debug)]
pub enum GeneratorError {
	ParseRnd(rand::Error),
	ParseExceedsTotalLengthError(ExceedsTotalLengthError),
	ParseLettersExceedsAvailableError(LettersExceedsAvailableError),
	ParseDigitsExceedsAvailableError(DigitsExceedsAvailableError),
	ParseSymbolsExceedsAvailableError(SymbolsExceedsAvailableError),
	ParseTryFromIntError(TryFromIntError),
}

impl Errorable for GeneratorError {
	fn as_any(&self) -> &dyn Any {
		self
	}
}

impl From<rand::Error> for GeneratorError {
	fn from(err: rand::Error) -> GeneratorError {
		GeneratorError::ParseRnd(err)
	}
}

impl From<TryFromIntError> for GeneratorError {
	fn from(err: TryFromIntError) -> GeneratorError {
		GeneratorError::ParseTryFromIntError(err)
	}
}

impl From<ExceedsTotalLengthError> for GeneratorError {
	fn from(err: ExceedsTotalLengthError) -> GeneratorError {
		GeneratorError::ParseExceedsTotalLengthError(err)
	}
}

impl From<LettersExceedsAvailableError> for GeneratorError {
	fn from(err: LettersExceedsAvailableError) -> GeneratorError {
		GeneratorError::ParseLettersExceedsAvailableError(err)
	}
}

impl From<DigitsExceedsAvailableError> for GeneratorError {
	fn from(err: DigitsExceedsAvailableError) -> GeneratorError {
		GeneratorError::ParseDigitsExceedsAvailableError(err)
	}
}

impl From<SymbolsExceedsAvailableError> for GeneratorError {
	fn from(err: SymbolsExceedsAvailableError) -> GeneratorError {
		GeneratorError::ParseSymbolsExceedsAvailableError(err)
	}
}

impl fmt::Display for GeneratorError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			GeneratorError::ParseRnd(ref err) => write!(f, "Rnd error: {}", err),
			GeneratorError::ParseTryFromIntError(ref err) => write!(f, "int32 to usize error: {}", err),
			GeneratorError::ParseExceedsTotalLengthError(ref err) => write!(f, "{}", err),
			GeneratorError::ParseLettersExceedsAvailableError(ref err) => write!(f, "{}", err),
			GeneratorError::ParseDigitsExceedsAvailableError(ref err) => write!(f, "{}", err),
			GeneratorError::ParseSymbolsExceedsAvailableError(ref err) => write!(f, "{}", err),
		}
	}
}

// PasswordGenerator is an interface that implements the Generate function. This
// is useful for testing where you can pass this interface instead of a real
// password generator to mock responses for predicability.
pub trait PasswordGenerator {
	fn generate(&self, length: usize, num_gigits: usize, num_symbols: usize, no_upper: bool, allow_repeat: bool) -> Result<String, GeneratorError>;
}

pub struct GeneratorErrorConst;

impl GeneratorErrorConst {
	// ErrExceedsTotalLength is the error returned with the number of digits and
	// symbols is greater than the total length.
	pub const ERR_EXCEEDS_TOTAL_LENGTH: &'static str = "number of digits and symbols must be less than total length";

	// ErrLettersExceedsAvailable is the error returned with the number of letters
	// exceeds the number of available letters and repeats are not allowed.
	pub const ERR_LETTERS_EXCEEDS_AVAILABLE: &'static str = "number of letters exceeds available letters and repeats are not allowed";

	// ErrDigitsExceedsAvailable is the error returned with the number of digits
	// exceeds the number of available digits and repeats are not allowed.
	pub const ERR_DIGITS_EXCEEDS_AVAILABLE: &'static str = "number of digits exceeds available digits and repeats are not allowed";

	// ErrSymbolsExceedsAvailable is the error returned with the number of symbols
	// exceeds the number of available symbols and repeats are not allowed.
	pub const ERR_SYMBOLS_EXCEEDS_AVAILABLE: &'static str = "number of symbols exceeds available symbols and repeats are not allowed";	
}

pub struct GeneratorConst;

impl GeneratorConst {
	// LowerLetters is the list of lowercase letters.
	pub const LOWER_LETTERS: &'static str = "abcdefghijklmnopqrstuvwxyz";
	
	// UpperLetters is the list of uppercase letters.
	pub const UPPER_LETTERS: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
	
	// Digits is the list of permitted digits.
	pub const DIGITS: &'static str = "0123456789";
	
	// Symbols is the list of symbols.
	pub const SYMBOLS: &'static str = "~!@#$%^&*()_+`-={}|[]\\:\"<>?,./";
}

// Generator is the stateful generator which can be used to customize the list
// of letters, digits, and/or symbols.
pub struct Generator<'a> {
	lower_letters: &'a str,
	upper_letters: &'a str,
	digits:       &'a str,
	symbols:      &'a str,
}

// GeneratorInput is used as input to the NewGenerator function.
pub struct GeneratorInput<'a> {
	pub lower_letters: &'a str,
	pub upper_letters: &'a str,	
	pub digits:       &'a str,
	pub symbols:      &'a str,
}

// NewGenerator creates a new Generator from the specified configuration. If no
// input is given, all the default values are used. This function is safe for
// concurrent use.
impl<'a> Generator<'a> {
	pub fn new(i: &'a Option<&GeneratorInput>) -> Result<Generator<'a>, GeneratorError> {
		let gi: &GeneratorInput =  i.unwrap_or(&GeneratorInput{lower_letters: "", upper_letters: "", digits: "", symbols: ""});

		let mut g = Generator{
			lower_letters: gi.lower_letters,
			upper_letters: gi.upper_letters,
			digits:       gi.digits,
			symbols:      gi.symbols,
		};

		if g.lower_letters == "" {
			g.lower_letters = GeneratorConst::LOWER_LETTERS;
		};

		if g.upper_letters == "" {
			g.upper_letters = GeneratorConst::UPPER_LETTERS;
		};

		if g.digits == "" {
			g.digits = GeneratorConst::DIGITS;
		};

		if g.symbols == "" {
			g.symbols = GeneratorConst::SYMBOLS;
		};

		Ok(g)
	}
}

// Generate generates a password with the given requirements. length is the
// total number of characters in the password. numDigits is the number of digits
// to include in the result. numSymbols is the number of symbols to include in
// the result. noUpper excludes uppercase letters from the results. allowRepeat
// allows characters to repeat.
//
// The algorithm is fast, but it's not designed to be performant; it favors
// entropy over speed. This function is safe for concurrent use.
impl<'a> PasswordGenerator for Generator<'a> {
	fn generate(&self, length: usize, num_digits: usize, num_symbols: usize, no_upper: bool, allow_repeat: bool) -> Result<String, GeneratorError> {
		let mut letters: String = self.lower_letters.to_owned();
		if !no_upper {
			letters.push_str(self.upper_letters);
		};

		let chars: usize;
		match length.checked_sub(num_digits + num_symbols) {
			Some(x) => chars = x,
			None => return Err(GeneratorError::ParseExceedsTotalLengthError(ExceedsTotalLengthError())),
		};

		if !allow_repeat && chars > letters.len() {
			return Err(GeneratorError::ParseLettersExceedsAvailableError(LettersExceedsAvailableError()));
		};

		if !allow_repeat && num_digits > self.digits.len() {
			return Err(GeneratorError::ParseDigitsExceedsAvailableError(DigitsExceedsAvailableError()));
		};

		if !allow_repeat && num_symbols > self.symbols.len() {
			return Err(GeneratorError::ParseSymbolsExceedsAvailableError(SymbolsExceedsAvailableError()));
		};

		let mut result: String = String::with_capacity(length);
		
		// Characters
		let mut i: usize = 0;
		while i < chars {
			let ch: &str = random_element(&letters)?;

			if !allow_repeat && result.contains(ch) {
				continue;
			};

			random_insert(&mut result, ch)?;
			i = i + 1;
		};

		// Digits
		i = 0;
		while i < num_digits {
			let d: &str = random_element(self.digits)?;

			if !allow_repeat && result.contains(d) {
				continue;
			};

			random_insert(&mut result, d)?;
			i = i + 1;
		};

		// Symbols
		i = 0;
		while i < num_symbols {
			let sym: &str = random_element(self.symbols)?;

			if !allow_repeat && result.contains(sym) {
				continue;
			};

			random_insert(&mut result, sym)?;
			i = i + 1;
		};

		Ok(result)
	}
}

// Generate is the package shortcut for Generator.Generate.
pub fn generate<'a>(length: usize, num_gigits: usize, num_symbols: usize, no_upper: bool, allow_repeat: bool) -> Result<String, GeneratorError> {
	let gen: Generator = Generator::new(&None)?;

	gen.generate(length, num_gigits, num_symbols, no_upper, allow_repeat)
}

// randomInsert randomly inserts the given value into the given string.
pub fn random_insert<'a>(s: &mut String, val: &'a str) -> Result<(), GeneratorError> {
	if s == "" {
		s.push_str(val);
		return Ok(())
	}

	let n: usize = rand::rngs::OsRng.gen_range(0..s.chars().count());
    s.push_str(val);

	unsafe {
		let bytes = s.as_bytes_mut();
		let el = bytes[bytes.len()-1];

		for i in 0..bytes.len() - n - 1 { 
			bytes[bytes.len() - i - 1] = bytes[bytes.len() - i - 2];
		}
		bytes[n] = el;
	}

    Ok(())
}

// randomElement extracts a random element from the given string.
pub fn random_element<'a>(s: &'a str) -> Result<&'a str, GeneratorError> {
	let count = s.chars().count();
    let n: usize = rand::rngs::OsRng.gen_range(0..count);

	if n == 0 {
		Ok(&s[0..1])
	}
	else if n == count {
		Ok(&s[n-1..n])
	}
	else {
		Ok(&s[n..n+1])
	}
}
