use std::env;
fn main() {
  let args: Vec<String> = env::args().skip(1).map(|x| x.to_string()).collect();
  let mut use_current_dir = true;

  let mut opt: i32;

  match args.as_slice(){
    [_, "-l"] => opt = 1,
  }
}