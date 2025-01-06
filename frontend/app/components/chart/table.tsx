import {
  CaretLeft,
  CaretRight,
  CaretUpDown,
  MagnifyingGlass,
} from "@phosphor-icons/react";
import {
  createColumnHelper,
  flexRender,
  getCoreRowModel,
  getPaginationRowModel,
  getSortedRowModel,
  getFilteredRowModel,
  useReactTable,
  type SortingState,
} from "@tanstack/react-table";
import React, { useState } from "react";

// Define the User type
type User = {
  customerName: string;
  email: string;
  invoiceId: string;
  status: Status;
  amount: number;
  issueDate: string;
};

// Define the string enum
enum Status {
  Pending = "Pending",
  Paid = "Paid",
  Cancelled = "Cancelled",
}

// Create column helper
const columnHelper = createColumnHelper<User>();

// Sample data
const userData: User[] = [
  {
    customerName: "John Doe",
    email: "john@doe.com",
    invoiceId: "INV123456",
    status: Status.Paid,
    amount: 240,
    issueDate: "2023-01-01",
  },
  {
    customerName: "Jane Smith",
    email: "jane@smith.com",
    invoiceId: "INV123457",
    status: Status.Pending,
    amount: 150,
    issueDate: "2023-01-02",
  },
  {
    customerName: "Alice Johnson",
    email: "alice@johnson.com",
    invoiceId: "INV123458",
    status: Status.Cancelled,
    amount: 300,
    issueDate: "2023-01-03",
  },
  {
    customerName: "Bob Brown",
    email: "bob@brown.com",
    invoiceId: "INV123459",
    status: Status.Paid,
    amount: 200,
    issueDate: "2023-01-04",
  },
  {
    customerName: "Charlie Davis",
    email: "charlie@davis.com",
    invoiceId: "INV123460",
    status: Status.Pending,
    amount: 100,
    issueDate: "2023-01-05",
  },
  {
    customerName: "Diana Evans",
    email: "diana@evans.com",
    invoiceId: "INV123461",
    status: Status.Paid,
    amount: 500,
    issueDate: "2023-01-06",
  },
  {
    customerName: "Ethan Green",
    email: "ethan@green.com",
    invoiceId: "INV123462",
    status: Status.Cancelled,
    amount: 250,
    issueDate: "2023-01-07",
  },
  {
    customerName: "Fiona Harris",
    email: "fiona@harris.com",
    invoiceId: "INV123463",
    status: Status.Paid,
    amount: 400,
    issueDate: "2023-01-08",
  },
  {
    customerName: "George Clark",
    email: "george@clark.com",
    invoiceId: "INV123464",
    status: Status.Pending,
    amount: 120,
    issueDate: "2023-01-09",
  },
  {
    customerName: "Hannah Lewis",
    email: "hannah@lewis.com",
    invoiceId: "INV123465",
    status: Status.Paid,
    amount: 350,
    issueDate: "2023-01-10",
  },
  {
    customerName: "Ian Walker",
    email: "ian@walker.com",
    invoiceId: "INV123466",
    status: Status.Cancelled,
    amount: 180,
    issueDate: "2023-01-11",
  },
  {
    customerName: "Jessica Hall",
    email: "jessica@hall.com",
    invoiceId: "INV123467",
    status: Status.Paid,
    amount: 220,
    issueDate: "2023-01-12",
  },
  {
    customerName: "Kevin Allen",
    email: "kevin@allen.com",
    invoiceId: "INV123468",
    status: Status.Pending,
    amount: 90,
    issueDate: "2023-01-13",
  },
  {
    customerName: "Laura Young",
    email: "laura@young.com",
    invoiceId: "INV123469",
    status: Status.Paid,
    amount: 600,
    issueDate: "2023-01-14",
  },
  {
    customerName: "Michael Hernandez",
    email: "michael@hernandez.com",
    invoiceId: "INV123470",
    status: Status.Cancelled,
    amount: 280,
    issueDate: "2023-01-15",
  },
  {
    customerName: "Nancy King",
    email: "nancy@king.com",
    invoiceId: "INV123471",
    status: Status.Paid,
    amount: 320,
    issueDate: "2023-01-16",
  },
  {
    customerName: "Oscar Wright",
    email: "oscar@wright.com",
    invoiceId: "INV123472",
    status: Status.Pending,
    amount: 110,
    issueDate: "2023-01-17",
  },
  {
    customerName: "Patricia Lopez",
    email: "patricia@lopez.com",
    invoiceId: "INV123473",
    status: Status.Paid,
    amount: 450,
    issueDate: "2023-01-18",
  },
  {
    customerName: "Quincy Scott",
    email: "quincy@scott.com",
    invoiceId: "INV123474",
    status: Status.Cancelled,
    amount: 200,
    issueDate: "2023-01-19",
  },
  {
    customerName: "Rachel Adams",
    email: "rachel@adams.com",
    invoiceId: "INV123475",
    status: Status.Paid,
    amount: 700,
    issueDate: "2023-01-20",
  },
];
// Define columns
const columns = [
  columnHelper.accessor("customerName", {
    header: "Customer Name",
    cell: (info) => info.getValue(),
  }),
  columnHelper.accessor("email", {
    header: "Email",
    cell: (info) => info.getValue(),
  }),
  columnHelper.accessor("invoiceId", {
    header: "Invoice Id",
    cell: (info) => info.getValue(),
  }),
  columnHelper.accessor("status", {
    header: "Status",
    cell: (info) => info.getValue(),
  }),
  columnHelper.accessor("amount", {
    header: (props) => {
      const sortDirection = props.column.getIsSorted();
      return (
        <div
          className="flex cursor-pointer items-center gap-1"
          onClick={() => {
            if (sortDirection === "desc") {
              props.column.clearSorting(); // Reset to default (unsorted)
            } else {
              props.column.toggleSorting(sortDirection === "asc"); // Toggle between asc and desc
            }
          }}
        >
          <span>Amount</span>
          <CaretUpDown />
          {sortDirection === "asc"}
          {sortDirection === "desc"}
          {!sortDirection}
        </div>
      );
    },
    cell: (info) => `$${info.getValue().toLocaleString()}`,
  }),
  columnHelper.accessor("issueDate", {
    header: "Issue Date",
    cell: (info) => new Date(info.getValue()).toLocaleDateString(),
  }),
];

// Table component
export default function RecentInvoicesTable() {
  const [data] = React.useState<User[]>(userData);
  const [sorting, setSorting] = useState<SortingState>([]); // State for sorting
  const [globalFilter, setGlobalFilter] = useState(""); // State for global search

  const table = useReactTable({
    data,
    columns,
    state: {
      sorting, // Pass sorting state
      globalFilter, // Pass global filter state
    },
    onSortingChange: setSorting, // Update sorting state
    onGlobalFilterChange: setGlobalFilter, // Update global filter state
    getCoreRowModel: getCoreRowModel(),
    getSortedRowModel: getSortedRowModel(), // Enable sorting
    getFilteredRowModel: getFilteredRowModel(), // Enable filtering
    getPaginationRowModel: getPaginationRowModel(), // Enable pagination
  });

  return (
    <div className="p-4">
      {/* Global Search Input */}
      <div className="relative mb-4 max-w-sm">
        <input
          type="text"
          value={globalFilter}
          onChange={(e) => setGlobalFilter(e.target.value)}
          placeholder="Search"
          className="peer w-full max-w-sm rounded-2xl border border-gray-100 p-2 focus:bg-gray-100 focus:outline-none"
        />
        <MagnifyingGlass className="peer-focus:text-gray absolute top-3 right-3 size-5 text-gray-200" />
      </div>

      {/* Table */}
      <div className="overflow-hidden rounded-3xl border border-gray-100">
        <table className="w-full border-separate border-spacing-0">
          <thead>
            {table.getHeaderGroups().map((headerGroup) => (
              <tr key={headerGroup.id}>
                {headerGroup.headers.map((header, index) => (
                  <th
                    className={`border-b border-gray-100 bg-gray-100 p-2 text-left ${
                      index === 0
                        ? "rounded-tl-3xl" // Round top-left corner of the first header
                        : index === headerGroup.headers.length - 1
                          ? "rounded-tr-3xl" // Round top-right corner of the last header
                          : ""
                    }`}
                    key={header.id}
                  >
                    <div className="flex items-center">
                      {header.isPlaceholder
                        ? null
                        : flexRender(
                            header.column.columnDef.header,
                            header.getContext(),
                          )}
                    </div>
                  </th>
                ))}
              </tr>
            ))}
          </thead>
          <tbody>
            {table.getRowModel().rows.map((row) => (
              <tr className="hover:bg-gray-100" key={row.id}>
                {row.getVisibleCells().map((cell) => (
                  <td className="border-b border-gray-100 p-2" key={cell.id}>
                    {flexRender(cell.column.columnDef.cell, cell.getContext())}
                  </td>
                ))}
              </tr>
            ))}
          </tbody>
        </table>
      </div>

      {/* Pagination Controls */}
      <div className="mt-4 flex items-center justify-between px-4 py-2">
        <div className="flex w-full items-center justify-between">
          {/* Page Tracking */}
          <span className="rounded-2xl bg-gray-200 p-2 text-sm">
            Page {table.getState().pagination.pageIndex + 1} of{" "}
            {table.getPageCount()}
          </span>

          {/* Next/Previous Buttons */}
          <div className="flex space-x-2">
            <button
              onClick={() => table.previousPage()}
              disabled={!table.getCanPreviousPage()}
              className="flex items-center gap-1 rounded-2xl bg-gray-200 p-2 text-sm disabled:opacity-50"
            >
              <CaretLeft />
              Previous
            </button>
            <button
              onClick={() => table.nextPage()}
              disabled={!table.getCanNextPage()}
              className="flex items-center gap-1 rounded-2xl bg-gray-200 p-2 text-sm disabled:opacity-50"
            >
              Next
              <CaretRight />
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}
