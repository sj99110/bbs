#![allow(dead_code, unused_imports)]
use crate::user::User;
use crate::pty::*;
extern crate thrussh;
extern crate thrussh_keys;
extern crate futures;
extern crate tokio;
use std::sync::{Mutex, Arc};
use thrussh::{server, ChannelId, Disconnect, CryptoVec};
use thrussh::server::{Auth, Session, Handle, Handler};
use thrussh_keys::*;
use futures::Future;
use sqlite;

#[derive(Clone)]
pub struct Connection {
    pub auth: User,
    pub pty: Pty,
    pub db: Arc<Mutex<sqlite::Connection>>,
}

impl server::Server for Connection {
    type Handler = Self;
    fn new(&mut self, _: Option<std::net::SocketAddr>) -> Self {
        self.clone()
    }
}

impl Handler for Connection {
    type Error = anyhow::Error;
    type FutureAuth = futures::future::Ready<Result<(Self, Auth), anyhow::Error>>;
    type FutureBool = futures::future::Ready<Result<(Self, Session, bool), anyhow::Error>>;
    type FutureUnit = futures::future::Ready<Result<(Self, Session), anyhow::Error>>;

    fn finished_auth(self, auth: Auth) -> Self::FutureAuth {
        futures::future::ready(Ok((self, auth)))
    }
    fn finished_bool(self, b: bool, s: Session) -> Self::FutureBool {
        futures::future::ready(Ok((self, s, b)))
    }
    fn finished(self, s: Session) -> Self::FutureUnit {
        futures::future::ready(Ok((self, s)))
    }
    fn auth_publickey(self, _: &str, _: &key::PublicKey) -> Self::FutureAuth {
        self.finished_auth(Auth::Accept)
    }
    fn data(self, id: ChannelId, data: &[u8], mut session: Session) -> Self::FutureUnit {
        // do stuff
        /*if data[0] == 'q' as u8 {
            session.disconnect(Disconnect::ByApplication, "client requested close", "");
        }
        match data {
            [10] => (),
            _ =>{
                println!("data received: {:?}", data);
                session.data(id, CryptoVec::from(self.clone().pty.render()));
            },
        };*/
        println!("{:?}", data);
        let server = self.pty.page.handle_event(data, &id, &mut session, &self);
        let s = server.clone().pty.build_page(&server);
        session.data(id, CryptoVec::from(s.clone().pty.render()));
        s.finished(session)
    }
    fn channel_open_session(self, _: ChannelId, session: Session) -> Self::FutureUnit {
        self.finished(session)
    }
    fn pty_request(self, channel: ChannelId, _: &str, colw: u32, rowh: u32, _: u32, _: u32, _: &[(thrussh::Pty, u32)], mut session: Session) -> Self::FutureUnit {
        let mut conn = self.clone();
        conn.pty = Pty::new(rowh, colw, "000000");
        let c = conn.clone().pty.build_page(&conn);
        session.data(channel, CryptoVec::from_slice(b"\x1b[1;1H\r"));
        session.data(channel, CryptoVec::from(c.clone().pty.render()));
        c.finished(session)        
    }
    fn shell_request(self, _: ChannelId, session: Session) -> Self::FutureUnit {
        self.finished(session)
    }
    fn channel_open_direct_tcpip(self, _: ChannelId, _: &str, _: u32, _: &str, _: u32, session: Session) -> Self::FutureUnit {
        println!("dtcpip");
        self.finished(session)
    }
}