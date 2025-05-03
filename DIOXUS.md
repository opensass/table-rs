# 🧬 Table RS Dioxus Usage

Adding Table RS to your project is simple:

1. Make sure your project is set up with **Dioxus**. Refer to the [Dioxus Getting Started Guide](https://dioxuslabs.com/learn/0.6/getting_started) for setup instructions.

1. Add the **table-rs** library to your dependencies by including it in your `Cargo.toml` file:

   ```sh
   cargo add table-rs --features=dio
   ```

1. Import the `Table` component into your Dioxus application.

## 🛠️ Usage

Incorporating Table RS into your Dioxus app involves just a few steps:

1. Import the `Table` component and types:

   ```rust
   use dioxus::prelude::*;
   use table_rs::dioxus::table::Table;
   use table_rs::dioxus::types::Column;
   use maplit::hashmap;
   ```

1. Use the `Table` component in your Dioxus app:

   ```rust
   use dioxus::prelude::*;
   use table_rs::dioxus::table::Table;
   use table_rs::dioxus::types::Column;
   use maplit::hashmap;


   fn App() -> Element {
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
           Table {
               data: data,
               columns: columns,
           }
       }
   }
   ```

## 🔧 Props

### `Table` Component Props

| Prop        | Type                                  | Description                       | Default |
| ----------- | ------------------------------------- | --------------------------------- | ------- |
| `data`      | `Vec<HashMap<&'static str, String>>`  | The row data to render.           | `[]`    |
| `columns`   | `Vec<Column>`                         | Column definitions.               | `[]`    |
| `page_size` | `usize`                               | Number of rows per page.          | `10`    |
| `loading`   | `bool`                                | Show loading state if true.       | `false` |
| `paginate`  | `bool`                                | Enable pagination.                | `false` |
| `search`    | `bool`                                | Enable global search input.       | `false` |
| `classes`   | `TableClasses`                        | CSS class overrides.              | Default |
| `styles`    | `HashMap<&'static str, &'static str>` | Inline style overrides.           | `{}`    |
| `texts`     | `TableTexts`                          | Text customization for UI labels. | Default |

### `Column` Props

| Prop       | Type                   | Description                               | Default                                                   |
| ---------- | ---------------------- | ----------------------------------------- | --------------------------------------------------------- |
| `id`       | `&'static str`         | Column key (used to fetch from row data). | `""`                                                      |
| `header`   | `&'static str`         | Display name in the table header.         | `""`                                                      |
| `sortable` | `bool`                 | Allow sorting on this column.             | `false`                                                   |
| `style`    | `Option<&'static str>` | Inline CSS for the header.                | Some("padding: 8px; font-weight: 600; text-align: left;") |
| `class`    | `Option<&'static str>` | Optional class name for this column.      | Some("table-header-cell")                                 |

### `TableClasses`

| Prop                | Type           | Description                          | Default                 |
| ------------------- | -------------- | ------------------------------------ | ----------------------- |
| `container`         | `&'static str` | Outer container class.               | `"table-container"`     |
| `table`             | `&'static str` | Main table class.                    | `"table"`               |
| `thead`             | `&'static str` | Table head (`<thead>`) class.        | `"thead"`               |
| `tbody`             | `&'static str` | Table body (`<tbody>`) class.        | `"tbody"`               |
| `row`               | `&'static str` | Row (`<tr>`) class.                  | `"tr"`                  |
| `header_cell`       | `&'static str` | Header cell (`<th>`) class.          | `"th"`                  |
| `body_cell`         | `&'static str` | Body cell (`<td>`) class.            | `"td"`                  |
| `loading_row`       | `&'static str` | Row shown when loading.              | `"loading-row"`         |
| `empty_row`         | `&'static str` | Row shown when no data is available. | `"empty-row"`           |
| `search_input`      | `&'static str` | Search input field class.            | `"search-input"`        |
| `pagination`        | `&'static str` | Pagination controls wrapper.         | `"pagination-controls"` |
| `pagination_button` | `&'static str` | Pagination buttons.                  | `"pagination-button"`   |

### `TableTexts`

| Prop                 | Type           | Description                       | Default                       |
| -------------------- | -------------- | --------------------------------- | ----------------------------- |
| `loading`            | `&'static str` | Text shown when loading.          | `"Loading..."`                |
| `empty`              | `&'static str` | Text when no data is present.     | `"No results found"`          |
| `search_placeholder` | `&'static str` | Placeholder for search input.     | `"Search..."`                 |
| `previous_button`    | `&'static str` | Label for previous page button.   | `"Previous"`                  |
| `next_button`        | `&'static str` | Label for next page button.       | `"Next"`                      |
| `page_indicator`     | `&'static str` | Format string for page indicator. | `"Page {current} of {total}"` |

### 🧱 Style/Layout Structure

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
|   |   +--------------------[tbody]--------------------+ |   |  <-- class: "tbody"
|   |   |  Data rows (from `data` prop, each row = <tr>)| |   |
|   |   +-----------------------------------------------+ |   |
|   +-----------------------------------------------------+   |
|                                                             |
|   +-----------------------------------------------------+   |
|   |                  [pagination]                       |   |  <-- class: "pagination-controls"
|   |   Page selector / next-prev buttons (if enabled)    |   |
|   +-----------------------------------------------------+   |
+-------------------------------------------------------------+
```

## 💡 Notes

- The `data` must match the `id` values defined in each `Column`.
- The `search` prop enables input-based filtering across all columns.
- Pagination is controlled using the `page_size` and `paginate` props.
- Sorting is column-specific via `sortable = true` and `on_sort_column`.
- All style classes can be customized via `TableClasses`.
- All texts are configurable via `TableTexts`.
- The component handles loading and empty states out-of-the-box.
- You can inject additional per-column styling via `Column.style` and `Column.class`.
