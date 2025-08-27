use ic_stable_structures::{DefaultMemoryImpl, memory_manager::MemoryManager};
use std::{cell::RefCell, io::Write};

thread_local! {
    // The memory manager enables multiple virtual memories in one stable memory.
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));
}

#[ic_cdk::update]
fn greet(name: String) -> String {
    // append the name to a greetings file
    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open("greetings.txt")
        .unwrap();

    writeln!(file, "{name}").unwrap();

    // Write some message into the DFX debug console
    println!("Hello from WASI: {name}");

    // Return some value
    format!("Hello, {name}!")
}

#[ic_cdk::query]
fn log() -> Vec<String> {
    let file = std::fs::File::open("greetings.txt").unwrap();

    let reader = std::io::BufReader::new(file);

    std::io::BufRead::lines(reader)
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
}

#[ic_cdk::init]
fn init() {
    MEMORY_MANAGER.with(|m| {
        let m = m.borrow();
        // initialize file system with the memory manager
        ic_wasi_polyfill::init_with_memory_manager(&[0u8; 32], &[], &m, 101..119);
    });
}

#[ic_cdk::post_upgrade]
fn post_upgrade() {
    init();
}
