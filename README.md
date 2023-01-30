![Discord](https://img.shields.io/discord/1069639833816932413?color=%237289DA&label=Discord&logo=discord&logoColor=%23FFFFFF&style=flat-square)
![GitHub](https://img.shields.io/github/license/myuujiku/ygo_destiny?logo=gnu&logoColor=%23FFFFFF&style=flat-square)

<div align="center"><img src="resources/icons/hicolor/scalable/com.myujiku.ygo_destiny.svg"><h1>YGO Destiny</h1></div>


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

## Roadmap

Features with high priority are marked with a *.

- [ ] Collections
    - [ ] * Collection list
    - [ ] * Collection creation
    - [ ] * Choose between different draft types
        - [ ] * Choice Draft
        - [ ] Standard Draft (with rarities)
        - [ ] Battle Pack 1 like Draft
        - [ ] Shop Draft (makes more sense in online multiplayer)
        - [ ] Cube Draft (online multiplayer)
    - [ ] * Add option to add different types of drafts to the collection menu (e.g. a main draft, bonus draft, bonus card from the last set, ...)
    - [ ] * Banlists
        - [ ] Custom banlists
        - [ ] Banlist presets
        - [ ] Follow current set
    - [ ] * Collection editor
    - [ ] * Collection history
    - [ ] * User defined card categories (e.g. a "staples" category where the user adds their best generic cards to)
    - [ ] * Set selection menu
        - [ ] Basic set selection
        - [ ] Filter by
            - [ ] Name
            - [ ] Categories
                - [ ] ygod_core: assign sensible categories to all sets
            - [ ] Date range
            - [ ] Series (via categories)
        - [ ] Sort by
            - [ ] Date
            - [ ] Name
            - [ ] Number of cards
        - [ ] Set grouping
            - [ ] Manual
            - [ ] Automated
            - [ ] Option for deciding what to do with grouped sets (combine/chose/separate)
- [ ] Draft
    - [ ] Make draft types function
        - [ ] * Choice Draft
        - [ ] Standard Draft
            - [ ] Implement rarities in the RNG
            - [ ] Rarity editor
                - [ ] Rarity slots
                    - [ ] Box slot (makes sure rarities appears exactly a predefined number of times)
                    - [ ] Chance slot (assigns probabilities to rarities)
                    - [ ] Support multiple pack structures (e.g. 45% chance for a pack with 10 commons, 45% for a pack of 7 cards and one SR or higher and 10% for a pack of 5 cards and only cards that are SR or higher)
        - [ ] Battle Pack Draft
            - [ ] Separate editor
    - [ ] * Card info viewer
    - [ ] Show rarities on cards
- [ ] Deck Editor
    - [ ] Tabs for searches
    - [ ] Integration with collections
    - [ ] Sort/Filter menu
    - [ ] Adjustable main/extra/side deck sizes
    - [ ] Adjustable deck requirements
        - [ ] Special rules like all monsters have to be from the same archetype
- [ ] * Export collections
    - [ ] * As banlist (lflist.conf)
    - [ ] As YGOPD collection
    - [ ] As deck (.ydk, .ydke)
    - [ ] As plain text card list (e.g. 1x Raigeki [newline] 2x Breaker the Magical Warrior ...)
- [ ] * First start setup screen
    - [ ] Adjust card size
    - [ ] Select image size (small/big)
    - [ ] Select theme (dark/light)
    - [ ] Small introduction to the application's features
    - [ ] Language settings (or only in the settings menu)
- [ ] User/App templates
    - [ ] Collection settings
    - [ ] Sets + rarities
- [ ] Project structure
    - [ ] Better documentation
    - [ ] Unit tests
    - [ ] Logging
