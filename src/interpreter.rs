use std::collections::HashMap;
use std::fmt;

use crate::parser;
//mod parser;

#[derive(Debug, PartialEq)]
pub enum Name {
  Atom(String),
  Variable(String),
}

#[derive(Debug)]
pub struct Fact {
  len: usize,
  universe: String,
  args: Vec<Name>,
}

impl Fact {
  pub fn new(name: String, args: Vec<Name>) -> Self{
    Fact {len: args.len(), universe: name, args: args}
  }

  pub fn matches(&self, other: &Fact) -> bool {
    if self.len != other.len {
      return false;
    }

    for i in 0..self.len {
      match (&self.args[i], &other.args[i]) {
        (Name::Variable(_), Name::Atom(_)) => (),
        (Name::Atom(_), Name::Variable(_)) => (),
        (Name::Variable(_), Name::Variable(_)) => (),
        (Name::Atom(x), Name::Atom(y)) => {
          if x != y {
            return false;
          }
        },
      };
    };

    true
  }
}

/*#[derive(Debug)]
pub struct Query {
  name: String,
  args: Vec<Name>,
}*/


#[derive(Debug)]
pub struct Env {
  facts: HashMap<String, Vec<Fact>>,
  pub queries: Vec<Fact>,
}

impl Env {
  pub fn new() -> Self {
    //Env {atoms: vec![Atom("⊤")], facts: vec![]}
    Env {facts: HashMap::new(), queries: vec![]}
  }
  
  fn unpack_fact(&self, ast: &parser::Node) -> Option<Fact> {
    let (name, mut arg_head) = match ast{
      parser::Node::Fact(name, h) => (name.clone(), h),
      _ => return None,
    };

    let mut args: Vec<Name> = vec![];
    while let Some(x) = arg_head {
      let (h, t) = match &**x {
        parser::Node::Atoms(x, y) => (x, y),
        _ => return None,
      };
      //let parser::Node::Atoms(h, t) = &**x;

      match &**h {
        parser::Node::Atom(name) => args.push(Name::Atom(name.clone())),
        parser::Node::Variable(name) => args.push(Name::Variable(name.clone())),
        _ => return None,
      };

      arg_head = t;
    }

    /*while let Some(x) = arg_head {
      match &**x {
        parser::Node::Atoms(h, t) => {
          match &**h {
            parser::Node::Atom(name) => {
              args.push(Name::Atom(name.clone()));
              arg_head = t;
            },
            _ => return None,
          };
        },
        _ => return None,
      }
    };*/

    Some(Fact {len: args.len(), universe: name, args: args})
  }


  pub fn add_query(&mut self, ast: &parser::Node) -> Result<(), &'static str> {
    let fact_ast : &parser::Node = match ast {
      parser::Node::Query(x) => &*x,
      _ => return Err("Expected Fact afte query"),
    };

    let query_fact= self.unpack_fact(fact_ast).unwrap();


    self.queries.push(query_fact);
    Ok(())
  }


  pub fn add_fact(&mut self, ast: &parser::Node) -> Result<(), &'static str> {
    // get name
    let fact = self.unpack_fact(ast).unwrap();
    /*
    let mut query = false;

    let (name, mut args) = match ast {
      parser::Node::Fact(name, args) => (name.clone(), args),
      parser::Node::Query(x) => {
        query = true;

        match &**x {
          parser::Node::Fact(name, args) => (name.clone(), args),
          _ => return Err("Error: could not unpack query"),
        }
      },
      _ => return Err("Error: Could not add fact"),
    };

    // unpack facts
    let mut arg_vec: Vec<Name> = vec![];

    while let Some(x) = args {
      match &**x {
        parser::Node::Atoms(head, tail) => {
          match &**head {
            parser::Node::Atom(name) => {
              arg_vec.push(Name::Atom(name.clone()));
              args = tail;
            },
            _ => return Err("Not a valid name for argument"),
          };
        },
        _ => return Err("Undefined Token unpacking arguments"),
      }
    }*/

    // add
    match self.facts.get_mut(&fact.universe) {
      Some(ref mut x) => {
        /*if x.len != arg_vec.len() {
          return Err("Not the same number of arguments");
        } 
          
          x.args.push(arg_vec);*/
        //x.add(fact)?;
        x.push(fact)
      },
      None => {
        //self.facts.insert(name, FactArgs {len: arg_vec.len(), args: vec![arg_vec]}); 
        self.facts.insert(fact.universe.clone(), vec![fact]);
      },
    };
    Ok(())
  }
  

  pub fn print_all_facts(&self) {
    println!("facts: {:?}", self.facts);
  }
  pub fn print_query(&self) {
    println!("queries: {:?}", self.queries);
  }

  /*pub fn pop_query(&mut self) -> Option<Query> {
    self.queries.pop()
  }*/

  // query a simple fact
  pub fn query_fact(&self, query: &Fact) -> Option<&Fact> {
    let universe = self.facts.get(&query.universe)?;
    universe.iter().filter(|x| x.matches(query)).next()

    //if universe.len != query.args.len() {
    //  return None;
    //}

    //universe.args.iter().filter(|&x| x == &query.args).next()
  }
}
