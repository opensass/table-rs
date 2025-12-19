use crate::dioxus::types::Column;
use crate::dioxus::types::SortOrder;
use crate::dioxus::types::TableClasses;
use dioxus::prelude::*;

/// A table header component that renders sortable column headers for use within the `Table` component.
///
/// This component produces the `<thead>` section of a table using the provided column definitions,
/// handling rendering, sorting indicators (`aria-sort`), and user interaction to trigger sort changes.
///
/// # Props
/// - `columns`: A `Vec<Column>` defining the columns to display in the header. Each `Column` may be sortable and have optional styles or class overrides.
/// - `sort_column`: A `Signal<Option<&'static str>>` indicating which column (if any) is currently being sorted.
/// - `sort_order`: A `Signal<SortOrder>` indicating the current sort direction (`Asc` or `Desc`).
/// - `on_sort_column`: An `EventHandler<&'static str>` triggered when a sortable header cell is clicked. The column ID is passed as the event payload.
/// - `classes`: A `TableClasses` struct allowing custom class names for `<thead>`, `<tr>`, and `<th>` elements.
///
/// # Behavior
/// - Sortable columns show proper `aria-sort` attributes for accessibility (`ascending`, `descending`, or `none`).
/// - Clicking a sortable column emits an event to update sort state.
/// - Each column can override default styles and classes via `Column::style` and `Column::class`.
///
/// # Returns
/// Returns a `Dioxus` `Element` containing the `<thead>` with all column headers rendered as `<th>` elements.
///
/// # Example
/// ```rust
/// use dioxus::prelude::*;
/// use maplit::hashmap;
/// use table_rs::dioxus::table::Table;
/// use table_rs::dioxus::types::{Column, TableClasses, SortOrder};
/// use table_rs::dioxus::header::TableHeader;
///
///
/// fn App() -> Element {
///     let columns = vec![
///         Column { id: "name", header: "Name", sortable: true, ..Default::default() },
///         Column { id: "email", header: "Email", sortable: false, ..Default::default() },
///     ];
///
///     let sort_column = use_signal(|| Some("name"));
///     let sort_order = use_signal(|| SortOrder::Asc);
///
///     rsx! {
///         TableHeader {
///             columns: columns,
///             sort_column: sort_column,
///             sort_order: sort_order,
///             on_sort_column: move |col_id| println!("Sort column changed: {}", col_id),
///             classes: TableClasses::default(),
///         }
///     }
/// }
/// ```
///
/// # See Also
/// - [MDN `<thead>` Element](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/thead)
#[component]
pub fn TableHeader(
    columns: Vec<Column>,
    sort_column: Signal<Option<&'static str>>,
    sort_order: Signal<SortOrder>,
    on_sort_column: EventHandler<&'static str>,
    classes: TableClasses,
    #[props(default)]
    has_row_end: bool,
) -> Element {
    let header_cells = columns.iter().map(|col| {
        let col_id = col.id;
        let is_sorted = sort_column() == Some(col_id);
        let aria_sort = if is_sorted {
            match sort_order() {
                SortOrder::Asc => "ascending",
                SortOrder::Desc => "descending",
            }
        } else {
            "none"
        };

        let class = format!("{} {}", classes.header_cell, col.class.unwrap_or_default());
        let style = col.style.unwrap_or_default();
        let header = col.header;

        let onclick = if col.sortable {
            Callback::new(move |_| on_sort_column.call(col_id))
        } else {
            Callback::new(|_| {})
        };

        rsx! {
            th {
                key: "{col_id}",
                role: "columnheader",
                class: "{class}",
                style: "{style}",
                aria_sort: "{aria_sort}",
                onclick: onclick,
                "{header}"
            }
        }
    });

    rsx! {
        thead { class: "{classes.thead}",
            tr { class: "{classes.row}", role: "row",
                {header_cells}
                if has_row_end {
                    th { class: "{classes.header_cell}", role: "columnheader" }
                }
            }
        }
    }
}
