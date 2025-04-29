# 📊 Table RS Yew Usage

Adding Table RS to your project is simple:

1. Make sure your project is set up with **Yew**. Follow their [Getting Started Guide](https://yew.rs/docs/getting-started/introduction) for setup instructions.

1. Add the Table RS component to your dependencies by including it in your `Cargo.toml` file:

   ```sh
   cargo add table-rs --features=yew
   ```

1. Import the `Table` component into your Yew component and start using it in your app.

## 🛠️ Usage

Incorporating Table RS into your Yew application is easy. Follow these steps:

1. Import the `Table` component into your Yew project:

   ```rust
   use yew::prelude::*;
   use table_rs::yew::table::Table;
   use table_rs::yew::types::Column;
   use maplit::hashmap;
   ```

1. Use the `Table` component within your Yew application:

   ```rust
   use yew::prelude::*;
   use table_rs::yew::table::Table;
   use table_rs::yew::types::Column;
   use maplit::hashmap;


   #[function_component(App)]
   pub fn app() -> Html {
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

       html! {
           <Table data={data} columns={columns} />
       }
   }
   ```

## 🔧 Props

### `Table` Component Props

#### Main Props

| Property    | Type                                  | Description                                 | Default  |
| ----------- | ------------------------------------- | ------------------------------------------- | -------- |
| `data`      | `Vec<HashMap<&'static str, String>>`  | The row data to be rendered in the table.   | `[]`     |
| `columns`   | `Vec<Column>`                         | List of column definitions.                 | `[]`     |
| `page_size` | `usize`                               | Number of rows per page.                    | `10`     |
| `loading`   | `bool`                                | Whether to show a loading state.            | `false`  |
| `paginate`  | `bool`                                | Enables pagination UI.                      | `false`  |
| `search`    | `bool`                                | Enables search input field.                 | `false`  |
| `classes`   | `TableClasses`                        | CSS class names for customization.          | Defaults |
| `styles`    | `HashMap<&'static str, &'static str>` | Inline styles for different parts of table. | `{}`     |

#### Column Props

| Property   | Type                   | Description                          | Default |
| ---------- | ---------------------- | ------------------------------------ | ------- |
| `id`       | `&'static str`         | Key used to fetch data for the cell. | `""`    |
| `header`   | `&'static str`         | Column header text.                  | `""`    |
| `sortable` | `bool`                 | Whether the column is sortable.      | `false` |
| `class`    | `Option<&'static str>` | Optional CSS class for the column.   | `None`  |
| `style`    | `Option<&'static str>` | Optional inline style.               | `None`  |

#### Style Props

```sh
+-------------------------------------------------------------+
|                      [container]                            |  <-- class: "table-container"
|                                                             |
|   +-----------------------------------------------------+   |
|   |                    [search_input]                   |   |  <-- class: "search-input"
|   |          (optional search <input> element)          |   |
|   +-----------------------------------------------------+   |
|                                                             |
|   +-----------------------------------------------------+   |
|   |                       [table]                       |   |  <-- class: "table"
|   |   +--------------------[thead]--------------------+ |   |  <-- class: "thead"
|   |   |   Column Headers (e.g., Name, Email)          | |   |
|   |   +-----------------------------------------------+ |   |
|   |   +--------------------[tbody]-------------------0+ |   |  <-- class: "tbody"
|   |   |  Data rows (from `data` prop, each row = <tr>)| |   |
|   |   +---------------------------------------------- + |   |
|   +-----------------------------------------------------+   |
|                                                             |
|   +-----------------------------------------------------+   |
|   |                  [pagination]                       |   |  <-- class: "pagination-controls"
|   |   Page selector / next-prev buttons (if enabled)    |   |
|   +-----------------------------------------------------+   |
+-------------------------------------------------------------+
```

| Property       | Type           | Description                             | Default                 |
| -------------- | -------------- | --------------------------------------- | ----------------------- |
| `container`    | `&'static str` | Wrapper container class.                | `"table-container"`     |
| `table`        | `&'static str` | The `<table>` element class.            | `"table"`               |
| `thead`        | `&'static str` | The `<thead>` class.                    | `"thead"`               |
| `tbody`        | `&'static str` | The `<tbody>` class.                    | `"tbody"`               |
| `pagination`   | `&'static str` | Pagination controls wrapper class.      | `"pagination-controls"` |
| `search_input` | `&'static str` | Class for the search `<input>` element. | `"search-input"`        |

## 💡 Notes

- The `data` must match the `id` values defined in each `Column`.
- The `search` prop enables URL-synced filtering across all columns.
- Pagination is controlled using the `page_size` and `paginate` props.
- Column sorting is supported with `sortable = true` and visually indicated via `aria-sort`.
- You can override any table styles or classes via the `styles` and `classes` props.
- The component supports custom loading and empty states out of the box.
- Each column can define its own style and class for deeper customization.
- URL search parameters like `?search=term` can be used for search input hydration.
- The table body will show "Loading..." or "No results found" automatically.
