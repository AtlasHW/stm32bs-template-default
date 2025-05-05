# `stm32bs default template`

* The file is only for template, it will be not in generated project directory
* README.md.liquid is the README file that will be in generated project 
  directory.

> The template for building applications for STM32 microcontrollers

This project is modifyed and maintained by the [Atlas Song Kai][atlas email].
The origin project is developed and maintained by the [Cortex-M team][team].

## Dependencies

To build stm32 programs using this template you'll need:

- Rust 1.31, 1.30-beta, nightly-2018-09-13 or a newer toolchain. e.g. `rustup
  default beta`

- The `cargo stm32bs` subcommand. [Installation
  instructions]`cargo install stm32bs`.

- `rust-std` components (pre-compiled `core` crate) for the ARM Cortex-M
  targets. Run:

``` console
$ rustup target add thumbv6m-none-eabi thumbv7m-none-eabi thumbv7em-none-eabi thumbv7em-none-eabihf thumbv8m.base-none-eabi thumbv8m.main-none-eabi thumbv8m.main-none-eabihf
```

## Using this template

**NOTE**: This is the very short version that only covers building programs. For
the long version, which additionally covers flashing, running and debugging
programs, check [the embedded Rust book][book].

[book]: https://rust-embedded.github.io/book

1. Instantiate the template.

``` console
$ cargo stm32bs --git template_path
🤷 Project Name: app
🤷 Chip Part Number (eg. stm32g071cbt6): stm32f103c8t6
✔ 🤷 Choose a project type · Empty Project
Create a Empty STM32 Project...
🔧 Destination: /home/atlassong-k/rust/cargo-stm32bs/app ...
🔧 project-name: app ...
🔧 username: "atlasHW" (placeholder provided by cli argument)
🔧 Generating template ...
[1/6]   Done: Cargo.toml
[2/6]   Done: src/main.rs
[3/6]   Done: build.rs
[4/6]   Done: .cargo/config.toml
[5/6]   Done: memory.x
[6/6]   Done: README.md
✨ Done! New project created /home/atlassong-k/rust/cargo-stm32bs/app

$ cd app
```

2. Buid and run the applications.

``` console
$ cargo build
$ cargo run
```

## Template structure

The template files system is as following

  - stm32bs.toml
  - README.md
  - README.md.liquid
  - build.rs
  - memory.x
  - .cargo/
  -  └─	config.toml
  - src/
  -  └─ main.rs
  - demo/
  -  ├─ demo1.rs
  -  └─ demo2.rs
  - ProductsList.csv
  - pac_info.csv
  
## Variables in template

- for project with BSP
 
- for empty project

1. authors 
2. target
3. pac_name
4. pac_ver
5. pac_featrue
6. flash_origin
7. flash_size
8. ram1_origin
9. ram1_size
10. pn


- for demo project

your placeholder on your demo configuration


# License

This template is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## Code of Conduct

Contribution to this crate is organized under the terms of the [Rust Code of
Conduct][CoC], the maintainer of this crate, the [Cortex-M team][team], promises
to intervene to uphold that code of conduct.

[CoC]: https://www.rust-lang.org/policies/code-of-conduct
[team]: https://github.com/rust-embedded/wg#the-cortex-m-team
[atlas email]: atlas.songk@gmail.com
