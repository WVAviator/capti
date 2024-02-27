pub trait Indent {
    fn indent(self) -> String;
}

impl Indent for String {
    fn indent(self) -> Self {
        let mut result = self
            .lines()
            .map(|line| format!("  {}\n", line))
            .collect::<String>();
        result.pop();

        result
    }
}

impl Indent for &str {
    fn indent(self) -> String {
        self.to_string().indent()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn indents_every_line() {
        let test_str = String::from("A\nB\nC");

        assert_eq!(test_str.indent(), "  A\n  B\n  C");
    }
}
