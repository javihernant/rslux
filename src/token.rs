use std::fmt::{self,Display};

pub struct Token {
    token_type: TokenType,
    lexeme: Option<String>,
    literal: Option<String>,
    line: i32,
}

impl Token {
    pub fn new(token_type: TokenType) -> Token {
        Token {
            token_type,
            lexeme: None,
            literal: None,
            line: 0,
        }
    }


}

impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //TODO: , self.lexeme, self.literal
        write!(f, "{}", self.token_type)
    }
}

pub enum TokenType {
    //One char
    LeftParen,
    RightParent,
    LeftBrace,
    RightBrace, 
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    //One or two chars
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    //Literals
    Identifier(String),
    String(String),
    Number(f64),
    //Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Eof,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenType::LeftParen => write!(f,"LEFT_PAREN"),
            TokenType::RightParent => write!(f,"RIGHT_PAREN"),
            TokenType::LeftBrace => write!(f,"LEFT_BRACE"),
            TokenType::RightBrace => write!(f,"RIGHT_BRACE"), 
            TokenType::Comma => write!(f,"COMMA"),
            TokenType::Dot => write!(f,"DOT"),
            TokenType::Minus => write!(f,"MINUS"),
            TokenType::Plus => write!(f,"PLUS"),
            TokenType::Semicolon => write!(f,"SEMICOLON"),
            TokenType::Slash => write!(f,"SLASH"),
            TokenType::Star => write!(f,"STAR"),
            TokenType::Bang => write!(f,"BANG"),
            TokenType::BangEqual => write!(f,"BANG_EQUAL"),
            TokenType::Equal => write!(f,"EQUAL"),
            TokenType::EqualEqual => write!(f,"EQUAL_EQUAL"),
            TokenType::Greater => write!(f,"GREATER"),
            TokenType::GreaterEqual => write!(f,"GREATER_EQUAL"),
            TokenType::Less => write!(f,"LESS"),
            TokenType::LessEqual => write!(f,"LESS_EQUAL"),
            TokenType::Identifier(s) => write!(f,"IDENTIFIER ({})", s),
            TokenType::String(s) => write!(f,"STRING ({})",s),
            TokenType::Number(n) => write!(f,"NUMBER ({})",n),
            TokenType::And => write!(f,"AND"),
            TokenType::Class => write!(f,"CLASS"),
            TokenType::Else => write!(f,"ELSE"),
            TokenType::False => write!(f,"FALSE"),
            TokenType::Fun => write!(f,"FUN"),
            TokenType::For => write!(f,"FOR"),
            TokenType::If => write!(f,"IF"),
            TokenType::Nil => write!(f,"NIL"),
            TokenType::Or => write!(f,"OR"),
            TokenType::Print => write!(f,"PRINT"),
            TokenType::Return => write!(f,"RETURN"),
            TokenType::Super => write!(f,"SUPER"),
            TokenType::This => write!(f,"THIS"),
            TokenType::True => write!(f,"TRUE"),
            TokenType::Var => write!(f,"VAR"),
            TokenType::While => write!(f,"WHILE"),
            TokenType::Eof => write!(f,"EOF"),
        }
    }
}

impl TokenType {
    pub fn from_ident(ident: String) -> TokenType {
        match ident.as_ref() {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "for" => TokenType::For,
            "fun" => TokenType::Fun,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "true" => TokenType::True,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            ident => TokenType::Identifier(ident.to_string()),
        }
    }
}

