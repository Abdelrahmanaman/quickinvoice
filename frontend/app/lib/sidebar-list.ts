import {
  Faders,
  FilePdf,
  House,
  Invoice,
  ReadCvLogo,
} from "@phosphor-icons/react";
import type { LinkOptions } from "@tanstack/react-router";
import type { ForwardRefExoticComponent, SVGProps } from "react";

type SidebarListItem = {
  title: string;
  path: LinkOptions["to"];
  icon: ForwardRefExoticComponent<SVGProps<SVGSVGElement>>;
  children?: SidebarListItem[];
};
const sidebarList: SidebarListItem[] = [
  {
    title: "Dashboard",
    path: "/app",
    icon: House,
  },
  {
    title: "Account",
    path: "/app/account",
    icon: Faders,
  },
  {
    title: "Invoices",
    path: "/",
    icon: Invoice,
    children: [
      {
        title: "View Invoices",
        path: "/app",
        icon: ReadCvLogo,
      },
      {
        title: "Create Invoice",
        path: "/app",
        icon: FilePdf,
      },
    ],
  },
];

export default sidebarList;
