#![feature(test)]
extern crate chrono;
extern crate item05;
extern crate test;

use item05::*;
use chrono::*;

#[bench]
fn functional(b: &mut test::Bencher) {
    let boomer = UTC.ymd(1946, 4, 2);
    b.iter(|| {
        test::black_box(is_baby_boomer(&boomer))
    })
}

#[bench]
fn performance_implementation(b: &mut test::Bencher) {
    let boomer = UTC.ymd(1946, 4, 2);
    let checker = BabyBoomChecker::new();
    b.iter(|| {
        test::black_box(checker.is_baby_boomer(&boomer))
    })
}