use std::{future::Future};

pub fn foo<T>(
    executor: impl FnOnce(T) -> dyn Future<Output = ()>,
) -> Box<dyn FnOnce(T) -> dyn Future<Output = ()>> {
    Box::new(executor) //~ ERROR the parameter type
}

fn main() {}
