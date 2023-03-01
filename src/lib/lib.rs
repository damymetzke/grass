/// List categories
///
/// # Example:
///
/// ```
/// # use grass::list_categories;
///
/// assert_eq!(vec!{"general", "work"}, list_categories());
/// ```
pub fn list_categories() -> Vec<String> {
    vec!{String::from("general"), String::from("work")}
}
