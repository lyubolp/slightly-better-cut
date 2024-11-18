/// Module containing the three cut methods - via a delimiter, via bytes and via characters.
/// Each cut function returns the selected items from the line.
///
/// `cut_line_with_delimiter` - cut a line based on a range and a delimiter
/// 
/// `cut_line_with_character` - cut a line based on a range over the characters
/// 
/// `cut_line_with_bytes` - cut a line based on a range over the bytes. 
/// Treat each byte as UTF-8. If byte is not utf-8 encoded, print it with '0x' as prefix

use crate::range_parser::Range;
use std::collections::HashSet;

pub fn cut_line_with_delimiter(
    line: &str,
    range: Range,
    delimiter: String,
    is_showing_complement: bool,
) -> Vec<String> {
    //! Cut the line with a given delimiter, and return the selected range. 
    let items: Vec<String> = line.split(&delimiter).map(String::from).collect();
    let n = items.len() as i32;

    cut_line(items, range, n, is_showing_complement)
}

pub fn cut_line_with_bytes(line: &str, range: Range, is_showing_complement: bool) -> Vec<String> {
    //! Cut the line and return the selected range of bytes.
    let items: Vec<String> = line.bytes().map(handle_bytes).collect();
    let n = line.len() as i32;

    cut_line(items, range, n, is_showing_complement)
}

pub fn cut_line_with_characters(
    line: &str,
    range: Range,
    is_showing_complement: bool,
) -> Vec<String> {
    //! Cut the line and return the selected range of characters.
    let items: Vec<String> = line.chars().map(String::from).collect();
    let n = line.len() as i32;

    cut_line(items, range, n, is_showing_complement)
}

fn cut_line(items: Vec<String>, range: Range, n: i32, is_showing_complement: bool) -> Vec<String> {
    //! Return the corresponding items to the range from the group.
    let (start, end, step) = range.to_tuple();

    if step == 0 {
        return vec![];
    }

    // TODO - Ugly hack
    if !is_showing_complement {
        let is_start_within_bounds = -n <= start && start < n;
        //TODO - Add test for getting the last field from line
        let is_end_within_bounds = -n <= end && end != 0 && end <= n;

        if !(is_start_within_bounds && is_end_within_bounds) {
            return vec![];
        }
    }

    let indexes_to_get = match calculate_indexes_to_get(start, n, end, step) {
        Some(value) => value,
        None => return vec![],
    };

    let mut result: Vec<String> = items
        .iter()
        .enumerate()
        .filter(|(index, _)| {
            if is_showing_complement {
                !indexes_to_get.contains(index)
            } else {
                indexes_to_get.contains(index)
            }
        })
        .map(|(_, item)| item.clone())
        .collect();

    if step < 0 {
        result.reverse();
    }

    result
}

fn calculate_indexes_to_get(start: i32, n: i32, end: i32, step: i32) -> Option<HashSet<usize>> {
    //! Calculate the indexes that correspond to the range
    let actual_start = handle_negative_index(start, n);
    let actual_end = handle_negative_index(end, n);
    if actual_start >= actual_end {
        return None;
    }
    let actual_step = step.unsigned_abs() as usize;
    let indexes_to_get: HashSet<usize> = (actual_start..actual_end).step_by(actual_step).collect();
    Some(indexes_to_get)
}

fn handle_negative_index(index: i32, n: i32) -> usize {
    if index >= 0 {
        index as usize
    } else {
        (n + index) as usize
    }
}

fn handle_bytes(byte: u8) -> String {
    match String::from_utf8(vec![byte]) {
        Ok(result) => result,
        Err(_) => String::from(format!("{:#02x}", byte)),
    }
}

#[cfg(test)]
mod unit_tests_cut_line_with_delimiter {
    use crate::range_parser::Range;

    use super::cut_line_with_delimiter;

    /*
    start values:
        A: (-inf; -n) => []
        B: [-n; 0)
        C: [0; end)
        D: [end: n) => []
        E: [n: +inf) => []

    end values:
        A: (-inf; -n] => []
        B: (-n; 0)
        C: =0 => []
        D: (0; n]
        E: (n; +inf) => []

    step values:
        A: step = 1
        B: step = -1
        C: step = 2
        D: step = -2
        E: step = 0
     */

    static CONTENT: &str = "first second third fourth fifth";
    static DELIMITER: &str = " ";

    static N: i32 = 5 as i32;

    static START_A: i32 = -N - 3; // -8
    static START_B: i32 = -N + 1; // -4
    static START_C: i32 = 2;
    static START_D: i32 = 4;
    static START_E: i32 = N + 2; // 7

    static END_A: i32 = -N - 4; // -9
    static END_B: i32 = -N + 4; // -1
    static END_C: i32 = 0;
    static END_D: i32 = N - 2; // 3
    static END_E: i32 = N + 4; // 9
    static END_F: i32 = N; // 5

    static STEP_A: i32 = 1;
    static STEP_B: i32 = -1;
    static STEP_C: i32 = 2;
    static STEP_D: i32 = -2;
    static STEP_E: i32 = 0;

    // START_A
    #[test]
    fn test_01_start_a_end_a_step_a() {
        // start: (-inf; -n), end (-inf; -n], step = 1
        base_test(START_A, END_A, STEP_A, vec![], false)
    }

    #[test]
    fn test_02_start_a_end_b_step_a() {
        // start: (-inf; -n), end: (-n; 0), step = 1
        base_test(START_A, END_B, STEP_A, vec![], false)
    }

    #[test]
    fn test_03_start_a_end_c_step_a() {
        // start: (-inf; -n), end=0, step = 1
        base_test(START_A, END_C, STEP_A, vec![], false)
    }

    #[test]
    fn test_04_start_a_end_d_step_a() {
        // start: (-inf; -n), end: (0; n], step = 1
        base_test(START_A, END_D, STEP_A, vec![], false)
    }

    #[test]
    fn test_05_start_a_end_e_step_a() {
        // start: (-inf; -n), end: (n; +inf), step = 1
        base_test(START_A, END_E, STEP_A, vec![], false)
    }

    #[test]
    fn test_06_start_a_end_a_step_b() {
        // start: (-inf; -n), end (-inf; -n], step = -1
        base_test(START_A, END_A, STEP_B, vec![], false)
    }

    #[test]
    fn test_07_start_a_end_b_step_b() {
        // start: (-inf; -n), end: (-n; 0), step = -1
        base_test(START_A, END_B, STEP_B, vec![], false)
    }

    #[test]
    fn test_08_start_a_end_c_step_b() {
        // start: (-inf; -n), end=0, step = -1
        base_test(START_A, END_C, STEP_B, vec![], false)
    }

    #[test]
    fn test_09_start_a_end_d_step_b() {
        // start: (-inf; -n), end: (0; n], step = -1
        base_test(START_A, END_D, STEP_B, vec![], false)
    }

    #[test]
    fn test_10_start_a_end_e_step_b() {
        // start: (-inf; -n), end: (n; +inf), step = -1
        base_test(START_A, END_E, STEP_B, vec![], false)
    }

    // START_B
    #[test]
    fn test_11_start_b_end_a_step_a() {
        // start: [-n; 0), end (-inf; -n], step = 1
        base_test(START_B, END_A, STEP_A, vec![], false)
    }

    #[test]
    fn test_12_start_b_end_b_step_a() {
        // start: [-n; 0), end: (-n; 0), step = 1
        base_test(
            START_B,
            END_B,
            STEP_A,
            vec!["second", "third", "fourth"],
            false,
        )
    }

    #[test]
    fn test_13_start_b_end_c_step_a() {
        // start: [-n; 0), end=0, step = 1
        base_test(START_B, END_C, STEP_A, vec![], false)
    }

    #[test]
    fn test_14_start_b_end_d_step_a() {
        // start:[-n; 0), end: (0; n], step = 1
        base_test(START_B, END_D, STEP_A, vec!["second", "third"], false)
    }

    #[test]
    fn test_15_start_b_end_e_step_a() {
        // start: [-n; 0), end: (n; +inf), step = 1
        base_test(START_B, END_E, STEP_A, vec![], false)
    }

    #[test]
    fn test_16_start_b_end_a_step_b() {
        // start: [-n; 0), end (-inf; -n], step = -1
        base_test(START_B, END_A, STEP_B, vec![], false)
    }

    #[test]
    fn test_17_start_b_end_b_step_b() {
        // start: [-n; 0), end: (-n; 0), step = -1
        base_test(
            START_B,
            END_B,
            STEP_B,
            vec!["fourth", "third", "second"],
            false,
        )
    }

    #[test]
    fn test_18_start_b_end_c_step_b() {
        // start: [-n; 0), end=0, step = -1
        base_test(START_B, END_C, STEP_B, vec![], false)
    }

    #[test]
    fn test_19_start_b_end_d_step_b() {
        // start: [-n; 0), end: (0; n], step = -1
        base_test(START_B, END_D, STEP_B, vec!["third", "second"], false)
    }

    #[test]
    fn test_20_start_b_end_e_step_b() {
        // start: [-n; 0), end: (n; +inf), step = -1
        base_test(START_B, END_E, STEP_B, vec![], false)
    }

    // START_C

    #[test]
    fn test_21_start_c_end_a_step_a() {
        // start: [0; end), end (-inf; -n], step = 1
        base_test(START_C, END_A, STEP_A, vec![], false)
    }

    #[test]
    fn test_22_start_c_end_b_step_a() {
        // start: [0; end), end: (-n; 0), step = 1
        base_test(START_C, END_B, STEP_A, vec!["third", "fourth"], false)
    }

    #[test]
    fn test_23_start_c_end_c_step_a() {
        // start: [0; end), end=0, step = 1
        base_test(START_C, END_C, STEP_A, vec![], false)
    }

    #[test]
    fn test_24_start_c_end_d_step_a() {
        // start:[0; end), end: (0; n], step = 1
        base_test(START_C, END_D, STEP_A, vec!["third"], false)
    }

    #[test]
    fn test_25_start_c_end_e_step_a() {
        // start: [0; end), end: (n; +inf), step = 1
        base_test(START_C, END_E, STEP_A, vec![], false)
    }

    #[test]
    fn test_26_start_c_end_a_step_b() {
        // start: [0; end), end (-inf; -n], step = -1
        base_test(START_C, END_A, STEP_B, vec![], false)
    }

    #[test]
    fn test_27_start_c_end_b_step_b() {
        // start: [0; end), end: (-n; 0), step = -1
        base_test(START_C, END_B, STEP_B, vec!["fourth", "third"], false)
    }

    #[test]
    fn test_28_start_c_end_c_step_b() {
        // start: [0; end), end=0, step = -1
        base_test(START_C, END_C, STEP_B, vec![], false)
    }

    #[test]
    fn test_29_start_c_end_d_step_b() {
        // start: [0; end), end: (0; n], step = -1
        base_test(START_C, END_D, STEP_B, vec!["third"], false)
    }

    #[test]
    fn test_30_start_c_end_e_step_b() {
        // start: [0; end), end: (n; +inf), step = -1
        base_test(START_C, END_E, STEP_B, vec![], false)
    }

    // START_D

    #[test]
    fn test_31_start_d_end_a_step_a() {
        // start: [end: n) , end (-inf; -n], step = 1
        base_test(START_D, END_A, STEP_A, vec![], false)
    }

    #[test]
    fn test_32_start_d_end_b_step_a() {
        // start: [end: n) , end: (-n; 0), step = 1
        base_test(START_D, END_B, STEP_A, vec![], false)
    }

    #[test]
    fn test_33_start_d_end_c_step_a() {
        // start: [end: n) , end=0, step = 1
        base_test(START_D, END_C, STEP_A, vec![], false)
    }

    #[test]
    fn test_34_start_d_end_d_step_a() {
        // start:[end: n) , end: (0; n], step = 1
        base_test(START_D, END_D, STEP_A, vec![], false)
    }

    #[test]
    fn test_35_start_d_end_e_step_a() {
        // start: [end: n) , end: (n; +inf), step = 1
        base_test(START_D, END_E, STEP_A, vec![], false)
    }

    #[test]
    fn test_36_start_d_end_a_step_b() {
        // start: [end: n) , end (-inf; -n], step = -1
        base_test(START_D, END_A, STEP_B, vec![], false)
    }

    #[test]
    fn test_37_start_d_end_b_step_b() {
        // start: [end: n) , end: (-n; 0), step = -1
        base_test(START_D, END_B, STEP_B, vec![], false)
    }

    #[test]
    fn test_38_start_d_end_c_step_b() {
        // start: [end: n) , end=0, step = -1
        base_test(START_D, END_C, STEP_B, vec![], false)
    }

    #[test]
    fn test_39_start_d_end_d_step_b() {
        // start: [end: n) , end: (0; n], step = -1
        base_test(START_D, END_D, STEP_B, vec![], false)
    }

    #[test]
    fn test_40_start_d_end_e_step_b() {
        // start: [end: n) , end: (n; +inf), step = -1
        base_test(START_D, END_E, STEP_B, vec![], false)
    }

    // START_E

    #[test]
    fn test_41_start_e_end_a_step_a() {
        // start: [n: +inf) , end (-inf; -n], step = 1
        base_test(START_E, END_A, STEP_A, vec![], false)
    }

    #[test]
    fn test_42_start_e_end_b_step_a() {
        // start: [n: +inf) , end: (-n; 0), step = 1
        base_test(START_E, END_B, STEP_A, vec![], false)
    }

    #[test]
    fn test_43_start_e_end_c_step_a() {
        // start: [n: +inf) , end=0, step = 1
        base_test(START_E, END_C, STEP_A, vec![], false)
    }

    #[test]
    fn test_44_start_e_end_d_step_a() {
        // start: [n: +inf) , end: (0; n], step = 1
        base_test(START_E, END_D, STEP_A, vec![], false)
    }

    #[test]
    fn test_45_start_e_end_e_step_a() {
        // start: [n: +inf) , end: (n; +inf), step = 1
        base_test(START_E, END_E, STEP_A, vec![], false)
    }

    #[test]
    fn test_46_start_e_end_a_step_b() {
        // start: [n: +inf) , end (-inf; -n], step = -1
        base_test(START_E, END_A, STEP_B, vec![], false)
    }

    #[test]
    fn test_47_start_e_end_b_step_b() {
        // start: [n: +inf) , end: (-n; 0), step = -1
        base_test(START_E, END_B, STEP_B, vec![], false)
    }

    #[test]
    fn test_48_start_e_end_c_step_b() {
        // start: [n: +inf) , end=0, step = -1
        base_test(START_E, END_C, STEP_B, vec![], false)
    }

    #[test]
    fn test_49_start_e_end_d_step_b() {
        // start: [n: +inf) , end: (0; n], step = -1
        base_test(START_E, END_D, STEP_B, vec![], false)
    }

    #[test]
    fn test_50_start_e_end_e_step_b() {
        // start: [n: +inf) , end: (n; +inf), step = -1
        base_test(START_E, END_E, STEP_B, vec![], false)
    }

    // Step C
    #[test]
    fn test_51_start_b_end_b_step_c() {
        // start: [-n; 0), end: (-n; 0), step = 2
        base_test(START_B, END_B, STEP_C, vec!["second", "fourth"], false)
    }

    #[test]
    fn test_52_start_b_end_d_step_c() {
        // start: [-n; 0), end: (-n; 0), step = 2
        base_test(START_B, END_D, STEP_C, vec!["second"], false)
    }

    #[test]
    fn test_53_start_c_end_b_step_c() {
        // start: [-n; 0), end: (-n; 0), step = 2
        base_test(START_C, END_B, STEP_C, vec!["third"], false)
    }

    #[test]
    fn test_54_start_c_end_d_step_c() {
        // start: [-n; 0), end: (-n; 0), step = 2
        base_test(START_C, END_D, STEP_C, vec!["third"], false)
    }

    // Step D
    #[test]
    fn test_55_start_b_end_b_step_d() {
        // start: [-n; 0), end: (-n; 0), step = -2
        base_test(START_B, END_B, STEP_D, vec!["fourth", "second"], false)
    }

    #[test]
    fn test_56_start_b_end_d_step_d() {
        // start: [-n; 0), end: (-n; 0), step = -2
        base_test(START_B, END_D, STEP_D, vec!["second"], false)
    }

    #[test]
    fn test_57_start_c_end_b_step_d() {
        // start: [-n; 0), end: (-n; 0), step = -2
        base_test(START_C, END_B, STEP_D, vec!["third"], false)
    }

    #[test]
    fn test_58_start_c_end_d_step_d() {
        // start: [-n; 0), end: (-n; 0), step = -2
        base_test(START_C, END_D, STEP_D, vec!["third"], false)
    }

    // Step E
    #[test]
    fn test_59_start_b_end_b_step_e() {
        // start: [-n; 0), end: (-n; 0), step = 0
        base_test(START_B, END_B, STEP_E, vec![], false)
    }

    #[test]
    fn test_60_start_b_end_d_step_e() {
        // start: [-n; 0), end: (-n; 0), step = 0
        base_test(START_B, END_D, STEP_E, vec![], false)
    }

    #[test]
    fn test_61_start_c_end_b_step_e() {
        // start: [-n; 0), end: (-n; 0), step = 0
        base_test(START_C, END_B, STEP_E, vec![], false)
    }

    #[test]
    fn test_62_start_c_end_d_step_e() {
        // start: [-n; 0), end: (-n; 0), step = 0
        base_test(START_C, END_D, STEP_E, vec![], false)
    }

    #[test]
    fn test_63_start_a_end_d_step_a_complement() {
        // start: (-inf; -n), end: (0; n], step = 1, start >= end
        base_test(START_A, END_D, STEP_A, vec![], true)
    }

    #[test]
    fn test_64_start_c_end_d_step_a_complement() {
        // start:[0; end), end: (0; n], step = 1
        base_test(
            START_C,
            END_D,
            STEP_A,
            vec!["first", "second", "fourth", "fifth"],
            true,
        )
    }

    #[test]
    fn test_65_start_c_end_f_step_c_complement() {
        // start:[0; end), end: (0; n], step = 2
        base_test(
            START_C,
            END_F,
            STEP_C,
            vec!["first", "second", "fourth"],
            true,
        )
    }

    #[test]
    fn test_66_start_c_end_f_step_b_complement() {
        // start:[0; end), end: (0; n], step = -1
        base_test(START_C, END_F, STEP_B, vec!["second", "first"], true)
    }

    #[test]
    fn test_67_start_e_end_e_step_a_complement() {
        // start: (-inf; -n), end: (0; n], step = 1, start >= end
        base_test(
            START_E,
            END_E,
            STEP_A,
            vec!["first", "second", "third", "fourth", "fifth"],
            true,
        )
    }

    fn base_test(
        start: i32,
        end: i32,
        step: i32,
        expected_content: Vec<&str>,
        is_showing_complement: bool,
    ) {
        // Arrange
        let range = Range::new(start, end, step);

        // Act
        let actual_content = cut_line_with_delimiter(
            CONTENT,
            range,
            String::from(DELIMITER),
            is_showing_complement,
        );

        // Assert
        assert_eq!(expected_content, actual_content);
    }
}

#[cfg(test)]
mod unit_tests_cut_line_with_characters {
    use crate::range_parser::Range;

    use super::cut_line_with_characters;

    /*
    start values:
        A: (-inf; -n) => []
        B: [-n; 0)
        C: [0; end)
        D: [end: n) => []
        E: [n: +inf) => []

    end values:
        A: (-inf; -n] => []
        B: (-n; 0)
        C: =0 => []
        D: (0; n]
        E: (n; +inf) => []

    step values:
        A: step = 1
        B: step = -1
        C: step = 2
        D: step = -2
        E: step = 0
     */

    static CONTENT: &str = "abcde";

    static N: i32 = 5 as i32;

    static START_A: i32 = -N - 3; // -8
    static START_B: i32 = -N + 1; // -4
    static START_C: i32 = 2;
    static START_D: i32 = 4;
    static START_E: i32 = N + 2; // 7

    static END_A: i32 = -N - 4; // -9
    static END_B: i32 = -N + 4; // -1
    static END_C: i32 = 0;
    static END_D: i32 = N - 2; // 3
    static END_E: i32 = N + 4; // 9
    static END_F: i32 = N; // 5

    static STEP_A: i32 = 1;
    static STEP_B: i32 = -1;
    static STEP_C: i32 = 2;
    static STEP_D: i32 = -2;
    static STEP_E: i32 = 0;

    // START_A
    #[test]
    fn test_01_start_a_end_a_step_a() {
        // start: (-inf; -n), end (-inf; -n], step = 1
        base_test(START_A, END_A, STEP_A, vec![], false)
    }

    #[test]
    fn test_02_start_a_end_b_step_a() {
        // start: (-inf; -n), end: (-n; 0), step = 1
        base_test(START_A, END_B, STEP_A, vec![], false)
    }

    #[test]
    fn test_03_start_a_end_c_step_a() {
        // start: (-inf; -n), end=0, step = 1
        base_test(START_A, END_C, STEP_A, vec![], false)
    }

    #[test]
    fn test_04_start_a_end_d_step_a() {
        // start: (-inf; -n), end: (0; n], step = 1
        base_test(START_A, END_D, STEP_A, vec![], false)
    }

    #[test]
    fn test_05_start_a_end_e_step_a() {
        // start: (-inf; -n), end: (n; +inf), step = 1
        base_test(START_A, END_E, STEP_A, vec![], false)
    }

    #[test]
    fn test_06_start_a_end_a_step_b() {
        // start: (-inf; -n), end (-inf; -n], step = -1
        base_test(START_A, END_A, STEP_B, vec![], false)
    }

    #[test]
    fn test_07_start_a_end_b_step_b() {
        // start: (-inf; -n), end: (-n; 0), step = -1
        base_test(START_A, END_B, STEP_B, vec![], false)
    }

    #[test]
    fn test_08_start_a_end_c_step_b() {
        // start: (-inf; -n), end=0, step = -1
        base_test(START_A, END_C, STEP_B, vec![], false)
    }

    #[test]
    fn test_09_start_a_end_d_step_b() {
        // start: (-inf; -n), end: (0; n], step = -1
        base_test(START_A, END_D, STEP_B, vec![], false)
    }

    #[test]
    fn test_10_start_a_end_e_step_b() {
        // start: (-inf; -n), end: (n; +inf), step = -1
        base_test(START_A, END_E, STEP_B, vec![], false)
    }

    // START_B
    #[test]
    fn test_11_start_b_end_a_step_a() {
        // start: [-n; 0), end (-inf; -n], step = 1
        base_test(START_B, END_A, STEP_A, vec![], false)
    }

    #[test]
    fn test_12_start_b_end_b_step_a() {
        // start: [-n; 0), end: (-n; 0), step = 1
        base_test(START_B, END_B, STEP_A, vec!["b", "c", "d"], false)
    }

    #[test]
    fn test_13_start_b_end_c_step_a() {
        // start: [-n; 0), end=0, step = 1
        base_test(START_B, END_C, STEP_A, vec![], false)
    }

    #[test]
    fn test_14_start_b_end_d_step_a() {
        // start:[-n; 0), end: (0; n], step = 1
        base_test(START_B, END_D, STEP_A, vec!["b", "c"], false)
    }

    #[test]
    fn test_15_start_b_end_e_step_a() {
        // start: [-n; 0), end: (n; +inf), step = 1
        base_test(START_B, END_E, STEP_A, vec![], false)
    }

    #[test]
    fn test_16_start_b_end_a_step_b() {
        // start: [-n; 0), end (-inf; -n], step = -1
        base_test(START_B, END_A, STEP_B, vec![], false)
    }

    #[test]
    fn test_17_start_b_end_b_step_b() {
        // start: [-n; 0), end: (-n; 0), step = -1
        base_test(START_B, END_B, STEP_B, vec!["d", "c", "b"], false)
    }

    #[test]
    fn test_18_start_b_end_c_step_b() {
        // start: [-n; 0), end=0, step = -1
        base_test(START_B, END_C, STEP_B, vec![], false)
    }

    #[test]
    fn test_19_start_b_end_d_step_b() {
        // start: [-n; 0), end: (0; n], step = -1
        base_test(START_B, END_D, STEP_B, vec!["c", "b"], false)
    }

    #[test]
    fn test_20_start_b_end_e_step_b() {
        // start: [-n; 0), end: (n; +inf), step = -1
        base_test(START_B, END_E, STEP_B, vec![], false)
    }

    // START_C

    #[test]
    fn test_21_start_c_end_a_step_a() {
        // start: [0; end), end (-inf; -n], step = 1
        base_test(START_C, END_A, STEP_A, vec![], false)
    }

    #[test]
    fn test_22_start_c_end_b_step_a() {
        // start: [0; end), end: (-n; 0), step = 1
        base_test(START_C, END_B, STEP_A, vec!["c", "d"], false)
    }

    #[test]
    fn test_23_start_c_end_c_step_a() {
        // start: [0; end), end=0, step = 1
        base_test(START_C, END_C, STEP_A, vec![], false)
    }

    #[test]
    fn test_24_start_c_end_d_step_a() {
        // start:[0; end), end: (0; n], step = 1
        base_test(START_C, END_D, STEP_A, vec!["c"], false)
    }

    #[test]
    fn test_25_start_c_end_e_step_a() {
        // start: [0; end), end: (n; +inf), step = 1
        base_test(START_C, END_E, STEP_A, vec![], false)
    }

    #[test]
    fn test_26_start_c_end_a_step_b() {
        // start: [0; end), end (-inf; -n], step = -1
        base_test(START_C, END_A, STEP_B, vec![], false)
    }

    #[test]
    fn test_27_start_c_end_b_step_b() {
        // start: [0; end), end: (-n; 0), step = -1
        base_test(START_C, END_B, STEP_B, vec!["d", "c"], false)
    }

    #[test]
    fn test_28_start_c_end_c_step_b() {
        // start: [0; end), end=0, step = -1
        base_test(START_C, END_C, STEP_B, vec![], false)
    }

    #[test]
    fn test_29_start_c_end_d_step_b() {
        // start: [0; end), end: (0; n], step = -1
        base_test(START_C, END_D, STEP_B, vec!["c"], false)
    }

    #[test]
    fn test_30_start_c_end_e_step_b() {
        // start: [0; end), end: (n; +inf), step = -1
        base_test(START_C, END_E, STEP_B, vec![], false)
    }

    // START_D

    #[test]
    fn test_31_start_d_end_a_step_a() {
        // start: [end: n) , end (-inf; -n], step = 1
        base_test(START_D, END_A, STEP_A, vec![], false)
    }

    #[test]
    fn test_32_start_d_end_b_step_a() {
        // start: [end: n) , end: (-n; 0), step = 1
        base_test(START_D, END_B, STEP_A, vec![], false)
    }

    #[test]
    fn test_33_start_d_end_c_step_a() {
        // start: [end: n) , end=0, step = 1
        base_test(START_D, END_C, STEP_A, vec![], false)
    }

    #[test]
    fn test_34_start_d_end_d_step_a() {
        // start:[end: n) , end: (0; n], step = 1
        base_test(START_D, END_D, STEP_A, vec![], false)
    }

    #[test]
    fn test_35_start_d_end_e_step_a() {
        // start: [end: n) , end: (n; +inf), step = 1
        base_test(START_D, END_E, STEP_A, vec![], false)
    }

    #[test]
    fn test_36_start_d_end_a_step_b() {
        // start: [end: n) , end (-inf; -n], step = -1
        base_test(START_D, END_A, STEP_B, vec![], false)
    }

    #[test]
    fn test_37_start_d_end_b_step_b() {
        // start: [end: n) , end: (-n; 0), step = -1
        base_test(START_D, END_B, STEP_B, vec![], false)
    }

    #[test]
    fn test_38_start_d_end_c_step_b() {
        // start: [end: n) , end=0, step = -1
        base_test(START_D, END_C, STEP_B, vec![], false)
    }

    #[test]
    fn test_39_start_d_end_d_step_b() {
        // start: [end: n) , end: (0; n], step = -1
        base_test(START_D, END_D, STEP_B, vec![], false)
    }

    #[test]
    fn test_40_start_d_end_e_step_b() {
        // start: [end: n) , end: (n; +inf), step = -1
        base_test(START_D, END_E, STEP_B, vec![], false)
    }

    // START_E

    #[test]
    fn test_41_start_e_end_a_step_a() {
        // start: [n: +inf) , end (-inf; -n], step = 1
        base_test(START_E, END_A, STEP_A, vec![], false)
    }

    #[test]
    fn test_42_start_e_end_b_step_a() {
        // start: [n: +inf) , end: (-n; 0), step = 1
        base_test(START_E, END_B, STEP_A, vec![], false)
    }

    #[test]
    fn test_43_start_e_end_c_step_a() {
        // start: [n: +inf) , end=0, step = 1
        base_test(START_E, END_C, STEP_A, vec![], false)
    }

    #[test]
    fn test_44_start_e_end_d_step_a() {
        // start: [n: +inf) , end: (0; n], step = 1
        base_test(START_E, END_D, STEP_A, vec![], false)
    }

    #[test]
    fn test_45_start_e_end_e_step_a() {
        // start: [n: +inf) , end: (n; +inf), step = 1
        base_test(START_E, END_E, STEP_A, vec![], false)
    }

    #[test]
    fn test_46_start_e_end_a_step_b() {
        // start: [n: +inf) , end (-inf; -n], step = -1
        base_test(START_E, END_A, STEP_B, vec![], false)
    }

    #[test]
    fn test_47_start_e_end_b_step_b() {
        // start: [n: +inf) , end: (-n; 0), step = -1
        base_test(START_E, END_B, STEP_B, vec![], false)
    }

    #[test]
    fn test_48_start_e_end_c_step_b() {
        // start: [n: +inf) , end=0, step = -1
        base_test(START_E, END_C, STEP_B, vec![], false)
    }

    #[test]
    fn test_49_start_e_end_d_step_b() {
        // start: [n: +inf) , end: (0; n], step = -1
        base_test(START_E, END_D, STEP_B, vec![], false)
    }

    #[test]
    fn test_50_start_e_end_e_step_b() {
        // start: [n: +inf) , end: (n; +inf), step = -1
        base_test(START_E, END_E, STEP_B, vec![], false)
    }

    // Step C
    #[test]
    fn test_51_start_b_end_b_step_c() {
        // start: [-n; 0), end: (-n; 0), step = 2
        base_test(START_B, END_B, STEP_C, vec!["b", "d"], false)
    }

    #[test]
    fn test_52_start_b_end_d_step_c() {
        // start: [-n; 0), end: (-n; 0), step = 2
        base_test(START_B, END_D, STEP_C, vec!["b"], false)
    }

    #[test]
    fn test_53_start_c_end_b_step_c() {
        // start: [-n; 0), end: (-n; 0), step = 2
        base_test(START_C, END_B, STEP_C, vec!["c"], false)
    }

    #[test]
    fn test_54_start_c_end_d_step_c() {
        // start: [-n; 0), end: (-n; 0), step = 2
        base_test(START_C, END_D, STEP_C, vec!["c"], false)
    }

    // Step D
    #[test]
    fn test_55_start_b_end_b_step_d() {
        // start: [-n; 0), end: (-n; 0), step = -2
        base_test(START_B, END_B, STEP_D, vec!["d", "b"], false)
    }

    #[test]
    fn test_56_start_b_end_d_step_d() {
        // start: [-n; 0), end: (-n; 0), step = -2
        base_test(START_B, END_D, STEP_D, vec!["b"], false)
    }

    #[test]
    fn test_57_start_c_end_b_step_d() {
        // start: [-n; 0), end: (-n; 0), step = -2
        base_test(START_C, END_B, STEP_D, vec!["c"], false)
    }

    #[test]
    fn test_58_start_c_end_d_step_d() {
        // start: [-n; 0), end: (-n; 0), step = -2
        base_test(START_C, END_D, STEP_D, vec!["c"], false)
    }

    // Step E
    #[test]
    fn test_59_start_b_end_b_step_e() {
        // start: [-n; 0), end: (-n; 0), step = 0
        base_test(START_B, END_B, STEP_E, vec![], false)
    }

    #[test]
    fn test_60_start_b_end_d_step_e() {
        // start: [-n; 0), end: (-n; 0), step = 0
        base_test(START_B, END_D, STEP_E, vec![], false)
    }

    #[test]
    fn test_61_start_c_end_b_step_e() {
        // start: [-n; 0), end: (-n; 0), step = 0
        base_test(START_C, END_B, STEP_E, vec![], false)
    }

    #[test]
    fn test_62_start_c_end_d_step_e() {
        // start: [-n; 0), end: (-n; 0), step = 0
        base_test(START_C, END_D, STEP_E, vec![], false)
    }

    #[test]
    fn test_63_start_a_end_d_step_a_complement() {
        // start: (-inf; -n), end: (0; n], step = 1
        base_test(START_A, END_D, STEP_A, vec![], true)
    }

    #[test]
    fn test_64_start_c_end_d_step_a_complement() {
        // start:[0; end), end: (0; n], step = 1
        base_test(START_C, END_D, STEP_A, vec!["a", "b", "d", "e"], true)
    }

    #[test]
    fn test_65_start_c_end_f_step_c_complement() {
        // start:[0; end), end: (0; n], step = 2
        base_test(START_C, END_F, STEP_C, vec!["a", "b", "d"], true)
    }

    #[test]
    fn test_66_start_c_end_f_step_b_complement() {
        // start:[0; end), end: (0; n], step = -1
        base_test(START_C, END_F, STEP_B, vec!["b", "a"], true)
    }

    #[test]
    fn test_67_start_e_end_e_step_a_complement() {
        // start: (-inf; -n), end: (0; n], step = 1, start >= end
        base_test(START_E, END_E, STEP_A, vec!["a", "b", "c", "d", "e"], true)
    }

    fn base_test(
        start: i32,
        end: i32,
        step: i32,
        expected_content: Vec<&str>,
        is_showing_complement: bool,
    ) {
        // Arrange
        let range = Range::new(start, end, step);

        // Act
        let actual_content = cut_line_with_characters(CONTENT, range, is_showing_complement);

        // Assert
        assert_eq!(expected_content, actual_content);
    }
}

#[cfg(test)]
mod unit_tests_cut_line_with_bytes {
    use crate::range_parser::Range;

    use super::cut_line_with_bytes;

    /*
    start values:
        A: (-inf; -n) => []
        B: [-n; 0)
        C: [0; end)
        D: [end: n) => []
        E: [n: +inf) => []

    end values:
        A: (-inf; -n] => []
        B: (-n; 0)
        C: =0 => []
        D: (0; n]
        E: (n; +inf) => []

    step values:
        A: step = 1
        B: step = -1
        C: step = 2
        D: step = -2
        E: step = 0
     */

    static CONTENT: &str = "abcde";

    static N: i32 = 5 as i32;

    static START_A: i32 = -N - 3; // -8
    static START_B: i32 = -N + 1; // -4
    static START_C: i32 = 2;
    static START_D: i32 = 4;
    static START_E: i32 = N + 2; // 7

    static END_A: i32 = -N - 4; // -9
    static END_B: i32 = -N + 4; // -1
    static END_C: i32 = 0;
    static END_D: i32 = N - 2; // 3
    static END_E: i32 = N + 4; // 9
    static END_F: i32 = N; // 5

    static STEP_A: i32 = 1;
    static STEP_B: i32 = -1;
    static STEP_C: i32 = 2;
    static STEP_D: i32 = -2;
    static STEP_E: i32 = 0;

    // START_A
    #[test]
    fn test_01_start_a_end_a_step_a() {
        // start: (-inf; -n), end (-inf; -n], step = 1
        base_test(START_A, END_A, STEP_A, vec![], false)
    }

    #[test]
    fn test_02_start_a_end_b_step_a() {
        // start: (-inf; -n), end: (-n; 0), step = 1
        base_test(START_A, END_B, STEP_A, vec![], false)
    }

    #[test]
    fn test_03_start_a_end_c_step_a() {
        // start: (-inf; -n), end=0, step = 1
        base_test(START_A, END_C, STEP_A, vec![], false)
    }

    #[test]
    fn test_04_start_a_end_d_step_a() {
        // start: (-inf; -n), end: (0; n], step = 1
        base_test(START_A, END_D, STEP_A, vec![], false)
    }

    #[test]
    fn test_05_start_a_end_e_step_a() {
        // start: (-inf; -n), end: (n; +inf), step = 1
        base_test(START_A, END_E, STEP_A, vec![], false)
    }

    #[test]
    fn test_06_start_a_end_a_step_b() {
        // start: (-inf; -n), end (-inf; -n], step = -1
        base_test(START_A, END_A, STEP_B, vec![], false)
    }

    #[test]
    fn test_07_start_a_end_b_step_b() {
        // start: (-inf; -n), end: (-n; 0), step = -1
        base_test(START_A, END_B, STEP_B, vec![], false)
    }

    #[test]
    fn test_08_start_a_end_c_step_b() {
        // start: (-inf; -n), end=0, step = -1
        base_test(START_A, END_C, STEP_B, vec![], false)
    }

    #[test]
    fn test_09_start_a_end_d_step_b() {
        // start: (-inf; -n), end: (0; n], step = -1
        base_test(START_A, END_D, STEP_B, vec![], false)
    }

    #[test]
    fn test_10_start_a_end_e_step_b() {
        // start: (-inf; -n), end: (n; +inf), step = -1
        base_test(START_A, END_E, STEP_B, vec![], false)
    }

    // START_B
    #[test]
    fn test_11_start_b_end_a_step_a() {
        // start: [-n; 0), end (-inf; -n], step = 1
        base_test(START_B, END_A, STEP_A, vec![], false)
    }

    #[test]
    fn test_12_start_b_end_b_step_a() {
        // start: [-n; 0), end: (-n; 0), step = 1
        base_test(START_B, END_B, STEP_A, vec!["b", "c", "d"], false)
    }

    #[test]
    fn test_13_start_b_end_c_step_a() {
        // start: [-n; 0), end=0, step = 1
        base_test(START_B, END_C, STEP_A, vec![], false)
    }

    #[test]
    fn test_14_start_b_end_d_step_a() {
        // start:[-n; 0), end: (0; n], step = 1
        base_test(START_B, END_D, STEP_A, vec!["b", "c"], false)
    }

    #[test]
    fn test_15_start_b_end_e_step_a() {
        // start: [-n; 0), end: (n; +inf), step = 1
        base_test(START_B, END_E, STEP_A, vec![], false)
    }

    #[test]
    fn test_16_start_b_end_a_step_b() {
        // start: [-n; 0), end (-inf; -n], step = -1
        base_test(START_B, END_A, STEP_B, vec![], false)
    }

    #[test]
    fn test_17_start_b_end_b_step_b() {
        // start: [-n; 0), end: (-n; 0), step = -1
        base_test(START_B, END_B, STEP_B, vec!["d", "c", "b"], false)
    }

    #[test]
    fn test_18_start_b_end_c_step_b() {
        // start: [-n; 0), end=0, step = -1
        base_test(START_B, END_C, STEP_B, vec![], false)
    }

    #[test]
    fn test_19_start_b_end_d_step_b() {
        // start: [-n; 0), end: (0; n], step = -1
        base_test(START_B, END_D, STEP_B, vec!["c", "b"], false)
    }

    #[test]
    fn test_20_start_b_end_e_step_b() {
        // start: [-n; 0), end: (n; +inf), step = -1
        base_test(START_B, END_E, STEP_B, vec![], false)
    }

    // START_C

    #[test]
    fn test_21_start_c_end_a_step_a() {
        // start: [0; end), end (-inf; -n], step = 1
        base_test(START_C, END_A, STEP_A, vec![], false)
    }

    #[test]
    fn test_22_start_c_end_b_step_a() {
        // start: [0; end), end: (-n; 0), step = 1
        base_test(START_C, END_B, STEP_A, vec!["c", "d"], false)
    }

    #[test]
    fn test_23_start_c_end_c_step_a() {
        // start: [0; end), end=0, step = 1
        base_test(START_C, END_C, STEP_A, vec![], false)
    }

    #[test]
    fn test_24_start_c_end_d_step_a() {
        // start:[0; end), end: (0; n], step = 1
        base_test(START_C, END_D, STEP_A, vec!["c"], false)
    }

    #[test]
    fn test_25_start_c_end_e_step_a() {
        // start: [0; end), end: (n; +inf), step = 1
        base_test(START_C, END_E, STEP_A, vec![], false)
    }

    #[test]
    fn test_26_start_c_end_a_step_b() {
        // start: [0; end), end (-inf; -n], step = -1
        base_test(START_C, END_A, STEP_B, vec![], false)
    }

    #[test]
    fn test_27_start_c_end_b_step_b() {
        // start: [0; end), end: (-n; 0), step = -1
        base_test(START_C, END_B, STEP_B, vec!["d", "c"], false)
    }

    #[test]
    fn test_28_start_c_end_c_step_b() {
        // start: [0; end), end=0, step = -1
        base_test(START_C, END_C, STEP_B, vec![], false)
    }

    #[test]
    fn test_29_start_c_end_d_step_b() {
        // start: [0; end), end: (0; n], step = -1
        base_test(START_C, END_D, STEP_B, vec!["c"], false)
    }

    #[test]
    fn test_30_start_c_end_e_step_b() {
        // start: [0; end), end: (n; +inf), step = -1
        base_test(START_C, END_E, STEP_B, vec![], false)
    }

    // START_D

    #[test]
    fn test_31_start_d_end_a_step_a() {
        // start: [end: n) , end (-inf; -n], step = 1
        base_test(START_D, END_A, STEP_A, vec![], false)
    }

    #[test]
    fn test_32_start_d_end_b_step_a() {
        // start: [end: n) , end: (-n; 0), step = 1
        base_test(START_D, END_B, STEP_A, vec![], false)
    }

    #[test]
    fn test_33_start_d_end_c_step_a() {
        // start: [end: n) , end=0, step = 1
        base_test(START_D, END_C, STEP_A, vec![], false)
    }

    #[test]
    fn test_34_start_d_end_d_step_a() {
        // start:[end: n) , end: (0; n], step = 1
        base_test(START_D, END_D, STEP_A, vec![], false)
    }

    #[test]
    fn test_35_start_d_end_e_step_a() {
        // start: [end: n) , end: (n; +inf), step = 1
        base_test(START_D, END_E, STEP_A, vec![], false)
    }

    #[test]
    fn test_36_start_d_end_a_step_b() {
        // start: [end: n) , end (-inf; -n], step = -1
        base_test(START_D, END_A, STEP_B, vec![], false)
    }

    #[test]
    fn test_37_start_d_end_b_step_b() {
        // start: [end: n) , end: (-n; 0), step = -1
        base_test(START_D, END_B, STEP_B, vec![], false)
    }

    #[test]
    fn test_38_start_d_end_c_step_b() {
        // start: [end: n) , end=0, step = -1
        base_test(START_D, END_C, STEP_B, vec![], false)
    }

    #[test]
    fn test_39_start_d_end_d_step_b() {
        // start: [end: n) , end: (0; n], step = -1
        base_test(START_D, END_D, STEP_B, vec![], false)
    }

    #[test]
    fn test_40_start_d_end_e_step_b() {
        // start: [end: n) , end: (n; +inf), step = -1
        base_test(START_D, END_E, STEP_B, vec![], false)
    }

    // START_E

    #[test]
    fn test_41_start_e_end_a_step_a() {
        // start: [n: +inf) , end (-inf; -n], step = 1
        base_test(START_E, END_A, STEP_A, vec![], false)
    }

    #[test]
    fn test_42_start_e_end_b_step_a() {
        // start: [n: +inf) , end: (-n; 0), step = 1
        base_test(START_E, END_B, STEP_A, vec![], false)
    }

    #[test]
    fn test_43_start_e_end_c_step_a() {
        // start: [n: +inf) , end=0, step = 1
        base_test(START_E, END_C, STEP_A, vec![], false)
    }

    #[test]
    fn test_44_start_e_end_d_step_a() {
        // start: [n: +inf) , end: (0; n], step = 1
        base_test(START_E, END_D, STEP_A, vec![], false)
    }

    #[test]
    fn test_45_start_e_end_e_step_a() {
        // start: [n: +inf) , end: (n; +inf), step = 1
        base_test(START_E, END_E, STEP_A, vec![], false)
    }

    #[test]
    fn test_46_start_e_end_a_step_b() {
        // start: [n: +inf) , end (-inf; -n], step = -1
        base_test(START_E, END_A, STEP_B, vec![], false)
    }

    #[test]
    fn test_47_start_e_end_b_step_b() {
        // start: [n: +inf) , end: (-n; 0), step = -1
        base_test(START_E, END_B, STEP_B, vec![], false)
    }

    #[test]
    fn test_48_start_e_end_c_step_b() {
        // start: [n: +inf) , end=0, step = -1
        base_test(START_E, END_C, STEP_B, vec![], false)
    }

    #[test]
    fn test_49_start_e_end_d_step_b() {
        // start: [n: +inf) , end: (0; n], step = -1
        base_test(START_E, END_D, STEP_B, vec![], false)
    }

    #[test]
    fn test_50_start_e_end_e_step_b() {
        // start: [n: +inf) , end: (n; +inf), step = -1
        base_test(START_E, END_E, STEP_B, vec![], false)
    }

    // Step C
    #[test]
    fn test_51_start_b_end_b_step_c() {
        // start: [-n; 0), end: (-n; 0), step = 2
        base_test(START_B, END_B, STEP_C, vec!["b", "d"], false)
    }

    #[test]
    fn test_52_start_b_end_d_step_c() {
        // start: [-n; 0), end: (-n; 0), step = 2
        base_test(START_B, END_D, STEP_C, vec!["b"], false)
    }

    #[test]
    fn test_53_start_c_end_b_step_c() {
        // start: [-n; 0), end: (-n; 0), step = 2
        base_test(START_C, END_B, STEP_C, vec!["c"], false)
    }

    #[test]
    fn test_54_start_c_end_d_step_c() {
        // start: [-n; 0), end: (-n; 0), step = 2
        base_test(START_C, END_D, STEP_C, vec!["c"], false)
    }

    // Step D
    #[test]
    fn test_55_start_b_end_b_step_d() {
        // start: [-n; 0), end: (-n; 0), step = -2
        base_test(START_B, END_B, STEP_D, vec!["d", "b"], false)
    }

    #[test]
    fn test_56_start_b_end_d_step_d() {
        // start: [-n; 0), end: (-n; 0), step = -2
        base_test(START_B, END_D, STEP_D, vec!["b"], false)
    }

    #[test]
    fn test_57_start_c_end_b_step_d() {
        // start: [-n; 0), end: (-n; 0), step = -2
        base_test(START_C, END_B, STEP_D, vec!["c"], false)
    }

    #[test]
    fn test_58_start_c_end_d_step_d() {
        // start: [-n; 0), end: (-n; 0), step = -2
        base_test(START_C, END_D, STEP_D, vec!["c"], false)
    }

    // Step E
    #[test]
    fn test_59_start_b_end_b_step_e() {
        // start: [-n; 0), end: (-n; 0), step = 0
        base_test(START_B, END_B, STEP_E, vec![], false)
    }

    #[test]
    fn test_60_start_b_end_d_step_e() {
        // start: [-n; 0), end: (-n; 0), step = 0
        base_test(START_B, END_D, STEP_E, vec![], false)
    }

    #[test]
    fn test_61_start_c_end_b_step_e() {
        // start: [-n; 0), end: (-n; 0), step = 0
        base_test(START_C, END_B, STEP_E, vec![], false)
    }

    #[test]
    fn test_62_start_c_end_d_step_e() {
        // start: [-n; 0), end: (-n; 0), step = 0
        base_test(START_C, END_D, STEP_E, vec![], false)
    }

    #[test]
    fn test_63_start_a_end_d_step_a_complement() {
        // start: (-inf; -n), end: (0; n], step = 1
        base_test(START_A, END_D, STEP_A, vec![], true)
    }

    #[test]
    fn test_64_start_c_end_d_step_a_complement() {
        // start:[0; end), end: (0; n], step = 1
        base_test(START_C, END_D, STEP_A, vec!["a", "b", "d", "e"], true)
    }

    #[test]
    fn test_65_start_c_end_f_step_c_complement() {
        // start:[0; end), end: (0; n], step = 2
        base_test(START_C, END_F, STEP_C, vec!["a", "b", "d"], true)
    }

    #[test]
    fn test_66_start_c_end_f_step_b_complement() {
        // start:[0; end), end: (0; n], step = -1
        base_test(START_C, END_F, STEP_B, vec!["b", "a"], true)
    }

    #[test]
    fn test_67_start_e_end_e_step_a_complement() {
        // start: (-inf; -n), end: (0; n], step = 1, start >= end
        base_test(START_E, END_E, STEP_A, vec!["a", "b", "c", "d", "e"], true)
    }

    #[test]
    fn test_68_non_utf_8_bytes() {
        // Arrange
        let range = Range::new(0, 3, 1);
        let bytes: Vec<u16> = vec![0x68, 0xC0];
        let content: String = String::from_utf16(&bytes).unwrap();

        println!("{}", content);

        // Act
        let actual_content = cut_line_with_bytes(&content, range, false);

        // Assert
        assert_eq!(vec!["h", "0xc3", "0x80"], actual_content);
    }

    fn base_test(
        start: i32,
        end: i32,
        step: i32,
        expected_content: Vec<&str>,
        is_showing_complement: bool,
    ) {
        // Arrange
        let range = Range::new(start, end, step);

        // Act
        let actual_content = cut_line_with_bytes(CONTENT, range, is_showing_complement);

        // Assert
        assert_eq!(expected_content, actual_content);
    }
}
