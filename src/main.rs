
fn main() {
    println!("Hello, world!");

    another_function(243, -3);
}

fn another_function(x: i32, y: i32)
{
   println!("Another function has been called with 32-bit ints. X={} Y={}.", x, y);
}
