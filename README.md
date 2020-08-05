# My first embedded rust

Based off of the `cortex-m-quickstart` template: https://github.com/rust-embedded/cortex-m-quickstart

## Requires:

Followed this:
https://docs.rust-embedded.org/discovery/03-setup/macos.html

```
rustup target add thumbv7em-none-eabihf
```


```
$ # Arm GCC toolchain
$ brew tap ArmMbed/homebrew-formulae
$ brew install arm-none-eabi-gcc

$ # Minicom and OpenOCD
$ brew install minicom openocd
```


## Building:

``` console
$ cargo build
```

## Loading onto a board:

### Step 1

Starting an open OpenOCD session:

```
openocd
```

### Step 2

Load and run the program via gdb:

```
cargo run
```


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
