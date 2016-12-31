fn main() {

    print_number(42);

    print_sum(21, 21);

    println!("function returned {}", add_one(34));

    // ++ example for assignment vs statement
    let mut y = 5;

    // x is assigned `()` instead of 6 as an assignment can have only one owner.
    let x = (y = 6);

    println!("x={0:?} and y={1}", x, y);
    // -- example for assignment vs statement

    // Use of diverging function; uncomment to use
    //let x: String = diverges();

    // ++ Use of function pointers

    // without type inference
    let f1: fn(i32) -> i32 = plus_one;

    // with type inference
    let f2 = plus_one;

    println!("function returned {}", f1(34));
    println!("function returned {}", f2(34));
    // -- Use of function pointers
}

// types are required for arguments
fn print_number(x: i32) {
    println!("x is: {}", x);
}

fn print_sum(x: i32, y: i32) {
    println!("sum is: {}", x + y);
}

// types for return values are also required.
// NB: the lack of `;`.
//
fn add_one(x: i32) -> i32 {
    x + 1
}

// this is syntactically correct but considered poor style!
fn add_one_2(x: i32) -> i32 {
    return x + 1;
}

// Diverging functions
fn diverges() -> ! {
    panic!("This function never returns!");
    //println!("hello");
}

// Function pointers
fn plus_one(i: i32) -> i32 {
        i + 1
}

