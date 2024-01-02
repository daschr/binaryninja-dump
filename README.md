# Binary Ninja Plugin for dumping a selection of bytes
## Usage
- select bytes in the *linear* or *hex editor* view
- *right click* -> *Plugins* -> *Dump to file*
![pic](https://raw.githubusercontent.com/daschr/binaryninja-dump/main/dump.png)
## Installation
- install rust and cargo (https://www.rust-lang.org/tools/install)
- `cargo b --release`
- copy `./target/release/libdump.so` or `./target/release/libdump.dll` to
  | OS      | Plugin Path                                          |
  |---------|------------------------------------------------------|
  | Linux   | `~/.binaryninja/plugins`                             |
  | MacOS   | `~/Library/Application Support/Binary Ninja/plugins` |
  | Windows | `%AppData%\Binary Ninja\plugins`                     |
