# webshot

A simple CLI tool to create screenshots from web pages in the terminal.

This tool uses the [headless_chrome](https://crates.io/crates/headless_chrome) crate, which requires a local installation of Chrome or Chromium.

> **Info**  
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