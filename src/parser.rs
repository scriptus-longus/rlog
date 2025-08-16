use crate::lexer;

#[derive(Debug)]
pub enum Node {
  Var(char),
  And(Box<Node>, Box<Node>),
  Or(Box<Node>, Box<Node>),
  Neg(Box<Node>),
}

impl Node {
  pub fn to_string(&self) -> String{
    match self {
      Node::Var(c) => return c.to_string(),
      Node::And(x, y) => return String::from(format!("(∧ {} {})", x.to_string(), y.to_string())),
      Node::Or(x, y) => return String::from(format!("(∨ {}, {})", x.to_string(), y.to_string())),
      Node::Neg(x) => return String::from(format!("¬{}", x.to_string())),
    }
  }
}


#[derive(Debug)]
pub struct Parser {
  //lex: lexer::lexer::Lexer,
  //pub ast: Option<Node>,
  pub buffer: Vec<Node>,
}

impl Parser {
  pub fn new() -> Self {
    let p = Parser{buffer: vec![]};
    //p.parse();
    p
  }

  /*pub fn parse(&mut self, &mut lexer::Lexer) {
    //self.ast = Some(self.parse_expr());
    let mut x = lexer::Tokens::EOF;

    while Some(x) == lex.peek() {
      ast =    
    }
  }*/

  pub fn parse(&mut self, lex: &mut lexer::Lexer) -> Result<(), &'static str> {
    while !lex.buffer.is_empty() {
      // read line and check for syntax error
      let ast = self.parse_expr(lex)?;

      // if line is not yet finished wait for lexer
      if Some(lexer::Tokens::EOF) != lex.pop() {
        return Err("Syntax Error: No EOL");
      }

      // prepare to read next line
      self.buffer.push(ast);
    }

    Ok(())
  }

  pub fn parse_expr(&self, lex: &mut lexer::Lexer) -> Result<Node, &'static str> {
    //let mut lh = match self.ast {
    //  Some(N) => N,
    //  None => self.parse_fact(lex),
    //};
    let mut lh = self.parse_fact(lex)?;

    while let Some(c) = lex.peek() {
      match c {
        lexer::Tokens::Op('∨') => {
          lex.pop();
          let rh =  self.parse_fact(lex)?;
          lh = Node::Or(Box::new(lh), Box::new(rh));
        },

        lexer::Tokens::EOF | lexer::Tokens::Bclose => {break; },
        
        _ => return Err("Syntax Error: Unexpected token found: while parsing expression"),
      }
    }

    Ok(lh)
  }


  fn parse_fact(&self, lex: &mut lexer::Lexer) -> Result<Node, &'static str> {
    let mut lh = self.parse_term(lex)?;
    
    while let Some(c) = lex.peek() {
      match c {
        lexer::Tokens::Op('∧') => {
          lex.pop();
          let rh =  self.parse_term(lex)?;
          lh = Node::And(Box::new(lh), Box::new(rh));
        },

        lexer::Tokens::EOF | lexer::Tokens::Op('∨') | lexer::Tokens::Bclose => {break; },
        
        _ => return Err("Unexpected token found while parsing fact"),
      }
    }

    Ok(lh)
  }


  fn parse_term(&self, lex: &mut lexer::Lexer) -> Result<Node, &'static str> {
    match lex.pop() {
      Some(lexer::Tokens::Variable(c)) => return Ok(Node::Var(c)),
      Some(lexer::Tokens::Neg) => {
        let sub = self.parse_term(lex)?;

        Ok(Node::Neg(Box::new(sub))) 
        //return Ok(Node::Neg(Box::new(self.parse_term(lex)?)))
      },
      Some(lexer::Tokens::Bopen) => {
        let ret = self.parse_expr(lex)?;
        
        if lex.pop().unwrap() != lexer::Tokens::Bclose {  
          return Err("Syntax Error: Expected closing ')'");
        }

        Ok(ret)
      },
      _ => return Err("Syntax Error: Could not resolve term"),
    }
  }
}

