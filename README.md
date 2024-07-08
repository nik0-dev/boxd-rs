# Installing 
- Note: MacOS will need to build or install via **Cargo**.
  
**With Cargo (Recommended):**  
- If you don't already have **Cargo** you can get it [here](https://rustup.rs).
- With **Cargo** you can use the following command in the terminal to install `boxd` so it can be used system-wide:  

```
cargo install github.com/nik0-dev/boxd-rs
```

**Manual Install:**  
- This will require more effort to install `boxd`, but gives you more finite control over where and how its installed. 
- The binaries for Windows and Linux can be found at the [latest releases](https://github.com/nik0-dev/boxd-rs/releases/).

# Usage

`correctly handles prefixed literals (0b/0x/0d/0o), leading zeros, and arguments can be in any order.`

```
boxd - converts between [b]inary, [o]ctal, he[x]adecimal, and [d]ecimal representations.  

Usage: boxd --from <FROM> --to <TO> <VALUE>  

Arguments:  
  <VALUE>  

Options:
  -f, --from <FROM>  [possible values: b, o, x, d]  
  -t, --to <TO>      [possible values: b, o, x, d]  
  -h, --help         Print help  
  -V, --version      Print version
```
  
