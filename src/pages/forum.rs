use crate::pages::*;
use crate::pages::main::MainPage;
use crate::pty::*;
use crate::pty::attributes::Color;
use thrussh::{ChannelId};
use thrussh::server::{Session};
use array2d::Array2D;
use sqlite;

#[derive(Clone)]
pub struct ForumPage {

}

impl ForumPage {
    pub fn new(conn: &Connection) -> Connection {
        let mut c = conn.clone();
        let rows = conn.pty.rows;
        let cols = conn.pty.cols;
        c.pty.view = (0,rows,0,cols);
        c.pty.bg_color = Color::from_hex("000000");
        c.pty.page = Pages::Forum(ForumPage{});
        c
    }

}

impl Page for ForumPage {
    fn handle_event(self, data: &[u8], _: &ChannelId, _: &mut Session, conn: &Connection) -> Connection {
        let mut c = conn.clone();
        match data {
            [10] => c.pty.page = Pages::Forum(self),
            [112] => {
                if conn.auth.auth == 0 {
                    c.pty.page = Pages::Forum(self)
                } else {
                    c.pty.page = Pages::Forum(self)
                }
            },
            [113] => c = MainPage::new(conn),
            _ => c.pty.page = Pages::Forum(self),
        };
        c
    }
    fn build_page(self, conn: &Connection) -> Array2D<Character> {
        let db = match conn.db.lock() {
            Ok(d) => d,
            Err(p) => p.into_inner(),
        };
        let mut vec = Vec::new();
        let mut st = db.prepare("
        SELECT user, body FROM posts LIMIT 10").unwrap().cursor();
        while let Some(row) = st.next().unwrap() {
            let user = row[0].as_string().unwrap();
            let body = row[1].as_string().unwrap();
            vec.push((user.to_owned(),body.to_owned()));
        }
        drop(st);
        drop(db);
        let mut cols = 0;
        let mut rows = 0;
        for (_, post) in &vec {
            if post.len() > cols {
                cols = post.len();
            }
            if rows == 0 {
                rows = post.to_string().chars().collect::<Vec<char>>().iter().fold(0, |c, x| if *x == '\n' { c+1 } else {c});
            }
        }
        if cols < conn.pty.cols {
            cols = conn.pty.cols;
        }
        if rows < conn.pty.rows {
            rows = conn.pty.rows;
        }
        let mut page = Array2D::filled_with(Character::default(), rows + (vec.len()*2) as usize, cols);
        if vec.len() < 1 {
            return page;
        }
        let mut x = 0;
        let mut y = 0;
        for (name, post) in &vec {
            y = 0;
            //write name underlined followed by |
            for (i,c) in name.chars().enumerate() {
                page.set(x, i, Character::new(c, "BOLD|UL", "FFFFFF"));
                y = i;
            }
            page.set(x, y+1, Character::new('|', "", "FFFFFF"));
            x += 1;
            for(i,c) in post.chars().enumerate() {
                page.set(x, i, Character::new(c, "", "FFFFFF"));
            }
            x += 1;
            for n in 0 .. page.num_columns() {
                page.set(x, n, Character::new(' ', "OL|UL", "FFFFFF"));
            }
            x += 1;
        }
        page
    }
}
