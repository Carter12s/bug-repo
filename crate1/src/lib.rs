
#[cfg(feature = "rosapi")]
pub mod rosapi;

/// ```
/// let wd = macro_crate::what_is_working_dir!();
/// // Should not be equal, just getting output from macro this way
/// assert_eq!(wd, "/home/carter/open_source/bug-repo");
/// // With this below line present the assert above fails
/// // If you remove these two line it runs correctly
/// let wd2 = crate2::get_working_dir();
/// assert_eq!(wd2, "/home/carter/open_source/bug-repo")
/// ```
pub fn foo() {

}