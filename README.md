# When to (not) use typestate

This repo contains the slides and the demo code for my 'When to (not) use typestate' talk at
RustFest Zurich 2024 given on Friday, June 21st 2024.

The talk is about the typestate pattern.
How it is implemented, its benefits but also its disadvantages.

## Demo

The talk contains a section on how to implement the typestate pattern.
The demo code shows various implementation options depending on your requirements and state machine.
For further details read the demo's docs.

## Building the slides

The project uses [marpit] to render its [slides](./presentation.md).

If you use VS Code and have the recommended marpit extension installed you can simple run the
`Marp: Export Slide Deck...` command to render the slides yourself.

If you don't use VS Code you have to use the marp CLI.
You may need to pass the theme's URL to render the slides with the same style.
You can find the theme's URL in [`./.vscode/settings.json`](./.vscode/settings.json).
Alternatively you can change to the similar `gaia` theme a default theme of marp.

## Rendering the assets

All state diagram images are rendered via PlantUML from the according `.puml` files.

[marpit]: https://marpit.marp.app/
