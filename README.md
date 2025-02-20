# MC Plugin Finder

A search aggregator for finding Minecraft server plugins on [Spigot](https://www.spigotmc.org), [Modrinth](https://modrinth.com/plugins), and [Hangar](https://hangar.papermc.io).

### Try it out here: [https://mcpluginfinder.com](https://mcpluginfinder.com)

![mc-plugin-finder-screenshot](https://github.com/user-attachments/assets/bed098ff-9db5-414b-90dd-26c7c6cf8c4b)

## Elevator Pitch

As a Minecraft server admin, have you ever been annoyed when trying to find a plugin? Do I have to search for it on Spigot? Or maybe it's on Modrinth? Or Hangar? Then you have to check if the plugin has been recently updated or supports your Minecraft version. And on top of all that, the plugin developer might have only uploaded their latest version on some platforms but not others.

What a mess.

MC Plugin Finder seeks to solve this problem by being the one-stop shop for searching all three of these platforms simultaneously, thanks to their public APIs. You can filter and sort your search results as desired, and you can compare versions of the same plugin on each platform to ensure that you are getting the latest version.

## How it Works

MC Plugin Finder has two main components: The **ingest tool** and the **web app**.

![MC Plugin Finder drawio](https://github.com/user-attachments/assets/826e4b22-5e8f-440a-b2bc-b903ef3e858f)

### Ingest Tool

The **ingest tool** is a CLI application that retrieves plugin project data from the [Spiget API](https://spiget.org/) (for Spigot), [Modrinth API](https://docs.modrinth.com/), and [Hangar API](https://hangar.papermc.io/api-docs). The tool runs daily to update the database with the latest plugin information. It also considers projects from different plugin repositories to be the same if they **share the same source code repository URL**.

For example, suppose there was a project named "Foo" on Spigot and another project named "Bar" on Modrinth, and both projects have `https://github.com/foo/foo` as their source code repository URL. Both projects would be considered the same on MC Plugin Finder, even though their project names are different.

On the other hand, if there were two projects named "Baz" on Spigot and Modrinth each, but the plugin developer forgot to add a source code URL to one of these plugin repositories, then these projects would **not** be considered be the same on MC Plugin Finder, even though their project names match.

**Examples illustrated:**

| Spigot Plugin Name | Spigot Plugin Source URL     | Modrinth Plugin Name | Modrinth Plugin Source URL   | Considered the same plugin by MC Plugin Finder |
| ------------------ | ---------------------------- | -------------------- | ---------------------------- | ---------------------------------------------- |
| Foo                | `https://github.com/foo/foo` | Bar                  | `https://github.com/foo/foo` | :heavy_check_mark:                             |
| Baz                | `https://github.com/baz/baz` | Baz                  | N/A                          | :x:                                            |

MC Plugin Finder will only recognize URLs from these source code repository hosts:
- [github.com](https://github.com)
- [gitlab.com](https://gitlab.com)
- [bitbucket.org](https://bitbucket.org)
- [codeberg.org](https://codeberg.org)

### Web App

The **web app** allows users to search the database for plugins.

The MC Plugin Finder hosted infrastructure runs an instance of [imageproxy](https://github.com/willnorris/imageproxy) to cache plugin project icons from Modrinth and Hangar as they are requested by users. This reduces the load on the Modrinth and Hangar CDNs, and provides improved image loading performance. Icon data for Spigot-hosted plugins are provided directly by the Spiget API and stored in the database, so no proxy or caching is needed in that case.

## Development Setup

### Rust

Install [rustup](https://www.rust-lang.org/tools/install) and the latest version of Rust.

Add the wasm32 target:
- `rustup target add wasm32-unknown-unknown`

Install Cargo extensions:
- `cargo install cornucopia`
- `cargo install cargo-nextest`
- `cargo install cargo-leptos`
- `cargo install refinery_cli`

Build the workspace:
- `cargo build --workspace`

### PostgreSQL

Ensure that you have Docker or Podman installed on your system. For more details, see the [Cornucopia installation instructions](https://cornucopia-rs.netlify.app/book/introduction/installation).

Set the database url in the .env file as desired:
```
MCPF_DATABASE_URL=postgres://postgres:postgres@localhost:5432/mc_plugin_finder
```

Set the MCPF_DATABASE_URL variable in your environment. This varies depending on your operating system.

Run the migrations:
- `refinery migrate -e MCPF_DATABASE_URL`

Run the ingest tool to populate the database, starting with these commands:
- `ingest populate spigot authors`
- `ingest populate spigot resources`
- `ingest populate modrinth projects`
- `ingest populate hangar projects`

Optionally, you may populate plugin versions as well (However, note that populating Spigot versions takes several hours):
- `ingest populate spigot versions`
- `ingest populate modrinth versions`
- `ingest populate hangar versions`

After populating all data, run the ingest tool again to fix some errors in upstream resources and projects:
- `ingest --fix`

Run the ingest tool yet again to refresh the common projects:
- `ingest --refresh`

The database can then be later updated using these commands:
- `ingest update spigot resources`
- `ingest update modrinth projects`
- `ingest update hangar projects`

For daily updates in a live environment, this command is used:
- `ingest update all --fix --refresh`

### Commands

After making any changes to queries, regenerate your cornucopia.rs file:
- `cornucopia -d src/database/cornucopia.rs schema schema.sql`

Run tests:
- `cargo nextest run --workspace`

Run the web server:
- `cargo leptos watch`

## Major Dependencies
- [Cornucopia](https://github.com/cornucopia-rs/cornucopia) - Rust code generator for PostgreSQL queries.
- [Leptos](https://github.com/leptos-rs/leptos) - Full-stack isomorphic web framework.

## Attributions
- [GitHub Corners](https://github.com/tholman/github-corners) ([MIT License](https://github.com/tholman/github-corners/blob/master/license.md))
- [Material Symbols & Icons](https://fonts.google.com/icons) ([Apache License 2.0](https://www.apache.org/licenses/LICENSE-2.0.html))

## License

Copyright © 2025 Frumple

MC Plugin Finder is provided under the [GNU Affero General Public License 3.0](https://github.com/Frumple/mc-plugin-finder/blob/main/LICENSE).

MC Plugin Finder is not an official Minecraft service, and is not approved or associated with Mojang, Microsoft, SpigotMC, Modrinth, or PaperMC.
