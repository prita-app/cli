# prita

[![PyPI](https://img.shields.io/pypi/v/prita)](https://pypi.org/project/prita/)

Command-line interface for [Prita][prita], a read-it-later and knowledge library.
Save links, read them, tag them, and find them again, all from your terminal.

Use it yourself or hand it to your agents.

## For Agents

`prita` is built to be driven by LLM agents, so every command returns JSON default.
Use `--plain` for humans.

## Running

`prita` is published to PyPI. The quickest way to run it is with [uv][uv].

Hand this to your agents:

```sh
uvx prita --help
```

## Authentication

The CLI authenticates with a personal access token.

```sh
prita auth login                    # interactively prompts for the token
prita auth login prita_xxxxxxxx     # pass the token directly
export PRITA_TOKEN=prita_xxxxxxxx   # or set it in the environment
```

## Commands

```sh
prita list                    # saved articles, newest first
prita list --tag <id> -n 50   # scope to a tag, cap the count
prita list --after <cursor>   # grab the next page
```

Run `prita --help` for the full command list.

## License

Licensed under [MIT][mit].

[prita]: https://prita.app
[uv]: https://docs.astral.sh/uv
[cynic]: https://cynic-rs.dev
[maturin]: https://www.maturin.rs
[mit]: https://github.com/prita-app/prita/blob/main/LICENSE
