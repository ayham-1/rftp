pub mod auth {
    #[macro_use]
    use serde::{Deserialize, Serialize};
    use serde_json::Result;
    use std::fs::File;
    use std::fs::OpenOptions;
    use std::io::Write;
    use std::io::Read;

    #[derive(Debug, Serialize, Deserialize, Copy, Clone)]
    pub enum Rights {
        List, Read, All,
        Nothing
    }

    impl Default for Rights {
        fn default() -> Self { Rights::List }
    }

    #[derive(Default, Debug, Serialize, Deserialize)]
    pub struct User {
        pub username: String,
        pub password: String,
        pub rights: Rights,
    }

    #[derive(Default, Debug, Serialize, Deserialize)]
    pub struct DB {
        pub user: Vec<User>,
    }

    pub fn addUser(_db: &mut DB, _username: String, _password: String, _rights: Rights) {
        let mut user: User = User::default();
        user.username = _username;
        user.password = _password;
        user.rights = _rights;

        _db.user.push(user);
    }

    pub fn rmUser(_db: &mut DB, _username: String) {
        _db.user.retain(|user| {
            let delete = {
                if user.username == _username {
                    false
                }
                else { true }
            };
            !delete
        });
    }

    pub fn saveDB(_db: &DB) {
        println!("Saving DB...");

        let serialized = serde_json::to_string(&_db).unwrap();

        let mut file = OpenOptions::new().create(true).write(true).open(".db").unwrap();

        file.write_all(serialized.as_bytes());
    }

    pub fn loadDB() -> Option<&mut DB> {
        println!("Loading DB...");

        let mut file = OpenOptions::new().create(true).read(true).write(true).open(".db").unwrap();

        let mut contents: String = "".to_string();;
        file.read_to_string(&mut contents);

        if contents == "".to_string() {
            return DB::default();
        }

        let mut db: DB = serde_json::from_str(&contents).unwrap();
        return db;
    }
}
