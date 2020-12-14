#![allow(dead_code)]
#![feature(str_split_once)]
#[macro_use]
extern crate aoc_runner_derive;
extern crate jemallocator;

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[inline(always)]
pub fn parse_usizes(input: &[u8]) -> Vec<usize> {
    input
        .split(|&b| b == b'\n')
        .map(|line| parse_usize(line))
        .collect()
}

#[inline(always)]
pub fn parse_usize(input: &[u8]) -> usize {
    input.iter().fold(0, |a, &c| a * 10 + (c & 0x0f) as usize)
}

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

aoc_lib! { year = 2020 }
