fn main() {
    ex_run_threads();
    ex_mutex();
}

fn ex_run_threads() {
    use std::thread;

    // Define thread task
    let handle = thread::spawn(|| {
        println!("Hello from a thread!");
    });

    // Join spawned thread
    handle.join().unwrap();
}

fn ex_mutex() {
    use std::sync::Mutex;

    // Create mutex resource
    let m = Mutex::new(5);

    {
        // Lock mutex
        let mut num = m.lock().unwrap();
        // Modify
        *num = 1000;
    }

    println!("m = {:?}", m);
}
