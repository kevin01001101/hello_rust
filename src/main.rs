use std::mem;


struct Point {
  x: f64,
  y: f64
}

fn origin() -> Point {
  Point {x: 0.0, y: 0.0}
}

pub fn stack_and_heap()
{

}

fn add() -> i32
{
  let a = 2 + 3;
  return a;
}

fn main() 
{  
  let a:u8 = 123; // 8 bits
  let mut b:u8 = 43; 

  println!("Hello {}", a);

  println!("Before {}", b);
  b = 55;
  println!("After {}", b);

  let c = 1234596;

  println!("Value of {} with a size of {}", c, mem::size_of_val(&c));
  

  let z:isize = 123;
  let size_of_z = mem::size_of_val(&z);
  println!("z = {}, takes up {} bytes, {}-bit OS",
    z, size_of_z, size_of_z * 8);

  let d = 'x';
  println!("Value of {} with a size of {}", d, mem::size_of_val(&d));

  let f = 2.5;
  println!("Value of {} with a size of {}", f, mem::size_of_val(&f));

  let f:f32 = 2.5;  //f64
  println!("Value of {} with a size of {}", f, mem::size_of_val(&f));

  let g = 0 == 1;
  println!("Value of {} with a size of {}", g, mem::size_of_val(&g));

  println!("Add is {}", add());
  let p1 = origin();
  let p2 = Box::new(origin());
  //let p3 = *p2;

  println!("Value of p1: {} and value of p2: {}", mem::size_of_val(&p1), mem::size_of_val(&p2));
  println!("Value of p1: {} and value of p2: {}", p2.x, (*p2).y);









  if_statement();

  do_some_loop();

  do_for_loop();

  try_match();

}

fn try_match()
{

  let country_code = 1865;
  let country = match country_code
  {
    44 => "UK",
    46 => "Sweden",
    7 => "Russia",
    1..=999 => "Unknown",
    _ => "invalid"
  };
  println!("Country: {} has code: {}", country, country_code);

}

fn do_for_loop()
{
  for x in 5..15
  {
    println!("x is {}", x);
  }
  
  for (i,val) in (1..11 ).enumerate()
  {
    println!("element {} is {}", i,val);
  }

}


fn do_some_loop()
{
  let mut x = 1;
  while x < 1000
  {
    x *= 2;
    println!("x = {}", x);

    if x == 64 { break; }

  }

}

fn if_statement()
{
  let temp = 5;
  if temp > 30
  {
    println!("really hot outside!")
  } else if temp < 10
  {
    println!("it's really cold")
  }
}