#[macro_use]
extern crate aoc_runner_derive;
extern crate jemallocator;

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

mod day1;
// pub mod day2;
mod day2_fast;
mod day3;

aoc_lib! { year = 2020 }
