import sidebarList from "@/lib/sidebar-list";
import { CaretRight } from "@phosphor-icons/react";
import { Link } from "@tanstack/react-router";

export default function Sidebar() {
  return (
    <aside className="h-full w-[250px] space-y-12 rounded-3xl bg-white p-4 text-black">
      <h1>Quickinvoice</h1>
      <ul className="space-y-3">
        {sidebarList.map((item) => (
          <li key={item.title}>
            {item.children ? (
              <details className="group -sm group w-full [&::details-content]:overflow-hidden [&::details-content]:transition-all [&::details-content]:transition-discrete [&::details-content]:duration-500 [&::details-content]:ease-out [&::details-content]:[block-size:0] open:[&::details-content]:[block-size:calc-size(auto,size)]">
                <summary className="flex cursor-pointer items-center gap-1">
                  <div className="flex w-full items-center justify-between">
                    <div className="flex items-center gap-1">
                      <item.icon />
                      <span>{item.title}</span>
                    </div>
                    <div>
                      <CaretRight className="transform transition-transform duration-200 ease-in-out group-open:rotate-90" />
                    </div>
                  </div>
                </summary>
                <ul className="mt-2 ml-2 space-y-2 border-l pl-2">
                  {item.children.map((child) => (
                    <li className="" key={child.title}>
                      <Link className="flex items-center gap-1" to={child.path}>
                        <child.icon />
                        <span>{child.title}</span>
                      </Link>
                    </li>
                  ))}
                </ul>
              </details>
            ) : (
              <Link className="flex items-center gap-1" to={item.path}>
                <item.icon />
                <span>{item.title}</span>
              </Link>
            )}
          </li>
        ))}
      </ul>
    </aside>
  );
}
