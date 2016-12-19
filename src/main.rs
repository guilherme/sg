
fn main() {
  use std::collections::VecDeque;
  let mut buf = VecDeque::new();
  buf.push_back(3);
  buf.push_back(4);
  buf.push_back(5);
  loop {
    match buf.pop_front() {
      Some(x) => println!("{}", x),
      None => break,
    }
  }
}
