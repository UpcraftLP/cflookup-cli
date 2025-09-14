# cflookup-cli

[![License](https://img.shields.io/github/license/UpcraftLP/cflookup-cli?style=for-the-badge)](LICENSE.md)
[![GitHub commit activity](https://img.shields.io/github/commit-activity/t/UpcraftLP/cflookup-cli?style=for-the-badge)](https://github.com/UpcraftLP/cflookup-cli/commits/main/ "GitHub commit activity")
[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/UpcraftLP/cflookup-cli/ci.yml?branch=main&style=for-the-badge&label=build)](https://github.com/UpcraftLP/cflookup-cli/actions/workflows/ci.yml?query=branch%3Amain)

a CLI tool for getting info from https://cflookup.com

## How to use:

The general output will be in YAML format, so it is recommended to pipe it into a tool like `yq` to process it:
```shell
cflookup slug ender-io | yq .id,.name,.downloadCount
```
...outputs:
```
64578
Ender IO
105552007
```

<br>

To see CLI usage use the built-in help command (works withs with subcommands too!):
```shell
cflookup --help
```

## Install

### You can download prebuilt binaries from [nightly.link](https://nightly.link/UpcraftLP/cflookup-cli/workflows/ci/main)

### Cargo
If you have a rust toolchain installed, you can install from source using `cargo`:
```shell
cargo install --git https://github.com/UpcraftLP/cflookup-cli
```
