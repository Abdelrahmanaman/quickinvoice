import { createFileRoute, Outlet } from "@tanstack/react-router";
import Sidebar from "@/components/sidebar";

export const Route = createFileRoute("/app")({
  component: RouteComponent,
});

function RouteComponent() {
  return (
    <div className="flex h-screen bg-black p-4 text-white">
      <Sidebar />
      <Outlet />
    </div>
  );
}
