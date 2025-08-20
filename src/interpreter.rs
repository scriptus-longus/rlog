use std::collections::HashMap;

use crate::parser;
//mod parser;

#[derive(Debug)]
pub enum Name {
  Atom(String),
  Variable(String),
}

#[derive(Debug)]
struct FactArgs {
  len: usize,
  args: Vec<Vec<Name>>,
}

impl FactArgs {
  pub fn new(args: Vec<Name>) -> Self{
    FactArgs {len: args.len(), args: vec![args]}
  }

  pub fn add(&mut self, args: Vec<Name>) -> Result<(), &'static str>{
    if args.len() != self.len {
      return Err("Size of arguments does not match");
    }

    self.args.push(args);

    Ok(())
  }
}

#[derive(Debug)]
pub struct Env {
  facts: HashMap<String, FactArgs>,
}

impl Env {
  pub fn new() -> Self {
    //Env {atoms: vec![Atom("⊤")], facts: vec![]}
    Env {facts: HashMap::new()}
  }

  pub fn add_fact(&mut self, ast: &parser::Node) -> Result<(), &'static str>{
    // get name
    let (name, mut args) = match ast {
      parser::Node::Fact(name, args) => (name.clone(), args),
      _ => return Err("Error: Could not add fact"),
    };

    // unpack facts
    let mut fact_arguments: Vec<Name> = vec![];

    while let Some(x) = args {
      match &**x {
        parser::Node::Atoms(head, tail) => {
          match &**head {
            parser::Node::Atom(name) => {
              fact_arguments.push(Name::Atom(name.clone()));
              args = tail;
            },
            _ => return Err("Not a valid name for argument"),
          };
        },
        _ => return Err("Undefined Token unpacking arguments"),
      }
    }

    // add
    match self.facts.get_mut(&name) {
      Some(ref mut x) => {
        /*if x.len != fact_arguments.len() {
          return Err("Not the same number of arguments");
        } 
        
        x.args.push(fact_arguments);*/
        x.add(fact_arguments)?;
      },
      None => {
        //self.facts.insert(name, FactArgs {len: fact_arguments.len(), args: vec![fact_arguments]}); 
        self.facts.insert(name, FactArgs::new(fact_arguments));
      },
    }

    Ok(())
  }

  pub fn print_all_facts(&self) {
    for (name, args) in &self.facts {
      for arg in &args.args {
        println!("Fact: {} over {:?}", name, arg);
      }
    }
  }
}
