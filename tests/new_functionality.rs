#[cfg(test)]
mod functional_tests_indexing {
    use std::process::Command;
    // Indexing non-delimited lines
    // Indexing in general
    /*
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
        positive::positive
    */
    static SAMPLE_FILE: &str = "sample_bigger.csv";
    
    static POSITIVE_N: &str = "1";
    static NEGATIVE_N: &str = "-5";
    
    static POSITIVE_M: &str = "6";
    static NEGATIVE_M: &str = "-3";
    
    static POSITIVE_S: &str = "2";
    static NEGATIVE_S: &str = "-2";
    
    static SEPARATOR: &str = ":";
    
    #[test]
    fn test_01_1_characters_positive_positive_positive() {
        // Arrange
        let fields = String::from("") + POSITIVE_N + SEPARATOR + POSITIVE_M + SEPARATOR + POSITIVE_S;
        let sbcut_arguments = vec!["-c", &fields, SAMPLE_FILE];
        let expected_output = (String::from("DNm\n,on\n,ae\n,o \n"), String::from(""), 0);
    
        // Act
        let actual_output = call_sbcut(sbcut_arguments);
    
        // Assert
        assert_eq!(expected_output, actual_output.unwrap())
    }
    
    #[test]
    fn test_01_2_characters_positive_positive_negative() {
        // Arrange
        let fields = String::from("") + POSITIVE_N + SEPARATOR + POSITIVE_M + SEPARATOR + NEGATIVE_S;
        let sbcut_arguments = vec!["-c", &fields, SAMPLE_FILE];
        let expected_output = (String::from("mND\nno,\nea,\n o,\n"), String::from(""), 0);
    
        // Act
        let actual_output = call_sbcut(sbcut_arguments);
    
        // Assert
        assert_eq!(expected_output, actual_output.unwrap())
    }
    
    #[test]
    fn test_01_3_characters_positive_positive_separator() {
        // Arrange
        let fields = String::from("") + POSITIVE_N + SEPARATOR + POSITIVE_M + SEPARATOR;
        let sbcut_arguments = vec!["-c", &fields, SAMPLE_FILE];
        let expected_output = (
            String::from("D,Nam\n,John\n,Jane\n,Bob \n"),
            String::from(""),
            0,
        );
    
        // Act
        let actual_output = call_sbcut(sbcut_arguments);
    
        // Assert
        assert_eq!(expected_output, actual_output.unwrap())
    }
    
    #[test]
    fn test_01_4_characters_positive_positive() {
        // Arrange
        let fields = String::from("") + POSITIVE_N + SEPARATOR + POSITIVE_M;
        let sbcut_arguments = vec!["-c", &fields, SAMPLE_FILE];
        let expected_output = (
            String::from("D,Nam\n,John\n,Jane\n,Bob \n"),
            String::from(""),
            0,
        );
    
        // Act
        let actual_output = call_sbcut(sbcut_arguments);
    
        // Assert
        assert_eq!(expected_output, actual_output.unwrap())
    }
    
    #[test]
    fn test_01_5_characters_positive_negative() {
        // Arrange
        let fields = String::from("") + POSITIVE_N + SEPARATOR + NEGATIVE_M;
        let sbcut_arguments = vec!["-c", &fields, SAMPLE_FILE];
        let expected_output = (
            String::from("D,Name,Age,Email,City,Country,Occupation,Sala\n,John Doe,28,john.doe@example.com,New York,USA,Software Engineer,800\n,Jane Smith,34,jane.smith@example.com,Los Angeles,USA,Data Analyst,750\n,Bob Johnson,45,bob.johnson@example.com,Chicago,USA,Project Manager,90\n"),
            String::from(""),
            0,
        );
    
        // Act
        let actual_output = call_sbcut(sbcut_arguments);
    
        // Assert
        assert_eq!(expected_output, actual_output.unwrap())
    }
    
    #[test]
    fn test_01_6_characters_positive_separator() {
        // Arrange
        let fields = String::from("") + POSITIVE_N + SEPARATOR;
        let sbcut_arguments = vec!["-c", &fields, SAMPLE_FILE];
        let expected_output = (
            String::from("D,Name,Age,Email,City,Country,Occupation,Salary\r\n,John Doe,28,john.doe@example.com,New York,USA,Software Engineer,80000\r\n,Jane Smith,34,jane.smith@example.com,Los Angeles,USA,Data Analyst,75000\r\n,Bob Johnson,45,bob.johnson@example.com,Chicago,USA,Project Manager,90000\n"),
            String::from(""),
            0,
        );
    
        // Act
        let actual_output = call_sbcut(sbcut_arguments);
    
        // Assert
        assert_eq!(expected_output, actual_output.unwrap())
    }
    
    #[test]
    fn test_01_7_characters_positive() {
        // Arrange
        let fields = String::from("") + POSITIVE_N;
        let sbcut_arguments = vec!["-c", &fields, SAMPLE_FILE];
        let expected_output = (String::from("D\n,\n,\n,\n"), String::from(""), 0);
    
        // Act
        let actual_output = call_sbcut(sbcut_arguments);
    
        // Assert
        assert_eq!(expected_output, actual_output.unwrap())
    }
    
    #[test]
    fn test_01_8_characters_negative() {
        // Arrange
        let fields = String::from("") + NEGATIVE_N;
        let sbcut_arguments = vec!["-c", &fields, SAMPLE_FILE];
        let expected_output = (String::from("a\n0\n0\n0\n"), String::from(""), 0);
    
        // Act
        let actual_output = call_sbcut(sbcut_arguments);
    
        // Assert
        assert_eq!(expected_output, actual_output.unwrap())
    }
    
    #[test]
    fn test_01_9_characters_separator_positive() {
        // Arrange
        let fields = String::from("") + SEPARATOR + POSITIVE_M;
        let sbcut_arguments = vec!["-c", &fields, SAMPLE_FILE];
        let expected_output = (
            String::from("ID,Nam\n1,John\n2,Jane\n3,Bob \n"),
            String::from(""),
            0,
        );
    
        // Act
        let actual_output = call_sbcut(sbcut_arguments);
    
        // Assert
        assert_eq!(expected_output, actual_output.unwrap())
    }
    
    #[test]
    fn test_01_10_characters_separator_separator_positive() {
        // Arrange
        let fields = String::from("") + SEPARATOR + SEPARATOR + POSITIVE_S;
        let sbcut_arguments = vec!["-c", &fields, SAMPLE_FILE];
        let expected_output = (
            String::from("I,aeAeEalCt,onr,cuainSlr\r\n1Jh o,8jh.o@xml.o,e okUASfwr nier800\n2Jn mt,4jn.mt@xml.o,o nee,S,aaAayt700\n3BbJhsn4,o.ono@xml.o,hcg,S,rjc aae,00\n"),
            String::from(""),
            0,
        );
    
        // Act
        let actual_output = call_sbcut(sbcut_arguments);
    
        // Assert
        assert_eq!(expected_output, actual_output.unwrap())
    }
    
    #[test]
    fn test_01_11_characters_positive_separator_positive() {
        // Arrange
        let fields = String::from("") + POSITIVE_N + SEPARATOR + SEPARATOR + POSITIVE_S;
        let sbcut_arguments = vec!["-c", &fields, SAMPLE_FILE];
        let expected_output = (
            String::from("DNm,g,mi,iyCutyOcpto,aay\n,onDe2,ondeeapecmNwYr,S,otaeEgne,00\r\n,aeSih3,aesiheapecmLsAglsUADt nls,50\r\n,o ono,5bbjhsneapecmCiaoUAPoetMngr900\n"),
            String::from(""),
            0,
        );
    
        // Act
        let actual_output = call_sbcut(sbcut_arguments);
    
        // Assert
        assert_eq!(expected_output, actual_output.unwrap())
    }
    
    #[test]
    fn test_02_1_characters_positive_positive_positive() {
        // Arrange
        let fields = String::from("") + POSITIVE_N + SEPARATOR + POSITIVE_M + SEPARATOR + POSITIVE_S;
        let sbcut_arguments = vec!["-b", &fields, SAMPLE_FILE];
        let expected_output = (String::from("DNm\n,on\n,ae\n,o \n"), String::from(""), 0);
    
        // Act
        let actual_output = call_sbcut(sbcut_arguments);
    
        // Assert
        assert_eq!(expected_output, actual_output.unwrap())
    }
    
    #[test]
    fn test_02_2_characters_positive_positive_negative() {
        // Arrange
        let fields = String::from("") + POSITIVE_N + SEPARATOR + POSITIVE_M + SEPARATOR + NEGATIVE_S;
        let sbcut_arguments = vec!["-c", &fields, SAMPLE_FILE];
        let expected_output = (String::from("mND\nno,\nea,\n o,\n"), String::from(""), 0);
    
        // Act
        let actual_output = call_sbcut(sbcut_arguments);
    
        // Assert
        assert_eq!(expected_output, actual_output.unwrap())
    }
    
    #[test]
    fn test_02_3_characters_positive_positive_separator() {
        // Arrange
        let fields = String::from("") + POSITIVE_N + SEPARATOR + POSITIVE_M + SEPARATOR;
        let sbcut_arguments = vec!["-b", &fields, SAMPLE_FILE];
        let expected_output = (
            String::from("D,Nam\n,John\n,Jane\n,Bob \n"),
            String::from(""),
            0,
        );
    
        // Act
        let actual_output = call_sbcut(sbcut_arguments);
    
        // Assert
        assert_eq!(expected_output, actual_output.unwrap())
    }
    
    #[test]
    fn test_02_4_characters_positive_positive() {
        // Arrange
        let fields = String::from("") + POSITIVE_N + SEPARATOR + POSITIVE_M;
        let sbcut_arguments = vec!["-b", &fields, SAMPLE_FILE];
        let expected_output = (
            String::from("D,Nam\n,John\n,Jane\n,Bob \n"),
            String::from(""),
            0,
        );
    
        // Act
        let actual_output = call_sbcut(sbcut_arguments);
    
        // Assert
        assert_eq!(expected_output, actual_output.unwrap())
    }
    
    #[test]
    fn test_02_5_characters_positive_negative() {
        // Arrange
        let fields = String::from("") + POSITIVE_N + SEPARATOR + NEGATIVE_M;
        let sbcut_arguments = vec!["-b", &fields, SAMPLE_FILE];
        let expected_output = (
            String::from("D,Name,Age,Email,City,Country,Occupation,Sala\n,John Doe,28,john.doe@example.com,New York,USA,Software Engineer,800\n,Jane Smith,34,jane.smith@example.com,Los Angeles,USA,Data Analyst,750\n,Bob Johnson,45,bob.johnson@example.com,Chicago,USA,Project Manager,90\n"),
            String::from(""),
            0,
        );
    
        // Act
        let actual_output = call_sbcut(sbcut_arguments);
    
        // Assert
        assert_eq!(expected_output, actual_output.unwrap())
    }
    
    #[test]
    fn test_02_6_characters_positive_separator() {
        // Arrange
        let fields = String::from("") + POSITIVE_N + SEPARATOR;
        let sbcut_arguments = vec!["-b", &fields, SAMPLE_FILE];
        let expected_output = (
            String::from("D,Name,Age,Email,City,Country,Occupation,Salary\r\n,John Doe,28,john.doe@example.com,New York,USA,Software Engineer,80000\r\n,Jane Smith,34,jane.smith@example.com,Los Angeles,USA,Data Analyst,75000\r\n,Bob Johnson,45,bob.johnson@example.com,Chicago,USA,Project Manager,90000\n"),
            String::from(""),
            0,
        );
    
        // Act
        let actual_output = call_sbcut(sbcut_arguments);
    
        // Assert
        assert_eq!(expected_output, actual_output.unwrap())
    }
    
    #[test]
    fn test_02_7_characters_positive() {
        // Arrange
        let fields = String::from("") + POSITIVE_N;
        let sbcut_arguments = vec!["-b", &fields, SAMPLE_FILE];
        let expected_output = (String::from("D\n,\n,\n,\n"), String::from(""), 0);
    
        // Act
        let actual_output = call_sbcut(sbcut_arguments);
    
        // Assert
        assert_eq!(expected_output, actual_output.unwrap())
    }
    
    #[test]
    fn test_02_8_characters_negative() {
        // Arrange
        let fields = String::from("") + NEGATIVE_N;
        let sbcut_arguments = vec!["-b", &fields, SAMPLE_FILE];
        let expected_output = (String::from("a\n0\n0\n0\n"), String::from(""), 0);
    
        // Act
        let actual_output = call_sbcut(sbcut_arguments);
    
        // Assert
        assert_eq!(expected_output, actual_output.unwrap())
    }
    
    #[test]
    fn test_02_9_characters_separator_positive() {
        // Arrange
        let fields = String::from("") + SEPARATOR + POSITIVE_M;
        let sbcut_arguments = vec!["-b", &fields, SAMPLE_FILE];
        let expected_output = (
            String::from("ID,Nam\n1,John\n2,Jane\n3,Bob \n"),
            String::from(""),
            0,
        );
    
        // Act
        let actual_output = call_sbcut(sbcut_arguments);
    
        // Assert
        assert_eq!(expected_output, actual_output.unwrap())
    }
    
    #[test]
    fn test_02_10_characters_separator_separator_positive() {
        // Arrange
        let fields = String::from("") + SEPARATOR + SEPARATOR + POSITIVE_S;
        let sbcut_arguments = vec!["-b", &fields, SAMPLE_FILE];
        let expected_output = (
            String::from("I,aeAeEalCt,onr,cuainSlr\r\n1Jh o,8jh.o@xml.o,e okUASfwr nier800\n2Jn mt,4jn.mt@xml.o,o nee,S,aaAayt700\n3BbJhsn4,o.ono@xml.o,hcg,S,rjc aae,00\n"),
            String::from(""),
            0,
        );
    
        // Act
        let actual_output = call_sbcut(sbcut_arguments);
    
        // Assert
        assert_eq!(expected_output, actual_output.unwrap())
    }
    
    #[test]
    fn test_02_11_characters_positive_separator_positive() {
        // Arrange
        let fields = String::from("") + POSITIVE_N + SEPARATOR + SEPARATOR + POSITIVE_S;
        let sbcut_arguments = vec!["-b", &fields, SAMPLE_FILE];
        let expected_output = (
            String::from("DNm,g,mi,iyCutyOcpto,aay\n,onDe2,ondeeapecmNwYr,S,otaeEgne,00\r\n,aeSih3,aesiheapecmLsAglsUADt nls,50\r\n,o ono,5bbjhsneapecmCiaoUAPoetMngr900\n"),
            String::from(""),
            0,
        );
    
        // Act
        let actual_output = call_sbcut(sbcut_arguments);
    
        // Assert
        assert_eq!(expected_output, actual_output.unwrap())
    }
    
    #[test]
    fn test_03_1_characters_positive_positive_positive() {
        // Arrange
        let fields = String::from("") + POSITIVE_N + SEPARATOR + POSITIVE_M + SEPARATOR + POSITIVE_S;
        let sbcut_arguments = vec!["-d", ",", "-f", &fields, SAMPLE_FILE];
        let expected_output = (String::from("Name,Email,Country\nJohn Doe,john.doe@example.com,USA\nJane Smith,jane.smith@example.com,USA\nBob Johnson,bob.johnson@example.com,USA\n"), String::from(""), 0);
    
        // Act
        let actual_output = call_sbcut(sbcut_arguments);
    
        // Assert
        assert_eq!(expected_output, actual_output.unwrap())
    }
    
    #[test]
    fn test_03_2_characters_positive_positive_negative() {
        // Arrange
        let fields = String::from("") + POSITIVE_N + SEPARATOR + POSITIVE_M + SEPARATOR + NEGATIVE_S;
        let sbcut_arguments = vec!["-d", ",", "-f", &fields, SAMPLE_FILE];
        let expected_output = (String::from("Country,Email,Name\nUSA,john.doe@example.com,John Doe\nUSA,jane.smith@example.com,Jane Smith\nUSA,bob.johnson@example.com,Bob Johnson\n"), String::from(""), 0);
    
        // Act
        let actual_output = call_sbcut(sbcut_arguments);
    
        // Assert
        assert_eq!(expected_output, actual_output.unwrap())
    }
    
    #[test]
    fn test_03_3_characters_positive_positive_separator() {
        // Arrange
        let fields = String::from("") + POSITIVE_N + SEPARATOR + POSITIVE_M + SEPARATOR;
        let sbcut_arguments = vec!["-d", ",", "-f", &fields, SAMPLE_FILE];
        let expected_output = (
            String::from("Name,Age,Email,City,Country\nJohn Doe,28,john.doe@example.com,New York,USA\nJane Smith,34,jane.smith@example.com,Los Angeles,USA\nBob Johnson,45,bob.johnson@example.com,Chicago,USA\n"),
            String::from(""),
            0,
        );
    
        // Act
        let actual_output = call_sbcut(sbcut_arguments);
    
        // Assert
        assert_eq!(expected_output, actual_output.unwrap())
    }
    
    #[test]
    fn test_03_4_characters_positive_positive() {
        // Arrange
        let fields = String::from("") + POSITIVE_N + SEPARATOR + POSITIVE_M;
        let sbcut_arguments = vec!["-d", ",", "-f", &fields, SAMPLE_FILE];
        let expected_output = (
            String::from("Name,Age,Email,City,Country\nJohn Doe,28,john.doe@example.com,New York,USA\nJane Smith,34,jane.smith@example.com,Los Angeles,USA\nBob Johnson,45,bob.johnson@example.com,Chicago,USA\n"),
            String::from(""),
            0,
        );
    
        // Act
        let actual_output = call_sbcut(sbcut_arguments);
    
        // Assert
        assert_eq!(expected_output, actual_output.unwrap())
    }
    
    #[test]
    fn test_03_5_characters_positive_negative() {
        // Arrange
        let fields = String::from("") + POSITIVE_N + SEPARATOR + NEGATIVE_M;
        let sbcut_arguments = vec!["-d", ",", "-f", &fields, SAMPLE_FILE];
        let expected_output = (
            String::from("Name,Age,Email,City\nJohn Doe,28,john.doe@example.com,New York\nJane Smith,34,jane.smith@example.com,Los Angeles\nBob Johnson,45,bob.johnson@example.com,Chicago\n"),
            String::from(""),
            0,
        );
    
        // Act
        let actual_output = call_sbcut(sbcut_arguments);
    
        // Assert
        assert_eq!(expected_output, actual_output.unwrap())
    }
    
    #[test]
    fn test_03_6_characters_positive_separator() {
        // Arrange
        let fields = String::from("") + POSITIVE_N + SEPARATOR;
        let sbcut_arguments = vec!["-d", ",", "-f", &fields, SAMPLE_FILE];
        let expected_output = (
            String::from("Name,Age,Email,City,Country,Occupation,Salary\r\nJohn Doe,28,john.doe@example.com,New York,USA,Software Engineer,80000\r\nJane Smith,34,jane.smith@example.com,Los Angeles,USA,Data Analyst,75000\r\nBob Johnson,45,bob.johnson@example.com,Chicago,USA,Project Manager,90000\n"),
            String::from(""),
            0,
        );
    
        // Act
        let actual_output = call_sbcut(sbcut_arguments);
    
        // Assert
        assert_eq!(expected_output, actual_output.unwrap())
    }
    
    #[test]
    fn test_03_7_characters_positive() {
        // Arrange
        let fields = String::from("") + POSITIVE_N;
        let sbcut_arguments = vec!["-d", ",", "-f", &fields, SAMPLE_FILE];
        let expected_output = (
            String::from("Name\nJohn Doe\nJane Smith\nBob Johnson\n"),
            String::from(""),
            0,
        );
    
        // Act
        let actual_output = call_sbcut(sbcut_arguments);
    
        // Assert
        assert_eq!(expected_output, actual_output.unwrap())
    }
    
    #[test]
    fn test_03_8_characters_negative() {
        // Arrange
        let fields = String::from("") + NEGATIVE_N;
        let sbcut_arguments = vec!["-d", ",", "-f", &fields, SAMPLE_FILE];
        let expected_output = (
            String::from(
                "Email\njohn.doe@example.com\njane.smith@example.com\nbob.johnson@example.com\n",
            ),
            String::from(""),
            0,
        );
    
        // Act
        let actual_output = call_sbcut(sbcut_arguments);
    
        // Assert
        assert_eq!(expected_output, actual_output.unwrap())
    }
    
    #[test]
    fn test_03_9_characters_separator_positive() {
        // Arrange
        let fields = String::from("") + SEPARATOR + POSITIVE_M;
        let sbcut_arguments = vec!["-d", ",", "-f", &fields, SAMPLE_FILE];
        let expected_output = (
            String::from("ID,Name,Age,Email,City,Country\n1,John Doe,28,john.doe@example.com,New York,USA\n2,Jane Smith,34,jane.smith@example.com,Los Angeles,USA\n3,Bob Johnson,45,bob.johnson@example.com,Chicago,USA\n"),
            String::from(""),
            0,
        );
    
        // Act
        let actual_output = call_sbcut(sbcut_arguments);
    
        // Assert
        assert_eq!(expected_output, actual_output.unwrap())
    }
    
    #[test]
    fn test_03_10_characters_separator_separator_positive() {
        // Arrange
        let fields = String::from("") + SEPARATOR + SEPARATOR + POSITIVE_S;
        let sbcut_arguments = vec!["-d", ",", "-f", &fields, SAMPLE_FILE];
        let expected_output = (
            String::from("ID,Age,City,Occupation\n1,28,New York,Software Engineer\n2,34,Los Angeles,Data Analyst\n3,45,Chicago,Project Manager\n"),
            String::from(""),
            0,
        );
    
        // Act
        let actual_output = call_sbcut(sbcut_arguments);
    
        // Assert
        assert_eq!(expected_output, actual_output.unwrap())
    }
    
    #[test]
    fn test_03_11_characters_positive_separator_positive() {
        // Arrange
        let fields = String::from("") + POSITIVE_N + SEPARATOR + SEPARATOR + POSITIVE_S;
        let sbcut_arguments = vec!["-d", ",", "-f", &fields, SAMPLE_FILE];
        let expected_output = (
            String::from("Name,Email,Country,Salary\r\nJohn Doe,john.doe@example.com,USA,80000\r\nJane Smith,jane.smith@example.com,USA,75000\r\nBob Johnson,bob.johnson@example.com,USA,90000\n"),
            String::from(""),
            0,
        );
    
        // Act
        let actual_output = call_sbcut(sbcut_arguments);
    
        // Assert
        assert_eq!(expected_output, actual_output.unwrap())
    }
    
    fn call_sbcut(arguments: Vec<&str>) -> Result<(String, String, i32), String> {
        println!("sbcut: {}", arguments.join(" "));
    
        call_command("./target/debug/sbcut", arguments)
    }
    
    fn call_command(command: &str, arguments: Vec<&str>) -> Result<(String, String, i32), String> {
        let command_call = Command::new(command).args(arguments).output();
        match command_call {
            Ok(output) => Ok((
                String::from_utf8(output.stdout)
                    .unwrap()
                    .replace(0 as char, "\n"),
                String::from_utf8(output.stderr).unwrap(),
                output.status.code().unwrap(),
            )),
            Err(_) => Err(String::from("Can't execute command")),
        }
    }
    
}
