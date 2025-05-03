use crate::yew::types::PaginationControlsProps;
use yew::prelude::*;

#[function_component(PaginationControls)]
pub fn pagination_controls(props: &PaginationControlsProps) -> Html {
    let PaginationControlsProps {
        page,
        total_pages,
        classes,
        texts,
    } = props;
    let page_val = **page;

    let on_prev = {
        let page = page.clone();
        Callback::from(move |_| {
            if *page > 0 {
                page.set(*page - 1);
            }
        })
    };

    let on_next = {
        let page = page.clone();
        Callback::from(move |_| {
            page.set(*page + 1);
        })
    };

    html! {
        <div class={classes.pagination}>
            <button class={classes.pagination_button} onclick={on_prev} disabled={page_val == 0}>
                { texts.previous_button }
            </button>
            <span>
                { texts.page_indicator.replace("{current}", &(page_val + 1).to_string()).replace("{total}", &total_pages.to_string()) }
            </span>
            <button
                class={classes.pagination_button}
                onclick={on_next}
                disabled={page_val + 1 >= *total_pages}
            >
                { texts.next_button }
            </button>
        </div>
    }
}
