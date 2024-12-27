#[derive(Debug, Clone)]
pub struct Operation {
    pub l: [u8; 3],
    pub operator: Operator,
    pub r: [u8; 3],
    pub out: [u8; 3],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Operator {
    And,
    Or,
    Xor,
}

impl Operator {
    fn func(&self) -> fn(u8, u8) -> u8 {
        match self {
            Self::And => std::ops::BitAnd::bitand,
            Self::Or => std::ops::BitOr::bitor,
            Self::Xor => std::ops::BitXor::bitxor,
        }
    }
}
