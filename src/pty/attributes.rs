#[derive(Clone, PartialEq, Eq)]
pub enum Attrs {
    Bold,
    Underline,
    Blink,
    Crossed,
    Overline,
}

#[derive(Clone, PartialEq, Eq, Copy)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Attrs {
    pub fn from_str(attrs: &str) -> Vec<Attrs> {
        if attrs == "" {
            return Vec::new();
        }
        let attrs = attrs.split(|x: char| x == '|').collect::<Vec<_>>();
        let mut ret = Vec::new();
        for attr in attrs {
            match attr {
                "bold" | "Bold" | "BOLD" => ret.push(Attrs::Bold),
                "UL" | "ul" | "underline" => ret.push(Attrs::Underline),
                "blink" | "Blink" | "BLINK" => ret.push(Attrs::Blink),
                "crossed" | "Crossed" | "CROSSED" => ret.push(Attrs::Crossed),
                "OL" | "ol" | "overline" => ret.push(Attrs::Overline),
                _ => ()
            };
        }
        ret
    }
    pub fn to_str<'a>(attr: &Attrs) -> &'a str {
        return match attr{
            Attrs::Bold => &"\x1b[1m",
            Attrs::Crossed => &"\x1b[9m",
            Attrs::Underline => &"\x1b[4m",
            Attrs::Overline => &"\x1b[53m",
            Attrs::Blink => &"\x1b[5m",
        };
    }
}

impl<'a> Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color {
            red: r,
            green: g,
            blue: b
        }
    }
    pub fn from_hex(hex: &str) -> Self {
        if hex.len() < 6 {
            return Color {
                red: 0,
                green: 0,
                blue: 0};
        }
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap();
        Color{ red: r, green: g, blue: b }
    }
    pub fn foreground_str(self) -> String {
        return format!("\x1b[38;2;{};{};{}m", self.red, self.green, self.blue);
    }
    pub fn background_str(self) -> String {
        return format!("\x1b[48;2;{};{};{}m", self.red, self.green, self.blue);
    }
}

/*fn main() {
    let color = Color::from_hex("FF02F0");
    let color2 = Color::from_hex("FFFFFF");
    println!("\x1b[38;2;{};{};{}mhello world\x1b[m", color.red, color.green, color.blue);
    let attrs = Attrs::from_str("Bold|crossed|ul|ol");
    let mut output = color.foreground_str().to_owned();
    output += &color2.background_str().to_owned();
    for attr in attrs {
        output += Attrs::to_str(&attr);
    }
    println!("{}benis\x1b[m", output);
}*/