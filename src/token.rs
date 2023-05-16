use std::fmt::{self,Display};

#[derive(Clone)]
pub struct Token {
    kind: TokenKind,
    literal: Option<String>,
    lexeme: String,
    line: usize,
}

impl Token {
    pub fn new(kind: TokenKind, line: usize) -> Token {
        let lexeme = kind.lexeme();
        Token {
            kind,
            literal: None,
            lexeme,
            line,
        }
    }

    pub fn kind(&self) -> &TokenKind {
        &self.kind
    }

    pub fn lexeme(&self) -> &str {
        &self.lexeme
    }

    pub fn line(&self) -> usize {
        self.line
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //TODO: , self.lexeme, self.literal
        write!(f, "{}", self.kind)
    }
}


#[derive(PartialEq, Clone, Debug)]
pub enum TokenKind {
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

impl Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LeftParen => write!(f,"LEFT_PAREN"),
            Self::RightParent => write!(f,"RIGHT_PAREN"),
            Self::LeftBrace => write!(f,"LEFT_BRACE"),
            Self::RightBrace => write!(f,"RIGHT_BRACE"), 
            Self::Comma => write!(f,"COMMA"),
            Self::Dot => write!(f,"DOT"),
            Self::Minus => write!(f,"MINUS"),
            Self::Plus => write!(f,"PLUS"),
            Self::Semicolon => write!(f,"SEMICOLON"),
            Self::Slash => write!(f,"SLASH"),
            Self::Star => write!(f,"STAR"),
            Self::Bang => write!(f,"BANG"),
            Self::BangEqual => write!(f,"BANG_EQUAL"),
            Self::Equal => write!(f,"EQUAL"),
            Self::EqualEqual => write!(f,"EQUAL_EQUAL"),
            Self::Greater => write!(f,"GREATER"),
            Self::GreaterEqual => write!(f,"GREATER_EQUAL"),
            Self::Less => write!(f,"LESS"),
            Self::LessEqual => write!(f,"LESS_EQUAL"),
            Self::Identifier(s) => write!(f,"IDENTIFIER ({})", s),
            Self::String(s) => write!(f,"STRING ({})",s),
            Self::Number(n) => write!(f,"NUMBER ({})",n),
            Self::And => write!(f,"AND"),
            Self::Class => write!(f,"CLASS"),
            Self::Else => write!(f,"ELSE"),
            Self::False => write!(f,"FALSE"),
            Self::Fun => write!(f,"FUN"),
            Self::For => write!(f,"FOR"),
            Self::If => write!(f,"IF"),
            Self::Nil => write!(f,"NIL"),
            Self::Or => write!(f,"OR"),
            Self::Print => write!(f,"PRINT"),
            Self::Return => write!(f,"RETURN"),
            Self::Super => write!(f,"SUPER"),
            Self::This => write!(f,"THIS"),
            Self::True => write!(f,"TRUE"),
            Self::Var => write!(f,"VAR"),
            Self::While => write!(f,"WHILE"),
            Self::Eof => write!(f,"EOF"),
        }
    }
}

impl TokenKind {
    pub fn from_ident(ident: String) -> TokenKind {
        match ident.as_ref() {
            "and" => Self::And,
            "class" => Self::Class,
            "else" => Self::Else,
            "false" => Self::False,
            "for" => Self::For,
            "fun" => Self::Fun,
            "if" => Self::If,
            "nil" => Self::Nil,
            "or" => Self::Or,
            "print" => Self::Print,
            "return" => Self::Return,
            "super" => Self::Super,
            "this" => Self::This,
            "true" => Self::True,
            "var" => Self::Var,
            "while" => Self::While,
            ident => Self::Identifier(ident.to_string()),
        }
    }

    fn lexeme(&self) -> String {
        match self {
            Self::LeftParen => "(".to_string(),
            Self::RightParent => ")".to_string(),
            Self::LeftBrace => "[".to_string(),
            Self::RightBrace => "]".to_string(), 
            Self::Comma => ",".to_string(),
            Self::Dot => ".".to_string(),
            Self::Minus => "-".to_string(),
            Self::Plus => "+".to_string(),
            Self::Semicolon => ";".to_string(),
            Self::Slash => "/".to_string(),
            Self::Star => "*".to_string(),
            Self::Bang => "!".to_string(),
            Self::BangEqual => "!=".to_string(),
            Self::Equal => "=".to_string(),
            Self::EqualEqual => "==".to_string(),
            Self::Greater => ">".to_string(),
            Self::GreaterEqual => ">=".to_string(),
            Self::Less => "<".to_string(),
            Self::LessEqual => "<=".to_string(),
            Self::Identifier(s) => s.to_string(),
            Self::String(s) => s.to_string(),
            Self::Number(n) => n.to_string(),
            Self::And => "&&".to_string(),
            Self::Class => "class".to_string(),
            Self::Else => "else".to_string(),
            Self::False => "false".to_string(),
            Self::Fun => "fun".to_string(),
            Self::For => "for".to_string(),
            Self::If => "if".to_string(),
            Self::Nil => "nil".to_string(),
            Self::Or => "or".to_string(),
            Self::Print => "print".to_string(),
            Self::Return => "return".to_string(),
            Self::Super => "super".to_string(),
            Self::This => "this".to_string(),
            Self::True => "true".to_string(),
            Self::Var => "var".to_string(),
            Self::While => "while".to_string(),
            Self::Eof => "".to_string(),
        }
    }
}

