[![crates.io](https://img.shields.io/crates/v/option-ext.svg)](https://crates.io/crates/option-ext)
[![API documentation](https://docs.rs/option-ext/badge.svg)](https://docs.rs/option-ext/)
![actively developed](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)
![License: MPL-2.0](https://img.shields.io/badge/license-MPL--2.0-orange.svg)

# `option-ext`

## Introduction

This crate extends `Option` with additional methods, currently:

- `contains`

Its sister crate is `result-ext`, which extends `Result`. 

## Requirements

Rust 1.0 or newer.

## Usage

#### Dependency

Add the library as a dependency to your project by inserting

```toml
option-ext = "0.1.0"
```

into the `[dependencies]` section of your Cargo.toml file.

#### Example

```rust
use self::option_ext;

fn example() {
    let x: Option<u32> = Some(2);
    assert_eq!(x.contains(&2), true);

    let x: Option<u32> = Some(3);
    assert_eq!(x.contains(&2), false);

    let x: Option<u32> = None;
    assert_eq!(x.contains(&2), false);
}
```
