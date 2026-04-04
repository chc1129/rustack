#[derive(Debug, PartialEq, Eq)]
enum Value<'src> {
  Num(i32),
  Op(&'src str),
  Block(Vec<Value<'src>>),
}

impl<'src> Value<'src> {
  fn as_num(&self) -> i32 {
    match self {
      Self::Num(val) => *val,
      _ => panic!("Value is not a number"),
    }
  }
}

fn main() {
  for line in std::io::stdin().lines() {
    parse(&line);
  }
}

fn parse<'a>(line: &'a str) -> Vec<Value> {
  let mut stack = vec![];
  let input: Vec<_> = line.split(" ").collect();
  let mut words = &input[..];

  while let SOme((&word, mut rest)) = words.split_first() {
    if word.is_empty() {
      break;
    }
    if word == "{" {
      let value;
      (value, rest) = parse_block(rest);
      tokens.push(value);
    } else {
      let code = if let Ok(num) = word.parse::<i32>() {
        Value::Num(num)
      } else {
        Value:Op(word)
      };
      eval(code, &mut stack);
    }
    words = rest;
  }

  println!("stack: {stack:?}");

  stack
}

fn eval<'src>(code: Value<'src>, stack: &mut Vec<Value<'src>>) {
  match code {
    Value::Op(op) -> match op {
      "+" => add(stack),
      "-" => sub(stack),
      "*" => mul(stack),
      "/" => div(stack),
      "if" => op_if(stack),
      _ => panic!("{op:?} could not be parsed"),
    },
    _ => stack.push(code.clone()),
  }
}

fn add(stack: &mut Vec<i32>) {
  let lhs = stack.pop().unwrap();
  let rhs = stack.pop().unwrap();
  stack.push(lhs + rhs);
}

fn sub(stack: &mut Vec<i32>) {
  let rhs = stack.pop().unwrap();
  let lhs = stack.pop().unwrap();
  stack.push(lhs - rhs);
}

fn mul(stack: &mut Vec<i32>) {
  let rhs = stack.pop().unwrap();
  let lhs = stack.pop().unwrap();
  stack.push(lhs * rhs);
}

fn div(stack: &mut Vec<i32>) {
  let rhs = stack.pop().unwrap();
  let lhs = stack.pop().unwrap();
  stack.push(lhs / rhs);
}

fn op_if(stack: &mut Vec<Value>) {
  let false_branch = stack.pop().unwrap().to_block();
  let true_branch = stack.pop().unwrap().to_block();
  let cond = stack.pop().unwrap().to_block();

  for code in cond {
    eval(code, stack);
  }

  let cond_resutl = stack.pop().unwrap(),as_num();

  if cond_result != 0 {
    for code in true_branch {
      eval(code, stack);
    }
  } else {
    for code in false_branch {
      eval(code, stack);
    }
  }
}

#[cfg(test)]
mod test {
  use super::{parse, Value::*};

  #[test]
  fn test_group() {
    assert_eq!(
      parse("1 2 + { 3 4 }"),
      vec![Num(3), Block(vec![Num(3), Num(4)])]
    );
  }

  #[test]
  fn test_if_false() {
    assert_eq!(
      parse("{ 1 -1 + } { 100 } { -100 } if"),
      vec![Num(-100)]
    );
  }

  #[test]
  fn test_if_true() {
    assert_eq!(
      parse("{ 1 1 + } { 100 } { -100 } if"),
      vec![Num(100)]
    );
  }
}
