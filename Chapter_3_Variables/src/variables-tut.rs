
fn printing_variables() {
    let x = 5; // made a immutable variable
    println!("The Value of x is {x}");
    // x = 6; // tried assigning a new value and it threw an error
    
    let mut y = 5; // Creating a mutable variable
    println!("The value of y is {y}");
    // let's change the value of y 
    y = 7;
    println!("The value of y is now: {y}");
}


fn shadowing_function() {
    let x = 5;
    let x = x + 1;
    println!("Currently the value of X should be 6 and the value is: {x}");
    {
        // lets change the value in scope
        let x = x * 2; 
        println!("The value of x in inner scope is {x}");
    }
    println!("The value of x outside the scope is {x}");
}

fn difference_shadow_mutability() {
    // difference between shadow and mutability
    let spaces = "   "; // this is string type
    let spaces = spaces.len(); // this is now a number

    println!("{spaces}");
 
    /*
    let mut new_spaces = "   ";
    new_spaces = new_spaces.len(); // this will throw an error as you are not allowed to mutate a variable type
    // only value
    */
}

fn main() {
    printing_variables(); // variables and mutability
    shadowing_function(); // shadowing
    difference_shadow_mutability(); // difference between shadowing and mutability 
}