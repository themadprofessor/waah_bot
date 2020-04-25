# WAAAAAAAAAH

![GitHub](https://img.shields.io/github/license/themadprofessor/waah_bot)
[![standard-readme compliant](https://img.shields.io/badge/readme%20style-standard-brightgreen.svg?style=flat)](https://github.com/RichardLitt/standard-readme)
![Travis (.com)](https://img.shields.io/travis/com/themadprofessor/waah_bot)
![GitHub code size in bytes](https://img.shields.io/github/languages/code-size/themadprofessor/waah_bot)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)

A Waluigi-insipred Discord bot written in Rust, using the serenity library.

## Table of Contents

- [Install](#install)
- [Usage](#usage)
- [Contributing](#contributing)
- [License](#license)

## Install

```
git clone https://github.com/themadprofessor/waah_bot.git
cd waah_bot
cargo install
```

## Usage

```
waah_bot
```

Waah Bot looks for a config file according to the [XDG Base Directory Specification](https://specifications.freedesktop.org/basedir-spec/latest/ar01s03.html).
Essentially, it looks for `waah_bot.toml` in the following locations:
| Platform | Location                                                |
|----------|---------------------------------------------------------|
| *nix     | `$XDG_CONFIG_HOME/waah_bot` or `$HOME/.config/waah_bot` |
| macOS    | `$HOME/Library/Preferences/waah_bot`                    |
| Windows  | `{FOLDERID_RoamingAppData}\waah_bot`                    |

`waah_bot.toml` should have the following format:
```toml
imgur_id = String
discord_token = String
log_level = OFF | ERROR | WARN | INFO | DEBUG | TRACE
```

The entries on the config file can be overridden by setting the corresponding environment variable:
| Entry         | Environment Variable   |
|---------------|------------------------|
| imgur_id      | waah_bot_imgur_id      |
| discord_token | waah_bot_discord_token |
| log_level     | waah_bot_log_level     |

`log_level` defaults to `WARN` if it not specified in either the config or environment.
All other entries must be specified in either the config file or environment.

## Contributing

See [the contributing file](CONTRIBUTING.md)!

PRs accepted.

Small note: If editing the Readme, please conform to the [standard-readme](https://github.com/RichardLitt/standard-readme) specification.

## License

[MIT © Stuart Reilly.](../LICENSE)
