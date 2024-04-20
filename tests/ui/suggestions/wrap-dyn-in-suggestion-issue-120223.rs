#![feature(dyn_star)] //~ WARNING the feature `dyn_star` is incomplete

use std::future::Future;

pub fn dyn_func<T>(
    executor: impl FnOnce(T) -> dyn Future<Output = ()>,
) -> Box<dyn FnOnce(T) -> dyn Future<Output = ()>> {
    Box::new(executor) //~ ERROR may not live long enough
}

pub fn dyn_star_func<T>(
    executor: impl FnOnce(T) -> dyn* Future<Output = ()>,
) -> Box<dyn FnOnce(T) -> dyn* Future<Output = ()>> {
    Box::new(executor) //~ ERROR may not live long enough
}

fn main() {}
