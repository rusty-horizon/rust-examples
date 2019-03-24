# Rust examples

Containins several examples of the usage of libnx and Rust for devKitPro + Rust homebrew development.

## Building

Install Rust and prepare the environment according to [our guide](https://github.com/rusty-horizon/setup-guide).

Then, just get into the project's directory and hit `make`.

### Output

Binaries will be generated in `target/<toolchain>/<mode>/<name>.{nro/nso/nacp/elf/pfs0}`.

Toolchain is `aarch64-none-elf` by default, mode can be either `debug` or `release` and the name is specified in Cargo.toml.