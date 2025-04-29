use crate::yew::types::{Column, TableClasses};
use std::collections::HashMap;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TableBodyProps {
    pub columns: Vec<Column>,
    pub rows: Vec<HashMap<&'static str, String>>,
    pub loading: bool,
    pub classes: TableClasses,
}

#[function_component(TableBody)]
pub fn body(props: &TableBodyProps) -> Html {
    let TableBodyProps {
        columns,
        rows,
        loading,
        classes,
    } = props;

    html! {
        <tbody class={classes.tbody}>
            { if *loading {
                    html! { <tr><td colspan={columns.len().to_string()}>{ "Loading..." }</td></tr> }
                } else if rows.is_empty() {
                    html! { <tr><td colspan={columns.len().to_string()}>{ "No results found" }</td></tr> }
                } else {
                    html! { for rows.iter().map(|row| {
                        html! {
                            <tr role="row">
                                { for columns.iter().map(|col| {
                                    html! {
                                        <td role="cell">{ row.get(col.id).unwrap_or(&"".to_string()) }</td>
                                    }
                                }) }
                            </tr>
                        }
                    }) }
                } }
        </tbody>
    }
}
