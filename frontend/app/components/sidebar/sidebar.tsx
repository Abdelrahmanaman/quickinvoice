import sidebarList from "@/lib/sidebar-list";
import { CaretRight } from "@phosphor-icons/react";
import { Link } from "@tanstack/react-router";
import SidebarContent from "./sidebar-content";
import SidebarFooter from "./sidebar-footer";

export default function Sidebar() {
  return (
    <aside className="flex min-w-[250px] flex-col space-y-12 rounded-3xl bg-white p-4 text-black">
      <h1>Quickinvoice</h1>

      <div className="flex grow flex-col justify-between">
        <SidebarContent />
        <SidebarFooter />
      </div>
    </aside>
  );
}
