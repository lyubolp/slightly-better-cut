use regex::Regex;

pub fn parse_range(fields: &String) -> Result<(i32, i32, i32), String> {
    let re = Regex::new(r"(-{0,1}[0-9]+):{0,1}(-{0,1}[0-9]+)*:{0,1}(-{0,1}[0-9]+)*");

    match re {
        Ok(reg) => {
            unimplemented!()
        },
        Err(_) => Err(String::from("Can't build regex"))
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

    positive:positive
    positive:negative
    positive:
    positive

    positive
    negative
    :positive

    ::positive

    positive:negative:positive, with two digits

    */

    static POSITIVE_N: &str = "3";
    static NEGATIVE_N: &str = "-5";

    static POSITIVE_M: &str = "5";
    static NEGATIVE_M: &str = "-3";

    static POSITIVE_S: &str = "2";
    static NEGATIVE_S: &str = "-2";
    
    static SEPARATOR: &str = ":";

    static POSITIVE_N_PARSED: i32 = 3;
    static NEGATIVE_N_PARSED: i32 = -5;

    static POSITIVE_M_PARSED: i32 = 5;
    static NEGATIVE_M_PARSED: i32 = -3;

    static POSITIVE_S_PARSED: i32 = 2;
    static NEGATIVE_S_PARSED: i32 = -2;

    #[test]
    fn test_01_positive_positive_positive() {
        let fields = String::from("") + POSITIVE_N + SEPARATOR + POSITIVE_M + SEPARATOR + POSITIVE_S;
        let expected_range = Ok((POSITIVE_N_PARSED, POSITIVE_M_PARSED, POSITIVE_S_PARSED));

        base_test(&fields, expected_range);
    }

    #[test]
    fn test_01_positive_positive_negative() {
        let fields = String::from("") + POSITIVE_N + SEPARATOR + POSITIVE_M + SEPARATOR + POSITIVE_S;
        let expected_range = Ok((POSITIVE_N_PARSED, POSITIVE_M_PARSED, POSITIVE_S_PARSED));

        base_test(&fields, expected_range);
    }
    
    fn base_test(fields: &String, expected_range: Result<(i32, i32, i32), String>) {
        let actual_range = parse_range(fields);

        assert_eq!(actual_range, expected_range)
    }
}