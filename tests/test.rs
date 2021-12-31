use rust_password::*;
use std::collections::HashSet;
use std::hash::Hash;

static N: usize = 500;

#[test]
fn test_generator_generate_custom() {

    let gen = Generator::new(&Some(&GeneratorInput{
        lower_letters: "abcde",
        upper_letters: "ABCDE",
        symbols:      "!@#$%",
        digits:       "01234",
    })).unwrap();

    for _  in 0..N {
        let res = gen.generate(52, 10, 10, false, true).unwrap();

        assert!(!res.contains("f"), "{} should only contain lower letters abcde", res);

        assert!(!res.contains("F"), "{} should only contain upper letters ABCDE", res);

        assert!(!res.contains("&"), "{} should only include symbols !@#$%%", res);

        assert!(!res.contains("5"), "{} should only contain digits 01234", res);           
    }
}

#[test]
fn test_generator_generate_exceeds_length() {    
	let gen: Generator = Generator::new(&None).unwrap();

	let mut actual_error_kind = gen.generate(0, 1, 0, false, false).unwrap_err();
    assert_eq!(ExceedsTotalLengthError().to_string(), actual_error_kind.to_string(), "expected {} to be ExceedsTotalLengthError", actual_error_kind.to_string());		

	actual_error_kind = gen.generate(0, 0, 1, false, false).unwrap_err();
	assert_eq!(ExceedsTotalLengthError().to_string(), actual_error_kind.to_string(), "expected {} to be ExceedsTotalLengthError", actual_error_kind.to_string());
}
    
#[test]
fn test_generator_generate_exceeds_letters_available() {
 	let gen: Generator = Generator::new(&None).unwrap();

    let actual_error_kind = gen.generate(1000, 0, 0, false, false).unwrap_err();
 	assert_eq!(LettersExceedsAvailableError().to_string(), actual_error_kind.to_string(), "expected {} to be LettersExceedsAvailableError", actual_error_kind.to_string());
}

#[test]
fn test_generator_generate_exceeds_digits_available() {
	let gen: Generator = Generator::new(&None).unwrap();

    let actual_error_kind = gen.generate(52, 11, 0, false, false).unwrap_err();
 	assert_eq!(DigitsExceedsAvailableError().to_string(), actual_error_kind.to_string(), "expected {} to be DigitsExceedsAvailableError", actual_error_kind.to_string());
}	 	

#[test]
fn test_generator_generate_exceeds_symbols_available() {
	let gen: Generator = Generator::new(&None).unwrap();

    let actual_error_kind = gen.generate(52, 0, 31, false, false).unwrap_err();
 	assert_eq!(SymbolsExceedsAvailableError().to_string(), actual_error_kind.to_string(), "expected {} to be SymbolsExceedsAvailableError", actual_error_kind.to_string());
}

#[test]
fn test_generator_generate_lowercase() {
	let gen: Generator = Generator::new(&None).unwrap();

	for i in 0..N {
        let res = gen.generate(i % GeneratorConst::LOWER_LETTERS.len(), 0, 0, true, true).unwrap();

        assert_eq!(res, res.to_lowercase(), "{} is not lowercase", res);          
    }
}

#[test]
fn test_generator_generate_uppercase() {
	let gen: Generator = Generator::new(&None).unwrap();

	for _ in 0..N {
        let res = gen.generate(1000, 0, 0, false, true).unwrap();

        assert_ne!(res, res.to_lowercase(), "{} does not include uppercase", res);          
    }
}

#[test]
fn test_generator_no_repeats() {
	let gen: Generator = Generator::new(&None).unwrap();

	for _ in 0..N {
        let res = gen.generate(52, 10, 30, false, false).unwrap();

        assert_eq!(has_unique_elements(res.chars()), true, "{} should not have duplicates", res);
    }
}

fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}