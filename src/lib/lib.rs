/// Trivial function to test lib
///
/// # Example:
///
/// ```
/// # use grass::get_hello;
///
/// assert_eq!("Hello Damy!", get_hello("Damy"));
/// ```
pub fn get_hello(name: &str) -> String {
    format!("Hello {}!", name)
}
