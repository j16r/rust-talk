use std::thread::Thread;
use std::sync::mpsc;

fn fib(n: u64) -> u64 {
    if n < 2 {
        return n;
    }
    return fib(n - 1) + fib(n - 2);
}

fn main() {
    let (tx, rx) = mpsc::channel();

    Thread::spawn(move || {
        let result = fib(50);
        tx.send(result);
    });

    let result = rx.recv();
    println!("Fib(10) = {:?}", result);
}
