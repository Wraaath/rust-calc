use std::io;
use std::io::Write;


// fuck dig Linus
fn read_integer(prompt: &str) -> i64 {
    let input = read_input(prompt);
    return input.parse::<i64>().expect("Input er ikke et tal.");
}

// Read user input function
fn read_input(prompt: &str) -> String {
    // write prompt
    print!("{}: ", prompt);
    io::stdout().flush().unwrap();

    // read user input
    let mut read = String::new();
    io::stdin().read_line(&mut read).expect("Input er ugyldigt.");
    read = read.trim().to_string(); // ???

    return read;
}


// General & match of user-input
fn main() {
    println!("GYM CALCULATOR v2 (jannick æd lort)");
    println!("calc - Basics (+, -, *, /, ^, sqrt)");
    println!("calc py - Pythagoras");
    println!("");

    
    let choice = read_input("Input");
    match choice.as_str() {
        "calc" => basics(),
        "calc py" => pythagoras(),
        _ => println!("Ugyldigt input"),
    }
    println!("");
}

// Basics
fn basics() {
    println!("+, -, *, /, ^, sqrt");
}




// Pythagoras
fn pythagoras() {
    println!("Indtast a, b & c-værdierne, men hold den ukendte værdi til 0");
    let py_a = read_integer("a");
    let py_b = read_integer("b");
    let py_c = read_integer("c");

    //Calculate py_ans
    let py_ans = ((py_a.pow(2) + py_b.pow(2) + py_c.pow(2)) as f32).sqrt();

    //Print answer depending on what value is 0
    if py_a == 0 {
        println!("");
        println!("b² + c² = a²");
        println!("{}² + {}² = {}²", py_b, py_c, py_ans);
    }
    if py_b == 0 {
        println!("");
        println!("a² + c² = b²");
        println!("{}² + {}² = {}²", py_a, py_c, py_ans);
    }
    if py_c == 0 {
        println!("");
        println!("a² + b² = c²");
        println!("{}² + {}² = {}²", py_a, py_b, py_ans);
    }

}
