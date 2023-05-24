use std::fmt::Display;

use itertools::Itertools;

use crate::more_itertools::MoreItertools;

pub fn generate_fancy_vertical_list<T, U, V>(title: V, items: T) -> String
where
    T: IntoIterator<Item = U>,
    U: Display,
    V: AsRef<str>,
{
    format!(
        "┌ {}\n│\n{}",
        title.as_ref(),
        items
            .into_iter()
            .sandwich_map(
                |item| format!("├─ {}", item),
                |item| format!("├─ {}", item),
                |item| format!("└─ {}", item),
            )
            .join("\n")
    )
}
