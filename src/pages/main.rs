extern crate array2d;
use crate::pty::*;
use crate::pty::attributes::Color;
use crate::pages::{Page, Pages};
use crate::pages::forum::ForumPage;
use thrussh::server::{Session};
use thrussh::{Disconnect, ChannelId};
use array2d::Array2D;
use crate::connection::Connection;

#[derive(Clone)]
pub struct MainPage {

}

impl MainPage {
    pub fn new(conn: &Connection) -> Connection {
        let mut c = conn.clone();
        c.pty.page = Pages::Main(MainPage{});
        c.pty.bg_color = Color::from_hex("000000");
        c
    }
}

impl Page for MainPage {
    fn handle_event(self, data: &[u8], _: &ChannelId, session: &mut Session, conn: &Connection) -> Connection {
        let mut c = conn.clone();
        match data {
            [10] => c.pty.page = Pages::Main(self),
            [113] => {
                session.disconnect(Disconnect::ByApplication, "user requested exit", "");
                c.pty.page = Pages::Main(self)
            },
            [102] => {
                c = ForumPage::new(conn)
            },
            [27, 97] => { // alt-a admin
                c.pty.page = Pages::Main(self)
            },
            _ => c.pty.page = Pages::Main(self),
        };
        c
    }
    fn build_page(self, conn: &Connection) -> Array2D<Character> {
        let mut page = Array2D::filled_with(Character::default(), conn.pty.rows, conn.pty.cols);
        let title = &"Welcome to SmokeBBS";
        let tvec = title.chars().collect::<Vec<_>>();
        let commands;
        let auth = conn.auth.auth;
        if auth == 0 {
            commands = &"(q)uit, (f)orum (r)egister (l)ogin";
        } else if auth > 0 && auth < 9 {
            commands = &"(q)uit (f)orum, (l)ogout";
        } else {
            commands = &"(q)uit (f)orum (l)ogout (alt+a)admin";
        }
        let base = title.len() / 2;
        for n in 0 .. title.len() {
            page.set(0, conn.pty.cols/2 - base + n, Character::new(tvec[n], "BOLD|BLINK", "FFFFFF"));
        }
        let base = commands.len()/2;
        let tvec = commands.chars().collect::<Vec<_>>();
        for n in 0 .. commands.len() {
            page.set(conn.pty.rows - 1, conn.pty.cols/2 - base + n, Character::new(tvec[n], "", "FFFFFF"));
        }
        page
    }
}