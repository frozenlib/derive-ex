#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum BinaryOp {
    Add,
    BitAnd,
    BitOr,
    BitXor,
    Div,
    Mul,
    Rem,
    Shl,
    Shr,
    Sub,
}

impl BinaryOp {
    pub fn from_str(s: &str) -> Option<Self> {
        Some(match s {
            "Add" => Self::Add,
            "BitAnd" => Self::BitAnd,
            "BitOr" => Self::BitOr,
            "BitXor" => Self::BitXor,
            "Div" => Self::Div,
            "Mul" => Self::Mul,
            "Rem" => Self::Rem,
            "Shl" => Self::Shl,
            "Shr" => Self::Shr,
            "Sub" => Self::Sub,
            _ => return None,
        })
    }
    pub fn to_str(self) -> &'static str {
        match self {
            Self::Add => "Add",
            Self::BitAnd => "BitAnd",
            Self::BitOr => "BitOr",
            Self::BitXor => "BitXor",
            Self::Div => "Div",
            Self::Mul => "Mul",
            Self::Rem => "Rem",
            Self::Shl => "Shl",
            Self::Shr => "Shr",
            Self::Sub => "Sub",
        }
    }
    pub fn to_func_name(self) -> &'static str {
        match self {
            Self::Add => "add",
            Self::BitAnd => "bit_and",
            Self::BitOr => "bit_or",
            Self::BitXor => "bit_xor",
            Self::Div => "div",
            Self::Mul => "mul",
            Self::Rem => "rem",
            Self::Shl => "shl",
            Self::Shr => "shr",
            Self::Sub => "sub",
        }
    }
}
impl std::fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}
