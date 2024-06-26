use std::sync::atomic::AtomicI32;
use thread_priority::*;

static LOW_COUNT: AtomicI32 = AtomicI32::new(0);
static MEDIUM_COUNT: AtomicI32 = AtomicI32::new(0);
static HIGH_COUNT: AtomicI32 = AtomicI32::new(0);

fn low_priority() {
    set_current_thread_priority(ThreadPriority::Min).unwrap();
    loop {
        LOW_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        std::thread::yield_now();
    }
}

fn medium_priority() {
    loop {
        MEDIUM_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        std::thread::yield_now();
    }
}

fn high_priority() {
    set_current_thread_priority(ThreadPriority::Max).unwrap();
    loop {
        HIGH_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        std::thread::yield_now();
    }
}

fn main() {
    std::thread::spawn(low_priority);
    std::thread::spawn(medium_priority);
    std::thread::spawn(high_priority);
    std::thread::sleep(std::time::Duration::from_millis(5000));

    println!(
        "LOW: {}",
        LOW_COUNT.load(std::sync::atomic::Ordering::Relaxed)
    );
    println!(
        "MEDIUM: {}",
        MEDIUM_COUNT.load(std::sync::atomic::Ordering::Relaxed)
    );
    println!(
        "HIGH: {}",
        HIGH_COUNT.load(std::sync::atomic::Ordering::Relaxed)
    );
}
