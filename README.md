# pt / libpt

`pt` stands for either one of "personal tool", "plex tool", "pete" or something among those lines.
It is a collection of tools that i might or might not use. The intended purpose of this repo is that
I program whatever i feel is worth having in a personal thing into it, then use it as either a lib,
crate, python module or executable.

Let's see if I make it a bloated mess or stop committing after 30 hello worlds.

#### But the name `pt` / `libpt` already exists!
So what? I don't care. Besides, there is not enough names to name everything unique.

## Dependencies
- See `cargo.toml`
- [Python](https://www.python.org/)
- [`maturin`](https://maturin.rs) - `pip install maturin`

## Compiling & Installing from source
If you only want the rust library, you can simply build it with `cargo build`.
If you want to use the python variant too, you need to compile with maturing.

- Install in `venv`: `maturin develop --release`
- Install in system: `maturin build --release && pip install target/wheels/libpt-x.x.x-*`

## Installing from [crates.io](https://crates.io)
`libpt` has not yet been packaged for [crates.io](https://crates.io).

## Installing from my personal package registry
`libpt` has not yet been packaged for [git.cscherr.de](https://git.cscherr.de).

## Testing
Testing needs to be done separately for the rust and python parts:

- Rust testing with `cargo test`
- Python testing with `./scripts/pytests.sh` or `python -m unittest discover -fs tests/python`

## Documentation
The documentation can be automatically generated with `cargo doc --open`.

## Mirrored
The origin of this repository is [git.cscherr.de](https://git.cscherr.de/PlexSheep/pt)

It is mirrored to:
- [GitHub](https://github.com/PlexSheep/pt)
- [Codeberg](https://codeberg.org/PlexSheep/pt)


## License
##### GPL-3 or newer.
