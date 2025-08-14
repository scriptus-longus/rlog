#[derive(Debug, PartialEq)]
pub enum Tokens {
  Variable(char),
  Bopen,
  Bclose,
  Op(char),
  Neg,
  EOF,
}

#[derive(Debug)]
pub struct Lexer {
  pub buffer: Vec<Tokens>,
}

impl Lexer {
  pub fn new(text: &str) -> Self {  // TODO: check pass by reference
    let mut lex = Lexer {buffer : vec![]};
    lex.consume(text);
    lex
    /*text.retain(|c| !c.is_whitespace() && c != ' ');
    println!("{}", text);
  
    let mut buf = Vec::new();

    for c in text.chars() {
      match c {
        '(' => buf.push(Tokens::Bopen),
        ')' => buf.push(Tokens::Bclose),
        '∧' => buf.push(Tokens::Op('∧')),
        '∨' => buf.push(Tokens::Op('∨')),
        '¬' => buf.push(Tokens::Neg),
         _ => buf.push(Tokens::Variable(c)),
      }
    }

    //buf.push(Tokens::EOF); 
    buf.reverse();

    Lexer {buffer: buf}*/
  }


  pub fn consume(&mut self, text: &str) {
    let byte_text: Vec<char>  = text.chars().filter(|x| !x.is_whitespace()).collect(); //.map(|x| x as u8).collect();

    let mut idx = 0;
    while idx < byte_text.len() {
      //let c = byte_text.pop_front();
      let c = byte_text[idx]; 

      match c {
        '(' => self.buffer.insert(0, Tokens::Bopen),
        ')' => self.buffer.insert(0,Tokens::Bclose),
        '∧' => self.buffer.insert(0,Tokens::Op('∧')),
        '∨' => self.buffer.insert(0,Tokens::Op('∨')),
        '¬' => self.buffer.insert(0,Tokens::Neg),
        '.' => self.buffer.insert(0,Tokens::EOF),
        _ => if !c.is_whitespace() {
            self.buffer.insert(0, Tokens::Variable(c));
        },
      };
      
      idx += 1;
    }
  }

  pub fn pop(&mut self) -> Option<Tokens> {
    self.buffer.pop()
  }

  pub fn peek(&self) -> Option<&Tokens> {
    self.buffer.last()
  }
}
