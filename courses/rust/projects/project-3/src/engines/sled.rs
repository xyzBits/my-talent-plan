use sled::{Db, Tree};

use crate::{KvsError, Result};

use super::KvsEngine;

/// Wrapper of `sled::Db`
#[derive(Clone)]
pub struct SledKvsEngine(Db);

impl SledKvsEngine {
    /// Creates a `SledKvsEngine` from `sled::Db`
    pub fn new(db: Db) -> Self {
        SledKvsEngine(db)
    }
}

impl KvsEngine for SledKvsEngine {
    fn set(&mut self, key: String, value: String) -> Result<()> {
        let tree: &Tree = &self.0;
        tree.insert(key, value.into_bytes()).map(|_| ())?; // the closure discard the result and return ok
        tree.flush()?;
        Ok(())
    }

    fn get(&mut self, key: String) -> Result<Option<String>> {
        let tree: &Tree = &self.0;

        Ok(tree
            .get(key)?
            .map(|i_vec| AsRef::<[u8]>::as_ref(&i_vec).to_vec())
            .map(String::from_utf8)
            .transpose()?)
    }

    fn remove(&mut self, key: String) -> Result<()> {
        let tree: &Tree = &self.0;
        tree.remove(key)?.ok_or(KvsError::KeyNotFound)?;
        tree.flush()?;
        Ok(())
    }
}

#[test]
fn test_sled_db() -> Result<()> {
    // let temp_dir = TempDir::new().expect("unable to create temporary working directory");
    // let tree = sled::open(temp_dir)?;
    //
    // // let tree: &Tree = &db;
    //
    // // tree.insert("hello".to_string(), "world".to_string().into_bytes()).map(|_| ())?;
    // let option = tree.insert("hello".to_string(), "world".to_string().into_bytes())?;
    //
    // let value = tree
    //     .get("hello".to_string())?
    //     .map(|i_vec| AsRef::<[u8]>::as_ref(&i_vec).to_vec())
    //     .map(String::from_utf8)
    //     .transpose()?
    //     .unwrap();
    // println!("{:?}", value);
    //
    // assert_eq!(value, "world");

    Ok(())
}

#[cfg(test)]
mod test_as_ref {
    use std::mem::take;
    use std::os::unix::raw::mode_t;

    fn is_hello<T: AsRef<str>>(s: T) {
        assert_eq!("hello", s.as_ref());
    }

    #[test]
    fn test_as_ref() {
        let x = Box::new(5i32);
        let y = x.as_ref();
        let z: &i32 = &x;

        assert_eq!(y, z);

        let s = "hello";
        is_hello(s);

        let s = "hello".to_string();
        is_hello(s);
    }

    #[derive(Default)]
    struct User {
        email: String,
        age: u8,
    }

    // obviously
    impl AsRef<User> for User {
        fn as_ref(&self) -> &User {
            self
        }
    }

    enum Privilege {
        // imagine different moderator privileges here
    }

    #[derive(Default)]
    struct Moderator {
        user: User,
        privileges: Vec<Privilege>,
    }

    impl AsRef<User> for Moderator {
        fn as_ref(&self) -> &User {
            &self.user
        }
    }

    fn takes_user(user: &User) {}

    fn takes_user_as_ref<U: AsRef<User>>(user: U) {}

    #[test]
    fn test_ref() {
        let user = User::default();
        let moderator = Moderator::default();

        takes_user(&user);
        // takes_user(&moderator);

        takes_user_as_ref(&user);
        takes_user_as_ref(&moderator);
    }
}
