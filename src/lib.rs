#![feature(collections)]
#![no_std]


extern crate collections;

extern crate regex;

extern crate vector;
extern crate stack;


mod inflector;
mod rule;


pub use inflector::Inflector;
