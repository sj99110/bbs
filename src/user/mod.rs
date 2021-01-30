#![allow(dead_code)]
use sqlite;
use crate::connection::Connection;
use sha2::{Sha256, Digest};
use rand::{thread_rng, RngCore};

#[derive(Clone)]
pub struct User {
    pub name: String,
    pub auth: u64,
}

impl User {
    pub fn login(name: String, pass: String, conn: &Connection) -> Result<Self, sqlite::Error> {
        let db = match conn.db.lock() {
            Ok(g) => g,
            Err(p) => p.into_inner(),
        };
        let mut st = db.prepare("
            SELECT (name, password, salt, auth) FROM users WHERE name = ?")?;
        st.bind(1, &name[..])?;
        if let sqlite::State::Row = st.next()? {
            let name = st.read::<String>(0)?;
            let _pass = st.read::<String>(1)?;
            let salt = st.read::<String>(2)?;
            let auth = st.read::<i64>(3)?;
            drop(st);
            drop(db);
            let hash = Sha256::digest(&(salt + &pass).bytes().collect::<Vec<u8>>()[..]);
            let h: String = hash.as_slice().to_vec().iter().map(|x| *x as char).collect();
            if _pass == h {
                return Ok(User {
                    name: name,
                    auth: auth as u64,
                });
            }
            return Err(sqlite::Error{
                code: None,
                message: Some("failed to validate user".to_string()),
            });
        }
        Err(sqlite::Error{
            code: Some(-2),
            message: Some("login failure".to_string())
        })
    }
    pub fn register(name: String, pass: String, conn: &Connection) -> Result<Self, sqlite::Error> {
        let mut h = [0u8;32];
        let mut rng = thread_rng();
        let mut vec = Vec::new();
        for n in 0..32 {
            rng.fill_bytes(&mut h);
            vec.push(h[n] as char);
        }
        let salt = &vec.iter().map(|x| x.to_string()).collect::<String>()[..];
        let hash = Sha256::digest(&(pass + salt).bytes().collect::<Vec<u8>>()[..]);
        let _pass: String = hash.as_slice().to_vec().iter().map(|x| *x  as char).collect();
        let db = match conn.db.lock() {
            Ok(g) => g,
            Err(p) => p.into_inner(),
        };
        let mut st = db.prepare("
            INSERT (name, password, salt, auth) INTO users VALUES (?, ?, ?, 1)")?;
        st.bind(1, &name[..])?;
        st.bind(2, &_pass[..])?;
        st.bind(3, salt)?;
        st.next()?;
        Ok(User {
            name: name,
            auth: 1,
        })
    }
}
