# YGO Destiny

This project is in a pre-alpha state and not functional yet.

## Build instructions
Make sure you have everything on this list installed:
- rust (tested on 1.64.0)
- python3 (tested on 3.10)
- libadwaita
- everything listed [here](https://gtk-rs.org/gtk4-rs/git/book/installation.html)

Before compiling the project, run `utils/py2xml/project.py`.

```
python3 ./utils/py2xml/project.py
```

This only has to be done once if nothing in `utils/py2xml/` is changed.

To build the binary use
```
cargo build
```
or
```
cargo build --release
```
for performance optimisations.

The resulting executable is located in `target/debug/ygo_destiny` or `target/release/ygo_destiny` for release builds.