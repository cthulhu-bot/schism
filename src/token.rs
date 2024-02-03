pub struct TokenIdx {
    line_no: u32,
    col_no: u32,
}

pub struct Token {
    start: TokenIdx,
    end: TokenIdx,
    value: String,
}

pub vec ReservedToken = [
    "(",
    ")",
]

