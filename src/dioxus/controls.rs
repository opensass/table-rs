use crate::dioxus::types::TableClasses;
use crate::dioxus::types::TableTexts;
use dioxus::prelude::*;

#[component]
pub fn PaginationControls(
    page: Signal<usize>,
    total_pages: usize,
    classes: TableClasses,
    texts: TableTexts,
) -> Element {
    let on_prev = move |_| {
        if page() > 0 {
            page.set(page() - 1);
        }
    };

    let on_next = move |_| {
        if page() + 1 < total_pages {
            page.set(page() + 1);
        }
    };

    // Pre-compute page indicator to avoid multiple string allocations
    let page_indicator_text = texts.page_indicator
        .replace("{current}", &(page() + 1).to_string())
        .replace("{total}", &total_pages.to_string());

    rsx! {
        div { class: classes.pagination,
            button {
                class: classes.pagination_button,
                onclick: on_prev,
                disabled: page() == 0,
                "{texts.previous_button}"
            }
            span {
                "{ page_indicator_text }"
            }
            button {
                class: classes.pagination_button,
                onclick: on_next,
                disabled: page() + 1 >= total_pages,
                "{texts.next_button}"
            }
        }
    }
}
