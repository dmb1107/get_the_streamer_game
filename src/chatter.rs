#[derive(Debug, Clone, PartialEq)]
pub struct Chatter {
    pub name: String,
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Chatter {
    pub fn new(name: String, color: (u8, u8, u8)) -> Chatter {
        let (red, green, blue) = if color.0 == 0 && color.1 == 0 && color.2 == 0 {
            (100, 100, 100)
        } else {
            color
        };

        Chatter {
            name,
            red,
            green,
            blue,
        }
    }
}
