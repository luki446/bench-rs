#![feature(test)]

extern crate test;
use self::test::Bencher;
use std::option::Option;

const COUNT: usize = 25;

fn fibonacci_recursive(n: usize) -> u64
{
    match n
    {
        1 | 2=> {return 1;}
        _ =>
        {
            return fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2);
        }
    }
}

fn fibonacci_better(n: usize) -> u64
{
    let mut curr: u64 = 1;
    let mut next: u64 = 1;
    for _ in 1..=n
    {
        let new_next = curr + next;
        curr = next;
        next = new_next;
    }

    curr
}

struct Fibonacci
{
    curr: u64,
    next: u64
}

impl Iterator for Fibonacci
{
    type Item = u64;

    fn next(&mut self) -> Option<u64>
    {
        let new_next = self.curr + self.next;
        self.curr = self.next;
        self.next = new_next;

        Some(self.curr)
    }
}

fn fibonacci() -> Fibonacci
{
    Fibonacci
    {
        curr : 1,
        next : 1
    }
}

#[bench]
fn recursive(b: &mut Bencher) {
    b.iter(|| (1..COUNT).map(fibonacci_recursive).collect::<Vec<u64>>())
}

#[bench]
fn better(b: &mut Bencher) {
    b.iter(|| (1..COUNT).map(fibonacci_better).collect::<Vec<u64>>())
}

#[bench]
fn iterator(b: &mut Bencher) {
    b.iter(|| fibonacci().take(COUNT).collect::<Vec<u64>>())
}
