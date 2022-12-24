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

#[derive(Clone, Debug, Default)]
pub struct DraftOptions {
    pub rounds: i32,
    pub cards: i32,
    pub follow_sets: bool,
    pub rotate_sets: bool,
    pub rotate_after: i32,
}

#[derive(Clone, Debug, Default)]
pub struct CollectionOptions {
    pub draft_options: DraftOptions,
}
