use std::io;
use std::cmp::Ordering;


fn main() {
    let mut x = 16;
    let y = 10;
    let a;
    
    // -----------------Tinkering with variables--------------------
    a = x*y;
    println!("a is {a}");

    println!("x is equal to {x}");
    x = 2*x;
    println!("\nx is equal to {x}");
    // -------------------------------------------------------------
    
    // -----------------For loop examples---------------------------
    // Don't need to declare the variable I use in a for loop
    // Actually the compiler produces a warning if I declare the
    // variable i, so its scope is limited to the for loop
    for i in 1..=10 {
        println!("Iteration {i}");
    }
    
    let range = 1..5;
    // println!("range variable is bound to {range}");
    for i in range {
        println!("i is {i}");
    }
    // -------------------------------------------------------------

    // -----------------match statement examples--------------------
    let m_output: i32;
    let mut outcome;

    m_output = match y.cmp(&mut x) {
        Ordering::Greater => 1,
        Ordering::Equal => 0,
        Ordering::Less => -1,
    }; // here we need the semicolon because the match statement is part of
    // a variable assignment!

    println!("The first match statement has returned {m_output}");

    outcome = 1000;
    println!("outcome is initially bound to {outcome}");

    match x {
        1 => outcome = x*a,
        2 => outcome = x*a-1,
        16 => outcome = m_output + 100,
        _ => { //the code won't compile without the default case! Every case must be covered!
            println!("Default case encountered!");
            outcome = 0;
        }
    }

    println!("The second match statement has bound outcome to: {outcome}")

    let mut myinput = String::new();
    
    match
    // -------------------------------------------------------------
}
