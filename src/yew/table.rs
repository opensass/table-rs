use gloo_timers::callback::Timeout;
use web_sys::UrlSearchParams;
use web_sys::wasm_bindgen::JsValue;
use yew::prelude::*;

use crate::yew::body::TableBody;
use crate::yew::controls::PaginationControls;
use crate::yew::header::TableHeader;
use crate::yew::types::SortOrder;
use crate::yew::types::TableProps;

/// A fully featured table component with pagination, sorting, and search support.
///
/// This component renders a complete `<table>` element, including headers (`<thead>`), body (`<tbody>`),
/// and optional features such as client-side sorting, pagination, and search input.
/// It is built using Yew and supports flexible styling and customization.
///
/// # Arguments
/// * `props` - The properties passed to the component.
///   - `data` - A `Vec<HashMap<&'static str, String>>` representing the table's row data.
///   - `columns` - A `Vec<Column>` defining the structure and behavior of each column.
///   - `page_size` - A `usize` defining how many rows to show per page.
///   - `loading` - A `bool` indicating whether the table is in a loading state.
///   - `classes` - A `TableClasses` struct for customizing class names of elements.
///   - `styles` - A `HashMap<&'static str, &'static str>` for inline style overrides.
///   - `paginate` - A `bool` controlling whether pagination controls are displayed.
///   - `search` - A `bool` enabling a search input above the table.
///   - `texts` - A `TableTexts` struct for customizing placeholder and fallback texts.
///
/// # Features
/// - **Client-side search** with URL hydration via `?search=`
/// - **Column sorting** (ascending/descending toggle)
/// - **Pagination controls**
/// - **Custom class and inline style support**
/// - Displays a loading row or empty state message when appropriate
///
/// # Returns
/// (Html): A complete, styled and interactive table component rendered in Yew.
///
/// # Examples
/// ```rust
/// use yew::prelude::*;
/// use maplit::hashmap;
/// use table_rs::yew::table::Table;
/// use table_rs::yew::types::{Column, TableClasses, TableTexts};
///
/// #[function_component(App)]
/// pub fn app() -> Html {
///     let data = vec![
///         hashmap! { "name" => "Ferris".into(), "email" => "ferris@opensass.org".into() },
///         hashmap! { "name" => "Ferros".into(), "email" => "ferros@opensass.org".into() },
///     ];
///
///     let columns = vec![
///         Column { id: "name", header: "Name", sortable: true, ..Default::default() },
///         Column { id: "email", header: "Email", sortable: false, ..Default::default() },
///     ];
///
///     html! {
///         <Table
///             data={data}
///             columns={columns}
///             page_size={10}
///             loading={false}
///             paginate={true}
///             search={true}
///             classes={TableClasses::default()}
///             texts={TableTexts::default()}
///         />
///     }
/// }
/// ```
///
/// # See Also
/// - [MDN table Element](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/table)
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
        texts,
        row_end_component,
        default_sort_column,
        default_sort_order,
    } = props;

    let page = use_state(|| 0);
    let sort_column = use_state(|| *default_sort_column);
    let sort_order = use_state(|| *default_sort_order);
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
                            placeholder={texts.search_placeholder}
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
                    has_row_end={row_end_component.is_some()}
                />
                <TableBody
                    columns={columns.clone()}
                    rows={page_rows.to_vec()}
                    loading={loading}
                    classes={classes.clone()}
                    texts={texts.clone()}
                    row_end_component={row_end_component.clone()}
                />
            </table>
            { if *paginate {
                    html! {
                        <PaginationControls {page} {total_pages} classes={classes.clone()} texts={texts.clone()}/>
                    }
                } else {
                    html! {}
                } }
        </div>
    }
}
