use crate::rdoc::TestStatus;

#[derive(Debug)]
pub enum Line {
    Suite {
        location: String,
    },
    Test {
        name: String,
        modules: Vec<String>,
        status: TestStatus,
    },
}

pub(crate) fn parse(lines: Vec<String>) -> Vec<Line> {
    let mut result: Vec<Line> = vec![];

    for line in lines {
        let line = line.trim_start();

        if let Some(line) = try_unit_suite(line) {
            result.push(line);
            continue;
        }

        if let Some(line) = try_integration_suite(line) {
            result.push(line);
            continue;
        }

        if let Some(line) = try_test(line) {
            result.push(line);
            continue;
        }
    }

    result
}

fn try_unit_suite(line: &str) -> Option<Line> {
    if !line.starts_with("Running unittests") {
        return None;
    }

    let ar: Vec<&str> = line.split(" ").collect();
    let location = ar.iter().nth(2).unwrap().to_string();
    let line = Line::Suite { location };

    Some(line)
}

fn try_integration_suite(line: &str) -> Option<Line> {
    if !line.starts_with("Running") {
        return None;
    }

    let ar: Vec<&str> = line.split(" ").collect();
    let location = ar.iter().nth(1).unwrap().to_string();
    let line = Line::Suite { location };

    Some(line)
}

fn try_test(line: &str) -> Option<Line> {
    let Some(test) = line.strip_prefix("test ") else {
        return None;
    };

    if test.starts_with("result") || test.contains("line") {
        return None;
    }

    let ar: Vec<&str> = test.split(" ... ").collect();
    let test = *ar.first().unwrap();
    let status = *ar.last().unwrap();

    let ar: Vec<&str> = test.split("::").collect();
    let (test_name, modules) = ar.split_last().unwrap();

    let modules: Vec<String> = modules.iter().map(|s| s.to_string()).collect();
    let line = Line::Test {
        name: test_name.to_string(),
        modules,
        status: status.into(),
    };

    Some(line)
}
