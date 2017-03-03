#![feature(collections)]
#![no_std]


extern crate collections;

extern crate regex;
extern crate vector;
extern crate collection_traits;


mod inflector;
mod rule;


pub use inflector::Inflector;
