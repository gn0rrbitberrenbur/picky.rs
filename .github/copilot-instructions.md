# Project Instructions

## Overview

picky.rs is a desktop application for reviewing image folders in a fast swipe-based workflow. It is designed for photographers and editors who want to sort image pairs containing a preview image and a corresponding RAW file. The app lets the user select a folder, checks whether it contains compatible image pairs, and then previews them one by one with simple actions such as "Pass" or "Smash".

The project root contains a small wrapper package, while the actual application is located under the `picky/` folder.

## Tech Stack

- Frontend: SvelteKit + TypeScript + Vite
- Desktop shell: Tauri 2
- Backend logic: Rust
- Styling: plain Svelte component styles, no external component library
- Package management: npm for the frontend, Cargo for the Rust backend

## Framework and Runtime

### SvelteKit
The UI is built with SvelteKit in the `picky/` application folder. This gives the app a route-based structure and uses Vite for local development and build pipelines.

Relevant configuration:
- `picky/package.json` defines the SvelteKit scripts and dependencies.
- `picky/svelte.config.js` configures the SvelteKit app.
- `picky/vite.config.js` configures Vite and the frontend build.

### Tauri
The desktop application shell is built with Tauri v2. Tauri bridges the UI and the native OS environment, while keeping the frontend in web technologies and the desktop logic in Rust.

Relevant configuration:
- `picky/src-tauri/tauri.conf.json` defines app metadata, window settings, build commands, and bundle targets.
- `picky/src-tauri/Cargo.toml` contains the Rust crate configuration and Tauri dependencies.

## Repository Structure

```text
.
├── README.md
├── package.json
├── picky/
│   ├── package.json
│   ├── src/
│   │   ├── app.html
│   │   └── routes/
│   │       ├── +layout.svelte
│   │       ├── +layout.ts
│   │       ├── +page.svelte
│   │       └── nextscreen/
│   │           └── +page.svelte
│   └── src-tauri/
│       ├── Cargo.toml
│       ├── tauri.conf.json
│       ├── build.rs
│       ├── capabilities/
│       ├── icons/
│       └── src/
│           ├── commands.rs
│           ├── lib.rs
│           └── main.rs
└── instructions.md
```

## Architecture

### Frontend flow

The main entry is the folder selection screen in `picky/src/routes/+page.svelte`.

- The user clicks a button to choose a directory.
- The frontend calls the Tauri command `contains_images` with the selected path.
- If the folder has matching preview/RAW files, the app navigates to the next route with the selected folder in the query string.

### Backend logic

The Rust backend lives under `picky/src-tauri/src/commands.rs` and exposes Tauri commands that the frontend can invoke.

Key commands:
- `contains_images(path)`: validates a selected folder and ensures it contains supported image pairs.
- `map_images(path)`: scans the folder and builds a list of matching preview/RAW pairs.
- `image_to_base64_data_url(arg)`: reads an image file and converts it to a browser-ready data URL.
- `copy_image_to_wanna_edit(image_path)`: copies a selected RAW file into a `wanna_edit` folder next to the original.

This separation is important: the Rust layer handles filesystem scanning and file operations, while the Svelte UI handles the interactive viewer experience.

### Viewer workflow

The route in `picky/src/routes/nextscreen/+page.svelte`:

- reads the folder path from the URL query string,
- loads the mapped image list via `map_images`,
- converts the preview files to base64 data URLs for in-app rendering,
- allows the user to move backward or forward through the gallery,
- triggers the `Smash` action to copy the RAW file into a `wanna_edit` folder,
- advances to the next item after an action.

This means the app is intentionally simple: it is a local, filesystem-driven image review tool rather than a general-purpose media platform.

## Build and Run Commands

From the project root, use the app directory as the main working area:

```bash
cd picky
npm install
npm run tauri dev
```

To build the desktop app:

```bash
cd picky
npm run tauri build
```

To check TypeScript/Svelte correctness:

```bash
cd picky
npm run check
```

## Design Notes

- The app is intentionally local-first and file-system based.
- It relies on a custom pairing rule: preview files such as JPG/PNG are matched to RAW files with the same filename stem.
- The Tauri layer acts as a secure bridge between the desktop environment and the Svelte interface.
- The frontend is lightweight and focused on user interaction rather than complex state management.

## Summary

This repository is a compact Tauri desktop app with a SvelteKit frontend and a Rust backend. The project is built around a straightforward image-review workflow: choose a folder, detect matching RAW/preview pairs, review them in a swipe-like viewer, and copy the selected RAW files into a `wanna_edit` folder for later editing.
