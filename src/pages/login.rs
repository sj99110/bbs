use crate::connection::Connection;
use thrussh::ChannelId;
use thrussh::server::Session;
use crate::pages::{Page, Pages};
use crate::pty::Pty;
use crate::pty::attributes::Color;
use crate::pty::Character;
use array2d::Array2D;

#[derive(Clone)]
pub struct Login {
    buffer: Vec<String>,
}

impl Login {
    pub fn new(conn: &Connection) -> Connection {
        let mut c = conn.clone();
        c.pty.bg_color = Color::from_hex("000000");
        c.pty.page = Pages::Login(Login {buffer: Vec::new()});
        c
    }
}

impl Page for Login {
    fn handle_event(self, data: &[u8], _: &ChannelId, _: &mut Session, conn: &Connection) -> Connection {
        *conn
    }

    fn build_page(self, conn: &Connection) -> Array2D<Character>
    {

    }
}
