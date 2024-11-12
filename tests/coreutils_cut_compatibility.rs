use std::process::Command;

// -b 
// -c
// -d & -f
    // For each cutter, try
        // N
        // N-
        // N-M
        // -M
        // 1 & 2
        // 2 & 3 & 4

// --complement
// -s
// --output-delimiter
// -z
// STDIN

static SINGLE_RANGE_CUT: &str = "2";
static SINGLE_RANGE_SBCUT: &str = "1";

static START_RANGE_CUT: &str = "2-";
static START_RANGE_SBCUT: &str = "1:";

static START_END_RANGE_CUT: &str = "2-4";
static START_END_RANGE_SBCUT: &str = "1:4";

static END_RANGE_CUT: &str = "-4";
static END_RANGE_SBCUT: &str = ":4";

static SINGLE_START_RANGE_CUT: &str = "2,3-";
static SINGLE_START_RANGE_SBCUT: &str = "1,2:";

static SINGLE_SINGLE_START_RANGE_CUT: &str = "1,2,4-";
static SINGLE_SINGLE_START_RANGE_SBCUT: &str = "0,1,3:";

static SAMPLE_FILE: &str = "sample.csv";

#[test]
fn test_01_1_bytes_single_range(){
    // Arrange
    let cut_arguments = vec!["-b", SINGLE_RANGE_CUT, SAMPLE_FILE];
    let sbcut_arguments = vec!["-b", SINGLE_RANGE_SBCUT, SAMPLE_FILE];

    // Act
    let cut_output = call_cut(cut_arguments);
    let sbcut_output = call_sbcut(sbcut_arguments);

    // Assert
    assert_eq!(cut_output, sbcut_output)
}

#[test]
fn test_01_2_bytes_start_range(){
    // Arrange
    let cut_arguments = vec!["-b", START_RANGE_CUT, SAMPLE_FILE];
    let sbcut_arguments = vec!["-b", START_RANGE_SBCUT, SAMPLE_FILE];

    // Act
    let cut_output = call_cut(cut_arguments);
    let sbcut_output = call_sbcut(sbcut_arguments);

    // Assert
    assert_eq!(cut_output, sbcut_output)
}

#[test]
fn test_01_3_bytes_start_end_range(){
    // Arrange
    let cut_arguments = vec!["-b", START_END_RANGE_CUT, SAMPLE_FILE];
    let sbcut_arguments = vec!["-b", START_END_RANGE_SBCUT, SAMPLE_FILE];

    // Act
    let cut_output = call_cut(cut_arguments);
    let sbcut_output = call_sbcut(sbcut_arguments);

    // Assert
    assert_eq!(cut_output, sbcut_output)
}

#[test]
fn test_01_4_bytes_end_range(){
    // Arrange
    let cut_arguments = vec!["-b", END_RANGE_CUT, SAMPLE_FILE];
    let sbcut_arguments = vec!["-b", END_RANGE_SBCUT, SAMPLE_FILE];

    // Act
    let cut_output = call_cut(cut_arguments);
    let sbcut_output = call_sbcut(sbcut_arguments);

    // Assert
    assert_eq!(cut_output, sbcut_output)
}

#[test]
fn test_01_5_bytes_single_start_range(){
    // Arrange
    let cut_arguments = vec!["-b", SINGLE_START_RANGE_CUT, SAMPLE_FILE];
    let sbcut_arguments = vec!["-b", SINGLE_START_RANGE_SBCUT, SAMPLE_FILE];

    // Act
    let cut_output = call_cut(cut_arguments);
    let sbcut_output = call_sbcut(sbcut_arguments);

    // Assert
    assert_eq!(cut_output, sbcut_output)
}

#[test]
fn test_01_6_bytes_single_single_start_range(){
    // Arrange
    let cut_arguments = vec!["-b", SINGLE_SINGLE_START_RANGE_CUT, SAMPLE_FILE];
    let sbcut_arguments = vec!["-b", SINGLE_SINGLE_START_RANGE_SBCUT, SAMPLE_FILE];

    // Act
    let cut_output = call_cut(cut_arguments);
    let sbcut_output = call_sbcut(sbcut_arguments);

    // Assert
    assert_eq!(cut_output, sbcut_output)
}

#[test]
fn test_02_1_characters_single_range(){
    // Arrange
    let cut_arguments = vec!["-c", SINGLE_RANGE_CUT, SAMPLE_FILE];
    let sbcut_arguments = vec!["-c", SINGLE_RANGE_SBCUT, SAMPLE_FILE];

    // Act
    let cut_output = call_cut(cut_arguments);
    let sbcut_output = call_sbcut(sbcut_arguments);

    // Assert
    assert_eq!(cut_output, sbcut_output)
}

#[test]
fn test_02_2_characters_start_range(){
    // Arrange
    let cut_arguments = vec!["-c", START_RANGE_CUT, SAMPLE_FILE];
    let sbcut_arguments = vec!["-c", START_RANGE_SBCUT, SAMPLE_FILE];

    // Act
    let cut_output = call_cut(cut_arguments);
    let sbcut_output = call_sbcut(sbcut_arguments);

    // Assert
    assert_eq!(cut_output, sbcut_output)
}

#[test]
fn test_02_3_characters_start_end_range(){
    // Arrange
    let cut_arguments = vec!["-c", START_END_RANGE_CUT, SAMPLE_FILE];
    let sbcut_arguments = vec!["-c", START_END_RANGE_SBCUT, SAMPLE_FILE];

    // Act
    let cut_output = call_cut(cut_arguments);
    let sbcut_output = call_sbcut(sbcut_arguments);

    // Assert
    assert_eq!(cut_output, sbcut_output)
}

#[test]
fn test_02_4_characters_end_range(){
    // Arrange
    let cut_arguments = vec!["-c", END_RANGE_CUT, SAMPLE_FILE];
    let sbcut_arguments = vec!["-c", END_RANGE_SBCUT, SAMPLE_FILE];

    // Act
    let cut_output = call_cut(cut_arguments);
    let sbcut_output = call_sbcut(sbcut_arguments);

    // Assert
    assert_eq!(cut_output, sbcut_output)
}

#[test]
fn test_02_5_characters_single_start_range(){
    // Arrange
    let cut_arguments = vec!["-c", SINGLE_START_RANGE_CUT, SAMPLE_FILE];
    let sbcut_arguments = vec!["-c", SINGLE_START_RANGE_SBCUT, SAMPLE_FILE];

    // Act
    let cut_output = call_cut(cut_arguments);
    let sbcut_output = call_sbcut(sbcut_arguments);

    // Assert
    assert_eq!(cut_output, sbcut_output)
}

#[test]
fn test_02_6_characters_single_single_start_range(){
    // Arrange
    let cut_arguments = vec!["-c", SINGLE_SINGLE_START_RANGE_CUT, SAMPLE_FILE];
    let sbcut_arguments = vec!["-c", SINGLE_SINGLE_START_RANGE_SBCUT, SAMPLE_FILE];

    // Act
    let cut_output = call_cut(cut_arguments);
    let sbcut_output = call_sbcut(sbcut_arguments);

    // Assert
    assert_eq!(cut_output, sbcut_output)
}

#[test]
fn test_03_1_fields_single_range(){
    // Arrange
    let cut_arguments = vec!["-d", ",","-f", SINGLE_RANGE_CUT, SAMPLE_FILE];
    let sbcut_arguments = vec!["-d", ",","-f", SINGLE_RANGE_SBCUT, SAMPLE_FILE];

    // Act
    let cut_output = call_cut(cut_arguments);
    let sbcut_output = call_sbcut(sbcut_arguments);

    // Assert
    assert_eq!(cut_output, sbcut_output)
}

#[test]
fn test_03_2_fields_start_range(){
    // Arrange
    let cut_arguments = vec!["-d", ",","-f", START_RANGE_CUT, SAMPLE_FILE];
    let sbcut_arguments = vec!["-d", ",","-f", START_RANGE_SBCUT, SAMPLE_FILE];

    // Act
    let cut_output = call_cut(cut_arguments);
    let sbcut_output = call_sbcut(sbcut_arguments);

    // Assert
    assert_eq!(cut_output, sbcut_output)
}

#[test]
fn test_03_3_fields_start_end_range(){
    // Arrange
    let cut_arguments = vec!["-d", ",","-f", START_END_RANGE_CUT, SAMPLE_FILE];
    let sbcut_arguments = vec!["-d", ",","-f", START_END_RANGE_SBCUT, SAMPLE_FILE];

    // Act
    let cut_output = call_cut(cut_arguments);
    let sbcut_output = call_sbcut(sbcut_arguments);

    // Assert
    assert_eq!(cut_output, sbcut_output)
}

#[test]
fn test_03_4_fields_end_range(){
    // Arrange
    let cut_arguments = vec!["-d", ",","-f", END_RANGE_CUT, SAMPLE_FILE];
    let sbcut_arguments = vec!["-d", ",","-f", END_RANGE_SBCUT, SAMPLE_FILE];

    // Act
    let cut_output = call_cut(cut_arguments);
    let sbcut_output = call_sbcut(sbcut_arguments);

    // Assert
    assert_eq!(cut_output, sbcut_output)
}

#[test]
fn test_03_5_fields_single_start_range(){
    // Arrange
    let cut_arguments = vec!["-d", ",","-f", SINGLE_START_RANGE_CUT, SAMPLE_FILE];
    let sbcut_arguments = vec!["-d", ",","-f", SINGLE_START_RANGE_SBCUT, SAMPLE_FILE];

    // Act
    let cut_output = call_cut(cut_arguments);
    let sbcut_output = call_sbcut(sbcut_arguments);

    // Assert
    assert_eq!(cut_output, sbcut_output)
}

#[test]
fn test_03_6_fields_single_single_start_range(){
    // Arrange
    let cut_arguments = vec!["-d", ",", "-f", SINGLE_SINGLE_START_RANGE_CUT, SAMPLE_FILE];
    let sbcut_arguments = vec!["-d", ",","-f", SINGLE_SINGLE_START_RANGE_SBCUT, SAMPLE_FILE];

    // Act
    let cut_output = call_cut(cut_arguments);
    let sbcut_output = call_sbcut(sbcut_arguments);

    // Assert
    assert_eq!(cut_output, sbcut_output)
}


#[test]
fn test_04_1_complement_single_range() {
    // Arrange
    let cut_arguments = vec!["-d", ",", "-f", SINGLE_RANGE_CUT, SAMPLE_FILE, "--complement"];
    let sbcut_arguments = vec!["-d", ",","-f", SINGLE_RANGE_SBCUT, SAMPLE_FILE, "--complement"];

    // Act
    let cut_output = call_cut(cut_arguments);
    let sbcut_output = call_sbcut(sbcut_arguments);

    // Assert
    assert_eq!(cut_output, sbcut_output)
}

#[test]
fn test_04_2_complement_start_range() {
    // Arrange
    let cut_arguments = vec!["-d", ",", "-f", START_RANGE_CUT, SAMPLE_FILE, "--complement"];
    let sbcut_arguments = vec!["-d", ",","-f", START_RANGE_SBCUT, SAMPLE_FILE, "--complement"];

    // Act
    let cut_output = call_cut(cut_arguments);
    let sbcut_output = call_sbcut(sbcut_arguments);

    // Assert
    assert_eq!(cut_output, sbcut_output)
}

#[test]
fn test_04_3_complement_start_end_range() {
    // Arrange
    let cut_arguments = vec!["-d", ",", "-f", START_END_RANGE_CUT, SAMPLE_FILE, "--complement"];
    let sbcut_arguments = vec!["-d", ",","-f", START_END_RANGE_SBCUT, SAMPLE_FILE, "--complement"];

    // Act
    let cut_output = call_cut(cut_arguments);
    let sbcut_output = call_sbcut(sbcut_arguments);

    // Assert
    assert_eq!(cut_output, sbcut_output)
}

#[test]
fn test_04_4_complement_end_range() {
    // Arrange
    let cut_arguments = vec!["-d", ",", "-f", END_RANGE_CUT, SAMPLE_FILE, "--complement"];
    let sbcut_arguments = vec!["-d", ",","-f", END_RANGE_SBCUT, SAMPLE_FILE, "--complement"];

    // Act
    let cut_output = call_cut(cut_arguments);
    let sbcut_output = call_sbcut(sbcut_arguments);

    // Assert
    assert_eq!(cut_output, sbcut_output)
}

#[test]
fn test_04_5_complement_singe_start_range() {
    // Arrange
    let cut_arguments = vec!["-d", ",", "-f", SINGLE_START_RANGE_CUT, SAMPLE_FILE, "--complement"];
    let sbcut_arguments = vec!["-d", ",","-f", SINGLE_START_RANGE_SBCUT, SAMPLE_FILE, "--complement"];

    // Act
    let cut_output = call_cut(cut_arguments);
    let sbcut_output = call_sbcut(sbcut_arguments);

    // Assert
    assert_eq!(cut_output, sbcut_output)
}

#[test]
fn test_04_6_complement_single_single_start_range() {
    // Arrange
    let cut_arguments = vec!["-d", ",", "-f", SINGLE_SINGLE_START_RANGE_CUT, SAMPLE_FILE, "--complement"];
    let sbcut_arguments = vec!["-d", ",","-f", SINGLE_SINGLE_START_RANGE_SBCUT, SAMPLE_FILE, "--complement"];

    // Act
    let cut_output = call_cut(cut_arguments);
    let sbcut_output = call_sbcut(sbcut_arguments);

    // Assert
    assert_eq!(cut_output, sbcut_output)
}

fn call_cut(arguments: Vec<&str>) -> Result<(String, String, i32), String> {
    call_command("cut", arguments)
}

fn call_sbcut(arguments: Vec<&str>) -> Result<(String, String, i32), String> {
    call_command("./target/debug/slightly-better-cut", arguments)
}

fn call_command(command: &str, arguments: Vec<&str>) -> Result<(String, String, i32), String> {
    let command_call = Command::new(command).args(arguments).output();

    match command_call {
        Ok(output) =>Ok((String::from_utf8(output.stdout).unwrap(), String::from_utf8(output.stderr).unwrap(), output.status.code().unwrap())),
        Err(_) => Err(String::from("Can't execute command"))
    }
}