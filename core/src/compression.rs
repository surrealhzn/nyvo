use std::io::{Read, Write};

pub enum Algorithm {}

pub fn decompress(algo: Algorithm, source: &mut dyn Read, target: &mut dyn Write) {}
