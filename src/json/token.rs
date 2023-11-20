#[derive(Debug)]
pub enum TokenType {
    CurlyOpen,
    CurlyClose,
    Colon,
    String,
    Number,
    ArrayOpen,
    ArrayClose,
    Comma,
    Boolean,
    Null,
}

#[derive(Debug)]
pub struct Token {
    /// Value contains the token as a string. Ex. `{` or `,` or `some string`
    pub value: String,
    pub token_type: TokenType,
}

impl Token {
    pub fn to_string(&self) -> String {
        String::new()
    }
}
