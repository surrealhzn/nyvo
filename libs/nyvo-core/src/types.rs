use std::io::{Read, Seek, Write};

pub trait Rs: Read + Seek {}
impl<T: Read + Seek> Rs for T {}

pub trait Ws: Write + Seek {}
impl<T: Write + Seek> Ws for T {}

pub trait Rw: Read + Write {}
impl<T: Read + Write> Rw for T {}

pub trait Rws: Read + Write + Seek {}
impl<T: Read + Write + Seek> Rws for T {}
