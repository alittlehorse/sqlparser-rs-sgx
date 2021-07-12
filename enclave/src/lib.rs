// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License..



#![crate_name = "sqlparserenclave"]
#![crate_type = "staticlib"]

#![cfg_attr(not(target_env = "sgx"), no_std)]
#![cfg_attr(target_env = "sgx", feature(rustc_private))]
#![feature(slice_ptr_len)]
extern crate sgx_types;
#[cfg(not(target_env = "sgx"))]
#[macro_use]
extern crate sgx_tstd as std;

extern crate sgx_serialize;
use sgx_serialize::{SerializeHelper, DeSerializeHelper};
#[macro_use]
extern crate sgx_serialize_derive;


use std::ptr;
use sgx_types::*;
use std::string::String;
use std::vec::Vec;
use std::io::{self, Write};
use std::slice;
use std::fmt;

use std::iter::Peekable;
use std::str::Chars;
mod dialect;
use dialect::Dialect;
use dialect::keywords::Keyword;

#[no_mangle]
#[warn(unused_assignments)]
pub extern "C" fn lexer(sql: *const u8, sql_len: usize,mut output: *mut u8,mut output_len:*mut usize) -> sgx_status_t {

    println!("{}", "we are in Enclave now");
    let str_slice = unsafe { slice::from_raw_parts(sql, sql_len) };
    let mut query = String::from_utf8(str_slice.to_vec()).unwrap();
    println!("{}", &query);

    // assert_eq!(test_for_fmt(),true);
    // test_Tokenizer_new(&query);
    // test_make_word();
    // tokenize_select_1();
    // println!("test end!");

    let dialect = dialect::AnsiDialect {};
    let mut tokenizer = Tokenizer::new(&dialect, &query);
    let tokens = tokenizer.tokenize().unwrap();
    let helper = SerializeHelper::new();
    println!("------------------------------");
    println!("tokens   = {:?}", &tokens);
    let mut json = helper.encode(&tokens).unwrap();
    println!("{:?}",&json);
    //let serialized = serde_json::to_string(&json);
    let output_slice = (&mut json[..]);
    unsafe{
        *output_len = output_slice.len();
        //println!("in encalve :the len is {}\n",*output_len);
    }
    unsafe{
        output_slice.as_mut_ptr().copy_to(output, *output_len)
        //  output,
        //  output_len);
         }

   // output = output_slice.as_mut_ptr();
    // unsafe{
    //     let re = CString::from_vec_unchecked(json);
    //     result = re.into_raw();

    // };
    //result_len = len;
    sgx_status_t::SGX_SUCCESS
}

#[derive(Debug, Clone, PartialEq, Eq, Hash,Serializable, DeSerializable)]
pub enum Token {
    /// An end-of-file marker, not a real token
    EOF,
    /// A keyword (like SELECT) or an optionally quoted SQL identifier
    Word(Word),
    /// An unsigned numeric literal
    Number(String),
    /// A character that could not be tokenized
    Char(char),
    /// Single quoted string: i.e: 'string'
    SingleQuotedString(String),
    /// "National" string literal: i.e: N'string'
    NationalStringLiteral(String),
    /// Hexadecimal string literal: i.e.: X'deadbeef'
    HexStringLiteral(String),
    /// Comma
    Comma,
    /// Whitespace (space, tab, etc)
    Whitespace(Whitespace),
    /// Double equals sign `==`
    DoubleEq,
    /// Equality operator `=`
    Eq,
    /// Not Equals operator `<>` (or `!=` in some dialects)
    Neq,
    /// Less Than operator `<`
    Lt,
    /// Greater Than operator `>`
    Gt,
    /// Less Than Or Equals operator `<=`
    LtEq,
    /// Greater Than Or Equals operator `>=`
    GtEq,
    /// Plus operator `+`
    Plus,
    /// Minus operator `-`
    Minus,
    /// Multiplication operator `*`
    Mult,
    /// Division operator `/`
    Div,
    /// Modulo Operator `%`
    Mod,
    /// String concatenation `||`
    StringConcat,
    /// Left parenthesis `(`
    LParen,
    /// Right parenthesis `)`
    RParen,
    /// Period (used for compound identifiers or projections into nested types)
    Period,
    /// Colon `:`
    Colon,
    /// SemiColon `;` used as separator for COPY and payload
    SemiColon,
    /// Backslash `\` used in terminating the COPY payload with `\.`
    Backslash,
    /// Left bracket `[`
    LBracket,
    /// Right bracket `]`
    RBracket,
    /// Ampersand `&`
    Ampersand,
    /// Pipe `|`
    Pipe,
    /// Caret `^`
    Caret,
    /// Left brace `{`
    LBrace,
    /// Right brace `}`
    RBrace,
}

impl Token {
    pub fn make_keyword(keyword: &str) -> Self {
        Token::make_word(keyword, None)
    }
    pub fn make_word(word: &str, quote_style: Option<char>) -> Self {
        let word_uppercase = word.to_uppercase();
        Token::Word(Word {
            value: String::from(word),
            quote_style,
            keyword: Keyword::NONE
            //TODO: match value and keyword!!!!
        })
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::EOF => f.write_str("EOF"),
            Token::Word(ref w) => write!(f, "{}", w),
            Token::Number(ref n) => write!(f, "{}", n),
            Token::Char(ref c) => write!(f, "{}", c),
            Token::SingleQuotedString(ref s) => write!(f, "'{}'", s),
            Token::NationalStringLiteral(ref s) => write!(f, "N'{}'", s),
            Token::HexStringLiteral(ref s) => write!(f, "X'{}'", s),
            Token::Comma => f.write_str(","),
            Token::Whitespace(ws) => write!(f, "{}", ws),
            Token::DoubleEq => f.write_str("=="),
            Token::Eq => f.write_str("="),
            Token::Neq => f.write_str("<>"),
            Token::Lt => f.write_str("<"),
            Token::Gt => f.write_str(">"),
            Token::LtEq => f.write_str("<="),
            Token::GtEq => f.write_str(">="),
            Token::Plus => f.write_str("+"),
            Token::Minus => f.write_str("-"),
            Token::Mult => f.write_str("*"),
            Token::Div => f.write_str("/"),
            Token::StringConcat => f.write_str("||"),
            Token::Mod => f.write_str("%"),
            Token::LParen => f.write_str("("),
            Token::RParen => f.write_str(")"),
            Token::Period => f.write_str("."),
            Token::Colon => f.write_str(":"),
            Token::SemiColon => f.write_str(";"),
            Token::Backslash => f.write_str("\\"),
            Token::LBracket => f.write_str("["),
            Token::RBracket => f.write_str("]"),
            Token::Ampersand => f.write_str("&"),
            Token::Caret => f.write_str("^"),
            Token::Pipe => f.write_str("|"),
            Token::LBrace => f.write_str("{"),
            Token::RBrace => f.write_str("}"),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash,Serializable, DeSerializable)]
pub struct Word {
    /// The value of the token, without the enclosing quotes, and with the
    /// escape sequences (if any) processed (TODO: escapes are not handled)
    pub value: String,
    /// An identifier can be "quoted" (&lt;delimited identifier> in ANSI parlance).
    /// The standard and most implementations allow using double quotes for this,
    /// but some implementations support other quoting styles as well (e.g. \[MS SQL])
    pub quote_style: Option<char>,
    /// If the word was not quoted and it matched one of the known keywords,
    /// this will have one of the values from dialect::keywords, otherwise empty
    pub keyword: Keyword,
}


impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.quote_style {
            Some(s) if s == '"' => {
                write!(f, "\"{}\"", self.value )
            }
            None => f.write_str(&self.value),
            _ => panic!("Unexpected quote_style!"),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash,Serializable, DeSerializable)]
pub enum Whitespace {
    Space,
    Newline,
    Tab,
    LineComment(String),
}

impl fmt::Display for Whitespace {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Whitespace::Space => f.write_str(" "),
            Whitespace::Newline => f.write_str("\n"),
            Whitespace::Tab => f.write_str("\t"),
            Whitespace::LineComment(s) => write!(f, "{}", s),
        }
    }
}
#[derive(Debug, PartialEq,Serializable, DeSerializable)]
pub struct TokenizerError {
    pub message: String,
    pub line: u64,
    pub col: u64,
}

pub struct Tokenizer<'a> {
    dialect: &'a dyn Dialect,
    pub query: String,
    pub line: u64,
    pub col: u64,
}

impl<'a> Tokenizer<'a> {
    /// Create a new SQL tokenizer for the specified SQL statement
    pub fn new(dialect: &'a dyn Dialect, query: &str) -> Self {
        Self {
            dialect,
            query: String::from(query),
            line: 1,
            col: 1,
        }
    }
    pub fn tokenize(&mut self) -> Result<Vec<Token>, TokenizerError> {
        let mut peekable = self.query.chars().peekable();

        let mut tokens: Vec<Token> = vec![];

        while let Some(token) = self.next_token(&mut peekable)? {
            match &token {
                Token::Whitespace(Whitespace::Newline) => {
                    self.line += 1;
                    self.col = 1;
                }

                Token::Whitespace(Whitespace::Tab) => self.col += 4,
                Token::Word(w) if w.quote_style == None => self.col += w.value.len() as u64,
                Token::Word(w) if w.quote_style != None => self.col += w.value.len() as u64 + 2,
                Token::Number(s) => self.col += s.len() as u64,
                Token::SingleQuotedString(s) => self.col += s.len() as u64,
                _ => self.col += 1,
            }

            tokens.push(token);
        }
        Ok(tokens)
    }

    fn next_token(&self, chars: &mut Peekable<Chars<'_>>) -> Result<Option<Token>, TokenizerError> {
        //println!("next_token: {:?}", chars.peek());
        match chars.peek() {
            Some(&ch) => match ch {
                ' ' => self.consume_and_return(chars, Token::Whitespace(Whitespace::Space)),
                '\t' => self.consume_and_return(chars, Token::Whitespace(Whitespace::Tab)),
                '\n' => self.consume_and_return(chars, Token::Whitespace(Whitespace::Newline)),
                '\r' => {
                    // Emit a single Whitespace::Newline token for \r and \r\n
                    chars.next();
                    if let Some('\n') = chars.peek() {
                        chars.next();
                    }
                    Ok(Some(Token::Whitespace(Whitespace::Newline)))
                }
                // identifier or keyword
                ch if self.dialect.is_identifier_start(ch) => {
                    chars.next(); // consume the first char
                    let s = self.tokenize_word(ch, chars);

                    if s.chars().all(|x| ('0'..='9').contains(&x) || x == '.') {
                        let mut s = peeking_take_while(&mut s.chars().peekable(), |ch| {
                            matches!(ch, '0'..='9' | '.')
                        });
                        let s2 = peeking_take_while(chars, |ch| matches!(ch, '0'..='9' | '.'));
                        s += s2.as_str();
                        return Ok(Some(Token::Number(s)));
                    }
                    Ok(Some(Token::make_word(&s, None)))
                }
                // string
                '\'' => {
                    let s = self.tokenize_single_quoted_string(chars)?;
                    Ok(Some(Token::SingleQuotedString(s)))
                }
                // delimited (quoted) identifier
                '\"' if self.dialect.is_delimited_identifier_start('\"') => {
                    chars.next(); // consume the opening quote
                    let s = peeking_take_while(chars, |ch| ch != '\"');
                    if chars.next() == Some('\"') {
                        Ok(Some(Token::make_word(&s, Some('\"'))))
                    } else {
                        self.tokenizer_error(
                            format!("Expected close delimiter '{}' before EOF.", '\"')
                                .as_str(),
                        )
                    }
                }
                // numbers
                '0'..='9' => {
                    let mut s = peeking_take_while(chars, |ch| matches!(ch, '0'..='9'));
                    // match one period
                    Ok(Some(Token::Number(s)))
                }
                // punctuation
                '(' => self.consume_and_return(chars, Token::LParen),
                ')' => self.consume_and_return(chars, Token::RParen),
                ',' => self.consume_and_return(chars, Token::Comma),
                // operators
                '-' => self.consume_and_return(chars, Token::Minus),
                '/' => {
                    chars.next(); // consume the '/'
                    match chars.peek() {
                        Some('/')  => {
                            chars.next(); // consume the second '/', starting a snowflake single-line comment
                            let comment = self.tokenize_single_line_comment(chars);
                            Ok(Some(Token::Whitespace(Whitespace::LineComment(comment))))
                        }
                        // a regular '/' operator
                        _ => Ok(Some(Token::Div)),
                    }
                }
                '+' => self.consume_and_return(chars, Token::Plus),
                '*' => self.consume_and_return(chars, Token::Mult),
                '%' => self.consume_and_return(chars, Token::Mod),
                '|' => {
                    chars.next(); // consume the '|'
                    match chars.peek() {
                        Some('|') => self.consume_and_return(chars,Token::StringConcat),
                        _ => Ok(Some(Token::Pipe))
                        }
                    }
                '=' => self.consume_and_return(chars, Token::Eq),

                '<' => {
                    chars.next(); // consume
                    match chars.peek() {
                        Some('=') => self.consume_and_return(chars, Token::LtEq),
                        Some('>') => self.consume_and_return(chars, Token::Neq),
                        _ => Ok(Some(Token::Lt)),
                    }
                }
                '>' => {
                    chars.next(); // consume
                    match chars.peek() {
                        Some('=') => self.consume_and_return(chars, Token::GtEq),
                        _ => Ok(Some(Token::Gt)),
                    }
                }
                ':' =>  self.consume_and_return(chars, Token::Colon),
                ';' => self.consume_and_return(chars, Token::SemiColon),
                '\\' => self.consume_and_return(chars, Token::Backslash),
                '[' => self.consume_and_return(chars, Token::LBracket),
                ']' => self.consume_and_return(chars, Token::RBracket),
                '&' => self.consume_and_return(chars, Token::Ampersand),
                '^' => self.consume_and_return(chars, Token::Caret),
                '{' => self.consume_and_return(chars, Token::LBrace),
                '}' => self.consume_and_return(chars, Token::RBrace),
                other => self.consume_and_return(chars, Token::Char(other)),
            }
            None => Ok(None),
        }
    }

    fn tokenizer_error<R>(&self, message: &str) -> Result<R, TokenizerError> {
        Err(TokenizerError {
            message: String::from(message),
            col: self.col,
            line: self.line,
        })
    }


    #[allow(clippy::unnecessary_wraps)]
    fn consume_and_return(
        &self,
        chars: &mut Peekable<Chars<'_>>,
        t: Token,
    ) -> Result<Option<Token>, TokenizerError> {
        chars.next();
        Ok(Some(t))
    }

    /// Tokenize an identifier or keyword, after the first char is already consumed.
    fn tokenize_word(&self, first_char: char, chars: &mut Peekable<Chars<'_>>) -> String {
        let mut s = String::from(first_char);
        s.push_str(&peeking_take_while(chars, |ch| {
            self.dialect.is_identifier_part(ch)
        }));
        s
    }
        /// Read a single quoted string, starting with the opening quote.
    fn tokenize_single_quoted_string(
        &self,
        chars: &mut Peekable<Chars<'_>>,
    ) -> Result<String, TokenizerError> {
        let mut s = String::new();
        chars.next(); // consume the opening quote
        while let Some(&ch) = chars.peek() {
            match ch {
                '\'' => {
                    chars.next(); // consume
                    let escaped_quote = chars.peek().map(|c| *c == '\'').unwrap_or(false);
                    if escaped_quote {
                        s.push('\'');
                        chars.next();
                    } else {
                        return Ok(s);
                    }
                }
                _ => {
                    chars.next(); // consume
                    s.push(ch);
                }
            }
        }
        self.tokenizer_error("Unterminated string literal")
    }
    // Consume characters until newline
    fn tokenize_single_line_comment(&self, chars: &mut Peekable<Chars<'_>>) -> String {
        let mut comment = peeking_take_while(chars, |ch| ch != '\n');
        if let Some(ch) = chars.next() {
            assert_eq!(ch, '\n');
            comment.push(ch);
        }
        comment
    }
}

/// Read from `chars` until `predicate` returns `false` or EOF is hit.
/// Return the characters read as String, and keep the first non-matching
/// char available as `chars.next()`.
fn peeking_take_while(
    chars: &mut Peekable<Chars<'_>>,
    mut predicate: impl FnMut(char) -> bool,
) -> String {
    let mut s = String::new();
    while let Some(&ch) = chars.peek() {
        if predicate(ch) {
            chars.next(); // consume
            s.push(ch);
        } else {
            break;
        }
    }
    s
}


//========================================
//test scop
pub fn test_for_fmt()->bool{
    let word = Word{
        value:String::from("SELECT"),
        quote_style : None,
        keyword : Keyword::SELECT,
    };
    let mut result = true;
    result = match word.keyword{
       Keyword::SELECT => true,
       _ => false
    };
    println!("Word fmt:{}",word.value);
    if word.value != String::from("SELECT")
    {
       result = false;
    }
    let token = Token::Word(word);
    println!("Token fmt:{}",token);
    result = match token{
        Token::Word(Word)=>true,
        _ => false
    };
    result
}

pub fn test_Tokenizer_new(query:&String){
    let dialect = dialect::AnsiDialect {};
    //let result = true;
    let tokenizer = Tokenizer::new(&dialect,query);
    assert_eq!(tokenizer.col,1);
    assert_eq!(tokenizer.line,1);
    assert_eq!(tokenizer.query,String::from(query));
    assert_eq!(tokenizer.dialect.is_delimited_identifier_start('\"'),true);
    //result
}

pub fn test_make_word(){
    println!("test make_word fn!");
    let token = Token::make_keyword("ROW");
    //let token1 = Token::make_word("id#1", Optioin<char>('\"'));
    let a = match token{
        Token::Word(w)=> w,
        _ => Word{
            value:String::from("ERRO"),
            quote_style : None,
            keyword : Keyword::NONE,
        },
    };
    assert_eq!(a.value,String::from("ROW"));
}

pub fn tokenize_select_1() {
    let sql = String::from("SELECT 1");
    let dialect = dialect::AnsiDialect {};
    let mut tokenizer = Tokenizer::new(&dialect, &sql);
    let tokens = tokenizer.tokenize().unwrap();

    let expected = vec![
        Token::make_keyword("SELECT"),
        Token::Whitespace(Whitespace::Space),
        Token::Number(String::from("1")),
    ];

    compare(expected, tokens);
}


fn compare(expected: Vec<Token>, actual: Vec<Token>) {
    println!("------------------------------");
    println!("tokens   = {:?}", actual);
    println!("expected = {:?}", expected);
    println!("------------------------------");
    assert_eq!(expected, actual);
}