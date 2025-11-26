use dioxus::prelude::*;
use web_sys::UrlSearchParams;
use web_sys::wasm_bindgen::JsValue;

use crate::dioxus::body::TableBody;
use crate::dioxus::controls::PaginationControls;
use crate::dioxus::header::TableHeader;
use crate::dioxus::types::SortOrder;
use crate::dioxus::types::TableProps;

/// A fully featured table component with sorting, pagination, and search functionality in Dioxus.
///
/// This component renders an interactive HTML `<table>` with customizable columns, data,
/// class names, and labels. It supports client-side sorting, search with URL hydration,
/// and pagination.
///
/// # Props
/// `TableProps` defines the configuration for this component:
/// - `data`: A `Vec<HashMap<&'static str, String>>` representing row data.
/// - `columns`: A `Vec<Column>` describing each column's ID, header text, and behavior.
/// - `page_size`: Number of rows to display per page (default: `10`).
/// - `loading`: When `true`, displays a loading indicator (default: `false`).
/// - `paginate`: Enables pagination controls (default: `false`).
/// - `search`: Enables a search input for client-side filtering (default: `false`).
/// - `texts`: Customizable text labels for UI strings (default: `TableTexts::default()`).
/// - `classes`: Customizable CSS class names for each table part (default: `TableClasses::default()`).
///
/// # Features
/// - **Search**: Filters rows client-side using a text input; the query is persisted in the URL via `?search=`.
/// - **Sorting**: Clickable headers allow sorting columns ascending or descending.
/// - **Pagination**: Navigate between pages using prev/next buttons, with an indicator showing current page.
/// - **Custom Classes**: All elements are styled via `TableClasses` for full customization.
/// - **Text Overrides**: All UI strings (e.g., empty state, loading, buttons) can be customized using `TableTexts`.
///
/// # Returns
/// Returns a `Dioxus` `Element` that renders a complete table with the above features.
///
/// # Example
/// ```rust
/// use dioxus::prelude::*;
/// use maplit::hashmap;
/// use table_rs::dioxus::table::Table;
/// use table_rs::dioxus::types::Column;
///
///
/// fn App() -> Element {
///     let data = vec![
///         hashmap! { "name" => "ferris".to_string(), "email" => "ferris@opensass.org".to_string() },
///         hashmap! { "name" => "ferros".to_string(), "email" => "ferros@opensass.org".to_string() },
///     ];
///
///     let columns = vec![
///         Column { id: "name", header: "Name", sortable: true, ..Default::default() },
///         Column { id: "email", header: "Email", ..Default::default() },
///     ];
///
///     rsx! {
///         Table {
///             data: data,
///             columns: columns,
///             paginate: true,
///             search: true,
///         }
///     }
/// }
/// ```
///
/// # See Also
/// - [MDN `<table>` Element](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/table)
#[component]
pub fn Table(props: TableProps) -> Element {
    let TableProps {
        data,
        columns,
        page_size,
        loading,
        paginate,
        search,
        texts,
        classes,
    } = props;

    let mut page = use_signal(|| 0_usize);
    let mut sort_column = use_signal(|| None::<&'static str>);
    let mut sort_order = use_signal(SortOrder::default);
    let mut search_query = use_signal(String::new);

    #[cfg(target_arch = "wasm")]
    use_effect(move || {
        let window = web_sys::window().unwrap();
        let location = window.location();
        let search = location.search().unwrap_or_default();
        let params = UrlSearchParams::new_with_str(&search).unwrap();
        if let Some(search_val) = params.get("search") {
            search_query.set(search_val);
        }
    });

    #[cfg(target_arch = "wasm")]
    let update_search_param = move |query: &str| {
        let window = web_sys::window().unwrap();
        let href = window.location().href().unwrap();
        let url = web_sys::Url::new(&href).unwrap();
        let params = url.search_params();
        params.set("search", query);
        url.set_search(&params.to_string().as_string().unwrap_or_default());

        window
            .history()
            .unwrap()
            .replace_state_with_url(&JsValue::NULL, "", Some(&url.href()))
            .unwrap();
    };

    let filtered_rows = {
        let mut rows = data.clone();
        if !search_query().is_empty() {
            rows.retain(|row| {
                columns.iter().any(|col| {
                    row.get(col.id)
                        .map(|v| v.to_lowercase().contains(&search_query().to_lowercase()))
                        .unwrap_or(false)
                })
            });
        }

        if let Some(col_id) = sort_column() {
            if let Some(col) = columns.iter().find(|c| c.id == col_id) {
                rows.sort_by(|a, b| {
                    let val = "".to_string();
                    let a_val = a.get(col.id).unwrap_or(&val);
                    let b_val = b.get(col.id).unwrap_or(&val);
                    match sort_order() {
                        SortOrder::Asc => a_val.cmp(b_val),
                        SortOrder::Desc => b_val.cmp(a_val),
                    }
                });
            }
        }

        rows
    };

    let total_pages = (filtered_rows.len() as f64 / page_size as f64).ceil() as usize;
    let start = page() * page_size;
    let end = ((page() + 1) * page_size).min(filtered_rows.len());
    let page_rows = &filtered_rows[start..end];

    let on_sort_column = move |id: &'static str| {
        if Some(id) == sort_column() {
            sort_order.set(match sort_order() {
                SortOrder::Asc => SortOrder::Desc,
                SortOrder::Desc => SortOrder::Asc,
            });
        } else {
            sort_column.set(Some(id));
            sort_order.set(SortOrder::Asc);
        }
    };

    let pagination_controls = if paginate {
        rsx! {
            PaginationControls {
                page: page,
                total_pages: total_pages,
                classes: classes.clone(),
                texts: texts.clone(),
            }
        }
    } else {
        rsx! {}
    };

    rsx! {
        div {
            class: "{classes.container}",
            if search {
                input {
                    class: "{classes.search_input}",
                    r#type: "text",
                    value: "{search_query()}",
                    placeholder: "{texts.search_placeholder}",
                    oninput: move |e| {
                        let val = e.value();
                        search_query.set(val.clone());
                        page.set(0);
                        #[cfg(target_arch = "wasm")]
                        update_search_param(&val);
                    }
                }
            }
            table {
                class: "{classes.table}",
                TableHeader {
                    columns: columns.clone(),
                    sort_column: sort_column,
                    sort_order: sort_order,
                    on_sort_column: on_sort_column,
                    classes: classes.clone(),
                }
                TableBody {
                    columns: columns.clone(),
                    rows: page_rows.to_vec(),
                    loading: loading,
                    classes: classes.clone(),
                    texts: texts.clone(),
                }
            }
            {pagination_controls}
        }
    }
}
