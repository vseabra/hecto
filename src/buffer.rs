pub struct Buffer {
    pub lines: Vec<String>,
}

impl Default for Buffer {
    fn default() -> Self {
        Buffer {
            lines: vec![String::from("Hello world")],
        }
    }
}

impl Buffer {
    pub fn from_string(content: String) -> Self {
        Buffer {
            lines: content.lines().map(|line| line.to_string()).collect(),
        }
    }
}
