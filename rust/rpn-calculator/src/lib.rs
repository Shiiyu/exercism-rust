pub enum CalculatorInput {
  Add,
  Subtract,
  Multiply,
  Divide,
  Value(i32)
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
  if inputs.is_empty() {
    return None;
  }

  let mut stack = Vec::new();

  for input in inputs {
    if let CalculatorInput::Value(val) = input {
        stack.push(*val)
    } else {
        if stack.len() < 2 {
          return None;
        }

        let b = stack.pop().unwrap();
        let a = stack.pop().unwrap();

        match input {
          CalculatorInput::Add => stack.push(a + b),
          CalculatorInput::Subtract => stack.push(a - b),
          CalculatorInput::Multiply => stack.push(a * b),
          CalculatorInput::Divide => stack.push(a / b),
          _ => return None
        }
      }
  }

  if stack.len() != 1 {
    return None;
  }

  stack.pop()
}
