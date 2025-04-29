"use client";

import React, { useMemo, useState, useRef, useEffect } from "react";
import {
  useReactTable,
  getCoreRowModel,
  getSortedRowModel,
  getFilteredRowModel,
  flexRender,
} from "@tanstack/react-table";

import { useVirtualizer } from "@tanstack/react-virtual";

const generateData = (count = 1_000_000) => {
  return Array.from({ length: count }, (_, i) => ({
    id: i + 1,
    name: `User ${i + 1}`,
    email: `user${i + 1}@example.com`,
    age: Math.floor(Math.random() * 80) + 18,
    registered: new Date(Date.now() - Math.random() * 1e12).toISOString(),
  }));
};

const Home = () => {
  const [data, setData] = useState([]);

  useEffect(() => {
    performance.mark("data-gen-start");
    const generated = generateData();
    setData(generated);
    performance.mark("data-gen-end");
    performance.measure("Data Generation", "data-gen-start", "data-gen-end");
  }, []);

  const [sorting, setSorting] = useState([]);
  const [globalFilter, setGlobalFilter] = useState("");

  const rerenderCount = useRef(0);
  rerenderCount.current++;

  const columns = useMemo(
    () => [
      {
        accessorKey: "id",
        header: "ID",
      },
      {
        accessorKey: "name",
        header: "Name",
      },
      {
        accessorKey: "email",
        header: "Email",
      },
      {
        accessorKey: "age",
        header: "Age",
      },
      {
        accessorKey: "registered",
        header: "Registered",
      },
    ],
    [],
  );

  const table = useReactTable({
    data,
    columns,
    state: {
      sorting,
      globalFilter,
    },
    onSortingChange: (updater) => {
      performance.mark("sort-start");
      setSorting(updater);
    },
    getCoreRowModel: getCoreRowModel(),
    getSortedRowModel: getSortedRowModel(),
    getFilteredRowModel: getFilteredRowModel(),
  });

  const parentRef = useRef(null);
  const rowVirtualizer = useVirtualizer({
    count: table.getRowModel().rows.length,
    getScrollElement: () => parentRef.current,
    estimateSize: () => 35,
    overscan: 20,
  });

  useEffect(() => {
    performance.mark("render-end");

    try {
      performance.measure("Sort Duration", "sort-start", "render-end");
      performance.measure("Search Duration", "search-start", "render-end");
    } catch (e) {
    }
  });

  const handleSearchChange = (e) => {
    performance.mark("search-start");
    setGlobalFilter(e.target.value);
  };
  if (data.length === 0) {
    return <div>Loading table data...</div>;
  }

  return (
    <div>
      <h1>TanStack Table Benchmark (1M rows)</h1>
      <p>Re-renders: {rerenderCount.current}</p>
      <input
        type="text"
        placeholder="Search..."
        value={globalFilter}
        onChange={handleSearchChange}
        style={{ marginBottom: "10px", padding: "6px", width: "300px" }}
      />

      <div
        ref={parentRef}
        style={{
          height: "600px",
          overflow: "auto",
          border: "1px solid #ccc",
        }}
      >
        <table style={{ width: "100%", borderCollapse: "collapse" }}>
          <thead>
            {table.getHeaderGroups().map((headerGroup) => (
              <tr key={headerGroup.id}>
                {headerGroup.headers.map((header) => (
                  <th
                    key={header.id}
                    onClick={header.column.getToggleSortingHandler()}
                    style={{
                      cursor: "pointer",
                      borderBottom: "1px solid black",
                      background: "#f0f0f0",
                    }}
                  >
                    {flexRender(
                      header.column.columnDef.header,
                      header.getContext(),
                    )}
                    {header.column.getIsSorted()
                      ? header.column.getIsSorted() === "asc"
                        ? " 🔼"
                        : " 🔽"
                      : ""}
                  </th>
                ))}
              </tr>
            ))}
          </thead>
          <tbody style={{ position: "relative" }}>
            <tr style={{ height: `${rowVirtualizer.getTotalSize()}px` }} />
            {rowVirtualizer.getVirtualItems().map((virtualRow) => {
              const row = table.getRowModel().rows[virtualRow.index];
              return (
                <tr
                  key={row.id}
                  style={{
                    position: "absolute",
                    top: 0,
                    transform: `translateY(${virtualRow.start}px)`,
                    height: "35px",
                  }}
                >
                  {row.getVisibleCells().map((cell) => (
                    <td
                      key={cell.id}
                      style={{ padding: "4px", borderBottom: "1px solid #eee" }}
                    >
                      {flexRender(
                        cell.column.columnDef.cell,
                        cell.getContext(),
                      )}
                    </td>
                  ))}
                </tr>
              );
            })}
          </tbody>
        </table>
      </div>
    </div>
  );
};

export default Home;
