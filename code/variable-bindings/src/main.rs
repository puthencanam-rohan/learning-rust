fn main() {

    /// patterns
    let (x, y) = (1, 2);

    println!("x={0}, y={1}", x, y);

    /// type annotations
    let mut i: i32 = 5;

    println!("i={}", i);

    // re-assignment of variables requires `mut` upon initial declaration and assignment
    // another way is to use `let` i.e. `let i = 10;`
    i = 10;

    println!("i={}", i);

    /// initialization bindings
    //* if unused, ib will produce a warning
    //* if referenced later, ib will cause a compiler-error
    let ib: i32;

    println!("ib={}", "ib");
    //println!("ib={}", ib);

    /// scope and shadowing
    //* 
    let s1: i32 = 17;

    {
        let s2: i32 = 3;
        
        println!("The value of s1={0} and the value of s2={1}", s1, s2);
    }

    //* The code below will error as s2 is dead
    //* println!("The value of s1={0} and the value of s2={1}", s1, s2);
}
