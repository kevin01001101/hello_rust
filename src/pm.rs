pub mod pattern_matching {

  fn how_many(x:i32) -> &'static str
  {
    match x
    {
      0 => "no",
      1 | 2 => "one or two",
      9..=11 => "lots of",
      _ => "a few"

    }
  }
  pub fn match_this()
  {

    for x in 0..13 
    {
      println!("{}: I have {} oranges", x, how_many(x));
    }


    let point = (5,3);

    match point
    {
      (0,0) => println!("origin"),
      (0,y) => println!("x axis, y = {}", y),
      (x,0) => println!("y axis, x = {}", x),
      (_,y) => println!("(?,{})", y)
    }
  }

} 
