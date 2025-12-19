use std::collections::HashMap;
use yew::prelude::*;

/// Represents a column in the table with customization options.
#[derive(Properties, PartialEq, Clone, Default)]
pub struct Column {
    /// Unique identifier for the column.
    #[prop_or("")]
    pub id: &'static str,

    /// Header text displayed at the top of the column.
    #[prop_or("")]
    pub header: &'static str,

    /// Callback triggered when accessing column data; can be used for custom rendering.
    #[prop_or(Callback::noop())]
    pub accessor: Callback<()>,

    /// Determines if the column is sortable.
    #[prop_or(false)]
    pub sortable: bool,

    /// Minimum width of the column in pixels.
    #[prop_or(100)]
    pub min_width: u32,

    /// Optional inline style string for the column header.
    #[prop_or(Some("padding: 8px; font-weight: 600; text-align: left;"))]
    pub style: Option<&'static str>,

    /// Optional class name(s) for the column header.
    #[prop_or(Some("table-header-cell"))]
    pub class: Option<&'static str>,
}

/// Sort direction for a column: ascending or descending.
#[derive(Clone, PartialEq, Default)]
pub enum SortOrder {
    /// Ascending order (default).
    #[default]
    Asc,

    /// Descending order.
    Desc,
}

/// Class names used to style various parts of the table.
#[derive(Properties, PartialEq, Clone)]
pub struct TableClasses {
    /// Class name for the container wrapping the table.
    #[prop_or("table-container")]
    pub container: &'static str,

    /// Class name for the table element.
    #[prop_or("table")]
    pub table: &'static str,

    /// Class name for the `<thead>` element.
    #[prop_or("thead")]
    pub thead: &'static str,

    /// Class name for the `<tbody>` element.
    #[prop_or("tbody")]
    pub tbody: &'static str,

    /// Class name for pagination controls wrapper.
    #[prop_or("pagination-controls")]
    pub pagination: &'static str,

    /// Class name for the search input.
    #[prop_or("search-input")]
    pub search_input: &'static str,

    /// Class name for header cells (`<th>`).
    #[prop_or("th")]
    pub header_cell: &'static str,

    /// Class name for body cells (`<td>`).
    #[prop_or("td")]
    pub body_cell: &'static str,

    /// Class name for table rows (`<tr>`).
    #[prop_or("tr")]
    pub row: &'static str,

    /// Class name for the loading row.
    #[prop_or("loading-row")]
    pub loading_row: &'static str,

    /// Class name for the empty row.
    #[prop_or("empty-row")]
    pub empty_row: &'static str,

    /// Class name for pagination buttons.
    #[prop_or("pagination-button")]
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

/// Texts used in various parts of the table UI.
#[derive(Properties, PartialEq, Clone)]
pub struct TableTexts {
    /// Text shown while data is loading.
    #[prop_or("Loading...")]
    pub loading: &'static str,

    /// Text shown when no rows are found.
    #[prop_or("No results found")]
    pub empty: &'static str,

    /// Placeholder text for the search input.
    #[prop_or("Search...")]
    pub search_placeholder: &'static str,

    /// Label for the "Previous" pagination button.
    #[prop_or("Previous")]
    pub previous_button: &'static str,

    /// Label for the "Next" pagination button.
    #[prop_or("Next")]
    pub next_button: &'static str,

    /// Format string for the page indicator, e.g., "Page 1 of 5".
    #[prop_or("Page {current} of {total}")]
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

/// Props for the main table component.
#[derive(Properties, PartialEq, Clone)]
pub struct TableProps {
    /// Vector of row data as key-value pairs.
    #[prop_or_default]
    pub data: Vec<HashMap<&'static str, String>>,

    /// List of column definitions.
    #[prop_or_default]
    pub columns: Vec<Column>,

    /// Number of rows per page.
    #[prop_or(10)]
    pub page_size: usize,

    /// Whether the table is currently in a loading state.
    #[prop_or(false)]
    pub loading: bool,

    /// Class names used to style the table.
    #[prop_or_default]
    pub classes: TableClasses,

    /// Optional inline styles mapped by component part.
    #[prop_or_default]
    pub styles: HashMap<&'static str, &'static str>,

    /// Whether to enable pagination.
    #[prop_or(false)]
    pub paginate: bool,

    /// Whether to enable search functionality.
    #[prop_or(false)]
    pub search: bool,

    /// Text labels for the table UI.
    #[prop_or_default]
    pub texts: TableTexts,

    /// Optional component to render at the end of each row.
    #[prop_or_default]
    pub row_end_component: Option<Callback<HashMap<&'static str, String>, Html>>,
}

/// Props for the table header including sorting logic.
#[derive(Properties, PartialEq, Clone)]
pub struct HeaderProps {
    /// Column definitions for the header.
    #[prop_or_default]
    pub columns: Vec<Column>,

    /// Current column used for sorting.
    pub sort_column: UseStateHandle<Option<&'static str>>,

    /// Current sort order (ascending/descending).
    pub sort_order: UseStateHandle<SortOrder>,

    /// Callback triggered when a column header is clicked for sorting.
    #[prop_or(Callback::noop())]
    pub on_sort_column: Callback<&'static str>,

    /// CSS classes used to style the header.
    #[prop_or_default]
    pub classes: TableClasses,
}

/// Alias for `HeaderProps` to be used explicitly in header components.
#[derive(Properties, PartialEq, Clone)]
pub struct TableHeaderProps {
    /// Column definitions for the header.
    #[prop_or_default]
    pub columns: Vec<Column>,

    /// Current column used for sorting.
    pub sort_column: UseStateHandle<Option<&'static str>>,

    /// Current sort order (ascending/descending).
    pub sort_order: UseStateHandle<SortOrder>,

    /// Callback triggered when a column is sorted.
    #[prop_or(Callback::noop())]
    pub on_sort_column: Callback<&'static str>,

    /// CSS classes used to style the table header.
    #[prop_or_default]
    pub classes: TableClasses,

    /// Whether there is a row end component that needs an extra header cell.
    #[prop_or(false)]
    pub has_row_end: bool,
}

/// Props for the pagination controls component.
#[derive(Properties, PartialEq, Clone)]
pub struct PaginationControlsProps {
    /// Current page index.
    pub page: UseStateHandle<usize>,

    /// Total number of pages.
    #[prop_or(1)]
    pub total_pages: usize,

    /// Class names used to style pagination elements.
    #[prop_or_default]
    pub classes: TableClasses,

    /// Texts used in pagination controls.
    #[prop_or_default]
    pub texts: TableTexts,
}

/// Props for rendering the body of the table.
#[derive(Properties, PartialEq, Clone)]
pub struct TableBodyProps {
    /// Column definitions.
    #[prop_or_default]
    pub columns: Vec<Column>,

    /// List of row data to render.
    #[prop_or_default]
    pub rows: Vec<HashMap<&'static str, String>>,

    /// Indicates if the body is in a loading state.
    #[prop_or(false)]
    pub loading: bool,

    /// Class names used to style the table body.
    #[prop_or_default]
    pub classes: TableClasses,

    /// Text labels used in the body (e.g., loading, empty).
    #[prop_or_default]
    pub texts: TableTexts,

    /// Optional component to render at the end of each row.
    #[prop_or_default]
    pub row_end_component: Option<Callback<HashMap<&'static str, String>, Html>>,
}
