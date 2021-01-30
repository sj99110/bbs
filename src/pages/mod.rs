extern crate thrussh;
pub mod main;
pub mod forum;
//pub mod login;
use crate::pty::Character;
use crate::connection::Connection;
use thrussh::server::{Session};
use thrussh::{CryptoVec, ChannelId};
use array2d::Array2D;

#[derive(Clone)]
pub enum Pages {
    EmptyPage(EmptyPage),
    Main(main::MainPage),
    Forum(forum::ForumPage),
    //Login(login::Login),
}

pub trait Page{
    fn handle_event(self, data: &[u8], channel: &ChannelId, session: &mut Session, conn: &Connection) -> Connection; 
    fn build_page(self, conn: &Connection) -> Array2D<Character>;
}

#[derive(Clone)]
pub struct EmptyPage{}

impl Page for EmptyPage {
    fn handle_event(self, data: &[u8], channel: &ChannelId, session: &mut Session, conn: &Connection) -> Connection {
        session.data(channel.clone(), CryptoVec::from_slice(data));
        let mut c = conn.clone();
        c.pty.page = Pages::EmptyPage(self);
        c
    }
    fn build_page(self, _: &Connection) -> Array2D<Character> {
        Array2D::filled_with(Character::default(), 128, 80)
    }
}

impl Pages {
    pub fn handle_event(&self, data: &[u8], channel: &ChannelId, session: &mut Session, conn: &Connection) -> Connection {
        match self.clone() {
            Pages::Main(x) => x.handle_event(data, channel, session, conn),
            Pages::EmptyPage(x) => x.handle_event(data, channel, session,conn),
            Pages::Forum(x) => x.handle_event(data, channel, session, conn),
        }
    }
    pub fn build_page(self, conn: &Connection) -> Array2D<Character> {
        match self{
            Pages::Main(z) => z.build_page(conn),
            Pages::EmptyPage(z) => z.build_page(conn),
            Pages::Forum(z) => z.build_page(conn),
        }
    }
}