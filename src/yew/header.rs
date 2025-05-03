use crate::yew::types::{SortOrder, TableHeaderProps};
use yew::prelude::*;

/// A table header component that renders column headers with optional sorting functionality.
///
/// This component is part of the `table_rs` Yew integration and is responsible for rendering
/// the `<thead>` section of a table. It supports sortable columns and emits sort events when
/// a sortable header is clicked.
///
/// # Arguments
/// * `props` - The properties passed to the component.
///   - `columns` - A list of column definitions (`Vec<Column>`) specifying the headers to render.
///   - `sort_column` - An `Option<&'static str>` indicating the currently sorted column, if any.
///   - `sort_order` - A `SortOrder` indicating whether the sort is ascending or descending.
///   - `on_sort_column` - A `Callback<&'static str>` triggered when a sortable column is clicked.
///   - `classes` - A `TableClasses` object defining CSS class names for customization.
///
/// # Returns
/// (Html): A rendered `<thead>` element containing the table header row and interactive sorting logic.
///
/// # Examples
/// ```rust
/// use table_rs::yew::header::TableHeader;
/// use table_rs::yew::types::{TableHeaderProps, Column, SortOrder, TableClasses};
/// use yew::prelude::*;
///
/// #[function_component(App)]
/// pub fn app() -> Html {
///     let columns = vec![
///         Column { id: "name", header: "Name", sortable: true, ..Default::default() },
///         Column { id: "email", header: "Email", sortable: false, ..Default::default() },
///     ];
///
///     let sort_order = use_state(|| SortOrder::Asc);
///     let sort_column = use_state(|| Some("name"));
///
///     let props = TableHeaderProps {
///         columns,
///         sort_column: sort_column,
///         sort_order: sort_order,
///         on_sort_column: Callback::from(|col_id| web_sys::console::log_1(&format!("Sort: {}", col_id).into())),
///         classes: Default::default(),
///     };
///    
///     html! {
///         <TableHeader ..props />
///     }
/// };
/// ```
///
/// # See Also
/// - [MDN thead Element](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/thead)
#[function_component(TableHeader)]
pub fn header(props: &TableHeaderProps) -> Html {
    let TableHeaderProps {
        columns,
        sort_column,
        sort_order,
        on_sort_column,
        classes,
    } = props;

    html! {
        <thead class={classes.thead}>
            <tr class={classes.row} role="row">
                { for columns.iter().map(|col| {
                    let col_id = col.id;
                    let onclick = if col.sortable {
                        let on_sort_column = on_sort_column.clone();
                        Some(Callback::from(move |_| on_sort_column.emit(col_id)))
                    } else { None };

                    html! {
                        <th
                            {onclick}
                            role="columnheader"
                            class={format!("{} {}", classes.header_cell, col.class.unwrap_or("")).trim().to_string()}
                            style={col.style.unwrap_or_default()}
                            aria-sort={
                                if Some(col.id) == **sort_column {
                                    match **sort_order {
                                        SortOrder::Asc => "ascending",
                                        SortOrder::Desc => "descending",
                                    }
                                } else {
                                    "none"
                                }
                            }
                        >
                            { col.header }
                        </th>
                    }
                }) }
            </tr>
        </thead>
    }
}
