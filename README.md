![](https://github.com/LimeEng/lighthouse/workflows/CI/badge.svg)

# Lighthouse

Lighthouse is a simple utility to help guide your programs to safe ports. Simply run the program and it will print a valid and unoccupied (at the time) port. If no safe port can be found it simply exits with an exit code of 1.

- [Installation](#installation)
- [Usage](#usage)

## Installation

Install the program by running.
```
cargo install --git https://github.com/LimeEng/lighthouse
```

It is also possible to download a pre-built binary for either Windows, Linux or macOS from the [releases-page](https://github.com/LimeEng/lighthouse/releases).

## Usage

Simply invoke the program and it will either print a valid and unoccupied (at the time) port, or exit with an exit code of 1.
```
lighthouse
```
