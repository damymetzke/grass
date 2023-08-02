use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use grass::dev::{
    types::{SimpleCategoryDescription, SimpleRepositoryDescription},
    RepositoryLocation,
};
use thiserror::Error;

use crate::error::CliError;

pub trait Selectable {
    fn get_select_name(&self) -> &str;
}

#[derive(Error, Debug)]
pub enum DialoguerError {
    #[error("Dialoguer error")]
    Dialoguer(dialoguer::Error),
    #[error("User did not select an item")]
    NothingSelected,
}

impl From<dialoguer::Error> for DialoguerError {
    fn from(value: dialoguer::Error) -> Self {
        DialoguerError::Dialoguer(value)
    }
}

pub fn select_selectable<T>(selectables: &[T]) -> Result<&T>
where
    T: Selectable,
{
    let options: Vec<_> = selectables
        .iter()
        .map(|selectable| selectable.get_select_name())
        .collect();

    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .items(&options)
        .default(0)
        .vim_mode(true)
        .interact_opt()?;

    Ok(selectables
        .get(selection.ok_or(DialoguerError::NothingSelected)?)
        .ok_or(DialoguerError::NothingSelected)?)
}

pub fn select_category_and_repository(
    categories: &[RepositoryLocation],
) -> Result<&RepositoryLocation> {
    let options: Vec<_> = categories
        .iter()
        .map(
            |RepositoryLocation {
                 category,
                 repository,
             }| format!("{}/{}", category, repository),
        )
        .collect();

    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .items(&options)
        .default(0)
        .vim_mode(true)
        .interact_opt()?
        .ok_or(DialoguerError::NothingSelected)?;

    Ok(categories
        .get(selection)
        .ok_or(CliError::new("No option selected"))?)
}

impl Selectable for RepositoryLocation {
    fn get_select_name(&self) -> &str {
        self.repository.as_str()
    }
}

impl Selectable for SimpleRepositoryDescription {
    fn get_select_name(&self) -> &str {
        self.repository.as_str()
    }
}

impl Selectable for SimpleCategoryDescription {
    fn get_select_name(&self) -> &str {
        self.category.as_str()
    }
}

impl Selectable for String {
    fn get_select_name(&self) -> &str {
        self.as_str()
    }
}
