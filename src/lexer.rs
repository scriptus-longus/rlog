#[derive(Debug, PartialEq)]
pub enum Tokens {
  Name(String),
  Bopen,
  Bclose,
  Op(char),
  Exists,
  Seperator,
  Neg,
  Arrow,
  Delim,
  EOL,
  EOF,
}

#[derive(Debug)]
pub struct Lexer {
  pub buffer: Vec<Tokens>,
}

impl Lexer {
  pub fn new(text: String) -> Self {  // TODO: check pass by reference
    let mut lex = Lexer {buffer : vec![]};
    lex.consume(text);

    lex
  }

  fn get_var(text: &mut String) -> String {
    let idx = text.find(|c: char| !(c.is_lowercase() || c.is_lowercase()) ).unwrap_or(text.len());
    let name: String = text.drain(..idx).collect();

    name
  }

  pub fn consume(&mut self, mut text: String) {
    //let mut text: String = text.chars().filter(|&x| x != '\t' && x != '\n').collect();

    while !text.is_empty() {
      let c = text.remove(0);

      match c {
        '(' => self.buffer.insert(0, Tokens::Bopen),
        ')' => self.buffer.insert(0, Tokens::Bclose),
        '∧' => self.buffer.insert(0, Tokens::Op('∧')),
        '∨' => self.buffer.insert(0, Tokens::Op('∨')),
        '¬' => self.buffer.insert(0, Tokens::Neg),
        '.' => self.buffer.insert(0, Tokens::EOL),
        ',' => self.buffer.insert(0, Tokens::Delim),
        '?' => self.buffer.insert(0, Tokens::Exists),
        ':' => {
          if text.chars().nth(0).unwrap() == '-' {
            self.buffer.insert(0, Tokens::Arrow)
          } else {
            self.buffer.insert(0, Tokens::Seperator)
          }
        },
        _ => {
          if !c.is_whitespace() {
            if c.is_lowercase() || c.is_uppercase() {
              let name = format!("{}{}", c, Self::get_var(&mut text));

              self.buffer.insert(0, Tokens::Name(name));
            }
            //self.buffer.insert(0, Tokens::Variable(c));
          }
        },
      };
    
      //text.remove(0);
    }
    println!("BUFFER: {:?}", self.buffer);
  }

  pub fn pop(&mut self) -> Option<Tokens> {
    self.buffer.pop()
  }

  pub fn peek(&self) -> Option<&Tokens> {
    self.buffer.last()
  }
}
