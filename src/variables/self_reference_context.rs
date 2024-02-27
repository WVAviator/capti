use std::fmt;

use colored::Colorize;

pub struct SelfReferenceContext {
    context: Vec<String>,
    is_self_reference: bool,
}

impl SelfReferenceContext {
    pub fn new() -> Self {
        SelfReferenceContext {
            context: Vec::new(),
            is_self_reference: false,
        }
    }

    pub fn from_path(path: Vec<&str>, marker: &str) -> Self {
        let context = path
            .into_iter()
            .skip_while(|item| *item != marker)
            .map(|item| item.to_string())
            .collect::<Vec<String>>();

        SelfReferenceContext {
            context,
            is_self_reference: false,
        }
    }

    pub fn flag(&mut self) {
        self.is_self_reference = true;
    }

    pub fn is_flagged(&self) -> bool {
        self.is_self_reference
    }
}

impl fmt::Display for SelfReferenceContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for item in &self.context {
            write!(f, "${{{}}} {} ", item, "->".yellow())?;
        }
        writeln!(
            f,
            "{}",
            format!("${{{}}}", self.context.get(0).unwrap_or(&String::new())).red()
        )
    }
}
