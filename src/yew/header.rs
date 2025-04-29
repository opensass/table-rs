use crate::yew::types::{SortOrder, TableHeaderProps};
use yew::prelude::*;

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
            <tr role="row">
                { for columns.iter().map(|col| {
                    let col_id = col.id;
                    let is_sortable = col.sortable;
                    let onclick = if is_sortable {
                        let on_sort_column = on_sort_column.clone();
                        Some(Callback::from(move |_| on_sort_column.emit(col_id)))
                    } else {
                        None
                    };

                    html! {
                        <th
                            {onclick}
                            role="columnheader"
                            class={col.class.unwrap_or_default()}
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
