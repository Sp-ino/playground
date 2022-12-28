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
    println!("Example on for loops with range");
    for i in 1..=10 {
        println!("Iteration {i}");
    }
    

    // I can also assign a range to a variable and then use it
    println!("2nd example on for loops with range");
    let range = 1..5;
    // println!("range variable is bound to {range}");
    for i in range {
        println!("i is {i}");
    }

    // for loops can be used to iterate on arrays
    println!("Example on for loops with array");
    let arr: [i32; 6] = [1, 2, 3, 6, 1, 0];
    let mut count: i32 = 0;
    for val in arr {
        count = count + 1;
        println!("The {}-th element of arr is {}", count, val);
    }

    // But not on tuples
    // println!("Example on for loops with tuple");
    // let tup: (char, i8, i32, f32) = ('h', 1, 2, 9.45);
    // for val in tup {
    //     println!("val is {val}");
    // }
    // -------------------------------------------------------------

    // -----------------match statement examples--------------------
    let m_output: i32;
    let mut outcome;

    // match construct is an expression so I can use it to assign
    // the value of a variable
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
    };

    println!("The second match statement has bound outcome to: {outcome}");

    let mut myinput = String::new();

    println!("Who are you?");
    io::stdin().read_line(&mut myinput)
                .expect("String read failed.");
    
    // the .trim method removes formatting added by read_line 
    // and converts the String (str) to string literal (which is a &str). Without
    // calling trim(), myinput would remain a string and
    // we would not be able to compare it to "valerio", "luca"
    // or "gianni" for 2 reasons:
    // 1) the formatting from read_line()
    // 2) the fact that String and &str are not the same type
    let myinput = myinput.trim();

    println!("You entered {myinput}");
    match myinput {
        "valerio" => println!("Hello Valerio!"),
        "luca" => println!("Hello Luca!"),
        "gianni" => println!("Hello Gianni!"),
        _ => println!("I don't know who you are, sorry.")
    };

    let mut myinput2 = String::new();

    println!("Please enter some string:");
    let outcome = io::stdin()
                                            .read_line(&mut myinput2);


    let myinput2 = myinput2.trim();
    match outcome {
        Err(msg) => println!("{msg}"),
        Ok(nchars) => println!("{} characters were read successfully, the string is {}", nchars, myinput2),
    };
    // -------------------------------------------------------------

    // -----------------Vectors-------------------------------------
    let mut v: Vec<i32> = vec![1, 3, 2, 12, 4];

    println!("v is {}", v[2]);
    let a = v[2];
    v[2] = v[2]+80;

    v.push(100);
    
    // let vslice = &v[2..4];
    println!("a is {a}");

    let s = String::from("ciao");
    println!("s slice is {}", &s[1..3]);
    // let sslice = &v[1..3];
    // --------------------------------------------------------------
}
