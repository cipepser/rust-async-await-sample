#![allow(non_snake_case)]

use runtime::time::Delay;
use std::{thread, time};

async fn foo1() {
    for i in 0..10 {
        Delay::new(time::Duration::from_millis(500)).await;
        println!("foo1:{} Thread[{:?}]", i, thread::current().id());
    }
}

async fn foo2() {
    for i in 0..10 {
        Delay::new(time::Duration::from_millis(500)).await;
        println!("foo2:{} Thread[{:?}]", i, thread::current().id());
    }
}

#[runtime::main]
async fn main() {
    let handles = &mut [runtime::spawn(foo1()), runtime::spawn(foo2())];
    for handle in handles {
        handle.await;
    }
}
