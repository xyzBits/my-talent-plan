use std::sync::{Arc, Barrier};
use std::thread;

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
    use std::sync::{Arc, Mutex};
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

    #[test]
    fn test_arc_mutex() {
        // create my data, wrap it in a mutex, then add atomic reference counting
        let my_data = vec![1, 2, 3];
        let my_data = Mutex::new(my_data);
        let my_data = Arc::new(my_data);

        // spawn a thread that will update the values,
        // a clone of our Arc will be moved into the thread
        let thread_arc = my_data.clone();
        let t1 = thread::spawn(move || {
            println!("Thread 1 attempting to acquire lock.....");

            if let Ok(mut x) = thread_arc.lock() {
                println!("Thread 1 acquired lock");
                for num in x.iter_mut() {
                    *num += 1;
                }

                // simulate some long-running work
                thread::sleep(Duration::from_millis(750));
            }

            println!("Thread 1 dropped lock");

            // Do something else with the data
            thread::sleep(Duration::from_millis(900));
        });


        let thread_arc = my_data.clone();
        let t2 = thread::spawn(move || {
            println!("Thread 2 attempting to acquire lock...");
            if let Ok(mut x) = thread_arc.lock() {
                println!("Thread 2 acquired lock");
                for num in x.iter_mut() {
                    *num *= 2;
                }

                // simulate some long-running work
                thread::sleep(Duration::from_millis(1250));
            }

            println!("Thread 2 dropped lock");
            thread::sleep(Duration::from_millis(1100));
        });

        t1.join().unwrap();
        t2.join().unwrap();

        let my_data = my_data.lock().unwrap();

        println!("Values are: {:?}", my_data);
    }
}

#[test]
fn test_barrier() {
    // Step 1: place a Barrier in an Arc, and clone one Arc into each element of an array.
    let thread_count = 8;
    let shared = Barrier::new(thread_count);
    let mut v = vec![];
    let arc = Arc::new(shared);

    for _ in 0..8 {
        v.push(arc.clone());
    }

    // Step 2: loop over vector elements, and create thread for each element.
    let mut children = vec![];
    for item in v {
        children.push(thread::spawn(move || {
            // Step 3: run some code before the wait() function is called on the Barrier, and some after.
            println!("Part A");
            item.wait();
            println!("Part B");
        }));
    }

    // Step 4: join all threads
    for child in children {
        child.join().unwrap();
    }
}


#[test]
fn test_barrier_block_thread() {
    let n = 10;
    let mut handles = Vec::with_capacity(n);
    let barrier = Arc::new(Barrier::new(n));

    for _ in 0..n {
        let c = Arc::clone(&barrier);
        handles.push(thread::spawn(move || {
            println!("before await");
            c.wait();
            println!("after wait");
        }));
    }



    for handle in handles {
        handle.join().unwrap();
    }


}

#[cfg(test)]
mod cell_test {
    use std::cell::Cell;

    #[derive(Debug)]
    struct Person {
        name: String,
        age: u32,
    }

    #[test]
    fn test_mutability() {
        let mut person = Person {
            name: "bitch".to_string(),
            age: 23
        };

        println!("{:?}", person);
        person.age = 344;
        println!("{:?}", person);

    }

    #[derive(Debug)]
    struct CellPerson {
        // 一个人的名字从出生就决定了，不提供后续修改，
        name: String,
        // 而年龄每一年都在变，提供后续 修改
        age: Cell<u32>,
    }

    #[test]
    fn test_cell_mutate() {
        let person = CellPerson {
            name: "bitch".to_string(),
            age: Cell::new(21),
        };

        println!("{:?}", person);

        person.age.set(312);
        println!("{:?}", person);
    }
}

#[cfg(test)]
mod ref_cell_test {
    use std::cell::RefCell;

    #[derive(Debug)]
    struct User {
        id: u32,
        year_registered: u32,
        username: String,
        active: RefCell<bool>,
    }
    #[test]
    fn test_cell() {
        let bitch = User {
            id: 1,
            year_registered: 2029,
            username: "bitch".to_string(),
            active: RefCell::new(true),
        };
        println!("{:?}", bitch.active);
    }
}


#[cfg(test)]
mod box_test {
    use std::ops::Deref;

    #[test]
    fn test_1() {
        let b = Box::new(1);
        let k = b.deref();
        println!("k = {}", k);
        assert_eq!(*k, 1);
    }
}










