fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x

    *y += 1; // Modifying x through y
    println!("x = {}", x); // Output: x = 6

    // Correct way to change x if we need to use both mutable and immutable references:
    {
        let z = &x; // z is an immutable reference to x. It's within a new scope to prevent conflicts with y.
        println!("z = {}", *z); // Output: z = 6
    }

    *y += 2; //Modifying x again
    println!("x = {}", x); //Output: x = 8 
} 