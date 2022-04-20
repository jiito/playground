use std::env;

fn main() {
  let args: Vec<String> = env::args().skip(1).map(|x| x.to_string()).collect();
  let mut use_current_dir = true;
  let mut all_opt = false;
  let mut long_opt = false;

  let mut opt: i32;

  let mut files: Vec<String> = Vec::new();
  for arg in &args {
    match arg.as_str() {
      "-l" => long_opt = true,
      "-a" => all_opt = true,
      _ => files.push(arg.to_string()), 
    }
  }

  for arg in &args {
    if arg.as_str() == "-l" || arg.as_str() == "-a" {
      continue;
    }

    use std::fs;

    let f = fs::File::open(arg);

    let f = match f {
      Ok(file) => file,
      Err(_) => {
        println!("ls: {}: No such file or directory", arg);
        std::process::exit(1);
      }
    };

    let metadata = f.metadata();

    println!("{:?}", metadata);
  }

}
