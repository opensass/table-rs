use dioxus::prelude::*;
use dioxus_logger::tracing;
use maplit::hashmap;
use table_rs::dioxus::table::Table;
use table_rs::dioxus::types::{Column, TableClasses, TableTexts};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styles.css");

fn main() {
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    tracing::info!("starting app");
    launch(app);
}

fn app() -> Element {
    rsx! {
        document::Script { src: "https://kit.fontawesome.com/8f223ead6e.js" },
        document::Stylesheet { href: "https://unpkg.com/tailwindcss@2.2.19/dist/tailwind.min.css" },
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Examples {}
    }
}

#[component]
fn Examples() -> Element {
    rsx! {
        div { class: "m-6 min-h-screen flex flex-col items-center justify-center",
            h1 { class: "text-3xl font-bold mb-8 text-white", "Table RS Dioxus Examples" }
            div { class: "grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-8",
                div {
                    class: "flex flex-col items-center bg-gray-50 p-6 rounded-lg shadow-lg",
                    h2 { class: "text-xl font-semibold mb-4 text-gray-800", "Basic Table" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r##"use dioxus::prelude::*;
use maplit::hashmap;
use table_rs::dioxus::table::Table;
use table_rs::dioxus::types::{{Column, TableClasses, TableTexts}};


#[component]
fn Example1() -> Element {{
    let data = vec![
        hashmap! {{ "name" => "Ferris".to_string(), "email" => "ferris@opensass.org".to_string() }},
        hashmap! {{ "name" => "Ferros".to_string(), "email" => "ferros@opensass.org".to_string() }},
        hashmap! {{ "name" => "Crab".to_string(), "email" => "crab@opensass.org".to_string() }},
    ];

    let columns = vec![
        Column {{ id: "name", header: "Name", sortable: true, ..Default::default() }},
        Column {{ id: "email", header: "Email", sortable: false, ..Default::default() }},
    ];

    rsx! {{
        Table {{ data: data, columns: columns }}
    }}
}}"##
                    }
                    Example1 {}
                }

                div {
                    class: "flex flex-col items-center bg-gray-50 p-6 rounded-lg shadow-lg",
                    h2 { class: "text-xl font-semibold mb-4 text-gray-800", "Paginated Searchable Table" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r##"use dioxus::prelude::*;
use maplit::hashmap;
use table_rs::dioxus::table::Table;
use table_rs::dioxus::types::{{Column, TableClasses, TableTexts}};


#[component]
fn Example2() -> Element {{
    let data = (1..=50).map(|i| hashmap! {{
        "name" => format!("Ferris {{i}}"),
        "email" => format!("ferris{{i}}@opensass.org")
    }}).collect::<Vec<_>>();

    let columns = vec![
        Column {{ id: "name", header: "Name", sortable: true, ..Default::default() }},
        Column {{ id: "email", header: "Email", sortable: true, ..Default::default() }},
    ];

    rsx! {{
        Table {{
            data: data,
            columns: columns,
            page_size: 5,
            paginate: true,
            search: true
        }}
    }}
}}"##
                    }
                    Example2 {}
                }

                div {
                    class: "flex flex-col items-center bg-gray-50 p-6 rounded-lg shadow-lg",
                    h2 { class: "text-xl font-semibold mb-4 text-gray-800", "Custom Classes" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r##"use dioxus::prelude::*;
use maplit::hashmap;
use table_rs::dioxus::table::Table;
use table_rs::dioxus::types::{{Column, TableClasses, TableTexts}};


#[component]
fn Example3() -> Element {{
    let data = vec![
        hashmap! {{ "name" => "Ferris".to_string(), "email" => "ferris@opensass.org".to_string() }},
        hashmap! {{ "name" => "Ferros".to_string(), "email" => "ferros@opensass.org".to_string() }},
    ];

    let columns = vec![
        Column {{ id: "name", header: "Name", sortable: false, ..Default::default() }},
        Column {{ id: "email", header: "Email", sortable: false, ..Default::default() }},
    ];

    let custom_classes = TableClasses {{
        container: "custom-container",
        table: "custom-table",
        thead: "custom-thead",
        tbody: "custom-tbody",
        pagination: "custom-pagination",
        search_input: "custom-search-input",
        ..Default::default()
    }};

    rsx! {{
        Table {{
            data: data,
            columns: columns,
            classes: custom_classes
        }}
    }}
}}"##
                    }
                    Example3 {}
                }

                div {
                    class: "flex flex-col items-center bg-gray-50 p-6 rounded-lg shadow-lg",
                    h2 { class: "text-xl font-semibold mb-4 text-gray-800", "Loading State" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r##"use dioxus::prelude::*;
use maplit::hashmap;
use table_rs::dioxus::table::Table;
use table_rs::dioxus::types::{{Column, TableClasses, TableTexts}};


#[component]
fn Example4() -> Element {{
    let data = vec![];

    let columns = vec![
        Column {{ id: "name", header: "Name", sortable: true, ..Default::default() }},
        Column {{ id: "email", header: "Email", sortable: true, ..Default::default() }},
    ];

    rsx! {{
        Table {{
            data: data,
            columns: columns,
            loading: true
        }}
    }}
}}"##
                    }
                    Example4 {}
                }

                div {
                    class: "flex flex-col items-center bg-gray-50 p-6 rounded-lg shadow-lg",
                    h2 { class: "text-xl font-semibold mb-4 text-gray-800", "Styled Columns" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r##"use dioxus::prelude::*;
use maplit::hashmap;
use table_rs::dioxus::table::Table;
use table_rs::dioxus::types::{{Column, TableClasses, TableTexts}};


#[component]
fn Example5() -> Element {{
    let data = (0..=20).map(|i| hashmap! {{
        "name" => format!("Ferris {{i}}"),
        "email" => format!("ferris{{i}}@opensass.org")
    }}).collect::<Vec<_>>();

    let columns = vec![
        Column {{
            id: "name", header: "Name", sortable: true,
            class: Some("text-blue-500"), style: Some("min-width: 200px;"),
            ..Default::default()
        }},
        Column {{
            id: "email", header: "Email", sortable: true,
            class: Some("text-red-500"), style: Some("min-width: 300px;"),
            ..Default::default()
        }},
    ];

    rsx! {{
        Table {{ data: data, columns: columns }}
    }}
}}"##
                    }
                    Example5 {}
                }

                div {
                    class: "flex flex-col items-center bg-gray-50 p-6 rounded-lg shadow-lg",
                    h2 { class: "text-xl font-semibold mb-4 text-gray-800", "Striped Rows" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r##"use dioxus::prelude::*;
use maplit::hashmap;
use table_rs::dioxus::table::Table;
use table_rs::dioxus::types::{{Column, TableClasses, TableTexts}};


#[component]
fn Example6() -> Element {{
    let data = (1..=10).map(|i| hashmap! {{
        "name" => format!("Ferris {{i}}"),
        "email" => format!("ferris{{i}}@opensass.org")
    }}).collect::<Vec<_>>();

    let columns = vec![
        Column {{ id: "name", header: "Name", sortable: true, ..Default::default() }},
        Column {{ id: "email", header: "Email", sortable: true, ..Default::default() }},
    ];

    let custom_classes = TableClasses {{
        tbody: "divide-y divide-gray-200 odd:bg-gray-100 even:bg-white hover:bg-gray-200",
        ..Default::default()
    }};

    rsx! {{
        Table {{
            data: data,
            columns: columns,
            classes: custom_classes
        }}
    }}
}}"##
                    }
                    Example6 {}
                }

                div {
                    class: "flex flex-col items-center bg-gray-50 p-6 rounded-lg shadow-lg",
                    h2 { class: "text-xl font-semibold mb-4 text-gray-800", "Sticky Header" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r##"use dioxus::prelude::*;
use maplit::hashmap;
use table_rs::dioxus::table::Table;
use table_rs::dioxus::types::{{Column, TableClasses, TableTexts}};


#[component]
fn Example7() -> Element {{
    let data = (1..=10).map(|i| hashmap! {{
        "name" => format!("Ferris {{i}}"),
        "email" => format!("ferris{{i}}@opensass.org")
    }}).collect::<Vec<_>>();

    let columns = vec![
        Column {{ id: "name", header: "Sticky Name", sortable: true, ..Default::default() }},
        Column {{ id: "email", header: "Sticky Email", sortable: false, ..Default::default() }},
    ];

    let custom_classes = TableClasses {{
        thead: "bg-white sticky top-0 shadow",
        ..Default::default()
    }};

    rsx! {{
        div {{ class: "h-64 overflow-y-auto",
            Table {{
                data: data,
                columns: columns,
                classes: custom_classes
            }}
        }}
    }}
}}"##
                    }
                    Example7 {}
                }

                div {
                    class: "flex flex-col items-center bg-gray-50 p-6 rounded-lg shadow-lg",
                    h2 { class: "text-xl font-semibold mb-4 text-gray-800", "Search Only" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r##"use dioxus::prelude::*;
use maplit::hashmap;
use table_rs::dioxus::table::Table;
use table_rs::dioxus::types::{{Column, TableClasses, TableTexts}};


#[component]
fn Example8() -> Element {{
    let data = (1..=5).map(|i| hashmap! {{
        "name" => format!("Ferris {{i}}"),
        "email" => format!("ferris{{i}}@opensass.org")
    }}).collect::<Vec<_>>();

    let columns = vec![
        Column {{ id: "name", header: "Name", sortable: false, ..Default::default() }},
        Column {{ id: "email", header: "Email", sortable: false, ..Default::default() }},
    ];

    rsx! {{
        Table {{ data: data, columns: columns, search: true }}
    }}
}}"##
                    }
                    Example8 {}
                }

                div {
                    class: "flex flex-col items-center bg-gray-50 p-6 rounded-lg shadow-lg",
                    h2 { class: "text-xl font-semibold mb-4 text-gray-800", "Custom Text & Pagination" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r##"use dioxus::prelude::*;
use maplit::hashmap;
use table_rs::dioxus::table::Table;
use table_rs::dioxus::types::{{Column, TableClasses, TableTexts}};


#[component]
fn Example9() -> Element {{
    let data = (1..=3).map(|i| hashmap! {{
        "name" => format!("Crab {{i}}"),
        "email" => format!("crab{{i}}@shell.org")
    }}).collect::<Vec<_>>();

    let columns = vec![
        Column {{ id: "name", header: "Name", sortable: true, ..Default::default() }},
        Column {{ id: "email", header: "Email", sortable: true, ..Default::default() }},
    ];

    let texts = TableTexts {{
        search_placeholder: "Filter by name or email...",
        ..Default::default()
    }};

    rsx! {{
        Table {{
            data: data,
            columns: columns,
            page_size: 2,
            paginate: true,
            search: true,
            texts: texts
        }}
    }}
}}"##
                    }
                    Example9 {}
                }
                div {
                    class: "flex flex-col items-center bg-gray-50 p-6 rounded-lg shadow-lg",
                    h2 { class: "text-xl font-semibold mb-4 text-gray-800", "Custom Element" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r##"use dioxus::prelude::*;
use maplit::hashmap;
use table_rs::dioxus::table::Table;
use table_rs::dioxus::types::{{Column, TableClasses, TableTexts}};


#[component]
fn Example10() -> Element {{
    let data = vec![
        hashmap! {{
            "name" => "Ferris".to_string(),
            "email" => "ferris@opensass.org".to_string(),
            "role" => "admin".to_string()
        }},
        hashmap! {{
            "name" => "Rustacean".to_string(),
            "email" => "rust@opensass.org".to_string(),
            "role" => "member".to_string()
        }},
    ];

    let email_renderer = Callback::new(move |value: String| {{
        rsx! {{
            a {{ href: "mailto:{{value}}", "{{value}}" }}
        }}
    }});
    let role_renderer = Callback::new(move |value: String| {{
        if value == "admin" {{
            rsx!( span {{ "Admin" }} )
        }} else {{
            rsx!( span {{ "{{value}}" }} )
        }}
    }});

    let columns = vec![
        Column {{
            id: "name",
            header: "Name",
            ..Default::default()
        }},
        Column {{
            id: "email",
            header: "Email",
            cell: Some(email_renderer),
            ..Default::default()
        }},
        Column {{
            id: "role",
            header: "Role",
            cell: Some(role_renderer),
            ..Default::default()
        }},
    ];

    rsx! {{
            Table {{
                columns: columns,
                data: data,
                loading: false,
                classes: TableClasses::default(),
                texts: TableTexts::default(),
            }}
    }}
}}"##
                    }
                    Example10 {}
                }
            }
        }
    }
}

#[component]
fn Example1() -> Element {
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

    rsx! {
        Table { data: data, columns: columns }
    }
}

#[component]
fn Example2() -> Element {
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

    rsx! {
        Table {
            data: data,
            columns: columns,
            page_size: 5,
            paginate: true,
            search: true
        }
    }
}

#[component]
fn Example3() -> Element {
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

    rsx! {
        Table {
            data: data,
            columns: columns,
            classes: custom_classes
        }
    }
}

#[component]
fn Example4() -> Element {
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

    rsx! {
        Table {
            data: data,
            columns: columns,
            loading: true
        }
    }
}

#[component]
fn Example5() -> Element {
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

    rsx! {
        Table { data: data, columns: columns }
    }
}

#[component]
fn Example6() -> Element {
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

    rsx! {
        Table {
            data: data,
            columns: columns,
            classes: custom_classes
        }
    }
}

#[component]
fn Example7() -> Element {
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

    rsx! {
        div { class: "h-64 overflow-y-auto",
            Table {
                data: data,
                columns: columns,
                classes: custom_classes
            }
        }
    }
}

#[component]
fn Example8() -> Element {
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

    rsx! {
        Table { data: data, columns: columns, search: true }
    }
}

#[component]
fn Example9() -> Element {
    let data = (1..=3)
        .map(|i| {
            hashmap! {
                "name" => format!("Crab {i}"),
                "email" => format!("crab{i}@shell.org")
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

    let texts = TableTexts {
        search_placeholder: "Filter by name or email...",
        ..Default::default()
    };

    rsx! {
        Table {
            data: data,
            columns: columns,
            page_size: 2,
            paginate: true,
            search: true,
            texts: texts
        }
    }
}

#[component]
fn Example10() -> Element {
    let data = vec![
        hashmap! {
            "name" => "Ferris".to_string(),
            "email" => "ferris@opensass.org".to_string(),
            "role" => "admin".to_string()
        },
        hashmap! {
            "name" => "Rustacean".to_string(),
            "email" => "rust@opensass.org".to_string(),
            "role" => "member".to_string()
        },
    ];

    let email_renderer = Callback::new(move |value: String| {
        rsx! {
            a { href: "mailto:{value}", "{value}" }
        }
    });
    let role_renderer = Callback::new(move |value: String| {
        if value == "admin" {
            rsx!( span { "Admin" } )
        } else {
            rsx!( span { "{value}" } )
        }
    });

    let columns = vec![
        Column {
            id: "name",
            header: "Name",
            ..Default::default()
        },
        Column {
            id: "email",
            header: "Email",
            cell: Some(email_renderer),
            ..Default::default()
        },
        Column {
            id: "role",
            header: "Role",
            cell: Some(role_renderer),
            ..Default::default()
        },
    ];

    rsx! {
            Table {
                columns: columns,
                data: data,
                loading: false,
                classes: TableClasses::default(),
                texts: TableTexts::default(),
            }
    }
}
