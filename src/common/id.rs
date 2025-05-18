use std::hash::Hash;

pub trait Id: Hash + Eq + Copy {}
