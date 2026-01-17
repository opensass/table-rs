use crate::dioxus::types::Column;
use crate::dioxus::types::TableClasses;
use crate::dioxus::types::TableTexts;
use dioxus::prelude::*;
use std::collections::HashMap;

/// A table body component that renders rows of data, along with loading and empty states.
///
/// This component is responsible for rendering the `<tbody>` section of a table in a Dioxus application.
/// It dynamically displays row data, a loading message, or an empty state message based on the provided props.
///
/// # Props
/// - `columns`: A `Vec<Column>` defining which fields to render in each table row. Each column corresponds to a key in the row data.
/// - `rows`: A `Vec<HashMap<&'static str, String>>` representing the data for each row, where keys match column IDs.
/// - `loading`: A `bool` flag that, when true, displays a loading message instead of data rows.
/// - `classes`: A `TableClasses` struct for customizing the CSS class names of the body, rows, and cells.
/// - `texts`: A `TableTexts` struct that provides custom text for the loading and empty states.
///
/// # Behavior
/// - If `loading` is `true`, a single row with a loading message is shown spanning all columns.
/// - If `rows` is empty and not loading, an empty message row is displayed.
/// - Otherwise, each data row is rendered in a `<tr>`, with one `<td>` per column.
///
/// # Returns
/// A Dioxus `Element` representing the `<tbody>` of a table, with dynamic row content.
///
/// # Example
/// ```rust
/// use dioxus::prelude::*;
/// use maplit::hashmap;
/// use table_rs::dioxus::table::Table;
/// use table_rs::dioxus::types::{Column, TableClasses, TableTexts};
/// use table_rs::dioxus::body::TableBody;
///
///
/// fn App() -> Element {
///     let rows = vec![
///         hashmap! { "name" => "Ferris".to_string(), "email" => "ferris@opensass.org".to_string() },
///         hashmap! { "name" => "Rustacean".to_string(), "email" => "rust@opensass.org".to_string() },
///     ];
///
///     let columns = vec![
///         Column { id: "name", header: "Name", ..Default::default() },
///         Column { id: "email", header: "Email", ..Default::default() },
///     ];
///
///     rsx! {
///         TableBody {
///             columns: columns,
///             rows: rows,
///             loading: false,
///             classes: TableClasses::default(),
///             texts: TableTexts::default(),
///         }
///     }
/// }
/// ```
///
/// # See Also
/// - [MDN `<tbody>` Element](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/tbody)
#[component]
pub fn TableBody(
    columns: Vec<Column>,
    rows: Vec<HashMap<&'static str, String>>,
    loading: bool,
    classes: TableClasses,
    texts: TableTexts,
) -> Element {
    let content = if loading {
        rsx! {
            tr { class: "{classes.loading_row}",
                td {
                    colspan: "{columns.len()}",
                    "{texts.loading}"
                }
            }
        }
    } else if rows.is_empty() {
        rsx! {
            tr { class: "{classes.empty_row}",
                td {
                    colspan: "{columns.len()}",
                    "{texts.empty}"
                }
            }
        }
    } else {
        rsx! {
            for row in rows.iter() {
                tr { class: "{classes.row}", role: "row",
                    for col in columns.iter() {
                        td { class: "{classes.body_cell}", role: "cell",
                            BodyCell {
                                column: col.clone(),
                                content: row.get(col.id).unwrap_or(&String::new()),
                            }
                        }
                    }
                }
            }
        }
    };

    rsx! {
        tbody { class: "{classes.tbody}",
            {content}
        }
    }
}

#[component]
fn BodyCell(column: Column, content: String) -> Element {
    if let Some(cb) = column.cell {
        cb(content)
    } else {
        rsx! {
            "{content}"
        }
    }
}
