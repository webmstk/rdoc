mod cmd;
mod parse;
mod rdoc;

use parse::parse;
use rdoc::Suite;
use rdoc::build_suites;

pub fn rdoc(args: Vec<String>) {
    let output = cmd::exec(args).unwrap();
    let lines = parse(output);
    let suites = build_suites(lines);

    print(suites);
}

fn print(suites: Vec<Suite>) {
    for suite in suites {
        let tests = suite.tests();

        if !tests.is_empty() {
            println!("{suite}");
        }
        for test in tests {
            println!("{test}");
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn this_is_unit_lib_test() {}
}
