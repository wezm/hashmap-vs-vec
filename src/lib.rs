use rustc_hash::FxHashMap;
use std::collections::HashMap;

pub fn vec(i: u16, collection: &Vec<u16>) -> u16 {
    collection.iter().position(|&glyph| glyph == i).unwrap_or(0) as u16
}

pub fn fxmap(i: u16, collection: &FxHashMap<u16, u16>) -> u16 {
    collection.get(&i).copied().unwrap_or(0)
}

pub fn map(i: u16, collection: &HashMap<u16, u16>) -> u16 {
    collection.get(&i).copied().unwrap_or(0)
}
