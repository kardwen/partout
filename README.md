# Partout

Iced app for [pass](https://www.passwordstore.org/), work in progress

## Introduction

I have also written a TUI for pass that is available in the [AUR](https://aur.archlinux.org/packages/passepartui) and in [nixpkgs](https://github.com/NixOS/nixpkgs):

[passepartui](https://github.com/kardwen/passepartui)

## Installation

[crates.io](https://crates.io/crates/partout)

### Prerequisites

* Unix (tested on Linux so far)
* `pass`, optionally `pass-otp` for one-time passwords
* Rust and cargo

```sh
cargo install partout --locked
```

## Development

Workspace:

* `partout`: Iced application
* `passepartout`: pass library
