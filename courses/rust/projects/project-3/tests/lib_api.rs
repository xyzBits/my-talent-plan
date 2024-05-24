use std::ffi::OsStr;

mod macro_tests {
    #[macro_export]
    macro_rules! my_vec {
    ($( $x: expr );* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
            temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
    #[test]
    fn test_vec_macro() {
        let my_vec = my_vec!(1; 2; 3; 4);
        println!("{:?}", my_vec);

        let my_vec = my_vec!["hello"; "world"];

        println!["{:?}", my_vec];
    }
}


#[test]
fn test_str_ref() {
    let hello = "hello";

    // return type is generic, so you should specify the generic type at return value
    let output: &OsStr = hello.as_ref();
}

#[cfg(test)]
mod test_iterator {
    struct Fibonacci {
        curr: u32,
        next: u32,
    }

    impl Iterator for Fibonacci {
        type Item = u32;
        fn next(&mut self) -> Option<Self::Item> {
            let current = self.curr;

            self.curr = self.next;
            self.next = current + self.next;

            Some(current)
        }
    }

    fn fibonacci() -> Fibonacci {
        Fibonacci {
            curr: 0,
            next: 1,
        }
    }

    #[test]
    fn test_iterator() {
        let mut sequence = 0..3;


        println!("{:?}", sequence.next());
        println!("{:?}", sequence.next());
        println!("{:?}", sequence.next());
        println!("{:?}", sequence.next());
        println!("{:?}", sequence.next());

        for x in fibonacci().take(4) {
            println!("{}", x);
        }

        println!("=======================");

        for x in fibonacci().skip(4).take(4) {
            println!("{}", x);
        }


        for x in fibonacci().take(20).into_iter() {
            println!("{}", x);
        }

        println!("))))))))))))))))))))))))))))))))))))))");
        let array = [1, 3, 4, 6, 3];
        for x in array.into_iter() {
            println!("{}", x);
        }
    }
}


#[cfg(test)]
mod test_serde_json {
    use std::borrow::Cow;
    use std::time::Instant;
    use serde::Deserialize;

    #[derive(Deserialize)]
    struct UserA {
        name: String,
        age: i32,
        blog: String,
        addr: String,
    }

    #[derive(Deserialize)]
    struct UserB<'a> {
        name: &'a str,
        age: i32,
        blog: &'a str,
        addr: &'a str,
    }

    #[derive(Deserialize)]
    struct UserC<'a> {
        #[serde(borrow)]
        name: Cow<'a, str>,

        age: i32,

        #[serde(borrow)]
        blog: Cow<'a, str>,

        #[serde(borrow)]
        addr: Cow<'a, str>,
    }


    #[test]
    fn test_serde() -> serde_json::Result<()> {

        let json = r#"{
        "name": "rust",
        "age": 12,
        "blog": "rust book",
        "addr": "rust.org"
        }"#;


        let start = Instant::now();
        for i in 0..100_000 {
            let user = serde_json::from_str::<UserC>(json)?;
        }

        println!("time cost: {:?} ms", start.elapsed().as_millis());

        Ok(())

    }
}