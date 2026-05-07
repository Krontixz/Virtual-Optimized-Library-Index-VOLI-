pub enum VolicType {
    Jvm,
    Python,
    DotNet,
    Beam,
    Lua,
    Wasm,
    Lisp,
    Flash,
    Parrot,
    Unknown,
}

impl VolicType {
    pub fn to_byte(&self) -> u8 {
        match self {
            VolicType::Jvm => 1,
            VolicType::Python => 2,
            VolicType::DotNet => 3,
            VolicType::Beam => 4,
            VolicType::Lua => 5,
            VolicType::Wasm => 6,
            VolicType::Lisp => 7,
            VolicType::Flash => 8,
            VolicType::Parrot => 9,
            VolicType::Unknown => 0,
        }
    }

    pub fn from_byte(byte: u8) -> Self {
        match byte {
            1 => VolicType::Jvm,
            2 => VolicType::Python,
            3 => VolicType::DotNet,
            4 => VolicType::Beam,
            5 => VolicType::Lua,
            6 => VolicType::Wasm,
            7 => VolicType::Lisp,
            8 => VolicType::Flash,
            9 => VolicType::Parrot,
            _ => VolicType::Unknown,
        }
    }
}
