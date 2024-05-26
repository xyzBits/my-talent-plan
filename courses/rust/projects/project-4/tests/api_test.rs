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


    fn consume_with_relish<F>(func: F)// move occurs because func has type F, which does not implement the Copy trait
        where F: FnOnce(u32) -> String {
        // func consumes its captured variables, so it cannot be run more than once
        println!("Consumed: {}", func(1));// func moved due to this call

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
        where F: FnOnce(usize) -> bool + Copy {
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
            Box::new(Sheep{})
        } else {
            Box::new(Cow{})
        }
    }

    #[test]
    fn test_random_animal() {
        let random_number = 0.345;
        let animal = random_animal(random_number);
        println!("{}", animal.noise());

    }
}