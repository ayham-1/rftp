pub mod db {
    use serde::{Deserialize, Serialize};
    use std::fs::OpenOptions;
    use std::io::Write;
    use std::io::Read;
    use std::time::Duration;
    use std::thread;

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

    pub fn add_user(_db: &mut DB, _username: String, _password: String, _rights: Rights) {
        let mut user: User = User::default();
        user.username = _username;
        user.password = _password;
        user.rights = _rights;

        _db.user.push(user);
    }

    pub fn rm_user(_db: &mut DB, _username: String) {
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

    pub fn save_db(_db: &DB) -> Result<(), std::io::Error> {
        println!("Saving DB...");

        let serialized = serde_json::to_string(&_db).unwrap();

        std::fs::remove_file("~/.rftp.db")?;
        let mut file = OpenOptions::new().create(true).write(true).append(false).open("~/.rftp.db")?;
        Ok(file.write_all(serialized.as_bytes())?)
    }

    pub fn load_db() -> Result<DB, Box<dyn std::error::Error>> {
        println!("Loading DB...");

        let mut file = OpenOptions::new().create(true).read(true).write(true).open("../.rftp.db")?;

        let mut contents: String = "".to_string();
        file.read_to_string(&mut contents)?;
        Ok(serde_json::from_str(&contents)?)
    }

    pub fn clean_db() -> Result<(), std::io::Error> {
        println!("WARNING: CLEANING USER DATABASE!");
        println!("Press Ctrl-c to abort.");
        thread::sleep(Duration::from_secs(5));

        std::fs::remove_file("../.rftp.db")?;

        println!("Successfully cleaned user database.");
        Ok(())
    }

    #[derive(Debug)]
    pub enum CmdJob {
        Add, Remove, List, Clean
    }
    impl Default for CmdJob {
        fn default() -> Self { CmdJob::List }
    }

    #[derive(Default, Debug)]
    pub struct DBCmd {
        pub job: CmdJob,
        pub user: String,
        pub pass: String,
        pub rights: Rights
    }
    
    pub fn apply_dbcmd(_cmd: &DBCmd) -> Result<(), Box<dyn std::error::Error>> {
        let mut _db: DB = DB::default();
        println!("Currently processing command: {:?}", _cmd.job);
        _db = load_db()?;

        match _cmd.job {
            CmdJob::Add => db_add(&mut _db, _cmd),
            CmdJob::Remove => db_rm(&mut _db, _cmd),
            CmdJob::List => { db_list(&_db, _cmd); return Ok(()) },
            CmdJob::Clean => { clean_db()?; return Ok(()) },
        }

        Ok(save_db(&_db)?)
    }

    pub fn db_add(_db: &mut DB,_cmd: &DBCmd) {
        println!("Adding new user to local database.");
        println!("Username: {}", _cmd.user);
        println!("Password: {}", _cmd.pass);
        println!("Rights: {:?}", _cmd.rights);

        add_user(_db, (&_cmd.user).to_string(), (&_cmd.pass).to_string(), _cmd.rights);
    }

    pub fn db_rm(_db: &mut DB,_cmd: &DBCmd) {
        println!("Removing user to local database.");
        println!("Username: {}", _cmd.user);

        rm_user(_db, (&_cmd.user).to_string());
    }

    pub fn db_list(_db: &DB,_cmd: &DBCmd) {
        let mut counter: i32 = 0;
        for i in _db.user.iter() {
            println!("====================================");
            println!("User number: {}", counter);
            println!("User Name: {}", i.username);
            println!("User Rights: {:?}", i.rights);
            counter = counter + 1;
            println!("====================================");
        }
    }
}
