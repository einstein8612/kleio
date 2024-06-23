# Kleio

![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/einstein8612/kleio/rust.yml)
![Crates.io Version](https://img.shields.io/crates/v/kleio-rs)
![Crates.io Total Downloads](https://img.shields.io/crates/d/kleio-rs)
![GitHub code size in bytes](https://img.shields.io/github/languages/code-size/einstein8612/kleio)
![Crates.io License](https://img.shields.io/crates/l/kleio-rs)


NOTE ⚠️: This code for this library was thrown together within a few minutes as it was necessary for another project to continue. The API and generation methods will most likely be changed drastically (soontm).

Kleio is a small library that generates ids or alphabetical keys that can be used as (almost) unique identifiers that are at the same time seemingly random.

## Getting started
The API for Kleio as it stands right now is very simple. You can start using like this:

```rust
use kleio::{generate_key, generate_id};

let key: String = generate_key();
// or
let id: u64 = generate_id();
```
