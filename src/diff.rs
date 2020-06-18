use diff_utils::{Comparison, DisplayOptions};

pub fn compare(expected: &str, actual: &str) -> String {
    let expected: Vec<&str> = expected.lines().collect();
    let actual: Vec<&str> = actual.lines().collect();

    let result =
        Comparison::new(&expected, &actual)
            .compare()
            .expect("Difference");

    if result.is_empty() {
        Default::default()
    } else {
        result.display(DisplayOptions {
            offset: 0,
            msg_fmt: ""
        }).to_string()
    }
}
