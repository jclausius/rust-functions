
fn main() {

   // Function bodies are made up of a 
   // series of statements optionally 
   // ending in an expression.

    println!("Hello, world!");

    another_function(243, -3);

    yet_another_func();

   let x = 6;

   let y = {
      let x = 3;
      x+1
   };

   println!("value of y is {}", y);
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

fn yet_another_func()
{
  // Calling a function is an expression. 
  // Calling a macro is an expression. 
  // The block used to create new scopes, {}, is an expression

   let x = 22;

   let y = {
      println!("inside a scope block");
      let x = 9;
       x - 4
   };

   println!("The value of y was assigned from the last epxression in the scope of {}", y);

}
