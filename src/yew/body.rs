use crate::yew::types::TableBodyProps;
use yew::prelude::*;

/// A table body component that handles rendering of table rows, empty state, and loading state.
///
/// This component is part of the `table_rs` Yew integration and is responsible for rendering
/// the `<tbody>` section of a table, based on the provided data and configuration.
///
/// # Arguments
/// * `props` - The properties passed to the component.
///   - `columns` - A list of column definitions (`Vec<Column>`) specifying which fields to render.
///   - `rows` - A vector of row data (`Vec<HashMap<&'static str, String>>`) to display.
///   - `loading` - A boolean flag indicating whether the table is in a loading state.
///   - `classes` - A `TableClasses` object defining CSS class names for customization.
///   - `texts` - A `TableTexts` object defining UI text like loading or empty messages.
///
/// # Returns
/// (Html): A rendered `<tbody>` element, containing:
///   - A loading row if `loading` is `true`.
///   - An empty state row if `rows` is empty.
///   - The list of rows otherwise.
///
/// # Examples
/// ```rust
/// use table_rs::yew::body::TableBody;
/// use table_rs::yew::types::{TableBodyProps, Column, TableClasses, TableTexts};
/// use yew::prelude::*;
/// use maplit::hashmap;
///
/// #[function_component(App)]
/// pub fn app() -> Html {
///     let rows = vec![
///         hashmap! { "name" => "Ferris".to_string(), "email" => "ferris@opensass.org".to_string() },
///         hashmap! { "name" => "Crab".to_string(), "email" => "crab@opensass.org".to_string() },
///     ];
///    
///     let columns = vec![
///         Column { id: "name", header: "Name", ..Default::default() },
///         Column { id: "email", header: "Email", ..Default::default() },
///     ];
///    
///     let props = TableBodyProps {
///         columns,
///         rows,
///         loading: false,
///         classes: Default::default(),
///         texts: Default::default(),
///     };
///    
///     html! {
///         <TableBody ..props />
///     }
/// }
/// ```
///
/// # See Also
/// - [MDN tbody Element](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/tbody)
#[function_component(TableBody)]
pub fn body(props: &TableBodyProps) -> Html {
    let TableBodyProps {
        columns,
        rows,
        loading,
        classes,
        texts,
    } = props;

    html! {
        <tbody class={classes.tbody}>
            { if *loading {
                    html! {
                        <tr class={classes.loading_row}><td colspan={columns.len().to_string()}>{ texts.loading }</td></tr>
                    }
                } else if rows.is_empty() {
                    html! {
                        <tr class={classes.empty_row}><td colspan={columns.len().to_string()}>{ texts.empty }</td></tr>
                    }
                } else {
                    html! {
                        for row in rows.iter() {
                                <tr class={classes.row} role="row">
                                        for col in columns.iter() {
                                                <td class={classes.body_cell} role="cell">{ row.get(col.id).unwrap_or(&"".to_string()) }</td>
                                        }
                                </tr>
                        }
                    }
                } }
        </tbody>
    }
}
