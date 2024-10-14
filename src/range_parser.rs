use regex::Regex;

pub fn parse_range(fields: &String, n: i32) -> Result<(i32, i32, i32), String> {
    let error_message = String::from("Can't build regex");
    if fields == "" {
        return Err(error_message);
    }

    let groups: Vec<&str> = fields.split(":").collect();

    let colon_count = fields.match_indices(":").count();

    if colon_count < 3 {
        let parsed_start = get_parsed_item(groups[0], 0);

        let parsed_end = if colon_count != 0 {
            get_parsed_item(groups[1], n)
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
            (Ok(start), Ok(end), Ok(step)) => Ok((start, end, step)),
            _ => Err(error_message),
        }
    } else {
        Err(error_message)
    }
}

fn get_parsed_item(raw_item: &str, default: i32) -> Result<i32, std::num::ParseIntError> {
    if raw_item != "" {
        raw_item.parse::<i32>()
    } else {
        Ok(default)
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
