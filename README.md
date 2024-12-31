# MC Plugin Finder

A search aggregator for finding Minecraft: Java Edition server plugins on [Spigot](https://www.spigotmc.org/), [Modrinth](https://modrinth.com/plugins), and [Hangar](https://hangar.papermc.io/).

### Live Application: [https://mcpluginfinder.com](https://mcpluginfinder.com)

## How it Works

MC Plugin Finder has two main components: The **ingest** tool and the **web** app.

The **ingest** tool is a CLI application that retrieves plugin project data from the [Spiget API](https://spiget.org/) (for Spigot), [Modrinth API](https://docs.modrinth.com/), and [Hangar API](https://hangar.papermc.io/api-docs). The tool runs once per day to update the database with the latest projects, and then creates a common database view for those projects, merging some projects together if they **share the same source code repository URL**.

For example, suppose there was a project named "Foo" on Spigot and another project named "Bar" on Modrinth, and both projects have `https://github.com/username/repo` as their source code repository URL. Both projects would be considered the same on MC Plugin Finder, even though their project names are different.

MC Plugin Finder will only recognize URLs from these source code repositories:
- [github.com](https://github.com)
- [gitlab.com](https://gitlab.com)
- [bitbucket.org](https://bitbucket.org)
- [codeberg.org](https://codeberg.org)

The **web** app allows users to search the projects in the common database view.

## Development Setup

Add the wasm32 target:
- `rustup target add wasm32-unknown-unknown`

Install Cargo extensions:
- `cargo install cornucopia`
- `cargo install cargo-nextest`
- `cargo install cargo-leptos`

### Cornucopia

Ensure that you have Docker or Podman installed on your system. For more details, see the [Cornucopia installation instructions](https://cornucopia-rs.netlify.app/book/introduction/installation).

Setup the initial schema on your database by running [schema.sql](https://github.com/Frumple/mc-plugin-finder/blob/main/schema.sql) on it.

Set the database settings in the .env file as desired:
```
MCPF_DB_USER=postgres
MCPF_DB_PASSWORD=postgres
MCPF_DB_HOST=localhost
MCPF_DB_PORT=5432
MCPF_DB_NAME=mc_plugin_finder
```

Regenerate the cornucopia.rs file after making any changes to queries:
- `cornucopia -d src/database/cornucopia.rs schema schema.sql`

### Run Commands

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

MC Plugin Finder is provided under the [GNU Affero General Public License 3.0](https://github.com/Frumple/mc-plugin-finder/blob/main/LICENSE).

MC Plugin Finder is not an official Minecraft service, and is not approved or associated with Mojang or Microsoft.