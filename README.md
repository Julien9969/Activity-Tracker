# Activity Tracker

Desktop activity tracking app with a SvelteKit UI and a Tauri backend.

<p align="center">
	<img alt="Tauri" src="https://img.shields.io/badge/Tauri-2.x-000000?logo=tauri&logoColor=white" />
	<img alt="SvelteKit" src="https://img.shields.io/badge/SvelteKit-5.x-ff3e00?logo=svelte&logoColor=white" />
	<img alt="TypeScript" src="https://img.shields.io/badge/TypeScript-5.x-3178c6?logo=typescript&logoColor=white" />
	<img alt="Tailwind CSS" src="https://img.shields.io/badge/Tailwind%20CSS-4.x-38bdf8?logo=tailwindcss&logoColor=white" />
	<img alt="Vite" src="https://img.shields.io/badge/Vite-6.x-646cff?logo=vite&logoColor=white" />
</p>

## Screenshots

<p align="left">
	<img alt="Activity Tracker screenshot 1" src="docs/screenshots/1.png" width="50%" />
	<!-- <img alt="Activity Tracker screenshot 2" src="docs/screenshots/slide-02.svg" width="280" />
	<img alt="Activity Tracker screenshot 3" src="docs/screenshots/slide-03.svg" width="280" /> -->
</p>

## Tech Stack

- Tauri 2 (Rust)
- SvelteKit (Svelte 5) + Vite
- TypeScript
- DuckDB (local activity store)
- Tailwind CSS v4 + shadcn-svelte/bits-ui components

## Development

```bash
npm install
npm run dev
```

## Desktop App (Tauri)

```bash
npm run tauri dev
```

## Build

```bash
npm run build
npm run tauri build
```

## Type Check

```bash
npm run check
```

## Project Structure

- `src/` SvelteKit app
- `src-tauri/` Tauri (Rust) backend