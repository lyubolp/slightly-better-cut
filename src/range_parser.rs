//! Module, containing the range abstractions
//!
//! A range is defined by the following three items:
//!`N:M:S` - `N` is start, `M` is end (exclusive), `S` is the step.
//! Defaults - `N` = 1, `M` = last character of current line, `S` = 1.
//!
//! `N` should be less than `M`
//!
//! The range `1:8:2` would result in indexes `[1, 3, 5, 7]`
//! ```rust
//! parse_range("1:8:2", 10) => Range(1, 8, 2)
//! ```
//! `Range` is a struct that represents the range, with a start, end and a setp
//!
//! Multiple ranges are supported, if they are delimited with `,`

/// Sturct that represents a range
/// Each range has a start, end and a step
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Range {
    start: i32,
    end: i32,
    step: i32,
}

impl Range {
    pub fn new(start: i32, end: i32, step: i32) -> Self {
        //! Create a new range with a give start, end and step
        Range { start, end, step }
    }

    pub fn to_tuple(self) -> (i32, i32, i32) {
        //! Convert the `Range` object to a tuple
        (self.start, self.end, self.step)
    }
}

pub fn parse_range(input: &str, n: usize) -> Result<Vec<Range>, String> {
    //! Convert a string into a collection of `Range` objects
    //! If the `input` string is not a valid range, an `Err` is returned
    //!
    //! Split the string on `,` and parse each range separetly.
    //! If one of the ranges is not parsable, the whole input is deemed unparsable.
    let ranges = input.split(',');

    let mut result = vec![];

    for range in ranges {
        let parse_result = parse_single_range(range, n);

        match parse_result {
            Ok(range) => result.push(range),
            Err(error) => return Err(error),
        };
    }

    Ok(result)
}

fn parse_single_range(field: &str, n: usize) -> Result<Range, String> {
    let error_message = String::from("Invalid range");
    if field.is_empty() {
        return Err(error_message);
    }

    let groups: Vec<&str> = field.split(':').collect();

    let colon_count = field.match_indices(':').count();

    if colon_count < 3 {
        let parsed_start = get_parsed_item(groups[0], 0);

        let parsed_end = if colon_count != 0 {
            get_parsed_item(groups[1], n as i32)
        } else {
            match parsed_start.clone() {
                Ok(start) => Ok(start + 1),
                Err(parse_error) => Err(parse_error),
            }
        };

        let parsed_step = if colon_count == 2 {
            get_parsed_item(groups[2], 1)
        } else {
            Ok(1)
        };

        match (parsed_start, parsed_end, parsed_step) {
            (Ok(start), Ok(end), Ok(step)) => Ok(Range::new(start, end, step)),
            _ => Err(error_message),
        }
    } else {
        Err(error_message)
    }
}

fn get_parsed_item(raw_item: &str, default: i32) -> Result<i32, std::num::ParseIntError> {
    if !raw_item.is_empty() {
        raw_item.parse::<i32>()
    } else {
        Ok(default)
    }
}

#[cfg(test)]
mod unit_tests_parse_single_range {
    use super::{parse_single_range, Range};
    /*
    N:M:S

    N -> negative, positive, :
    M -> negative, positive, :, empty
    S -> negative, positive, :, empty

    positive:positive:positive
    positive:positive:negative
    positive:positive:
    positive:positive
    positive:negative
    positive:
    positive
    negative
    :positive
    ::positive
    positive:negative:positive, with two digits
    positive::positive

    Bad weather scenarios
    Basicaly

    */

    static POSITIVE_N: &str = "3";
    static NEGATIVE_N: &str = "-5";
    static POSITIVE_N_DOUBLE_DIGIT: &str = "34";

    static POSITIVE_M: &str = "5";
    static NEGATIVE_M: &str = "-3";
    static NEGATIVE_M_DOUBLE_DIGIT: &str = "-32";

    static POSITIVE_S: &str = "2";
    static NEGATIVE_S: &str = "-2";
    static POSITIVE_S_DOUBLE_DIGIT: &str = "20";

    static SEPARATOR: &str = ":";

    static POSITIVE_N_PARSED: i32 = 3;
    static NEGATIVE_N_PARSED: i32 = -5;
    static POSITIVE_N_DOUBLE_DIGIT_PARSED: i32 = 34;

    static POSITIVE_M_PARSED: i32 = 5;
    static NEGATIVE_M_PARSED: i32 = -3;
    static NEGATIVE_M_DOUBLE_DIGIT_PARSED: i32 = -32;

    static POSITIVE_S_PARSED: i32 = 2;
    static NEGATIVE_S_PARSED: i32 = -2;
    static POSITIVE_S_DOUBLE_DIGIT_PARSED: i32 = 20;

    static SAMPLE_LENGTH: usize = 10;

    static DEFAULT_START: i32 = 0;
    static DEFAULT_END: i32 = SAMPLE_LENGTH as i32;
    static DEFAULT_STEP: i32 = 1;

    static EXPECTED_ERROR: &str = "Invalid range";

    #[test]
    fn test_01_positive_positive_positive() {
        let fields =
            String::from("") + POSITIVE_N + SEPARATOR + POSITIVE_M + SEPARATOR + POSITIVE_S;
        let expected_range = Ok(Range::new(
            POSITIVE_N_PARSED,
            POSITIVE_M_PARSED,
            POSITIVE_S_PARSED,
        ));

        base_test(&fields, expected_range);
    }

    #[test]
    fn test_02_positive_positive_negative() {
        let fields =
            String::from("") + POSITIVE_N + SEPARATOR + POSITIVE_M + SEPARATOR + NEGATIVE_S;
        let expected_range = Ok(Range::new(
            POSITIVE_N_PARSED,
            POSITIVE_M_PARSED,
            NEGATIVE_S_PARSED,
        ));

        base_test(&fields, expected_range);
    }

    #[test]
    fn test_03_positive_positive_separator() {
        let fields = String::from("") + POSITIVE_N + SEPARATOR + POSITIVE_M + SEPARATOR;
        let expected_range = Ok(Range::new(
            POSITIVE_N_PARSED,
            POSITIVE_M_PARSED,
            DEFAULT_STEP,
        ));

        base_test(&fields, expected_range);
    }

    #[test]
    fn test_04_positive_positive() {
        let fields = String::from("") + POSITIVE_N + SEPARATOR + POSITIVE_M;
        let expected_range = Ok(Range::new(
            POSITIVE_N_PARSED,
            POSITIVE_M_PARSED,
            DEFAULT_STEP,
        ));

        base_test(&fields, expected_range);
    }

    #[test]
    fn test_05_positive_negative() {
        let fields = String::from("") + POSITIVE_N + SEPARATOR + NEGATIVE_M;
        let expected_range = Ok(Range::new(
            POSITIVE_N_PARSED,
            NEGATIVE_M_PARSED,
            DEFAULT_STEP,
        ));

        base_test(&fields, expected_range);
    }

    #[test]
    fn test_06_positive_separator() {
        let fields = String::from("") + POSITIVE_N + SEPARATOR;
        let expected_range = Ok(Range::new(POSITIVE_N_PARSED, DEFAULT_END, DEFAULT_STEP));

        base_test(&fields, expected_range);
    }

    #[test]
    fn test_07_positive() {
        let fields = String::from("") + POSITIVE_N;
        let expected_range = Ok(Range::new(
            POSITIVE_N_PARSED,
            POSITIVE_N_PARSED + 1,
            DEFAULT_STEP,
        ));

        base_test(&fields, expected_range);
    }

    #[test]
    fn test_08_negative() {
        let fields = String::from("") + NEGATIVE_N;
        let expected_range = Ok(Range::new(
            NEGATIVE_N_PARSED,
            NEGATIVE_N_PARSED + 1,
            DEFAULT_STEP,
        ));

        base_test(&fields, expected_range);
    }

    #[test]
    fn test_09_separator_positive() {
        let fields = String::from("") + SEPARATOR + POSITIVE_M;
        let expected_range = Ok(Range::new(DEFAULT_START, POSITIVE_M_PARSED, DEFAULT_STEP));

        base_test(&fields, expected_range);
    }

    #[test]
    fn test_10_separator_separator_positive() {
        let fields = String::from("") + SEPARATOR + SEPARATOR + POSITIVE_S;
        let expected_range = Ok(Range::new(DEFAULT_START, DEFAULT_END, POSITIVE_S_PARSED));

        base_test(&fields, expected_range);
    }

    #[test]
    fn test_11_positive_negative_positive_all_double_digits() {
        let fields = String::from("")
            + POSITIVE_N_DOUBLE_DIGIT
            + SEPARATOR
            + NEGATIVE_M_DOUBLE_DIGIT
            + SEPARATOR
            + POSITIVE_S_DOUBLE_DIGIT;
        let expected_range = Ok(Range::new(
            POSITIVE_N_DOUBLE_DIGIT_PARSED,
            NEGATIVE_M_DOUBLE_DIGIT_PARSED,
            POSITIVE_S_DOUBLE_DIGIT_PARSED,
        ));

        base_test(&fields, expected_range);
    }

    #[test]
    fn test_12_positive_separator_separator_positive() {
        let fields = String::from("") + POSITIVE_N + SEPARATOR + SEPARATOR + POSITIVE_S;
        let expected_range = Ok(Range::new(
            POSITIVE_N_PARSED,
            DEFAULT_END,
            POSITIVE_S_PARSED,
        ));

        base_test(&fields, expected_range);
    }

    #[test]
    fn test_13_empty_string() {
        let fields = String::from("");
        let expected_range = Err(String::from(EXPECTED_ERROR));

        base_test(&fields, expected_range);
    }

    #[test]
    fn test_14_start_with_alphabetical() {
        let fields = String::from("asd") + SEPARATOR + POSITIVE_M;
        let expected_range = Err(String::from(EXPECTED_ERROR));

        base_test(&fields, expected_range);
    }

    #[test]
    fn test_15_ends_with_alphabetical() {
        let fields = String::from("") + POSITIVE_N + SEPARATOR + POSITIVE_M + SEPARATOR + "asd";
        let expected_range = Err(String::from(EXPECTED_ERROR));

        base_test(&fields, expected_range);
    }

    #[test]
    fn test_16_triple_separator() {
        let fields = String::from("") + SEPARATOR + SEPARATOR + SEPARATOR;
        let expected_range = Err(String::from(EXPECTED_ERROR));

        base_test(&fields, expected_range);
    }
    fn base_test(fields: &String, expected_range: Result<Range, String>) {
        let actual_range = parse_single_range(fields, SAMPLE_LENGTH);

        assert_eq!(actual_range, expected_range)
    }
}

#[cfg(test)]
mod unit_tests_parse_range {
    // 1 range
    // 1 invalid range
    // multiple ranges
    // multiple ranges with 1 invalid

    use super::{parse_range, parse_single_range, Range};

    static SAMPLE_LENGTH: usize = 10;
    static SEPARATOR: &str = ",";
    static SAMPLE_RANGE_1: &str = "0:2";
    static SAMPLE_RANGE_2: &str = "3:7:1";
    static SAMPLE_INVALID_RANGE: &str = "asd";

    static EXPECTED_ERROR: &str = "Invalid range";

    #[test]
    fn test_01_1_range() {
        let fields = String::from("") + SAMPLE_RANGE_1;
        let expected_range: Result<Vec<Range>, String> =
            Ok(vec![parse_single_range(SAMPLE_RANGE_1, 3).unwrap()]);

        base_test(&fields, expected_range);
    }

    #[test]
    fn test_02_1_invalid_range() {
        let fields = String::from("") + SAMPLE_INVALID_RANGE;
        let expected_range: Result<Vec<Range>, String> = Err(String::from(EXPECTED_ERROR));

        base_test(&fields, expected_range);
    }

    #[test]
    fn test_03_multiple_ranges() {
        let fields = String::from("")
            + SAMPLE_RANGE_1
            + SEPARATOR
            + SAMPLE_RANGE_2
            + SEPARATOR
            + SAMPLE_RANGE_1;
        let expected_range: Result<Vec<Range>, String> = Ok(vec![
            parse_single_range(SAMPLE_RANGE_1, 3).unwrap(),
            parse_single_range(SAMPLE_RANGE_2, 13).unwrap(),
            parse_single_range(SAMPLE_RANGE_1, 3).unwrap(),
        ]);

        base_test(&fields, expected_range);
    }

    #[test]
    fn test_04_multiple_ranges_with_1_invalid() {
        let fields = String::from("")
            + SAMPLE_RANGE_1
            + SEPARATOR
            + SAMPLE_RANGE_2
            + SEPARATOR
            + SAMPLE_INVALID_RANGE
            + SEPARATOR
            + SAMPLE_RANGE_1;
        let expected_range: Result<Vec<Range>, String> = Err(String::from(EXPECTED_ERROR));

        base_test(&fields, expected_range);
    }

    fn base_test(fields: &String, expected_range: Result<Vec<Range>, String>) {
        let actual_range = parse_range(fields, SAMPLE_LENGTH);

        assert_eq!(actual_range, expected_range)
    }
}
