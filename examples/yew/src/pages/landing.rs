use maplit::hashmap;
use table_rs::yew::table::Table;
use table_rs::yew::types::{Column, TableClasses, TableTexts};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ExampleCardProps {
    pub title: &'static str,
    pub code: &'static str,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(ExampleCard)]
pub fn example_card(props: &ExampleCardProps) -> Html {
    html! {
        <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
            <h2 class="text-xl font-bold mb-2">{ props.title }</h2>
            <pre
                class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto"
            >
                { props.code }
            </pre>
            { for props.children.iter() }
        </div>
    }
}

#[function_component(Example1)]
pub fn example1() -> Html {
    let data = vec![
        hashmap! { "name" => "Ferris".to_string(), "email" => "ferris@opensass.org".to_string() },
        hashmap! { "name" => "Ferros".to_string(), "email" => "ferros@opensass.org".to_string() },
        hashmap! { "name" => "Crab".to_string(), "email" => "crab@opensass.org".to_string() },
    ];

    let columns = vec![
        Column {
            id: "name",
            header: "Name",
            sortable: true,
            ..Default::default()
        },
        Column {
            id: "email",
            header: "Email",
            sortable: false,
            ..Default::default()
        },
    ];

    html! { <Table data={data} columns={columns} /> }
}

#[function_component(Example2)]
pub fn example2() -> Html {
    let data = (1..=50)
        .map(|i| {
            hashmap! {
                "name" => format!("Ferris {i}"),
                "email" => format!("ferris{i}@opensass.org")
            }
        })
        .collect::<Vec<_>>();

    let columns = vec![
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
    ];

    html! { <Table data={data} columns={columns} page_size=5 paginate=true search=true /> }
}

#[function_component(Example3)]
pub fn example3() -> Html {
    let data = vec![
        hashmap! { "name" => "Ferris".to_string(), "email" => "ferris@opensass.org".to_string() },
        hashmap! { "name" => "Ferros".to_string(), "email" => "ferros@opensass.org".to_string() },
    ];

    let columns = vec![
        Column {
            id: "name",
            header: "Name",
            sortable: false,
            ..Default::default()
        },
        Column {
            id: "email",
            header: "Email",
            sortable: false,
            ..Default::default()
        },
    ];

    let custom_classes = TableClasses {
        container: "custom-container",
        table: "custom-table",
        thead: "custom-thead",
        tbody: "custom-tbody",
        pagination: "custom-pagination",
        search_input: "custom-search-input",
        ..Default::default()
    };

    html! { <Table data={data} columns={columns} classes={custom_classes} /> }
}

#[function_component(Example4)]
pub fn example4() -> Html {
    let data = vec![];

    let columns = vec![
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
    ];

    html! { <Table data={data} columns={columns} loading=true /> }
}

#[function_component(Example5)]
pub fn example5() -> Html {
    let data = (0..=20)
        .map(|i| {
            hashmap! {
                "name" => format!("Ferris {i}"),
                "email" => format!("ferris{i}@opensass.org")
            }
        })
        .collect::<Vec<_>>();

    let columns = vec![
        Column {
            id: "name",
            header: "Name",
            sortable: true,
            class: Some("text-blue-500"),
            style: Some("min-width: 200px;"),
            ..Default::default()
        },
        Column {
            id: "email",
            header: "Email",
            sortable: true,
            class: Some("text-red-500"),
            style: Some("min-width: 300px;"),
            ..Default::default()
        },
    ];

    html! { <Table data={data} columns={columns} /> }
}

#[function_component(Example6)]
pub fn example6() -> Html {
    let data = (1..=10)
        .map(|i| {
            hashmap! {
                "name" => format!("Ferris {i}"),
                "email" => format!("ferris{i}@opensass.org")
            }
        })
        .collect::<Vec<_>>();

    let columns = vec![
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
    ];

    let custom_classes = TableClasses {
        tbody: "divide-y divide-gray-200 odd:bg-gray-100 even:bg-white hover:bg-gray-200",
        ..Default::default()
    };

    html! { <Table data={data} columns={columns} classes={custom_classes} /> }
}

#[function_component(Example7)]
pub fn example7() -> Html {
    let data = (1..=10)
        .map(|i| {
            hashmap! {
                "name" => format!("Ferris {i}"),
                "email" => format!("ferris{i}@opensass.org")
            }
        })
        .collect::<Vec<_>>();

    let columns = vec![
        Column {
            id: "name",
            header: "Sticky Name",
            sortable: true,
            ..Default::default()
        },
        Column {
            id: "email",
            header: "Sticky Email",
            sortable: false,
            ..Default::default()
        },
    ];

    let custom_classes = TableClasses {
        thead: "bg-white sticky top-0 shadow",
        ..Default::default()
    };

    html! {
        <div class="h-64 overflow-y-auto">
            <Table data={data} columns={columns} classes={custom_classes} />
        </div>
    }
}

#[function_component(Example8)]
pub fn example8() -> Html {
    let data = (1..=5)
        .map(|i| {
            hashmap! {
                "name" => format!("Ferris {i}"),
                "email" => format!("ferris{i}@opensass.org")
            }
        })
        .collect::<Vec<_>>();

    let columns = vec![
        Column {
            id: "name",
            header: "Name",
            sortable: false,
            ..Default::default()
        },
        Column {
            id: "email",
            header: "Email",
            sortable: false,
            ..Default::default()
        },
    ];

    html! { <Table data={data} columns={columns} search=true /> }
}

#[function_component(Example9)]
pub fn example9() -> Html {
    let data = (1..=30)
        .map(|i| {
            hashmap! {
                "name" => format!("Ferris {i}"),
                "email" => format!("ferris{i}@eopensass.org")
            }
        })
        .collect::<Vec<_>>();

    let columns = vec![
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
    ];

    let custom_classes = TableClasses {
        search_input: "border border-gray-400 rounded px-3 py-1 focus:outline-none focus:ring focus:border-blue-300",
        ..Default::default()
    };

    html! {
        <Table
            data={data}
            columns={columns}
            search=true
            paginate=true
            page_size=5
            classes={custom_classes}
        />
    }
}

#[function_component(Example10)]
pub fn example10() -> Html {
    let data = (1..=40)
        .map(|i| {
            hashmap! {
                "name" => format!("Crab {i}"),
                "email" => format!("crab{i}@opensass.org")
            }
        })
        .collect::<Vec<_>>();

    let columns = vec![
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
    ];

    let custom_classes = TableClasses {
        table: "text-xs",
        tbody: "divide-y divide-gray-300",
        ..Default::default()
    };

    html! {
        <Table data={data} columns={columns} paginate=true page_size=10 classes={custom_classes} />
    }
}

#[function_component(Example11)]
pub fn example11() -> Html {
    let data = (1..=100)
        .map(|i| {
            hashmap! {
                "name" => format!("Crab {i}"),
                "email" => format!("crab{i}@opensass.org")
            }
        })
        .collect::<Vec<_>>();

    let columns = vec![
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
    ];

    html! { <Table data={data} columns={columns} paginate=true page_size=20 search=true /> }
}

#[function_component(Example12)]
pub fn example12() -> Html {
    let data = vec![
        hashmap! { "name" => "Ferris".to_string(), "email" => "ferris@opensass.org".to_string() },
        hashmap! { "name" => "Ferros".to_string(), "email" => "ferros@opensass.org".to_string() },
        hashmap! { "name" => "Crab".to_string(), "email" => "crab@opensass.org".to_string() },
        hashmap! { "name" => "CrabFerris".to_string(), "email" => "crabferris@opensass.org".to_string() },
    ];

    let columns = vec![
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
    ];

    let custom_classes = TableClasses {
        table: "min-w-full table-auto text-sm text-left text-gray-700",
        thead: "bg-blue-600 text-white",
        tbody: "bg-white",
        pagination: "flex justify-between items-center p-4",
        search_input: "px-3 py-2 rounded-md border border-gray-300",
        ..Default::default()
    };

    html! {
        <div class="overflow-x-auto bg-gray-100 p-4 rounded-lg shadow-md">
            <Table data={data} columns={columns} classes={custom_classes} />
        </div>
    }
}

#[function_component(Example13)]
pub fn example13() -> Html {
    let data = vec![
        hashmap! { "name" => "Ferris".to_string(), "email" => "ferris@opensass.org".to_string() },
        hashmap! { "name" => "Ferros".to_string(), "email" => "ferros@opensass.org".to_string() },
        hashmap! { "name" => "Crab".to_string(), "email" => "crab@opensass.org".to_string() },
        hashmap! { "name" => "CrabFerris".to_string(), "email" => "crabferris@opensass.org".to_string() },
    ];

    let columns = vec![
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
    ];

    let classes = TableClasses {
        container: "w-full max-w-5xl mx-auto mt-10",
        table: "min-w-full bg-white shadow-md rounded-lg overflow-hidden",
        thead: "bg-gray-100 text-gray-700 uppercase text-sm",
        tbody: "divide-y divide-gray-200",
        search_input: "mb-4 p-2 border rounded w-full max-w-xs",
        row: "hover:bg-gray-50",
        header_cell: "px-4 py-3",
        body_cell: "px-4 py-3 text-gray-900",
        loading_row: "text-center py-4",
        empty_row: "text-center py-4 text-gray-500",
        pagination: "flex justify-between items-center mt-4",
        pagination_button:
            "px-4 py-2 text-sm text-white bg-blue-500 rounded hover:bg-blue-600 disabled:opacity-50",
    };

    let styles = hashmap! {
        "table" => "border-collapse: collapse; width: 100%;",
    };

    let texts = TableTexts {
        loading: "Loading data...",
        empty: "No entries match your search.",
        search_placeholder: "Search by name or email...",
        previous_button: "← Previous",
        next_button: "Next →",
        page_indicator: "Page {current} of {total}",
    };

    html! {
        <Table
            data={data}
            columns={columns}
            page_size={5}
            loading={false}
            classes={classes}
            styles={styles}
            paginate={true}
            search={true}
            texts={texts}
        />
    }
}

#[function_component(LandingPage)]
pub fn landing_page() -> Html {
    html! {
        <div class="m-6 min-h-screen flex flex-col items-center justify-center">
            <h1 class="text-3xl font-bold mb-8 text-white">{ "Table RS Yew Examples" }</h1>
            <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-8">
                <ExampleCard
                    title="Basic Table"
                    code=r#"use yew::prelude::*;
use maplit::hashmap;
use table_rs::yew::table::Table;
use table_rs::yew::types::Column;

#[function_component(Example1)]
pub fn example1() -> Html {
    let data = vec![
        hashmap! { "name" => "Ferris".to_string(), "email" => "ferris@opensass.org".to_string() },
        hashmap! { "name" => "Ferros".to_string(), "email" => "ferros@opensass.org".to_string() },
        hashmap! { "name" => "Crab".to_string(), "email" => "crab@opensass.org".to_string() },
    ];

    let columns = vec![
        Column {
            id: "name",
            header: "Name",
            sortable: true,
            ..Default::default()
        },
        Column {
            id: "email",
            header: "Email",
            sortable: false,
            ..Default::default()
        },
    ];

    html! { <Table data={data} columns={columns} /> }
}"#
                >
                    <Example1 />
                </ExampleCard>
                <ExampleCard
                    title="Pagination & Search Table"
                    code=r#"use yew::prelude::*;
use maplit::hashmap;
use table_rs::yew::table::Table;
use table_rs::yew::types::Column;

#[function_component(Example2)]
pub fn example2() -> Html {
    let data = (1..=50)
        .map(|i| {
            hashmap! {
                "name" => format!("Ferris {i}"),
                "email" => format!("ferris{i}@opensass.org")
            }
        })
        .collect::<Vec<_>>();

    let columns = vec![
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
    ];

    html! {
        <Table data={data} columns={columns} page_size={5} paginate={true} search={true} />
    }
}"#
                >
                    <Example2 />
                </ExampleCard>
                <ExampleCard
                    title="Custom Classes Table"
                    code=r#"use yew::prelude::*;
use maplit::hashmap;
use table_rs::yew::table::Table;
use table_rs::yew::types::Column;

#[function_component(Example3)]
pub fn example3() -> Html {
    let data = vec![
        hashmap! { "name" => "Ferris".to_string(), "email" => "ferris@opensass.org".to_string() },
        hashmap! { "name" => "Ferros".to_string(), "email" => "ferros@opensass.org".to_string() },
    ];

    let columns = vec![
        Column {
            id: "name",
            header: "Name",
            sortable: false,
            ..Default::default()
        },
        Column {
            id: "email",
            header: "Email",
            sortable: false,
            ..Default::default()
        },
    ];

    let custom_classes = TableClasses {
        container: "custom-container",
        table: "custom-table",
        thead: "custom-thead",
        tbody: "custom-tbody",
        pagination: "custom-pagination",
        search_input: "custom-search-input",
    };

    html! {
        <Table data={data} columns={columns} classes={custom_classes} />
    }
}"#
                >
                    <Example3 />
                </ExampleCard>
                <ExampleCard
                    title="Loading State Table"
                    code=r#"use yew::prelude::*;
use table_rs::yew::table::Table;
use table_rs::yew::types::Column;

#[function_component(Example4)]
pub fn example4() -> Html {
    let data = vec![];

    let columns = vec![
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
    ];

    html! { <Table data={data} columns={columns} loading={true} /> }
}"#
                >
                    <Example4 />
                </ExampleCard>
                <ExampleCard
                    title="Custom Column Style"
                    code=r#"use yew::prelude::*;
use maplit::hashmap;
use table_rs::yew::table::Table;
use table_rs::yew::types::Column;

#[function_component(Example5)]
pub fn example5() -> Html {
    let data = (0..=20)
        .map(|i| {
            hashmap! {
                "name" => format!("Ferris {i}"),
                "email" => format!("ferris{i}@opensass.org")
            }
        })
        .collect::<Vec<_>>();

    let columns = vec![
        Column {
            id: "name",
            header: "Name",
            sortable: true,
            class: Some("text-blue-500"),
            style: Some("min-width: 200px;"),
            ..Default::default()
        },
        Column {
            id: "email",
            header: "Email",
            sortable: true,
            class: Some("text-red-500"),
            style: Some("min-width: 300px;"),
            ..Default::default()
        },
    ];

    html! { <Table data={data} columns={columns} /> }
}"#
                >
                    <Example5 />
                </ExampleCard>
                <ExampleCard
                    title="Custom Row Styles"
                    code=r#"use yew::prelude::*;
use maplit::hashmap;
use table_rs::yew::table::Table;
use table_rs::yew::types::Column;

#[function_component(Example6)]
pub fn example6() -> Html {
    let data = (1..=10)
        .map(|i| {
            hashmap! {
                "name" => format!("Ferris {i}"),
                "email" => format!("ferris{i}@opensass.org")
            }
        })
        .collect::<Vec<_>>();

    let columns = vec![
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
    ];

    let custom_classes = TableClasses {
        tbody: "divide-y divide-gray-200 odd:bg-gray-100 even:bg-white hover:bg-gray-200",
        ..Default::default()
    };

    html! { <Table data={data} columns={columns} classes={custom_classes} /> }
}"#
                >
                    <Example6 />
                </ExampleCard>
                <ExampleCard
                    title="Sticky Header Table"
                    code=r#"use yew::prelude::*;
use maplit::hashmap;
use table_rs::yew::table::Table;
use table_rs::yew::types::Column;

#[function_component(Example7)]
pub fn example7() -> Html {
    let data = (1..=10)
        .map(|i| {
            hashmap! {
                "name" => format!("Ferris {i}"),
                "email" => format!("ferris{i}@opensass.org")
            }
        })
        .collect::<Vec<_>>();

    let columns = vec![
        Column {
            id: "name",
            header: "Sticky Name",
            sortable: true,
            ..Default::default()
        },
        Column {
            id: "email",
            header: "Sticky Email",
            sortable: false,
            ..Default::default()
        },
    ];

    let custom_classes = TableClasses {
        thead: "bg-white sticky top-0 shadow",
        ..Default::default()
    };

    html! {
        <div class="h-64 overflow-y-auto">
            <Table data={data} columns={columns} classes={custom_classes} />
        </div>
    }
}"#
                >
                    <Example7 />
                </ExampleCard>
                <ExampleCard
                    title="Searchable Table"
                    code=r#"use yew::prelude::*;
use maplit::hashmap;
use table_rs::yew::table::Table;
use table_rs::yew::types::Column;

#[function_component(Example8)]
pub fn example8() -> Html {
    let data = (1..=5)
        .map(|i| {
            hashmap! {
                "name" => format!("Ferris {i}"),
                "email" => format!("ferris{i}@opensass.org")
            }
        })
        .collect::<Vec<_>>();

    let columns = vec![
        Column {
            id: "name",
            header: "Name",
            sortable: false,
            ..Default::default()
        },
        Column {
            id: "email",
            header: "Email",
            sortable: false,
            ..Default::default()
        },
    ];

    html! { <Table data={data} columns={columns} search={true} /> }
}"#
                >
                    <Example8 />
                </ExampleCard>
                <ExampleCard
                    title="Paginated & Searchable Table"
                    code=r#"use yew::prelude::*;
use maplit::hashmap;
use table_rs::yew::table::Table;
use table_rs::yew::types::Column;

#[function_component(Example9)]
pub fn example9() -> Html {
    let data = (1..=30)
        .map(|i| {
            hashmap! {
                "name" => format!("Ferris {i}"),
                "email" => format!("ferris{i}@eopensass.org")
            }
        })
        .collect::<Vec<_>>();

    let columns = vec![
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
    ];

    let custom_classes = TableClasses {
        search_input: "border border-gray-400 rounded px-3 py-1 focus:outline-none focus:ring focus:border-blue-300",
        ..Default::default()
    };

    html! {
        <Table
            data={data}
            columns={columns}
            search={true}
            paginate={true}
            page_size={5}
            classes={custom_classes}
        />
    }
}"#
                >
                    <Example9 />
                </ExampleCard>
                <ExampleCard
                    title="Crab Table"
                    code=r#"use yew::prelude::*;
use maplit::hashmap;
use table_rs::yew::table::Table;
use table_rs::yew::types::Column;

#[function_component(Example10)]
pub fn example10() -> Html {
    let data = (1..=40)
        .map(|i| {
            hashmap! {
                "name" => format!("Crab {i}"),
                "email" => format!("crab{i}@opensass.org")
            }
        })
        .collect::<Vec<_>>();

    let columns = vec![
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
    ];

    let custom_classes = TableClasses {
        table: "text-xs",
        tbody: "divide-y divide-gray-300",
        ..Default::default()
    };

    html! {
        <Table
            data={data}
            columns={columns}
            paginate={true}
            page_size={10}
            classes={custom_classes}
        />
    }
}"#
                >
                    <Example10 />
                </ExampleCard>
                <ExampleCard
                    title="Large Paginated Table"
                    code=r#"use yew::prelude::*;
use maplit::hashmap;
use table_rs::yew::table::Table;
use table_rs::yew::types::Column;

#[function_component(Example11)]
pub fn example11() -> Html {
    let data = (1..=100)
        .map(|i| {
            hashmap! {
                "name" => format!("Crab {i}"),
                "email" => format!("crab{i}@opensass.org")
            }
        })
        .collect::<Vec<_>>();

    let columns = vec![
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
    ];

    html! {
        <Table
            data={data}
            columns={columns}
            paginate={true}
            page_size={20}
            search={true}
        />
    }
}"#
                >
                    <Example11 />
                </ExampleCard>
                <ExampleCard
                    title="Tailwind CSS Custom Table"
                    code=r#"use yew::prelude::*;
use maplit::hashmap;
use table_rs::yew::table::Table;
use table_rs::yew::types::Column;

#[function_component(Example12)]
pub fn example12() -> Html {
    let data = vec![
        hashmap! { "name" => "Ferris".to_string(), "email" => "ferris@opensass.org".to_string() },
        hashmap! { "name" => "Ferros".to_string(), "email" => "ferros@opensass.org".to_string() },
        hashmap! { "name" => "Crab".to_string(), "email" => "crab@opensass.org".to_string() },
        hashmap! { "name" => "CrabFerris".to_string(), "email" => "crabferris@opensass.org".to_string() },
    ];

    let columns = vec![
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
    ];

    let custom_classes = TableClasses {
        table: "min-w-full table-auto text-sm text-left text-gray-700",
        thead: "bg-blue-600 text-white",
        tbody: "bg-white",
        pagination: "flex justify-between items-center p-4",
        search_input: "px-3 py-2 rounded-md border border-gray-300",
        ..Default::default()
    };

    html! {
        <div class="overflow-x-auto bg-gray-100 p-4 rounded-lg shadow-md">
            <Table data={data} columns={columns} classes={custom_classes} />
        </div>
    }
}"#
                >
                    <Example12 />
                </ExampleCard>
                <ExampleCard
                    title="Tailwind CSS Custom Table"
                    code=r#"use yew::prelude::*;
use maplit::hashmap;
use table_rs::yew::table::Table;
use table_rs::yew::types::Column;
use table_rs::yew::types::TableTexts;
use table_rs::yew::types::TableClasses;

#[function_component(Example13)]
pub fn example13() -> Html {
    let data = vec![
        hashmap! { "name" => "Ferris".to_string(), "email" => "ferris@opensass.org".to_string() },
        hashmap! { "name" => "Ferros".to_string(), "email" => "ferros@opensass.org".to_string() },
        hashmap! { "name" => "Crab".to_string(), "email" => "crab@opensass.org".to_string() },
        hashmap! { "name" => "CrabFerris".to_string(), "email" => "crabferris@opensass.org".to_string() },
    ];

    let columns = vec![
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
    ];

    let classes = TableClasses {
        container: "w-full max-w-5xl mx-auto mt-10",
        table: "min-w-full bg-white shadow-md rounded-lg overflow-hidden",
        thead: "bg-gray-100 text-gray-700 uppercase text-sm",
        tbody: "divide-y divide-gray-200",
        search_input: "mb-4 p-2 border rounded w-full max-w-xs",
        row: "hover:bg-gray-50",
        header_cell: "px-4 py-3",
        body_cell: "px-4 py-3 text-gray-900",
        loading_row: "text-center py-4",
        empty_row: "text-center py-4 text-gray-500",
        pagination: "flex justify-between items-center mt-4",
        pagination_button: "px-4 py-2 text-sm text-white bg-blue-500 rounded hover:bg-blue-600 disabled:opacity-50",
    };

    let styles = hashmap! {
        "table" => "border-collapse: collapse; width: 100%;",
    };

    let texts = TableTexts {
        loading: "Loading data...",
        empty: "No entries match your search.",
        search_placeholder: "Search by name or email...",
        previous_button: "← Previous",
        next_button: "Next →",
        page_indicator: "Page {current} of {total}",
    };

    html! {
        <Table
            data={data}
            columns={columns}
            page_size={5}
            loading={false}
            classes={classes}
            styles={styles}
            paginate={true}
            search={true}
            texts={texts}
        />
    }
}"#
                >
                    <Example13 />
                </ExampleCard>
            </div>
        </div>
    }
}
