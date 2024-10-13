use regex::Regex;

pub fn parse_range(fields: &String, n: i32) -> Result<(i32, i32, i32), String> {
    let groups: Vec<&str> = fields.split(":").collect();
    let error_message = String::from("Can't build regex");

    let colon_count = fields.match_indices(":").count();

    if colon_count == 0 {
        let raw_start = groups[0];

        match raw_start.parse::<i32>() {
            Ok(start) => Ok((start, start + 1, 1)),
            _ => Err(error_message),
        }
    } else if colon_count == 1 {
        let raw_start = groups[0];
        let raw_end = groups[1];

        if raw_end == "" {
            match raw_start.parse::<i32>() {
                Ok(start) => Ok((start, n, 1)),
                _ => Err(error_message),
            }
        } else if raw_start == "" {
            match raw_end.parse::<i32>() {
                Ok(end) => Ok((0, end, 1)),
                _ => Err(error_message),
            }
        } else {
            match (raw_start.parse::<i32>(), raw_end.parse::<i32>()) {
                (Ok(start), Ok(end)) => Ok((start, end, 1)),
                _ => Err(error_message),
            }
        }
    } else if colon_count == 2 {
        let raw_start = groups[0];
        let raw_end = groups[1];
        let raw_step = groups[2];

        if raw_start == "" && raw_end == "" {
            match raw_step.parse::<i32>() {
                Ok(step) => Ok((0, n, step)),
                _ => Err(error_message),
            }
        } else if raw_start == "" && raw_step == "" {
            match raw_end.parse::<i32>() {
                Ok(end) => Ok((0, end, 1)),
                _ => Err(error_message),
            }
        } else if raw_end == "" && raw_step == "" {
            match raw_start.parse::<i32>() {
                Ok(start) => Ok((start, n, 1)),
                _ => Err(error_message),
            }
        } else if raw_end == "" {
            match (raw_start.parse::<i32>(), raw_step.parse::<i32>()) {
                (Ok(start), Ok(step)) => Ok((start, n, step)),
                _ => Err(error_message),
            }
        } else if raw_start == "" {
            match (raw_end.parse::<i32>(), raw_step.parse::<i32>()) {
                (Ok(end), Ok(step)) => Ok((0, end, step)),
                _ => Err(error_message),
            }
        } else if raw_step == "" {
            match (raw_start.parse::<i32>(), raw_end.parse::<i32>()) {
                (Ok(start), Ok(end)) => Ok((start, end, 1)),
                _ => Err(error_message),
            }
        } else {
            match (
                raw_start.parse::<i32>(),
                raw_end.parse::<i32>(),
                raw_step.parse::<i32>(),
            ) {
                (Ok(start), Ok(end), Ok(step)) => Ok((start, end, step)),
                _ => Err(error_message),
            }
        }
    } else {
        Err(error_message)
    }
}

#[cfg(test)]
mod test {
    use super::parse_range;
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

    static SAMPLE_LENGTH: i32 = 10;

    static DEFAULT_START: i32 = 0;
    static DEFAULT_END: i32 = SAMPLE_LENGTH;
    static DEFAULT_STEP: i32 = 1;

    static EXPECTED_ERROR: &str = "Can't build regex";

    #[test]
    fn test_01_positive_positive_positive() {
        let fields =
            String::from("") + POSITIVE_N + SEPARATOR + POSITIVE_M + SEPARATOR + POSITIVE_S;
        let expected_range = Ok((POSITIVE_N_PARSED, POSITIVE_M_PARSED, POSITIVE_S_PARSED));

        base_test(&fields, expected_range);
    }

    #[test]
    fn test_02_positive_positive_negative() {
        let fields =
            String::from("") + POSITIVE_N + SEPARATOR + POSITIVE_M + SEPARATOR + NEGATIVE_S;
        let expected_range = Ok((POSITIVE_N_PARSED, POSITIVE_M_PARSED, NEGATIVE_S_PARSED));

        base_test(&fields, expected_range);
    }

    #[test]
    fn test_03_positive_positive_separator() {
        let fields = String::from("") + POSITIVE_N + SEPARATOR + POSITIVE_M + SEPARATOR;
        let expected_range = Ok((POSITIVE_N_PARSED, POSITIVE_M_PARSED, DEFAULT_STEP));

        base_test(&fields, expected_range);
    }

    #[test]
    fn test_04_positive_positive() {
        let fields = String::from("") + POSITIVE_N + SEPARATOR + POSITIVE_M;
        let expected_range = Ok((POSITIVE_N_PARSED, POSITIVE_M_PARSED, DEFAULT_STEP));

        base_test(&fields, expected_range);
    }

    #[test]
    fn test_05_positive_negative() {
        let fields = String::from("") + POSITIVE_N + SEPARATOR + NEGATIVE_M;
        let expected_range = Ok((POSITIVE_N_PARSED, NEGATIVE_M_PARSED, DEFAULT_STEP));

        base_test(&fields, expected_range);
    }

    #[test]
    fn test_06_positive_separator() {
        let fields = String::from("") + POSITIVE_N + SEPARATOR;
        let expected_range = Ok((POSITIVE_N_PARSED, DEFAULT_END, DEFAULT_STEP));

        base_test(&fields, expected_range);
    }

    #[test]
    fn test_07_positive() {
        let fields = String::from("") + POSITIVE_N;
        let expected_range = Ok((POSITIVE_N_PARSED, POSITIVE_N_PARSED + 1, DEFAULT_STEP));

        base_test(&fields, expected_range);
    }

    #[test]
    fn test_08_negative() {
        let fields = String::from("") + NEGATIVE_N;
        let expected_range = Ok((NEGATIVE_N_PARSED, NEGATIVE_N_PARSED + 1, DEFAULT_STEP));

        base_test(&fields, expected_range);
    }

    #[test]
    fn test_09_separator_positive() {
        let fields = String::from("") + SEPARATOR + POSITIVE_M;
        let expected_range = Ok((DEFAULT_START, POSITIVE_M_PARSED, DEFAULT_STEP));

        base_test(&fields, expected_range);
    }

    #[test]
    fn test_10_separator_separator_positive() {
        let fields = String::from("") + SEPARATOR + SEPARATOR + POSITIVE_S;
        let expected_range = Ok((DEFAULT_START, DEFAULT_END, POSITIVE_S_PARSED));

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
        let expected_range = Ok((
            POSITIVE_N_DOUBLE_DIGIT_PARSED,
            NEGATIVE_M_DOUBLE_DIGIT_PARSED,
            POSITIVE_S_DOUBLE_DIGIT_PARSED,
        ));

        base_test(&fields, expected_range);
    }

    #[test]
    fn test_12_positive_separator_separator_positive() {
        let fields = String::from("") + POSITIVE_N + SEPARATOR + SEPARATOR + POSITIVE_S;
        let expected_range = Ok((POSITIVE_N_PARSED, DEFAULT_END, POSITIVE_S_PARSED));

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
    fn base_test(fields: &String, expected_range: Result<(i32, i32, i32), String>) {
        let actual_range = parse_range(fields, SAMPLE_LENGTH);

        assert_eq!(actual_range, expected_range)
    }
}
