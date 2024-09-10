pub mod cut {
    pub fn cut_line_with_delimiter(line: &str, range: (i32, i32, i32), delimiter: char) -> Vec<&str> {
        let split: Vec<&str> = line.split(delimiter).collect();
        
        let (start, end, step) = range;

        if step >= 0 {
            let range = (start..end).step_by(step as usize);
        }
        
        for i in (start..end).step_by(step) {

        }
        split
    }
}