# 🥧 Pi-Time
We all know pi time is the best time, but when is it? Use this cli-tool to quickly find out and never miss out on pi time again.

## Usage
Running the `pi-time` binary will output either "Its pi time" or the remaining time until pi time. Run with `-h` to view other options.

```
> pi-time
10h 28m 42s until pi time
```

## Building
Building pi-time requires a copy of the rust compiler. You can install it using the rust toolchain available [here](https://www.rust-lang.org/tools/install).

1. Clone locally

```bash
git clone https://github.com/giraugh/pi-time
```
2. Build the crate

```bash
cd pi-time
cargo build
```
3. Optionally, install `pi-time` to your PATH.

```
cargo install --path .
```

## Contributing
If you encounter any issues or bugs, please leave an issue on Github. Additionally, all and any PRs are welcome :)


## Copyright
Ewan Breakey - MIT 2022
