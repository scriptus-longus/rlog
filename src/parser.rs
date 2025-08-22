use crate::lexer;

/*
---------------------------
Simple Prolog like grammar
---------------------------

<start> ::= <cmd> | <cmd> <start>

<cmd> ::= <fact> "." | <fact> ":-" <Rule> "." | ? (<Atoms>) : <fact>.
<fact> ::= <name>(<atoms>) | <atom>.

<atoms> ::=  <name> "," <Atoms> | <name>

<rule> ::= <Conj> ∨ <conj> | <Conj>
<conj> ::= <term> ∧ <term> | <term>
<term> ::= <fact> | ¬ <term> | ( <rule> )
*/

#[derive(Debug)]
pub enum Node {
  Query(Box<Node>),
  Fact(String, Option<Box<Node>>),
  Atoms(Box<Node>, Option<Box<Node>>),
  Rule(Box<Node>, Box<Node>),

  And(Box<Node>, Box<Node>),
  Or(Box<Node>, Box<Node>),

  Neg(Box<Node>),

  Atom(String),
  Variable(String)
}

impl Node {
  pub fn to_string(&self) -> String{
    match self {
      Node::Atom(x) => return x.to_string(),
      Node::Variable(x) => return x.to_string(),
      Node::And(x, y) => return String::from(format!("(∧ {} {})", x.to_string(), y.to_string())),
      Node::Or(x, y) => return String::from(format!("(∨ {}, {})", x.to_string(), y.to_string())),
      Node::Neg(x) => return String::from(format!("¬{}", x.to_string())),

      Node::Query(x) => return String::from(format!("? {} .", x.to_string())),
      Node::Fact(x, Some(y)) => return String::from(format!("{} ({}).", x, y.to_string())),
      Node::Fact(x, None) => return String::from(format!("{}.", x)),

      Node::Atoms(x, Some(y)) => return String::from(format!("{}, {}", x.to_string(), y.to_string())),
      Node::Atoms(x, None) => return String::from(format!("{}", x.to_string())),

      Node::Rule(x, y) => return String::from(format!("{} :- {}.", x.to_string(), y.to_string())),
    }
  }
}


#[derive(Debug)]
pub struct Parser {
  pub buffer: Vec<Node>,
}

impl Parser {
  pub fn new() -> Self {
    let p = Parser{buffer: vec![]};
    p
  }

  pub fn parse(&mut self, lex: &mut lexer::Lexer) -> Result<(), &'static str> {
    while !lex.buffer.is_empty() {
      // read line and check for syntax error
      let ast = self.parse_cmd(lex)?;

      // if line is not yet finished wait for lexer
      if Some(lexer::Tokens::EOL) != lex.pop() {
        return Err("Syntax Error: No EOL");
      }

      // prepare to read next line
      self.buffer.push(ast);
    }

    Ok(())
  }

  pub fn parse_cmd(&self, lex: &mut lexer::Lexer) -> Result<Node, &'static str> {
    match lex.peek().unwrap() {
      lexer::Tokens::Exists => {
        lex.pop(); 
        return self.parse_query(lex); 
      },
      _ => (),
    };

    let fact  = self.parse_fact(lex)?;

    match lex.peek().unwrap() {
      lexer::Tokens::Arrow => {
        lex.pop();
        let rule = self.parse_rule(lex)?;

        Ok(Node::Rule(Box::new(fact), Box::new(rule)))
      },
      lexer::Tokens::EOL => Ok(fact),
      _ => Err("Syntax Error: not a valid command"),
    }
  }

  pub fn parse_query(&self, lex: &mut lexer::Lexer) -> Result<Node, &'static str> {
    let fact = self.parse_fact(lex)?;

    match lex.peek().unwrap() {
      lexer::Tokens::EOL => Ok(Node::Query(Box::new(fact))),
      _ => Err("Syntax Error: Expected EOL"),
    }
  }

  pub fn parse_fact(&self, lex: &mut lexer::Lexer) -> Result<Node, &'static str> {
    // read name
    let name = match lex.pop().unwrap() {
      lexer::Tokens::Name(x) => x,
      _ => return Err("Syntax Error: Expected name parsing fact"),
    };

    // parse
    match lex.peek().unwrap() {
      lexer::Tokens::EOL | lexer::Tokens::Arrow => Ok(Node::Fact(name, None)),

      lexer::Tokens::Bopen => {
          lex.pop();
          let args = self.parse_atoms(lex)?;
          
          match lex.peek().unwrap() {
            lexer::Tokens::Bclose => {lex.pop(); Ok(Node::Fact(name, Some(Box::new(args))))},
            _ => Err("Syntax Error: Expected to find ')'"),
          }
        },

      _ => Err("Syntax Error: Unexpected Token inside of term"),
    }
  }

  pub fn parse_atoms(&self, lex: &mut lexer::Lexer) -> Result<Node, &'static str> {
    let atom = match lex.pop().unwrap() {
      lexer::Tokens::Name(atom) => {
        Node::Atom(atom)
      },
      lexer::Tokens::Variable(atom) => {
        Node::Variable(atom)
      },
      _ => return Err("Syntax Error: Expected to find atom"),
    };

    match lex.peek().unwrap() {
      lexer::Tokens::Delim => {
        lex.pop();
        let tail = self.parse_atoms(lex)?;
        Ok(Node::Atoms(Box::new(atom), Some(Box::new(tail))))
      },
      lexer::Tokens::EOL | lexer::Tokens::Bclose => Ok(Node::Atoms(Box::new(atom), None)),
      _ => Err("Syntax Error: Invalid Token parsing atom"),
    }
  }

  pub fn parse_rule(&self, lex: &mut lexer::Lexer) -> Result<Node, &'static str> {
    let mut lh = self.parse_conj(lex)?;

    while let Some(c) = lex.peek() {
      match c {
        lexer::Tokens::Op('∨') => {
          lex.pop();
          let rh =  self.parse_conj(lex)?;
          lh = Node::Or(Box::new(lh), Box::new(rh));
        },

        lexer::Tokens::EOL | lexer::Tokens::Bclose => {break; },
        
        _ => return Err("Syntax Error: Unexpected token found: while parsing expression"),
      }
    }

    Ok(lh)
  }


  fn parse_conj(&self, lex: &mut lexer::Lexer) -> Result<Node, &'static str> {
    let mut lh = self.parse_term(lex)?;
    
    while let Some(c) = lex.peek() {
      match c {
        lexer::Tokens::Op('∧') => {
          lex.pop();
          let rh =  self.parse_term(lex)?;
          lh = Node::And(Box::new(lh), Box::new(rh));
        },

        lexer::Tokens::EOL | lexer::Tokens::Op('∨') | lexer::Tokens::Bclose => {break; },
        
        _ => return Err("Unexpected token found while parsing fact"),
      }
    }

    Ok(lh)
  }


  fn parse_term(&self, lex: &mut lexer::Lexer) -> Result<Node, &'static str> {
    println!("{:?}", lex);
    match lex.peek().unwrap() {
      lexer::Tokens::Neg => {
        lex.pop();
        let sub = self.parse_term(lex)?;
        Ok(Node::Neg(Box::new(sub)))
      },

      lexer::Tokens::Bopen => {
        lex.pop();
        let ret = self.parse_term(lex)?;

        match lex.pop().unwrap() {
          lexer::Tokens::Bclose => Ok(ret),
          _ => Err("Syntax Error: Expected closing ')'")
        }
      },

      _ => self.parse_fact(lex),
    }
  }
}

