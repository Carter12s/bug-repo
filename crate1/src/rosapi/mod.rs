//! Provided blah blah by [rosapi](http://wiki.ros.org/rosapi) node.

const WORKING_DIR: &str = macro_crate::what_is_working_dir!();

///
/// ```
/// let wd = macro_crate::what_is_working_dir!();
/// assert_eq!(wd, "/home/carter/open_source/bug-repo");
/// ```
fn bar() {}

#[cfg(test)]
mod test {
    use super::WORKING_DIR;

    #[test]
    fn print_dir() {
        println!("{WORKING_DIR}");
        assert_eq!(WORKING_DIR, "/home/carter/open_source/bug-repo");
    }
}
