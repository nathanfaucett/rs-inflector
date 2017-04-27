#![feature(collections)]
#![no_std]


extern crate collections;

extern crate regex;


mod inflector;
mod rule;


pub use inflector::Inflector;
