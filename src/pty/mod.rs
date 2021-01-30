#![allow(dead_code)]
pub mod attributes;
extern crate array2d;
use crate::pages::{Pages};
use crate::pages::main::MainPage;
use crate::connection::Connection;
use array2d::Array2D;
use attributes::*;

#[derive(Clone)]
pub struct Character {
    pub character: char,
    pub attrs: Vec<Attrs>,
    pub color: Color,
}

#[derive(Clone)]
pub struct Pty {
    pub view: (usize, usize, usize, usize),
    pub rows: usize,
    pub cols: usize,
    pub term: Array2D<Character>,
    pub bg_color: Color,
    pub page: Pages,
}

impl Character {
    pub fn new(character: char, attributes: &str, hex: &str) -> Self {
        let color = Color::from_hex(hex);
        let attrs = Attrs::from_str(attributes);
        Character {
            character: character,
            attrs: attrs,
            color: color
        }
    }
    pub fn default() -> Self {
        let color = Color::from_hex("FFFFFF");
        let attrs = Vec::new();
        Character{
            character: ' ',
            attrs: attrs,
            color: color,
        }
    }
}

fn compare_vecs<T: PartialEq>(v1: &Vec<T>, v2: &Vec<T>) -> bool {
    if v1.len() != v2.len() {
        return false;
    }
    let len = v1.len();
    for n in 0..len {
        if v1[n] != v2[n] {
            return false;
        }
    }
    true
}

impl Pty {
    pub fn new(x: u32, y: u32, bg_color: &str) -> Self {
        let character = Character::default();
        let array = Array2D::filled_with(character, x as usize, y as usize);
        let bg_color = Color::from_hex(bg_color);
        Pty {
            view: (0,x as usize ,0,y as usize),
            rows: x as usize,
            cols: y as usize,
            term: array,
            bg_color: bg_color,
            page: Pages::Main(MainPage{}),
        }
    }
    pub fn render<'a>(self) -> String {
        let mut out = "\x1b[2J\x1b[1;1H\x1b[25l".to_string();
        out += &self.bg_color.background_str().to_owned();
        let mut current_attrs = &Vec::new();
        let mut current_color = Color::from_hex("FFFFFF");
        for x in 0..self.rows {
            for y in 0..self.cols {
                if self.term[(x,y)].character == '\n' {
                    out.push('\n');
                    break;
                }
                if !compare_vecs(&self.term[(x,y)].attrs, &current_attrs) {
                    current_attrs = &self.term[(x,y)].attrs;
                    out += "\x1b[m\x1b[?25l";
                    out += &self.bg_color.background_str().to_owned();
                    out += &current_color.foreground_str().to_owned();
                    for attr in current_attrs {
                        out += Attrs::to_str(attr);
                    }
                }
                if self.term[(x,y)].color != current_color {
                    current_color = self.term[(x,y)].color.clone();
                    out += &current_color.foreground_str().to_owned();
                }
                out.push(self.term[(x,y)].character);
            }
        }
        out + "\x1b[m"
    }
    pub fn build_page(self, conn: &Connection) -> Connection {
        let mut s = conn.clone();
        s.pty.term = self.page.build_page(conn);
        s
    }
}

