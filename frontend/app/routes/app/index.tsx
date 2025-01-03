import { createFileRoute, Outlet } from "@tanstack/react-router";

export const Route = createFileRoute("/app/")({
  component: RouteComponent,
});

function RouteComponent() {
  return (
    <div className="w-full">
      <div className="text-4xl">Hello world </div>
    </div>
  );
}
