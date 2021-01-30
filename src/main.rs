extern crate thrussh;
extern crate thrussh_keys;
extern crate futures;
extern crate tokio;
extern crate sqlite;
extern crate toml;
extern crate serde_derive;
extern crate rand;
extern crate sha2;
mod connection;
mod user;
mod pty;
mod pages;
use pages::Page;
use pty::*;
use user::*;
use connection::*;
use std::sync::{Mutex, Arc};
use thrussh::*;
use thrussh::server::{Auth, Session, Handle};
use thrussh_keys::*;
use futures::Future;
use std::cell::RefCell;
use serde_derive::Deserialize;
use rand::{thread_rng, Rng};
use sha2::{Sha256, Digest};

#[derive(Deserialize)]
struct Config {
    db_path: String,
}

fn create_post_test(db: String) {
    let conn = sqlite::open(db).unwrap();
    conn.execute("INSERT INTO posts (user, body) VALUES ('admin', 'test post 1 please ignore');").unwrap();
    conn.execute("INSERT INTO posts (user, body) VALUES ('admin', 'test post 2 please ignore');").unwrap();
    conn.execute("INSERT INTO posts (user, body) VALUES ('admin', 'test post 3 return of the doge');").unwrap();
    conn.execute("INSERT INTO posts (user, body) VALUES ('admin', 'test post 4 the doge strikes back');").unwrap();
    conn.execute("INSERT INTO posts (user, body) VALUES ('admin', 'test post 6 install gentoo');").unwrap();
    conn.execute("INSERT INTO posts (user, body) VALUES ('admin', 'test post 7 electric bugaloo');").unwrap();
    conn.execute("INSERT INTO posts (user, body) VALUES ('admin', 'test post 8 to many tests');").unwrap();
    println!("finished test post creation");
}

fn createdb(db: String) {
    let conn = sqlite::open(db).unwrap();
    conn.execute("
        CREATE TABLE users (id INTEGER NOT NULL PRIMARY KEY, name TEXT UNIQUE, password TEXT, salt TEXT, auth INTEGER);
        CREATE TABLE posts (id INTEGER NOT NULL PRIMARY KEY, user TEXT, body TEXT);").unwrap();
    let mut st = conn.prepare("INSERT INTO users (name, password, salt, auth) VALUES ('admin', ?, ?, 9)").unwrap();
    let hash = Sha256::digest(b"adminadminhash");
    let h: String = hash.as_slice().to_vec().iter().map(|x| *x as char).collect();
    st.bind(1, &h[..]).unwrap();
    st.bind(2, "adminhash").unwrap();
    assert_eq!(st.next().unwrap(), sqlite::State::Done);
}

#[tokio::main]
async fn main() {
    let user = User{
        name: "Guest".to_string(),
        auth: 0,
    };
    let contents = std::fs::read_to_string("config.toml").expect("unable to open config.toml");
    let config: Config = toml::from_str(&contents).expect("unable to parse config.toml");
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] == "createdb" {
        createdb(config.db_path);
    } else if args.len() > 1 && args[1] == "testposts" {
        create_post_test(config.db_path);
    } else {
        let key = thrussh_keys::load_secret_key("./keys/id_ed25519", None).unwrap();
        let mut config = thrussh::server::Config::default();
        config.methods = MethodSet::PUBLICKEY;
        config.auth_banner = Some("benis");
        config.keys.push(key);
        config.auth_rejection_time = std::time::Duration::from_secs(5);
        let db = sqlite::open("data");
        if let Ok(db) = db {
            let db = Arc::new(Mutex::new(db));
            let conn = Connection{auth: user, pty: pty::Pty::new(0, 0, "000000"), db: db};
            server::run(Arc::new(config), "0.0.0.0:1234", conn).await.unwrap();
        } else if let Err(e) = db {
            println!("failed to open db: {}", e);
        }
    }
}

