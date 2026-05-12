# MFlash Studio

**MFlash Studio** is a native Rust editor for building, inspecting, and refining `.mflash` flashcard decks.

It is part of the broader MFlash ecosystem: a set of tools for multilingual flashcards, deck editing, native desktop integration, and future study workflows.

## What It Does

MFlash Studio is designed as the flagship editor for `.mflash` decks.

Current goals include:

- Open and inspect `.mflash` deck files.
- Edit flashcard content in a native desktop interface.
- View decks as lists, individual flashcards, or raw JSON.
- Support undo and redo while editing.
- Provide configurable Studio Settings.
- Support audio and text-to-speech settings for multilingual cards.
- Prepare the `.mflash` format for deeper OS integration later.

## Features

### Deck Views

MFlash Studio currently includes multiple ways to inspect a deck:

- **List View** — browse cards in a deck.
- **Flashcard Studio** — focus on one card at a time.
- **Raw JSON** — inspect the underlying deck data directly.

### Studio Settings

The Studio Settings window includes categories for:

- Global
- List
- Flashcards
- Audio
- Plugins
- Raw JSON

The Flashcards tab currently controls editor behavior such as editor mode and image display.

The Audio tab includes early settings for:

- Muting Studio UI sound effects.
- Selecting a future text-to-speech engine.
- Adjusting speech rate.
- Assigning voices to languages.

### Audio

MFlash Studio includes early audio infrastructure for:

- UI sound effects.
- Save confirmation sounds.
- Future text-to-speech playback.
- Future per-language voice profiles.

The audio system is still in development, but the settings UI is being built around the idea that `.mflash` decks may contain multilingual content.

## Studio Shortcuts

| Shortcut | Action |
|---|---|
| `Ctrl + ,` | Open Studio Settings |
| `Arrow Up` / `Arrow Down` | Navigate through list items |
| `Ctrl + Z` | Undo last action |
| `Ctrl + Y` | Redo last action |
| `Ctrl + S` | Save current deck |

## The MFlash Ecosystem

MFlash Studio is part of a larger `.mflash` ecosystem.

### [`mflash-spec`](https://github.com/MoribundMurdoch/mflash-spec)

The source-of-truth specification for the `.mflash` deck format.

### `moribund-flash`

A lightweight companion flashcard app built with Tauri and JavaScript.

### [`mflash-os-integrations`](https://github.com/MoribundMurdoch/mflash-os-integrations)

Native system integrations for `.mflash` files, including MIME types and future thumbnail generation.

## Configuration

Configuration is managed through `config.toml` in the project root or standard config directories.

Planned or current configuration areas include:

- **UI** — window sizing, scaling, and interface preferences.
- **Audio** — global sound settings, TTS behavior, and playback rates.
- **Shortcuts** — future remappable navigation keys for specialized workflows or hardware.

## Development

Run the app locally:

    cargo run

Format the code:

    cargo fmt

Check that the project builds:

    cargo check

## Project Status

MFlash Studio is currently experimental and under active development.

Expect rough edges, goblin wiring, and occasional haunted drawers in the UI while the architecture settles.

## License

Copyright © 2026.

Part of the MorFlashcards / Moribund Institute ecosystem.
