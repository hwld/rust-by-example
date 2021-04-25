fn counter(start_with: i32) -> impl FnMut() {
  let mut count_val = start_with - 1;
  
  move || {
      count_val += 1;
      println!("{}", count_val);
  }
}

fn main() {
  let mut count = counter(1);

  for _ in 0..10 {
    count();
  }
}