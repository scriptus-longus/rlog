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
  lex: lexer::Lexer,
  pub ast: Option<Node>,
}

impl Parser {
  pub fn new(lex: lexer::Lexer) -> Self {
    let mut p = Parser{lex: lex, ast: None};
    p.parse();
    p
  }

  pub fn parse(&mut self) {
    self.ast = Some(self.parse_expr());
  }

  pub fn parse_expr(&mut self) -> Node {
    let mut lh = self.parse_fact();

    while let Some(c) = self.lex.peek() {
      println!("C is {:?}", c);
      match c {
        lexer::Tokens::Op('∨') => {
          self.lex.pop();
          let rh =  self.parse_fact();
          lh = Node::Or(Box::new(lh), Box::new(rh));
        }

        lexer::Tokens::EOF | lexer::Tokens::Bclose => {break; },
        
        _ => panic!("Unexpected token found"),
      }
    }

    lh
  }


  fn parse_fact(&mut self) -> Node {
    let mut lh = self.parse_term();
    
    while let Some(c) = self.lex.peek() {
      match c {
        lexer::Tokens::Op('∧') => {
          self.lex.pop();
          println!("Calling parse term");
          let rh =  self.parse_term();
          println!("Returning parse term");
          lh = Node::And(Box::new(lh), Box::new(rh));
        }

        lexer::Tokens::EOF | lexer::Tokens::Op('∨') | lexer::Tokens::Bclose => {break; },
        
        _ => panic!("Unexpected token found fact {:?} ", c),
      }
    }

    lh
  }


  fn parse_term(&mut self) -> Node {
    match self.lex.pop() {
      Some(lexer::Tokens::Variable(c)) => return Node::Var(c),
      Some(lexer::Tokens::Neg) => return Node::Neg(Box::new(self.parse_term())),
      Some(lexer::Tokens::Bopen) => {
        let ret = self.parse_expr();
        if self.lex.pop().unwrap() != lexer::Tokens::Bclose {
          panic!("Expected closing ')'"); 
        }
        return ret;
      },
      _ => panic!("Expected some token"),
    }
  }
}

