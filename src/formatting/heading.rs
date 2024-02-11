pub trait Heading {
    fn header(self) -> String;
    fn footer(self) -> String;
}

impl Heading for &str {
    fn header(self) -> String {
        format!("== {} =======", self)
    }

    fn footer(self) -> String {
        let len = self.len() + 11;
        "=".repeat(len)
    }
}
