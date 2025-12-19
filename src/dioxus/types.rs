use dioxus::prelude::*;
use std::collections::HashMap;

/// Represents a column definition for the table.
#[derive(PartialEq, Props, Clone, Default)]
pub struct Column {
    /// Unique identifier for the column.
    pub id: &'static str,

    /// Header text displayed in the column.
    pub header: &'static str,

    /// Whether this column is sortable.
    #[props(default)]
    pub sortable: bool,

    /// Minimum width of the column (default is 100).
    #[props(default = 100)]
    pub min_width: u32,

    /// Optional inline styles for the column header.
    #[props(default)]
    pub style: Option<&'static str>,

    /// Optional CSS classes for the column header.
    #[props(default)]
    pub class: Option<&'static str>,
}

/// Text labels for table UI elements.
#[derive(PartialEq, Props, Clone)]
pub struct TableTexts {
    /// Text shown when data is loading.
    #[props(default = "Loading...")]
    pub loading: &'static str,

    /// Text shown when no data is available.
    #[props(default = "No results found")]
    pub empty: &'static str,

    /// Placeholder text for the search input.
    #[props(default = "Search...")]
    pub search_placeholder: &'static str,

    /// Label for the previous page button.
    #[props(default = "Previous")]
    pub previous_button: &'static str,

    /// Label for the next page button.
    #[props(default = "Next")]
    pub next_button: &'static str,

    /// Page indicator text with placeholders `{current}` and `{total}`.
    #[props(default = "Page {current} of {total}")]
    pub page_indicator: &'static str,
}

impl Default for TableTexts {
    fn default() -> Self {
        Self {
            loading: "Loading...",
            empty: "No results found",
            search_placeholder: "Search...",
            previous_button: "Previous",
            next_button: "Next",
            page_indicator: "Page {current} of {total}",
        }
    }
}

/// Defines the styling classes for each part of the table.
#[derive(Clone, PartialEq)]
pub struct TableClasses {
    /// Wrapper around the entire table.
    pub container: &'static str,

    /// Class for the `<table>` element.
    pub table: &'static str,

    /// Class for the `<thead>` element.
    pub thead: &'static str,

    /// Class for the `<tbody>` element.
    pub tbody: &'static str,

    /// Wrapper for pagination controls.
    pub pagination: &'static str,

    /// Class for the search input field.
    pub search_input: &'static str,

    /// Class for header cells (`<th>`).
    pub header_cell: &'static str,

    /// Class for body cells (`<td>`).
    pub body_cell: &'static str,

    /// Class for each table row.
    pub row: &'static str,

    /// Class for the row shown while loading.
    pub loading_row: &'static str,

    /// Class for the row shown when no data is found.
    pub empty_row: &'static str,

    /// Class for pagination buttons.
    pub pagination_button: &'static str,
}

impl Default for TableClasses {
    fn default() -> Self {
        Self {
            container: "table-container",
            table: "table",
            thead: "thead",
            tbody: "tbody",
            pagination: "pagination-controls",
            search_input: "search-input",
            header_cell: "th",
            body_cell: "td",
            row: "tr",
            loading_row: "loading-row",
            empty_row: "empty-row",
            pagination_button: "pagination-button",
        }
    }
}

/// Main props for the table component.
#[derive(PartialEq, Props, Clone)]
pub struct TableProps {
    /// Data rows, where each row is a key-value map.
    #[props(default)]
    pub data: Vec<HashMap<&'static str, String>>,

    /// Definitions of columns to display.
    #[props(default)]
    pub columns: Vec<Column>,

    /// Number of rows per page (default is 10).
    #[props(default = 10)]
    pub page_size: usize,

    /// Indicates whether the table is loading.
    #[props(default)]
    pub loading: bool,

    /// Enables pagination controls.
    #[props(default = false)]
    pub paginate: bool,

    /// Enables the search input field.
    #[props(default = false)]
    pub search: bool,

    /// Texts for various table UI messages.
    #[props(default)]
    pub texts: TableTexts,

    /// CSS classes for styling different parts of the table.
    #[props(default)]
    pub classes: TableClasses,

    /// Optional component to render at the end of each row.
    #[props(default)]
    pub row_end_component: Option<ReadOnlySignal<Callback<HashMap<&'static str, String>, Element>>>,
}

/// Sort direction (ascending or descending).
#[derive(PartialEq, Clone, Copy, Default)]
pub enum SortOrder {
    /// Ascending (default).
    #[default]
    Asc,
    /// Descending.
    Desc,
}
