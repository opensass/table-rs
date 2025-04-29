<div align="center">

# 📋 Table RS

[![Crates.io](https://img.shields.io/crates/v/table-rs)](https://crates.io/crates/table-rs)
[![Crates.io Downloads](https://img.shields.io/crates/d/table-rs)](https://crates.io/crates/table-rs)
![Crates.io License](https://img.shields.io/crates/l/table-rs)
[![Made with Rust](https://img.shields.io/badge/Made%20with-Rust-1f425f.svg?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Rust](https://img.shields.io/badge/Rust-1.85%2B-blue.svg)](https://www.rust-lang.org)
[![Maintenance](https://img.shields.io/badge/Maintained%3F-yes-green.svg)](https://github.com/wiseaidev)

[![Join our Discord](https://dcbadge.limes.pink/api/server/b5JbvHW5nv)](https://discord.gg/b5JbvHW5nv)

<!-- absolute url for docs.rs cause assets is excluded from crate -->
![logo](https://raw.githubusercontent.com/opensass/table-rs/refs/heads/main/assets/logo.webp)

</div>

## 🎬 Demo

| Framework | Live Demo                                                                                                                |
| --------- | ------------------------------------------------------------------------------------------------------------------------ |
| Yew       | [![Netlify Status](https://api.netlify.com/api/v1/badges/4e1494d6-c19a-4a4c-b2d3-47d964214a71/deploy-status)](https://table-rs.netlify.app) |
| Dioxus    | TODO                                                                                                                     |
| Leptos    | TODO                                                                                                                     |

## 📜 Intro

**Table RS** is a **powerful**, **lightweight**, and **accessible** data table component designed specifically for **WASM frontends** like **Yew**, **Dioxus**, and **Leptos**. It supports sorting, pagination, searching (with URL sync), and is highly customizable via props.

## 🤔 Why Use Table RS?

The following are some of the reasons why **Table RS** should be your go-to table component:

1. **🔍 Built-in Search**: Real-time filtering with URL query parameter synchronization.
1. **🧹 Debounced Inputs**: Smooth user experience while searching.
1. **⬆️ Sorting Support**: Column-based sorting with accessible `aria-sort` attributes.
1. **📄 Pagination**: Built-in pagination controls for large datasets.
1. **🎨 Custom Styling**: Easily customize classes and styles.
1. **🧩 Accessibility First**: Semantic roles and ARIA attributes out of the box.

## Yew Usage

<!-- absolute url for docs.rs cause YEW.md is not included in crate -->
Refer to [our guide](https://github.com/opensass/table-rs/blob/main/YEW.md) to integrate this component into your Yew app.

## 🧬 Dioxus Usage (TODO)

<!-- absolute url for docs.rs cause DIOXUS.md is not included in crate -->
Refer to [our guide](https://github.com/opensass/table-rs/blob/main/DIOXUS.md) to integrate this component into your Dioxus app.

## 🌱 Leptos Usage (TODO)

<!-- absolute url for docs.rs cause LEPTOS.md is not included in crate -->
Refer to [our guide](https://github.com/opensass/table-rs/blob/main/LEPTOS.md) to integrate this component into your Leptos app.

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

## 🤝 Contributions

Contributions are welcome! Whether it's bug fixes, feature requests, or examples, we would love your help to make **Table RS** even better.

1. Fork the repository.
1. Create a new branch for your feature/bugfix.
1. Submit a pull request for review.

## 📜 License

**Table RS** is licensed under the [MIT License](LICENSE). You are free to use, modify, and distribute this library in your projects.
