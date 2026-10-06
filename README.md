# FlowDo Desktop

FlowDo is a Tauri 2 desktop application with a SvelteKit frontend. Authentication is handled locally by Better Auth Rust and SeaORM; no external auth server is required.

## Development

Install the JavaScript dependencies with `pnpm install`, then run the desktop app with `pnpm tauri dev`.

## Production build

Run `pnpm check` to validate Svelte and TypeScript, `pnpm build` to build the static frontend, or `pnpm tauri build` to produce the complete desktop application bundle.

On Linux, Tauri's GTK/WebKit development libraries and AppImage tooling must be installed for the configured AppImage target.

## Authentication and local data

Email/password sign-up, sign-in, session lookup, and sign-out are routed through Better Auth Rust Tauri commands. Commands return the shared client-result contract: `isSuccess`, `value`, `errorMsg`, and `statusCode`.

At first launch, the app creates the SQLite database `flowdodb.sqlite` in Tauri's per-user application data directory. Auth tables and the `task` table are created automatically. Tasks use the `INBOX`/`IN_PROGRESS`/`DONE` status and `LOW`/`MEDIUM`/`HIGH` priority values, and every task command derives its owner from the current Better Auth session. The Better Auth signing secret and optional Remember me session cookie are stored in the same directory with owner-only permissions on Unix systems. Sign-out removes the persisted session cookie.

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).
