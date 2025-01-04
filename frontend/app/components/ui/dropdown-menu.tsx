import { cn } from "@/lib/utils";
import { CaretUpDown } from "@phosphor-icons/react";
import { Link } from "@tanstack/react-router";
import React from "react";

// Dropdown Component
export function Dropdown({ children }: { children: React.ReactNode }) {
  return <div className="group relative">{children}</div>;
}

// Dropdown Trigger Component
export function Trigger({ children }: { children: React.ReactNode }) {
  return (
    <button className="flex w-full items-center justify-between gap-1 rounded-2xl p-2 hover:bg-gray-100 focus:ring-black/10 focus:outline-none">
      {children}
      <CaretUpDown />
    </button>
  );
}

// Dropdown Content Component
export function Content({ children }: { children: React.ReactNode }) {
  return (
    <div className="invisible absolute -bottom-0 left-56 mt-1 w-48 rounded-3xl border border-neutral-100 bg-white/50 p-2 opacity-0 shadow-lg backdrop-blur-sm transition-all duration-300 group-focus-within:visible group-focus-within:opacity-100 group-hover:visible">
      <ul>{children}</ul>
    </div>
  );
}

// Dropdown Item Component
export function Item({
  children,
  className,
}: {
  children: React.ReactNode;
  className?: string;
}) {
  return (
    <li>
      <Link
        to="."
        className={cn(
          "flex items-center justify-between rounded-[calc(1.5rem-0.5rem)] px-3 py-1.5 text-sm hover:bg-black/5 focus:bg-black/5 focus:outline-none",
          className,
        )}
        tabIndex={0}
      >
        {children}
      </Link>
    </li>
  );
}

// Dropdown Separator Component
export function Separator() {
  return <hr className="my-1 border-t border-neutral-100" />;
}
