# YGO Destiny

This project is in a pre-alpha state and not functional yet.

## Build instructions
Make sure you have everything on this list installed:
- rust (tested on 1.64.0)
- python3 (tested on 3.10)
- libadwaita
- everything listed [here](https://gtk-rs.org/gtk4-rs/git/book/installation.html)

Before compiling the project, run `py2xml/project.py`.

```
python3 ./py2xml/project.py
```

This only has to be done once if nothing in `py2xml/` is changed.

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

## To Do

Things that still need to be done in no particular order.

### Project structure

- [ ] Unit tests
- [ ] Logging
- [ ] Documentation
- [ ] .desktop entry
- [ ] Installation (makefile?)
- [ ] Persistent application settings

### Features
- [ ] Card draft
- [ ] Card info viewer + target lock
- [ ] Collections
- [ ] Settings page
- [ ] Banlists
- [ ] Layered banlists
- [ ] Collection editor
- [ ] Deck editor
- [ ] Progression mode
- [ ] Pack shop system
