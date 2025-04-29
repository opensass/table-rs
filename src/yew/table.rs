use gloo_timers::callback::Timeout;
use web_sys::wasm_bindgen::JsValue;
use web_sys::UrlSearchParams;
use yew::prelude::*;

use crate::yew::body::TableBody;
use crate::yew::controls::PaginationControls;
use crate::yew::header::TableHeader;
use crate::yew::types::SortOrder;
use crate::yew::types::TableProps;

#[function_component(Table)]
pub fn table(props: &TableProps) -> Html {
    let TableProps {
        data,
        columns,
        page_size,
        loading,
        classes,
        styles,
        paginate,
        search,
    } = props;

    let page = use_state(|| 0);
    let sort_column = use_state(|| None::<&'static str>);
    let sort_order = use_state(|| SortOrder::Asc);
    let search_query = use_state(|| {
        let window = web_sys::window().unwrap();
        let search_params =
            UrlSearchParams::new_with_str(&window.location().search().unwrap_or_default()).unwrap();
        search_params.get("search").unwrap_or_default()
    });

    let debounced_search = use_state(|| None::<Timeout>);

    let update_search_url = {
        let search_query = search_query.clone();
        Callback::from(move |query: String| {
            let window = web_sys::window().unwrap();
            let url = window.location().href().unwrap();
            let url_obj = web_sys::Url::new(&url).unwrap();
            let params = url_obj.search_params();
            params.set("search", &query);
            url_obj.set_search(&params.to_string().as_string().unwrap_or_default());
            window
                .history()
                .unwrap()
                .replace_state_with_url(&JsValue::NULL, "", Some(&url_obj.href()))
                .unwrap();
            search_query.set(query);
        })
    };

    let on_search_change = {
        let debounced_search = debounced_search.clone();
        let update_search_url = update_search_url.clone();
        Callback::from(move |e: InputEvent| {
            let update_search_url = update_search_url.clone();
            // TODO: Add debounce
            // let debounced_search_ref = debounced_search.clone();
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            let value = input.value();

            // let prev_timeout = {
            //     debounced_search_ref.take()
            // };

            // if let Some(prev) = prev_timeout {
            //     prev.cancel();
            // }

            let timeout = Timeout::new(50, move || {
                update_search_url.emit(value.clone());
            });

            debounced_search.set(Some(timeout));
        })
    };

    let mut filtered_rows = data.clone();
    if !search_query.is_empty() {
        filtered_rows.retain(|row| {
            columns.iter().any(|col| {
                row.get(col.id)
                    .map(|v| v.to_lowercase().contains(&search_query.to_lowercase()))
                    .unwrap_or(false)
            })
        });
    }

    if let Some(col_id) = *sort_column {
        if let Some(col) = columns.iter().find(|c| c.id == col_id) {
            filtered_rows.sort_by(|a, b| {
                let val = "".to_string();
                let a_val = a.get(col.id).unwrap_or(&val);
                let b_val = b.get(col.id).unwrap_or(&val);
                match *sort_order {
                    SortOrder::Asc => a_val.cmp(b_val),
                    SortOrder::Desc => b_val.cmp(a_val),
                }
            });
        }
    }

    let total_pages = (filtered_rows.len() as f64 / *page_size as f64).ceil() as usize;
    let start = *page * page_size;
    let end = ((*page + 1) * page_size).min(filtered_rows.len());
    let page_rows = &filtered_rows[start..end];

    let on_sort_column = {
        let sort_column = sort_column.clone();
        let sort_order = sort_order.clone();
        Callback::from(move |id: &'static str| {
            if Some(id) == *sort_column {
                sort_order.set(match *sort_order {
                    SortOrder::Asc => SortOrder::Desc,
                    SortOrder::Desc => SortOrder::Asc,
                });
            } else {
                sort_column.set(Some(id));
                sort_order.set(SortOrder::Asc);
            }
        })
    };

    html! {
        <div class={classes.container}>
            { if *search {
                    html! {
                        <input
                            class={classes.search_input}
                            type="text"
                            value={(*search_query).clone()}
                            placeholder="Search..."
                            aria-label="Search table"
                            oninput={on_search_change}
                        />
                    }
                } else {
                    html! {}
                } }
            <table class={classes.table} style={*styles.get("table").unwrap_or(&"")} role="table">
                <TableHeader
                    columns={columns.clone()}
                    {sort_column}
                    {sort_order}
                    {on_sort_column}
                    classes={classes.clone()}
                />
                <TableBody
                    columns={columns.clone()}
                    rows={page_rows.to_vec()}
                    loading={loading}
                    classes={classes.clone()}
                />
            </table>
            { if *paginate {
                    html! {
                        <PaginationControls {page} {total_pages} />
                    }
                } else {
                    html! {}
                } }
        </div>
    }
}
