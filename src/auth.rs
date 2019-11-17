pub mod auth {
    #[macro_use]
    use serde::{Deserialize, Serialize};
    use serde_json::Result;
    use std::fs::File;
    use std::fs::OpenOptions;
    use std::io::Write;
    use std::io::Read;

    #[derive(Debug, Serialize, Deserialize)]
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


    pub fn saveDB(_db: &DB) {
        println!("Saving DB...");

        let serialized = serde_json::to_string(&_db).unwrap();

        let mut file = OpenOptions::new().create(true).write(true).open(".db").unwrap();

        file.write_all(serialized.as_bytes());
    }

    pub fn loadDB() -> DB {
        println!("Loading DB...");

        let mut file = OpenOptions::new().read(true).open(".db").unwrap();

        let mut contents: String = "".to_string();;
        file.read_to_string(&mut contents);

        let mut db: DB = serde_json::from_str(&contents).unwrap();
        return db;
    }
}
