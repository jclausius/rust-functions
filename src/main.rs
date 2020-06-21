
fn main() {

   // Function bodies are made up of a 
   // series of statements optionally 
   // ending in an expression.

    println!("Hello, world!");

    another_function(243, -3);

    let z = yet_another_func();
    println!("yet_another_func() returned {}", z);

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

fn yet_another_func() -> i32
{
  // Calling a function is an expression. 
  // Calling a macro is an expression. 
  // The block used to create new scopes, {}, is an expression
  //
  // Note, expressions do not include ending semicolons. 
  // If you add a semicolon to the end of an expression,
  // you turn it into a statement, which will then not return
  // a value. Keep this in mind as you explore function return 
  // values and expressions next.
  //
  // Also note, expressions do not include ending semicolons.

   let x = 22;

   let y = {
      println!("inside a scope block");
      let x = 9;
       x - 4
   };

   println!("The value of y was assigned from the last epxression in the scope of {}", y);

  // ( the return value of the function is synonymous 
  //   with the value of the final expression in the 
  //   block of the body of a function. 
  // 
  // ou can return early from a function by using 
  // the return keyword... most functions return
  //  the last expression implicitly
  //  like below
  x + y
}
