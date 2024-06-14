#[test]
fn panic_test() {
    std::panic::set_hook(Box::new(|panic_info| {
        println!("{}", panic_info);
    }));
}


#[cfg(test)]
mod closure_test {
    /// Instances of FnOnce can be called, but might not be callable
    /// multiple times. Because of this, if the only thing known about a
    /// type is that it implements FnOnce, it can only be called once.
    ///
    /// FnOnce is implemented automatically by closures that might consume
    /// captured variables


    fn consume_with_relish<F>(func: F) // move occurs because func has type F, which does not implement the Copy trait
    where
        F: FnOnce(u32) -> String,
    {
        // func consumes its captured variables, so it cannot be run more than once
        println!("Consumed: {}", func(1)); // func moved due to this call

        println!("Delicious");

        // attempting to invoke func again will throw a use of moved value error for func
        // value used here after move
        // func();
    }

    #[test]
    fn fn_once_test() {
        let x = String::from("x");
        // let consume_and_return_x: fn() -> String = move || x;
        let consume_and_return_x = move |y| x;
        consume_with_relish(consume_and_return_x);

        // consume_and_return_x can no longer be invoked at this point
    }

    #[test]
    fn hello_closure() {
        let x = 1;
        let sum = |y| x + y;
        assert_eq!(3, sum(2));
    }


    fn fn_once<F>(func: F)
    where
        F: FnOnce(usize) -> bool + Copy,
    {
        println!("{:?}", func(3));
        println!("{:?}", func(4));
    }

    #[test]
    fn test_fn_once() {
        let x = vec![1, 2, 3];
        fn_once(|z| { z == x.len() });
    }

    #[test]
    fn test_fn_mute() {
        let mut some_vec = vec![1, 2, 3, 4];

        let mut update_vec = || {
            some_vec.push(5);
        };

        update_vec();
        update_vec();

        println!("{:?}", some_vec);
    }
}

/// Types that can be transferred across thread boundaries.
/// This trait is automatically implemented when the compiler
/// determines it's appropriate.
///
/// An example of a non-Send type is the reference-counting pointer rc::Rc.
#[test]
fn send_test() {}

#[cfg(test)]
mod test_trait_object {
    struct Sheep {}

    struct Cow {}

    trait Animal {
        // Instance method signature
        fn noise(&self) -> &'static str;
    }

    impl Animal for Sheep {
        fn noise(&self) -> &'static str {
            "baaaaaah"
        }
    }

    impl Animal for Cow {
        fn noise(&self) -> &'static str {
            "moooooo"
        }
    }

    // Returns some struct that implements animal, but we don't know which one at compile time
    fn random_animal(random_number: f64) -> Box<dyn Animal> {
        if random_number < 0.5 {
            Box::new(Sheep {})
        } else {
            Box::new(Cow {})
        }
    }

    #[test]
    fn test_random_animal() {
        let random_number = 0.345;
        let animal = random_animal(random_number);
        println!("{}", animal.noise());
    }
}

#[cfg(test)]
mod test_channel {
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::Duration;

    fn std_sync_mpsc_channel() {

        // multiple producer and single consumer
        // let (receiver, sender) = std::sync::mpsc::channel();
    }

    #[test]
    fn test() {
        let apple = Arc::new("the same apple");

        for _ in 0..10 {
            let apple = Arc::clone(&apple);

            thread::spawn(move || {
                println!("{}", apple);
            });
        }

        thread::sleep(Duration::from_secs(1));
    }


    #[test]
    fn test_add() {
        let data = Arc::new(Mutex::new(0));

        let mut handles = vec![];
        for _ in 0..100 {
            let data = Arc::clone(&data);
            let handle = thread::spawn(move || {
                let mut guard = data.lock().unwrap();
                *guard += 1
            });

            handles.push(handle);
        }


        for handle in handles {
            handle.join().unwrap();
        }

        println!("{:?}", data);
    }
}


#[cfg(test)]
mod test_arc {
    use std::sync::Arc;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_arc_1() {
        let foo = Arc::new(vec![1, 2, 3, 4]);


    }

    // When shared ownership between threads is need, Arc (atomically Reference Counted)
    // can be used, This struct via the Clone implementation can create a reference pointer
    // for the location of a value in the memory heap while increasing the reference counter.
    // As it shares ownership between threads, when the last reference pointer to a value
    // is out of scope, the variable is dropped.

    #[test]
    fn test_share_ownership_between_threads() {
        let apple = Arc::new("the same apple");

        for _ in 0..10 {
            // Here there is no value specification as it is a pointer to a
            // reference in the memory heap.
            let apple = Arc::clone(&apple);

            thread::spawn(move || {
                // As arc was used, threads can be spawned using the valued allocated
                // in the Arc variable pointer's location.
                println!("{}", apple);
            });
        }

        // Make sure all arc instances are printed from spawned threads.
        thread::sleep(Duration::from_secs(1));
    }















}
