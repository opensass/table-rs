use crate::yew::types::PaginationControlsProps;
use yew::prelude::*;

#[function_component(PaginationControls)]
pub fn pagination_controls(props: &PaginationControlsProps) -> Html {
    let PaginationControlsProps { page, total_pages } = props;
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
        <div class="pagination-controls">
            <button onclick={on_prev} disabled={page_val == 0}>{ "Previous" }</button>
            <span>{ format!(" Page {} of {} ", page_val + 1, total_pages) }</span>
            <button onclick={on_next} disabled={page_val + 1 >= *total_pages}>{ "Next" }</button>
        </div>
    }
}
