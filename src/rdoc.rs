mod suite;
mod test;

pub use suite::Suite;
pub use test::{Test, TestStatus};

use crate::parse::Line;

use colored::Colorize;

pub(crate) fn build_suites(lines: Vec<Line>) -> Vec<Suite> {
    let mut suites = vec![];
    let mut index = 0;

    for line in lines {
        match line {
            Line::Suite { location } => {
                let suite = Suite::new(location);
                suites.push(suite);
                index = suites.len() - 1;
            }
            Line::Test {
                name,
                modules: module_path,
                status,
            } => {
                let test = Test {
                    module: module_path,
                    name: prettify(name),
                    status,
                };
                suites[index].push(test);
            }
        }
    }

    for suite in &mut suites {
        suite.sort();
    }

    suites
}

pub fn prettify(input: impl AsRef<str>) -> String {
    if let Some((fn_name, sentence)) = input.as_ref().split_once("_fn") {
        format!("{} {}", fn_name.cyan(), humanize(sentence))
    } else {
        humanize(input)
    }
}

fn humanize(input: impl AsRef<str>) -> String {
    input
        .as_ref()
        .replace('_', " ")
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ")
}
