use maplit::hashmap;
use std::collections::HashMap;
use table_rs::yew::table::Table;
use table_rs::yew::types::Column;
use web_sys::window;
use web_sys::js_sys;
use yew::prelude::*;

#[function_component(LandingPage)]
pub fn landing_page() -> Html {
    let data = use_state(|| None::<Vec<HashMap<&'static str, String>>>);
    let loading = data.is_none();
    let data_val = (*data).clone();

    use_effect_with((), move |_| {
        let data = data.clone();
        let performance = window()
            .and_then(|w| w.performance())
            .expect("performance should be available");

        performance.mark("data-gen-start").ok();

        let rows = (1..=1_000_000)
            .map(|i| {
                hashmap! {
                    "id" => i.to_string(),
                    "name" => format!("User {}", i),
                    "email" => format!("user{}@example.com", i),
                    "age" => (18 + (js_sys::Math::random() * 80.0) as u8).to_string(),
                    "registered" => js_sys::Date::new_0().to_string().into()
                }
            })
            .collect::<Vec<_>>();

        performance.mark("data-gen-end").ok();
        performance
            .measure_with_start_mark_and_end_mark(
                "Data Generation",
                "data-gen-start",
                "data-gen-end",
            )
            .ok();

        data.set(Some(rows));
        || ()
    });

    let columns = vec![
        Column {
            id: "id",
            header: "ID",
            sortable: true,
            ..Default::default()
        },
        Column {
            id: "name",
            header: "Name",
            sortable: true,
            ..Default::default()
        },
        Column {
            id: "email",
            header: "Email",
            sortable: true,
            ..Default::default()
        },
        Column {
            id: "age",
            header: "Age",
            sortable: true,
            ..Default::default()
        },
        Column {
            id: "registered",
            header: "Registered",
            sortable: true,
            ..Default::default()
        },
    ];

    html! {
        <div class="table-container">
            <h1>{ "Yew Table-RS Benchmark (1M rows)" }</h1>
            if loading {
                <p>{ "Loading table data..." }</p>
            } else {
                <Table
                    data={data_val.unwrap()}
                    columns={columns}
                    page_size={50}
                    search={true}
                    paginate={true}
                    loading={false}
                />
            }
        </div>
    }
}
