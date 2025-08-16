use rustyline::{DefaultEditor, Result};
use rustyline::error::ReadlineError;
use std::collections::HashSet;
use itertools::Itertools;

mod lexer;
mod parser;

#[derive(Debug)] 
#[derive(Clone)]
pub struct GoalAtom { 
  name: char,
  neg: bool,
}

fn con_and(lh: &mut Vec<Vec<GoalAtom>>, rh: &mut Vec<Vec<GoalAtom>>) -> Vec<Vec<GoalAtom>> {
  let mut ret = lh.clone();

  for x in ret.iter_mut() {
    for y in rh.iter_mut() {
      x.append(y);
    }
  }

  return ret;
}

fn con_neg(goals: &[Vec<GoalAtom>]) -> Vec<Vec<GoalAtom>> {
  if goals.is_empty() {
    return vec![vec![]];
  }

  let first = &goals[0];
  let tail = con_neg(&goals[1..]);

  first.iter()
      .flat_map(|v| {
        tail.iter().map(move |t| {
          let mut new_tail = t.clone();
          new_tail.push(GoalAtom {name: v.name, neg: !v.neg} );
          new_tail
        })
      })
     .collect()
}


fn get_goals(ast: &parser::Node) -> Vec<Vec<GoalAtom>> {
  match ast {
    parser::Node::And(lh, rh) => {
      let mut lh_goals = get_goals(lh);
      let mut rh_goals = get_goals(rh);

      return con_and(&mut lh_goals, &mut rh_goals); 
    },

    parser::Node::Or(lh, rh) => {
      let mut lh_goals = get_goals(lh);
      let mut rh_goals = get_goals(rh);

      lh_goals.append(&mut rh_goals);
      return lh_goals;
    },

    parser::Node::Var(x) => {
      return  vec![vec![
                GoalAtom {name: x.clone(), neg: false} 
              ]];
    },

    parser::Node::Neg(x) => {
      return  con_neg(&get_goals(x));
    },
  }

}


fn main() -> Result<()>{
  let mut rl = DefaultEditor::new()?;

  let mut lex = lexer::Lexer::new("");
  let mut p = parser::Parser::new();

  loop {
    let readline = rl.readline("- ");

    match readline {
      Ok(l) => {
        if l == "exit" {
          println!("Exiting..."); 
          break;
        } else{
          lex.consume(&l);

          match p.parse(&mut lex) {
            Err(ref x) => {
              println!("{:?}", x);
              p.buffer.clear();
              lex.buffer.clear();
            },
            _ => {
              for ast in p.buffer.drain(..) {
                let goals = get_goals(&ast);
                println!("{:?}", goals);
              }

              println!("Buffer: {:?}", p.buffer);
            },
          };

        }
      },
      Err(ReadlineError::Interrupted) => {
        println!("Exiting...");
        break;
      },
      _ => {
        panic!("Problem reading line");
      },
    }
  }
  Ok(())
}
