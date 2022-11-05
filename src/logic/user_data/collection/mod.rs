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

use std::collections::HashSet;

use serde::{Deserialize, Serialize};

// TODO: implement externally
#[derive(Serialize, Deserialize, Default)]
pub struct BanlistDummy {}

#[derive(Serialize, Deserialize, Clone, Copy, Default, Debug, PartialEq)]
pub enum Action {
    #[default]
    None,
    Add,
    Remove,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, Eq, Hash, PartialEq)]
pub struct Card {
    pub id: u32,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Change {
    pub action: Action,
    pub cards: Vec<Card>,
    pub date: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Collection {
    pub changes: Vec<Change>,
    pub default_banlist: String, // Maybe something more robust than a String
    pub overlay_banlist: BanlistDummy,
}

impl Change {
    pub fn new(action: Action, cards: Vec<Card>, date: String) -> Self {
        Self {
            action: action,
            cards: cards,
            date: date,
        }
    }

    pub fn is_action_add(&self) -> bool {
        match self.action {
            Action::Add => true,
            _ => false,
        }
    }

    pub fn is_action_remove(&self) -> bool {
        match self.action {
            Action::Remove => true,
            _ => false,
        }
    }

    pub fn cards_as_hashset(&self) -> HashSet<Card> {
        HashSet::from_iter(self.cards.iter().cloned())
    }
}

impl Clone for Change {
    fn clone(&self) -> Self {
        Self::new(self.action, self.cards.to_vec(), self.date.to_string())
    }
}

impl Collection {
    // Add a new Change
    pub fn add_change(&mut self, change: Change) {
        self.changes.insert(0, change);
    }

    // Create a new Change and add it
    pub fn new_change(&mut self, action: Action, cards: Vec<Card>) {
        self.add_change(Change::new(
            action, cards, format!("{}", chrono::Local::now().format("%F %R")),
        ));
    }

    // Undo the most recent Change
    pub fn undo_change(&mut self) {
        self.changes.remove(0);
    }

    pub fn flatten_changes(&self) -> Vec<Card> {
        let mut cards = HashSet::new();

        for change in self.changes.iter().rev() {
            if change.is_action_add() {
                cards = &cards | &change.cards_as_hashset();
            } else if change.is_action_remove() {
                cards = &cards - &change.cards_as_hashset();
            }
        }

        return cards.into_iter().collect();
    }
}

#[cfg(test)]
mod tests;
