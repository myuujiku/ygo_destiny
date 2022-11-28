/*
YGO Destiny – A Yu-Gi-Oh! sealed draft simulator written in rust.
Copyright (C) 2022  myujiku

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License version 3 as
published by the Free Software Foundation.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use super::*;

fn collection() -> Collection {
    Collection::default()
}

fn vecs_eq<T: Eq + std::hash::Hash>(vec1: Vec<T>, vec2: Vec<T>) -> bool {
    let a: HashSet<_> = vec1.iter().collect();
    let b: HashSet<_> = vec2.iter().collect();

    return a == b;
}

#[test]
fn add_change() {
    let mut collection = collection();
    let change = Change::new(Action::Add, vec![Card { id: 123 }], "42".to_string());

    collection.add_change(change.clone());
    let added_change = &collection.changes[0];
    assert_eq!(added_change.action, change.action);
    assert_eq!(added_change.cards, change.cards);
    assert_eq!(added_change.date, change.date);
    assert_eq!(added_change.cards[0].id, change.cards[0].id)
}

#[test]
fn flatten_changes() {
    let mut collection = collection();

    collection.new_change(
        Action::Add,
        vec![Card { id: 1 }, Card { id: 2 }, Card { id: 3 }],
    );
    collection.new_change(Action::Remove, vec![Card { id: 2 }]);
    assert_eq!(
        true,
        vecs_eq(
            collection.flatten_changes(),
            vec![Card { id: 1 }, Card { id: 3 }]
        )
    );

    collection.new_change(Action::Remove, vec![Card { id: 4 }]);
    assert_eq!(
        true,
        vecs_eq(
            collection.flatten_changes(),
            vec![Card { id: 1 }, Card { id: 3 }]
        )
    );

    collection.new_change(Action::Remove, vec![Card { id: 3 }]);
    assert_eq!(
        true,
        vecs_eq(collection.flatten_changes(), vec![Card { id: 1 }])
    );
}
