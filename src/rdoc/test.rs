use colored::Colorize;

use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Test {
    pub module: Vec<String>,
    pub name: String,
    pub status: TestStatus,
}

impl Display for Test {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.module.is_empty() {
            write!(
                f,
                "  {} {} - {}",
                self.status,
                self.module.join("::").blue(),
                self.name
            )
        } else {
            write!(f, "  {} {}", self.status, self.name)
        }
    }
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum TestStatus {
    Ok,
    Failed,
    Ignored(Option<String>),
    Unknown,
}

impl Display for TestStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TestStatus::Ok => write!(f, "{}", "✔".green()),
            TestStatus::Failed => write!(f, "{}", "x".red()),
            TestStatus::Ignored(None) => write!(f, "{}", "?".yellow()),
            TestStatus::Ignored(Some(reason)) => {
                write!(f, "{} [{}]", "?".yellow(), reason.yellow())
            }
            TestStatus::Unknown => write!(f, "{}", "?".red()),
        }
    }
}

impl From<&str> for TestStatus {
    fn from(value: &str) -> Self {
        match value {
            "ok" => Self::Ok,
            "ignored" => Self::Ignored(None),
            "FAILED" => Self::Failed,
            _ => {
                if let Some(reason) = value.strip_prefix("ignored, ") {
                    Self::Ignored(Some(reason.to_string()))
                } else {
                    Self::Unknown
                }
            }
        }
    }
}
