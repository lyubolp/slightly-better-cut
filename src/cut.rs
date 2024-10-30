use crate::range_parser::Range;
use std::collections::HashSet;

pub fn cut_line_with_delimiter(line: &str, range: Range, delimiter: String) -> Vec<String> {
    let items: Vec<String> = line
        .split(&delimiter)
        .map(|item| String::from(item))
        .collect();
    let n = items.len() as i32;

    cut_line(items, range, n)
}

pub fn cut_line_with_bytes(line: &str, range: Range) -> Vec<String> {
    let items: Vec<String> = line.bytes().map(|byte| handle_bytes(byte)).collect();
    let n = line.len() as i32;

    cut_line(items, range, n)
}

pub fn cut_line_with_characters(line: &str, range: Range) -> Vec<String> {
    let items: Vec<String> = line.chars().map(|item| String::from(item)).collect();
    let n = line.len() as i32;

    cut_line(items, range, n)
}

fn cut_line(items: Vec<String>, range: Range, n: i32) -> Vec<String> {
    let (start, end, step) = range.to_tuple();

    if step == 0 {
        return vec![];
    }

    let is_start_within_bounds = -n <= start && start < n;
    //TODO - Add test for getting the last field from line
    let is_end_within_bounds = -n <= end && end != 0 && end <= n;

    if !(is_start_within_bounds && is_end_within_bounds) {
        return vec![];
    }

    let indexes_to_get = match calculate_indexes_to_get(start, n, end, step) {
        Some(value) => value,
        None => return vec![],
    };

    let mut result: Vec<String> = items
        .iter()
        .enumerate()
        .filter(|(index, _)| indexes_to_get.contains(index))
        .map(|(_, item)| item.clone())
        .collect();

    if step < 0 {
        result.reverse();
    }

    result
}

fn calculate_indexes_to_get(start: i32, n: i32, end: i32, step: i32) -> Option<HashSet<usize>> {
    let actual_start = handle_negative_index(start, n);
    let actual_end = handle_negative_index(end, n);
    if actual_start >= actual_end {
        return None;
    }
    let actual_step = step.abs() as usize;
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
        Err(_) => byte.to_string() + "0x",
    }
}

#[cfg(test)]
mod cut_line_with_delimiter {
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

    static STEP_A: i32 = 1;
    static STEP_B: i32 = -1;
    static STEP_C: i32 = 2;
    static STEP_D: i32 = -2;
    static STEP_E: i32 = 0;

    // START_A
    #[test]
    fn test_01_start_a_end_a_step_a() {
        // start: (-inf; -n), end (-inf; -n], step = 1
        base_test(START_A, END_A, STEP_A, vec![])
    }

    #[test]
    fn test_02_start_a_end_b_step_a() {
        // start: (-inf; -n), end: (-n; 0), step = 1
        base_test(START_A, END_B, STEP_A, vec![])
    }

    #[test]
    fn test_03_start_a_end_c_step_a() {
        // start: (-inf; -n), end=0, step = 1
        base_test(START_A, END_C, STEP_A, vec![])
    }

    #[test]
    fn test_04_start_a_end_d_step_a() {
        // start: (-inf; -n), end: (0; n], step = 1
        base_test(START_A, END_D, STEP_A, vec![])
    }

    #[test]
    fn test_05_start_a_end_e_step_a() {
        // start: (-inf; -n), end: (n; +inf), step = 1
        base_test(START_A, END_E, STEP_A, vec![])
    }

    #[test]
    fn test_06_start_a_end_a_step_b() {
        // start: (-inf; -n), end (-inf; -n], step = -1
        base_test(START_A, END_A, STEP_B, vec![])
    }

    #[test]
    fn test_07_start_a_end_b_step_b() {
        // start: (-inf; -n), end: (-n; 0), step = -1
        base_test(START_A, END_B, STEP_B, vec![])
    }

    #[test]
    fn test_08_start_a_end_c_step_b() {
        // start: (-inf; -n), end=0, step = -1
        base_test(START_A, END_C, STEP_B, vec![])
    }

    #[test]
    fn test_09_start_a_end_d_step_b() {
        // start: (-inf; -n), end: (0; n], step = -1
        base_test(START_A, END_D, STEP_B, vec![])
    }

    #[test]
    fn test_10_start_a_end_e_step_b() {
        // start: (-inf; -n), end: (n; +inf), step = -1
        base_test(START_A, END_E, STEP_B, vec![])
    }

    // START_B
    #[test]
    fn test_11_start_b_end_a_step_a() {
        // start: [-n; 0), end (-inf; -n], step = 1
        base_test(START_B, END_A, STEP_A, vec![])
    }

    #[test]
    fn test_12_start_b_end_b_step_a() {
        // start: [-n; 0), end: (-n; 0), step = 1
        base_test(START_B, END_B, STEP_A, vec!["second", "third", "fourth"])
    }

    #[test]
    fn test_13_start_b_end_c_step_a() {
        // start: [-n; 0), end=0, step = 1
        base_test(START_B, END_C, STEP_A, vec![])
    }

    #[test]
    fn test_14_start_b_end_d_step_a() {
        // start:[-n; 0), end: (0; n], step = 1
        base_test(START_B, END_D, STEP_A, vec!["second", "third"])
    }

    #[test]
    fn test_15_start_b_end_e_step_a() {
        // start: [-n; 0), end: (n; +inf), step = 1
        base_test(START_B, END_E, STEP_A, vec![])
    }

    #[test]
    fn test_16_start_b_end_a_step_b() {
        // start: [-n; 0), end (-inf; -n], step = -1
        base_test(START_B, END_A, STEP_B, vec![])
    }

    #[test]
    fn test_17_start_b_end_b_step_b() {
        // start: [-n; 0), end: (-n; 0), step = -1
        base_test(START_B, END_B, STEP_B, vec!["fourth", "third", "second"])
    }

    #[test]
    fn test_18_start_b_end_c_step_b() {
        // start: [-n; 0), end=0, step = -1
        base_test(START_B, END_C, STEP_B, vec![])
    }

    #[test]
    fn test_19_start_b_end_d_step_b() {
        // start: [-n; 0), end: (0; n], step = -1
        base_test(START_B, END_D, STEP_B, vec!["third", "second"])
    }

    #[test]
    fn test_20_start_b_end_e_step_b() {
        // start: [-n; 0), end: (n; +inf), step = -1
        base_test(START_B, END_E, STEP_B, vec![])
    }

    // START_C

    #[test]
    fn test_21_start_c_end_a_step_a() {
        // start: [0; end), end (-inf; -n], step = 1
        base_test(START_C, END_A, STEP_A, vec![])
    }

    #[test]
    fn test_22_start_c_end_b_step_a() {
        // start: [0; end), end: (-n; 0), step = 1
        base_test(START_C, END_B, STEP_A, vec!["third", "fourth"])
    }

    #[test]
    fn test_23_start_c_end_c_step_a() {
        // start: [0; end), end=0, step = 1
        base_test(START_C, END_C, STEP_A, vec![])
    }

    #[test]
    fn test_24_start_c_end_d_step_a() {
        // start:[0; end), end: (0; n], step = 1
        base_test(START_C, END_D, STEP_A, vec!["third"])
    }

    #[test]
    fn test_25_start_c_end_e_step_a() {
        // start: [0; end), end: (n; +inf), step = 1
        base_test(START_C, END_E, STEP_A, vec![])
    }

    #[test]
    fn test_26_start_c_end_a_step_b() {
        // start: [0; end), end (-inf; -n], step = -1
        base_test(START_C, END_A, STEP_B, vec![])
    }

    #[test]
    fn test_27_start_c_end_b_step_b() {
        // start: [0; end), end: (-n; 0), step = -1
        base_test(START_C, END_B, STEP_B, vec!["fourth", "third"])
    }

    #[test]
    fn test_28_start_c_end_c_step_b() {
        // start: [0; end), end=0, step = -1
        base_test(START_C, END_C, STEP_B, vec![])
    }

    #[test]
    fn test_29_start_c_end_d_step_b() {
        // start: [0; end), end: (0; n], step = -1
        base_test(START_C, END_D, STEP_B, vec!["third"])
    }

    #[test]
    fn test_30_start_c_end_e_step_b() {
        // start: [0; end), end: (n; +inf), step = -1
        base_test(START_C, END_E, STEP_B, vec![])
    }

    // START_D

    #[test]
    fn test_31_start_d_end_a_step_a() {
        // start: [end: n) , end (-inf; -n], step = 1
        base_test(START_D, END_A, STEP_A, vec![])
    }

    #[test]
    fn test_32_start_d_end_b_step_a() {
        // start: [end: n) , end: (-n; 0), step = 1
        base_test(START_D, END_B, STEP_A, vec![])
    }

    #[test]
    fn test_33_start_d_end_c_step_a() {
        // start: [end: n) , end=0, step = 1
        base_test(START_D, END_C, STEP_A, vec![])
    }

    #[test]
    fn test_34_start_d_end_d_step_a() {
        // start:[end: n) , end: (0; n], step = 1
        base_test(START_D, END_D, STEP_A, vec![])
    }

    #[test]
    fn test_35_start_d_end_e_step_a() {
        // start: [end: n) , end: (n; +inf), step = 1
        base_test(START_D, END_E, STEP_A, vec![])
    }

    #[test]
    fn test_36_start_d_end_a_step_b() {
        // start: [end: n) , end (-inf; -n], step = -1
        base_test(START_D, END_A, STEP_B, vec![])
    }

    #[test]
    fn test_37_start_d_end_b_step_b() {
        // start: [end: n) , end: (-n; 0), step = -1
        base_test(START_D, END_B, STEP_B, vec![])
    }

    #[test]
    fn test_38_start_d_end_c_step_b() {
        // start: [end: n) , end=0, step = -1
        base_test(START_D, END_C, STEP_B, vec![])
    }

    #[test]
    fn test_39_start_d_end_d_step_b() {
        // start: [end: n) , end: (0; n], step = -1
        base_test(START_D, END_D, STEP_B, vec![])
    }

    #[test]
    fn test_40_start_d_end_e_step_b() {
        // start: [end: n) , end: (n; +inf), step = -1
        base_test(START_D, END_E, STEP_B, vec![])
    }

    // START_E

    #[test]
    fn test_41_start_e_end_a_step_a() {
        // start: [n: +inf) , end (-inf; -n], step = 1
        base_test(START_E, END_A, STEP_A, vec![])
    }

    #[test]
    fn test_42_start_e_end_b_step_a() {
        // start: [n: +inf) , end: (-n; 0), step = 1
        base_test(START_E, END_B, STEP_A, vec![])
    }

    #[test]
    fn test_43_start_e_end_c_step_a() {
        // start: [n: +inf) , end=0, step = 1
        base_test(START_E, END_C, STEP_A, vec![])
    }

    #[test]
    fn test_44_start_e_end_d_step_a() {
        // start: [n: +inf) , end: (0; n], step = 1
        base_test(START_E, END_D, STEP_A, vec![])
    }

    #[test]
    fn test_45_start_e_end_e_step_a() {
        // start: [n: +inf) , end: (n; +inf), step = 1
        base_test(START_E, END_E, STEP_A, vec![])
    }

    #[test]
    fn test_46_start_e_end_a_step_b() {
        // start: [n: +inf) , end (-inf; -n], step = -1
        base_test(START_E, END_A, STEP_B, vec![])
    }

    #[test]
    fn test_47_start_e_end_b_step_b() {
        // start: [n: +inf) , end: (-n; 0), step = -1
        base_test(START_E, END_B, STEP_B, vec![])
    }

    #[test]
    fn test_48_start_e_end_c_step_b() {
        // start: [n: +inf) , end=0, step = -1
        base_test(START_E, END_C, STEP_B, vec![])
    }

    #[test]
    fn test_49_start_e_end_d_step_b() {
        // start: [n: +inf) , end: (0; n], step = -1
        base_test(START_E, END_D, STEP_B, vec![])
    }

    #[test]
    fn test_50_start_e_end_e_step_b() {
        // start: [n: +inf) , end: (n; +inf), step = -1
        base_test(START_E, END_E, STEP_B, vec![])
    }

    // Step C
    #[test]
    fn test_51_start_b_end_b_step_c() {
        // start: [-n; 0), end: (-n; 0), step = 2
        base_test(START_B, END_B, STEP_C, vec!["second", "fourth"])
    }

    #[test]
    fn test_52_start_b_end_d_step_c() {
        // start: [-n; 0), end: (-n; 0), step = 2
        base_test(START_B, END_D, STEP_C, vec!["second"])
    }

    #[test]
    fn test_53_start_c_end_b_step_c() {
        // start: [-n; 0), end: (-n; 0), step = 2
        base_test(START_C, END_B, STEP_C, vec!["third"])
    }

    #[test]
    fn test_54_start_c_end_d_step_c() {
        // start: [-n; 0), end: (-n; 0), step = 2
        base_test(START_C, END_D, STEP_C, vec!["third"])
    }

    // Step D
    #[test]
    fn test_55_start_b_end_b_step_d() {
        // start: [-n; 0), end: (-n; 0), step = -2
        base_test(START_B, END_B, STEP_D, vec!["fourth", "second"])
    }

    #[test]
    fn test_56_start_b_end_d_step_d() {
        // start: [-n; 0), end: (-n; 0), step = -2
        base_test(START_B, END_D, STEP_D, vec!["second"])
    }

    #[test]
    fn test_57_start_c_end_b_step_d() {
        // start: [-n; 0), end: (-n; 0), step = -2
        base_test(START_C, END_B, STEP_D, vec!["third"])
    }

    #[test]
    fn test_58_start_c_end_d_step_d() {
        // start: [-n; 0), end: (-n; 0), step = -2
        base_test(START_C, END_D, STEP_D, vec!["third"])
    }

    // Step E
    #[test]
    fn test_59_start_b_end_b_step_e() {
        // start: [-n; 0), end: (-n; 0), step = 0
        base_test(START_B, END_B, STEP_E, vec![])
    }

    #[test]
    fn test_60_start_b_end_d_step_e() {
        // start: [-n; 0), end: (-n; 0), step = 0
        base_test(START_B, END_D, STEP_E, vec![])
    }

    #[test]
    fn test_61_start_c_end_b_step_e() {
        // start: [-n; 0), end: (-n; 0), step = 0
        base_test(START_C, END_B, STEP_E, vec![])
    }

    #[test]
    fn test_62_start_c_end_d_step_e() {
        // start: [-n; 0), end: (-n; 0), step = 0
        base_test(START_C, END_D, STEP_E, vec![])
    }

    fn base_test(start: i32, end: i32, step: i32, expected_content: Vec<&str>) {
        // Arrange
        let range = Range::new(start, end, step);

        // Act
        let actual_content = cut_line_with_delimiter(CONTENT, range, String::from(DELIMITER));

        // Assert
        assert_eq!(expected_content, actual_content);
    }
}

#[cfg(test)]
mod cut_line_with_characters {
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

    static STEP_A: i32 = 1;
    static STEP_B: i32 = -1;
    static STEP_C: i32 = 2;
    static STEP_D: i32 = -2;
    static STEP_E: i32 = 0;

    // START_A
    #[test]
    fn test_01_start_a_end_a_step_a() {
        // start: (-inf; -n), end (-inf; -n], step = 1
        base_test(START_A, END_A, STEP_A, vec![])
    }

    #[test]
    fn test_02_start_a_end_b_step_a() {
        // start: (-inf; -n), end: (-n; 0), step = 1
        base_test(START_A, END_B, STEP_A, vec![])
    }

    #[test]
    fn test_03_start_a_end_c_step_a() {
        // start: (-inf; -n), end=0, step = 1
        base_test(START_A, END_C, STEP_A, vec![])
    }

    #[test]
    fn test_04_start_a_end_d_step_a() {
        // start: (-inf; -n), end: (0; n], step = 1
        base_test(START_A, END_D, STEP_A, vec![])
    }

    #[test]
    fn test_05_start_a_end_e_step_a() {
        // start: (-inf; -n), end: (n; +inf), step = 1
        base_test(START_A, END_E, STEP_A, vec![])
    }

    #[test]
    fn test_06_start_a_end_a_step_b() {
        // start: (-inf; -n), end (-inf; -n], step = -1
        base_test(START_A, END_A, STEP_B, vec![])
    }

    #[test]
    fn test_07_start_a_end_b_step_b() {
        // start: (-inf; -n), end: (-n; 0), step = -1
        base_test(START_A, END_B, STEP_B, vec![])
    }

    #[test]
    fn test_08_start_a_end_c_step_b() {
        // start: (-inf; -n), end=0, step = -1
        base_test(START_A, END_C, STEP_B, vec![])
    }

    #[test]
    fn test_09_start_a_end_d_step_b() {
        // start: (-inf; -n), end: (0; n], step = -1
        base_test(START_A, END_D, STEP_B, vec![])
    }

    #[test]
    fn test_10_start_a_end_e_step_b() {
        // start: (-inf; -n), end: (n; +inf), step = -1
        base_test(START_A, END_E, STEP_B, vec![])
    }

    // START_B
    #[test]
    fn test_11_start_b_end_a_step_a() {
        // start: [-n; 0), end (-inf; -n], step = 1
        base_test(START_B, END_A, STEP_A, vec![])
    }

    #[test]
    fn test_12_start_b_end_b_step_a() {
        // start: [-n; 0), end: (-n; 0), step = 1
        base_test(START_B, END_B, STEP_A, vec!["b", "c", "d"])
    }

    #[test]
    fn test_13_start_b_end_c_step_a() {
        // start: [-n; 0), end=0, step = 1
        base_test(START_B, END_C, STEP_A, vec![])
    }

    #[test]
    fn test_14_start_b_end_d_step_a() {
        // start:[-n; 0), end: (0; n], step = 1
        base_test(START_B, END_D, STEP_A, vec!["b", "c"])
    }

    #[test]
    fn test_15_start_b_end_e_step_a() {
        // start: [-n; 0), end: (n; +inf), step = 1
        base_test(START_B, END_E, STEP_A, vec![])
    }

    #[test]
    fn test_16_start_b_end_a_step_b() {
        // start: [-n; 0), end (-inf; -n], step = -1
        base_test(START_B, END_A, STEP_B, vec![])
    }

    #[test]
    fn test_17_start_b_end_b_step_b() {
        // start: [-n; 0), end: (-n; 0), step = -1
        base_test(START_B, END_B, STEP_B, vec!["d", "c", "b"])
    }

    #[test]
    fn test_18_start_b_end_c_step_b() {
        // start: [-n; 0), end=0, step = -1
        base_test(START_B, END_C, STEP_B, vec![])
    }

    #[test]
    fn test_19_start_b_end_d_step_b() {
        // start: [-n; 0), end: (0; n], step = -1
        base_test(START_B, END_D, STEP_B, vec!["c", "b"])
    }

    #[test]
    fn test_20_start_b_end_e_step_b() {
        // start: [-n; 0), end: (n; +inf), step = -1
        base_test(START_B, END_E, STEP_B, vec![])
    }

    // START_C

    #[test]
    fn test_21_start_c_end_a_step_a() {
        // start: [0; end), end (-inf; -n], step = 1
        base_test(START_C, END_A, STEP_A, vec![])
    }

    #[test]
    fn test_22_start_c_end_b_step_a() {
        // start: [0; end), end: (-n; 0), step = 1
        base_test(START_C, END_B, STEP_A, vec!["c", "d"])
    }

    #[test]
    fn test_23_start_c_end_c_step_a() {
        // start: [0; end), end=0, step = 1
        base_test(START_C, END_C, STEP_A, vec![])
    }

    #[test]
    fn test_24_start_c_end_d_step_a() {
        // start:[0; end), end: (0; n], step = 1
        base_test(START_C, END_D, STEP_A, vec!["c"])
    }

    #[test]
    fn test_25_start_c_end_e_step_a() {
        // start: [0; end), end: (n; +inf), step = 1
        base_test(START_C, END_E, STEP_A, vec![])
    }

    #[test]
    fn test_26_start_c_end_a_step_b() {
        // start: [0; end), end (-inf; -n], step = -1
        base_test(START_C, END_A, STEP_B, vec![])
    }

    #[test]
    fn test_27_start_c_end_b_step_b() {
        // start: [0; end), end: (-n; 0), step = -1
        base_test(START_C, END_B, STEP_B, vec!["d", "c"])
    }

    #[test]
    fn test_28_start_c_end_c_step_b() {
        // start: [0; end), end=0, step = -1
        base_test(START_C, END_C, STEP_B, vec![])
    }

    #[test]
    fn test_29_start_c_end_d_step_b() {
        // start: [0; end), end: (0; n], step = -1
        base_test(START_C, END_D, STEP_B, vec!["c"])
    }

    #[test]
    fn test_30_start_c_end_e_step_b() {
        // start: [0; end), end: (n; +inf), step = -1
        base_test(START_C, END_E, STEP_B, vec![])
    }

    // START_D

    #[test]
    fn test_31_start_d_end_a_step_a() {
        // start: [end: n) , end (-inf; -n], step = 1
        base_test(START_D, END_A, STEP_A, vec![])
    }

    #[test]
    fn test_32_start_d_end_b_step_a() {
        // start: [end: n) , end: (-n; 0), step = 1
        base_test(START_D, END_B, STEP_A, vec![])
    }

    #[test]
    fn test_33_start_d_end_c_step_a() {
        // start: [end: n) , end=0, step = 1
        base_test(START_D, END_C, STEP_A, vec![])
    }

    #[test]
    fn test_34_start_d_end_d_step_a() {
        // start:[end: n) , end: (0; n], step = 1
        base_test(START_D, END_D, STEP_A, vec![])
    }

    #[test]
    fn test_35_start_d_end_e_step_a() {
        // start: [end: n) , end: (n; +inf), step = 1
        base_test(START_D, END_E, STEP_A, vec![])
    }

    #[test]
    fn test_36_start_d_end_a_step_b() {
        // start: [end: n) , end (-inf; -n], step = -1
        base_test(START_D, END_A, STEP_B, vec![])
    }

    #[test]
    fn test_37_start_d_end_b_step_b() {
        // start: [end: n) , end: (-n; 0), step = -1
        base_test(START_D, END_B, STEP_B, vec![])
    }

    #[test]
    fn test_38_start_d_end_c_step_b() {
        // start: [end: n) , end=0, step = -1
        base_test(START_D, END_C, STEP_B, vec![])
    }

    #[test]
    fn test_39_start_d_end_d_step_b() {
        // start: [end: n) , end: (0; n], step = -1
        base_test(START_D, END_D, STEP_B, vec![])
    }

    #[test]
    fn test_40_start_d_end_e_step_b() {
        // start: [end: n) , end: (n; +inf), step = -1
        base_test(START_D, END_E, STEP_B, vec![])
    }

    // START_E

    #[test]
    fn test_41_start_e_end_a_step_a() {
        // start: [n: +inf) , end (-inf; -n], step = 1
        base_test(START_E, END_A, STEP_A, vec![])
    }

    #[test]
    fn test_42_start_e_end_b_step_a() {
        // start: [n: +inf) , end: (-n; 0), step = 1
        base_test(START_E, END_B, STEP_A, vec![])
    }

    #[test]
    fn test_43_start_e_end_c_step_a() {
        // start: [n: +inf) , end=0, step = 1
        base_test(START_E, END_C, STEP_A, vec![])
    }

    #[test]
    fn test_44_start_e_end_d_step_a() {
        // start: [n: +inf) , end: (0; n], step = 1
        base_test(START_E, END_D, STEP_A, vec![])
    }

    #[test]
    fn test_45_start_e_end_e_step_a() {
        // start: [n: +inf) , end: (n; +inf), step = 1
        base_test(START_E, END_E, STEP_A, vec![])
    }

    #[test]
    fn test_46_start_e_end_a_step_b() {
        // start: [n: +inf) , end (-inf; -n], step = -1
        base_test(START_E, END_A, STEP_B, vec![])
    }

    #[test]
    fn test_47_start_e_end_b_step_b() {
        // start: [n: +inf) , end: (-n; 0), step = -1
        base_test(START_E, END_B, STEP_B, vec![])
    }

    #[test]
    fn test_48_start_e_end_c_step_b() {
        // start: [n: +inf) , end=0, step = -1
        base_test(START_E, END_C, STEP_B, vec![])
    }

    #[test]
    fn test_49_start_e_end_d_step_b() {
        // start: [n: +inf) , end: (0; n], step = -1
        base_test(START_E, END_D, STEP_B, vec![])
    }

    #[test]
    fn test_50_start_e_end_e_step_b() {
        // start: [n: +inf) , end: (n; +inf), step = -1
        base_test(START_E, END_E, STEP_B, vec![])
    }

    // Step C
    #[test]
    fn test_51_start_b_end_b_step_c() {
        // start: [-n; 0), end: (-n; 0), step = 2
        base_test(START_B, END_B, STEP_C, vec!["b", "d"])
    }

    #[test]
    fn test_52_start_b_end_d_step_c() {
        // start: [-n; 0), end: (-n; 0), step = 2
        base_test(START_B, END_D, STEP_C, vec!["b"])
    }

    #[test]
    fn test_53_start_c_end_b_step_c() {
        // start: [-n; 0), end: (-n; 0), step = 2
        base_test(START_C, END_B, STEP_C, vec!["c"])
    }

    #[test]
    fn test_54_start_c_end_d_step_c() {
        // start: [-n; 0), end: (-n; 0), step = 2
        base_test(START_C, END_D, STEP_C, vec!["c"])
    }

    // Step D
    #[test]
    fn test_55_start_b_end_b_step_d() {
        // start: [-n; 0), end: (-n; 0), step = -2
        base_test(START_B, END_B, STEP_D, vec!["d", "b"])
    }

    #[test]
    fn test_56_start_b_end_d_step_d() {
        // start: [-n; 0), end: (-n; 0), step = -2
        base_test(START_B, END_D, STEP_D, vec!["b"])
    }

    #[test]
    fn test_57_start_c_end_b_step_d() {
        // start: [-n; 0), end: (-n; 0), step = -2
        base_test(START_C, END_B, STEP_D, vec!["c"])
    }

    #[test]
    fn test_58_start_c_end_d_step_d() {
        // start: [-n; 0), end: (-n; 0), step = -2
        base_test(START_C, END_D, STEP_D, vec!["c"])
    }

    // Step E
    #[test]
    fn test_59_start_b_end_b_step_e() {
        // start: [-n; 0), end: (-n; 0), step = 0
        base_test(START_B, END_B, STEP_E, vec![])
    }

    #[test]
    fn test_60_start_b_end_d_step_e() {
        // start: [-n; 0), end: (-n; 0), step = 0
        base_test(START_B, END_D, STEP_E, vec![])
    }

    #[test]
    fn test_61_start_c_end_b_step_e() {
        // start: [-n; 0), end: (-n; 0), step = 0
        base_test(START_C, END_B, STEP_E, vec![])
    }

    #[test]
    fn test_62_start_c_end_d_step_e() {
        // start: [-n; 0), end: (-n; 0), step = 0
        base_test(START_C, END_D, STEP_E, vec![])
    }

    fn base_test(start: i32, end: i32, step: i32, expected_content: Vec<&str>) {
        // Arrange
        let range = Range::new(start, end, step);

        // Act
        let actual_content = cut_line_with_characters(CONTENT, range);

        // Assert
        assert_eq!(expected_content, actual_content);
    }
}
