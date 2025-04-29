use std::collections::HashMap;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone, Default)]
pub struct Column {
    pub id: &'static str,
    pub header: &'static str,
    pub accessor: Callback<()>,
    pub sortable: bool,
    #[prop_or(100)]
    pub min_width: u32,
    #[prop_or_default]
    pub class: Option<&'static str>,
    #[prop_or_default]
    pub style: Option<&'static str>,
}

#[derive(Clone, PartialEq)]
pub enum SortOrder {
    Asc,
    Desc,
}

#[derive(Properties, PartialEq, Clone, Default)]
pub struct TableClasses {
    #[prop_or("table-container")]
    pub container: &'static str,
    #[prop_or("table")]
    pub table: &'static str,
    #[prop_or("thead")]
    pub thead: &'static str,
    #[prop_or("tbody")]
    pub tbody: &'static str,
    #[prop_or("pagination-controls")]
    pub pagination: &'static str,
    #[prop_or("search-input")]
    pub search_input: &'static str,
}

#[derive(Properties, PartialEq)]
pub struct TableProps {
    pub data: Vec<HashMap<&'static str, String>>,
    pub columns: Vec<Column>,
    #[prop_or(10)]
    pub page_size: usize,
    #[prop_or(false)]
    pub loading: bool,
    #[prop_or_default]
    pub classes: TableClasses,
    #[prop_or_default]
    pub styles: HashMap<&'static str, &'static str>,
    #[prop_or(false)]
    pub paginate: bool,
    #[prop_or(false)]
    pub search: bool,
}

#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    pub columns: Vec<Column>,
    pub sort_column: UseStateHandle<Option<&'static str>>,
    pub sort_order: UseStateHandle<SortOrder>,
    pub on_sort_column: Callback<&'static str>,
    pub classes: TableClasses,
}

#[derive(Properties, PartialEq)]
pub struct TableHeaderProps {
    pub columns: Vec<Column>,
    pub sort_column: UseStateHandle<Option<&'static str>>,
    pub sort_order: UseStateHandle<SortOrder>,
    pub on_sort_column: Callback<&'static str>,
    pub classes: TableClasses,
}

#[derive(Properties, PartialEq)]
pub struct PaginationControlsProps {
    pub page: UseStateHandle<usize>,
    pub total_pages: usize,
}
