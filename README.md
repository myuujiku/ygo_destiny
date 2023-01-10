# YGO Destiny

This project is in a pre-alpha state and not functional yet.

## Documentation
The documentation is currently *not* available online and still work in progress.

To build the documentation yourself run
```
cargo doc --document-private-items
```
in the root of this repository.

If you don't care about private definitions you can run
```
cargo doc
```
instead.

## Build instructions
Make sure you have everything on this list installed:
- rust (tested on 1.66.0)
- libadwaita
- everything listed [here](https://gtk-rs.org/gtk4-rs/git/book/installation.html)

Clone the repository:
```
git clone --recursive https://github.com/myuujiku/ygo_destiny.git
```

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

### First functional alpha version roadmap
- [ ] Collections
- [ ] Card info viewer
- [ ] Simple collection menu
- [ ] First start setup

### Project structure
- [ ] Front end documentation
- [ ] Unit tests
- [ ] Logging
- [ ] .desktop entry
- [ ] Installation (makefile?)

### Features
- [ ] Card draft
- [ ] Optional draft timer
- [ ] Card info viewer + target lock
- [ ] Collections
- [ ] Settings page
- [ ] Banlists
- [ ] Layered banlists
- [ ] Collection editor
- [ ] Deck editor
- [ ] Progression mode
- [ ] Pack shop system
