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

pub fn in_ty_param<F: FnOnce() -> &'static dyn std::fmt::Debug>(f: F) {
    f();
    f(); //~ ERROR use of moved value
}

fn with_sized<T: Fn() -> &'static (dyn std::fmt::Debug) + ?Sized>() {
    without_sized::<T>();
    //~^ ERROR the size for values of type `T` cannot be known at compilation time
}

fn without_sized<T: Fn() -> &'static dyn std::fmt::Debug>() {}

fn main() {}
