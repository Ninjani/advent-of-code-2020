#![feature(str_split_once)]
#[macro_use]
extern crate aoc_runner_derive;
extern crate jemallocator;

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

mod day1;
mod day2_fast;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

aoc_lib! { year = 2020 }
