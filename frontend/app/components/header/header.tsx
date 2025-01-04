import { Link } from "@tanstack/react-router";

export default function Header() {
  return (
    <header className="sticky top-0 z-50 flex h-16 items-center justify-between rounded-3xl bg-white p-4 shadow-md">
      {" "}
      <Link>Quickinvoice</Link>
      <nav>
        <ul className="flex grow items-center gap-4">
          <li>
            <Link>Features</Link>
          </li>
          <li>
            <Link>Pricing</Link>
          </li>
          <li>
            <Link>Contact</Link>
          </li>
        </ul>
      </nav>
      <div>
        <Link>Sign in</Link>
      </div>
    </header>
  );
}
