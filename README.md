# webshot

A simple CLI tool to create screenshots from web pages in the terminal.

This tool uses the [headless_chrome](https://crates.io/crates/headless_chrome) crate, which requires a local installation of Chrome or Chromium.

> **Information**  
> This is the successor for my old tool [zekroTJA/webshot](https://github.com/zekroTJA/webshot), which uses [puppeteer](https://pptr.dev).

## Usage

```
$ webshot --help
Simply screenshot websites from your terminal

Usage: webshot [OPTIONS] <URL>

Arguments:
  <URL>  URL of the web page to be captured

Options:
  -c, --config <CONFIG>      Path to a config file
  -o, --output <OUTPUT>      Output directory or file name
  -W, --width <WIDTH>        Screen width
  -H, --height <HEIGHT>      Screen height
  -s, --scale <SCALE>        Scale factor
      --wait-for <WAIT_FOR>  Wait for DOM element (query selector) [default: body]
  -h, --help                 Print help
  -V, --version              Print version
```

## Install

You can either download the latest release builds form the [Releases page](https://github.com/shellshape/webshot/releases) or you can install it using cargo install.
```
cargo install --git https://github.com/shellshape/webshot
```

## Configuration

You can create a configuration to define some default parameters. The tool will look for a config file in one of the following locations.

|  | Local | Home Config |
|--|-------|-------------|
| **Linux** | `./webshot.*` | `$HOME/.config/webshot/config.*` |
| **Windows** | `.\webshot.*` | `%APPDATA%\webshot\config.*` |
| **OSX** | `./webshot.*` | `$HOME/Library/Application Support/webshot/config.*` |

Supported config types are the following.
- YAML (`.yaml`)
- TOML (`.toml`)
- JSON (`.json`)

An example config could look as following.

```toml
default_width = 1920
default_height = 1080
default_scale = 1.5
```