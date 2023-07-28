# Grouped Repository Assistant

GRAss is a repository assistant written in rust.
In short it is a CLI utility, which aims to assist in the managing of repositories.
It does this by grouping the repositories in abstract groups, called categories.
Each repository can be uniquely defined by a category and repository pair.
So for example, you could have a category for personal, and work repositories.
Then you could have the following repositories:

```
personal rust_example
personal dotfiles
work java_monorepo
work example_dot_com_frontend
```

Examples of what it can do:

- Manage categories of repositories.
- Fork any git repository, even across multiple services.
- Analyze and warn about uncommitted changes.
- Clean up repositories, including things like `node_modules/`.
- Collect and filter issues from multiple repositories.
- Export current repositories, to recreate on a different machine.
- Manage GitHub gists using git.
- Open up projects in separate sessions using a terminal multiplexer like tmux.
- With minimal configuration, upgrade your shell to be able to do the following:
    - Automatically open up the correct folder based on session name.
    - Automatically manage Python virtual environments

> Currently not all of these features may be implemented.
> The project is still a work in progress.

## Quick start

To install the CLI utility, run the following command:

```bash
cargo install --git https://github.com/damymetzke/grass.git
```

To add this to your own rust project as a dependency, run the following command:

```bash
cargo add --git https://github.com/damymetzke/grass.git
```

## Configuration

All configuration is located in the default configuration directory.
On Linux this is `$XDG_CONFIG_HOME/grass`, or `~/.config/grass`.
Grass will consider *any* TOML file a configuration file.
This can be used to split up configuration.
However I suggest using `config.toml`, if you only want to use a single configuration file.

To list all possible configuration options, run the following command:

```bash
grass config list

# To get an advanced explanation for a specific configuration value
grass config list <key>
```

## Use cases

The primary use case is for users.
I don't consider this a good fit for fully automated systems.
It is meant to be used in order to enrich your development process.
