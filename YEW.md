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

| Property    | Type                                  | Description                                     | Default   |
| ----------- | ------------------------------------- | ----------------------------------------------- | --------- |
| `data`      | `Vec<HashMap<&'static str, String>>`  | The row data to be rendered in the table.       | `[]`      |
| `columns`   | `Vec<Column>`                         | List of column definitions.                     | `[]`      |
| `page_size` | `usize`                               | Number of rows per page.                        | `10`      |
| `loading`   | `bool`                                | Whether to show a loading state.                | `false`   |
| `paginate`  | `bool`                                | Enables pagination UI.                          | `false`   |
| `search`    | `bool`                                | Enables search input field.                     | `false`   |
| `classes`   | `TableClasses`                        | CSS class names for customization.              | See below |
| `styles`    | `HashMap<&'static str, &'static str>` | Inline styles for different parts of the table. | `{}`      |
| `texts`     | `TableTexts`                          | Customizable text labels for UI elements.       | See below |

### `Column` Props

| Property    | Type                   | Description                                              | Default                                                     |
| ----------- | ---------------------- | -------------------------------------------------------- | ----------------------------------------------------------- |
| `id`        | `&'static str`         | Key used to fetch data from row objects.                 | `""`                                                        |
| `header`    | `&'static str`         | Text shown in the table header.                          | `""`                                                        |
| `accessor`  | `Callback<()>`         | Optional callback for custom rendering or cell behavior. | `Callback::noop()`                                          |
| `sortable`  | `bool`                 | Whether this column can be sorted.                       | `false`                                                     |
| `min_width` | `u32`                  | Minimum width for the column in pixels.                  | `100`                                                       |
| `style`     | `Option<&'static str>` | Optional inline styles for the column header.            | `Some("padding: 8px; font-weight: 600; text-align: left;")` |
| `class`     | `Option<&'static str>` | Optional CSS class for the column header.                | `Some("table-header-cell")`                                 |

### `TableClasses` (Class Name Overrides)

| Property            | Type           | Description                            | Default                 |
| ------------------- | -------------- | -------------------------------------- | ----------------------- |
| `container`         | `&'static str` | Wrapper container for the whole table. | `"table-container"`     |
| `table`             | `&'static str` | The `<table>` element.                 | `"table"`               |
| `thead`             | `&'static str` | The `<thead>` element.                 | `"thead"`               |
| `tbody`             | `&'static str` | The `<tbody>` element.                 | `"tbody"`               |
| `pagination`        | `&'static str` | Pagination controls wrapper.           | `"pagination-controls"` |
| `search_input`      | `&'static str` | Class for the search input element.    | `"search-input"`        |
| `header_cell`       | `&'static str` | Class for table header cells (`<th>`). | `"th"`                  |
| `body_cell`         | `&'static str` | Class for table body cells (`<td>`).   | `"td"`                  |
| `row`               | `&'static str` | Class for rows (`<tr>`).               | `"tr"`                  |
| `loading_row`       | `&'static str` | Row shown during loading state.        | `"loading-row"`         |
| `empty_row`         | `&'static str` | Row shown when there's no data.        | `"empty-row"`           |
| `pagination_button` | `&'static str` | Class for pagination buttons.          | `"pagination-button"`   |

### `TableTexts` (UI Labels)

| Property             | Type           | Description                                 | Default                       |
| -------------------- | -------------- | ------------------------------------------- | ----------------------------- |
| `loading`            | `&'static str` | Text shown during loading state.            | `"Loading..."`                |
| `empty`              | `&'static str` | Text shown when no data matches the filter. | `"No results found"`          |
| `search_placeholder` | `&'static str` | Placeholder text for search input.          | `"Search..."`                 |
| `previous_button`    | `&'static str` | Label for the previous page button.         | `"Previous"`                  |
| `next_button`        | `&'static str` | Label for the next page button.             | `"Next"`                      |
| `page_indicator`     | `&'static str` | Format string for pagination text.          | `"Page {current} of {total}"` |

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

## 📊 Benchmark: TanStack Table vs Table RS

| Metric                          | TanStack Table (React) | Table RS (Yew + WASM) |
|--------------------------------|-----------------------------|----------------------------|
| **Page Load Time (1M rows)**   | ~10 seconds                 | ~2 seconds                 |
| **Memory Heap Usage**          | >3 GB (heap overflow)       | ~1.1 GB (stable)             |
| **Initial Rendering**          | Heavy blocking, slow DOM paint | Efficient, lightweight rendering |
| **Browser Responsiveness**     | Delayed interactivity      | Smooth after hydration     |
| **Sorting Performance**        | 2-4s for large columns     | Sub-1s due to WASM speed   |
| **Search Performance**         | Acceptable, but slower     | Instantaneous, even at scale |
| **Lighthouse Performance Score** | 49/100                    | 60/100                     |
| **Scalability**                | Limited due to memory and VDOM | Near-native scalability     |

### 🟨 TanStack Table (React)
- Uses Virtual DOM and JS heap to manage massive data.
- Runtime bottlenecks emerge with >100k rows.
- Memory allocation during sorting and filtering can spike to **3GB+**, often leading to **heap overflow** during intensive usage.
- Lighthouse audit shows poor TTI and CPU blocking.

### 🟩 Table RS (Yew + WASM)
- WASM-compiled logic is highly memory-efficient and deterministic.
- DOM rendering is direct, bypassing React's reconciliation.
- ~1.1 GB of memory heap used even with **1 million rows**.
- Built-in support for search/sort with stable paging.
- No hydration issues (client-only generation).
- Lighthouse performance significantly better, especially in CPU/Memory metrics.

For large-data UI benchmarks like tables with millions of rows, **`table-rs` in Yew/WASM is a superior choice** compared to React + TanStack.
