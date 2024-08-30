use std::string::String;
fn multi_thread() {
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
       let message = String::from("Hello from another thread!");
        tx.send(message).unwrap();
        thread::sleep(Duration::from_millis(500));
    });

    let received = rx.recv().unwrap();
    println!("Received: {}", received);
}

#[allow(unused)]
pub(crate) fn multi_async() {
    multi_thread();
}