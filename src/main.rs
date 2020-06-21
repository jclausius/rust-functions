
fn main() {

   // Function bodies are made up of a 
   // series of statements optionally 
   // ending in an expression.

    println!("Hello, world!");

    another_function(243, -3);
}

fn another_function(x: i32, y: i32) -> i32
{
   // a statement (returns no value)
   println!("Another function has been called with 32-bit ints. X={} Y={}.", x, y);

  // since statements do not return values, the following 
  // result in a compiliation error.
  // let x = (let y = 55);

  // function ends in an expression
  return x * y;
}
