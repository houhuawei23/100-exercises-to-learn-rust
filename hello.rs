fn print_vec(v: &Vec<i32>) {
    for n in v.iter() {
        println!("{}", n);
    }
}

fn main() {
    // test_thread()
    // test_move()
    test_scope()
}

fn test_iter() {
    println!("Hello, world!");
    let mut arr = vec![1, 2, 3];
    for mut n in arr.iter() {
        // n = &5;
        // *n = 4;
        println!("{}", n);
    }
    print_vec(&arr);
    for n in arr.iter_mut() {
        *n = 4;
        println!("{}", n);
    }
    print_vec(&arr);

    for n in arr {
        println!("{}", n);
    }
}
use std::thread;
use std::time::Duration;

fn test_thread() {
    let handle = thread::spawn(|| loop {
        thread::sleep(Duration::from_secs(1));
        println!("Hello from a thread!");
    });

    loop {
        thread::sleep(Duration::from_secs(2));
        println!("Hello from the main thread!");
    }
}

fn test_join() {
    let handle = thread::spawn(|| {
        println!("Hello from a thread!");
    });

    handle.join().unwrap();
}

fn create_fn() -> impl Fn() {
    let text = "Fn".to_owned();
    move || println!("This is a: {text}")
}

fn test_move() {
    let fn_plain = create_fn();
    // fn_plain();
}

fn test_scope() {
    let v = vec![1, 2, 3];
    let midpoint = v.len() / 2;

    std::thread::scope(|scope| {
        scope.spawn(|| {
            let first = &v[..midpoint];
            println!("Here's the first half of v: {first:?}");
        });
        scope.spawn(|| {
            let second = &v[midpoint..];
            println!("Here's the second half of v: {second:?}");
        });
    });

    println!("Here's v: {v:?}");
}
