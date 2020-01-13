pub mod db {
    use std::fs::OpenOptions;
    use std::io::Write;
    use std::io::Read;
    use std::time::Duration;
    use std::thread;
    use std::error::Error;
    use log::{info, warn, trace};
    use crate::defines::defines::*;
    use std::path::Path;

    pub fn save_db(_db: &DB) -> Result<(), std::io::Error> {
        trace!("Saving DB...");

        let serialized = serde_json::to_string(&_db).unwrap();

        std::fs::remove_file(dirs::home_dir().unwrap().join(Path::new(".rftp.db")))?;
        let mut file = OpenOptions::new().create(true).write(true)
            .append(false).open(dirs::home_dir().unwrap().join(Path::new(".rftp.db")))?;
        Ok(file.write_all(serialized.as_bytes())?)
    }

    pub fn load_db() -> Result<DB, Box<dyn std::error::Error>> {
        trace!("Loading DB...");

        let mut file = OpenOptions::new().create(true).read(true)
            .write(true).open(dirs::home_dir().unwrap().join(Path::new(".rftp.db")))?;

        let mut contents: String = "".to_string();
        file.read_to_string(&mut contents)?;
        if contents == "" {
            return Ok(DB::default())
        }
        Ok(serde_json::from_str(&contents)?)
    }

    pub fn clean_db() -> Result<(), std::io::Error> {
        warn!("WARNING: CLEANING USER DATABASE!");
        warn!("Press Ctrl-c to abort.");
        thread::sleep(Duration::from_secs(5));

        std::fs::remove_file(dirs::home_dir().unwrap().join(Path::new(".rftp.db")))?;

        info!("Successfully cleaned user database.");
        Ok(())
    }

    
    pub fn apply_dbcmd(_cmd: &DBCmd) -> Result<(), Box<dyn Error>> {
        let mut _db: DB = DB::default();
        trace!("Currently processing command: {:?}", _cmd.job);
        _db = load_db()?;

        match _cmd.job {
            CmdJob::Add => db_add(&mut _db, _cmd),
            CmdJob::Remove => db_rm(&mut _db, _cmd),
            CmdJob::List => { db_list(&_db, _cmd); return Ok(()) },
            CmdJob::Clean => { clean_db()?; return Ok(()) },
        };

        Ok(save_db(&_db)?)
    }

    pub fn db_add(_db: &mut DB,_cmd: &DBCmd) {
        info!("Adding new user to local database.");
        info!("Username: {}", _cmd.user);
        info!("Password: {}", _cmd.pass);
        info!("Rights: {:?}", _cmd.rights);

        let mut user: User = User::default();
        user.username = _cmd.user.to_owned();
        user.password = _cmd.pass.to_owned();
        user.rights = _cmd.rights.to_owned();

        _db.user.push(user);
    }

    pub fn db_rm(_db: &mut DB, _cmd: &DBCmd) {
        warn!("Removing user from local database.");
        warn!("Username: {}", _cmd.user);

        _db.user.retain(|user| {
            let delete = {
                if user.username == _cmd.user {
                    false
                }
                else { true }
            };
            delete
        });
    }

    pub fn db_list(_db: &DB,_cmd: &DBCmd) {
        let mut counter: i32 = 0;
        for i in _db.user.iter() {
            info!("====================================");
            info!("User number: {}", counter);
            info!("User Name: {}", i.username);
            info!("User Rights: {:?}", i.rights);
            counter = counter + 1;
            info!("====================================");
        }
    }
}
