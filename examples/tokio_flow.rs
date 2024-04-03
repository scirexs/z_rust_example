use std::future::Future;
use tokio::{time::{sleep, Duration}, spawn, join, select};

#[tokio::main]
async fn main() {
    let (pf1, pf2) = reload_print_func();
    // Parallel run async functions as carefree.
    spawn(pf1);
    spawn(pf2);

    let (sf1, sf2) = reload_string_func();
    // Concurrent run, and sync results of all async funcs.
    let (s1, s2) = join!(sf1, sf2);
    println!("{}\n{}", s1, s2);

    let (sf1, sf2) = reload_string_func();
    // Parallel run, and sync results of all async funcs.
    let h1 = spawn(sf1);
    let h2 = spawn(sf2);
    if let (Ok(s1), Ok(s2)) = join!(h1, h2) {
        println!("{}\n{}", s1, s2);
    }

    let (sf1, sf2) = reload_string_func();
    // Parallel run, and get one result of some async funcs.
    select! {
        s1 = sf1 => { println!("{}", s1); },
        s2 = sf2 => { println!("{}", s2); },
        else => { println!("Any async funcs were not successed."); },
    }

    // Hold until end of async func
    hold_program().await;
    println!("Finish program");
}

async fn delay_print_1000() {
    sleep(Duration::from_millis(1000)).await;
    println!("Hello after 1000");
}
async fn delay_print_2000() {
    sleep(Duration::from_millis(2000)).await;
    println!("Hello after 2000");
}
async fn delay_string_1000() -> String {
    sleep(Duration::from_millis(1000)).await;
    String::from("Hello after 1000")
}
async fn delay_string_2000() -> String {
    sleep(Duration::from_millis(2000)).await;
    String::from("Hello after 2000")
}
async fn hold_program() {
    sleep(Duration::from_millis(5000)).await;
}
fn reload_print_func() -> (impl Future<Output = ()>, impl Future<Output = ()>) {
    (delay_print_1000(), delay_print_2000())
}
fn reload_string_func() -> (impl Future<Output = String>, impl Future<Output = String>) {
    (delay_string_1000(), delay_string_2000())
}