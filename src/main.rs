use std::io;
use std::io::Write;
use std::cmp::Ordering;


// Integer-input
fn read_float(prompt: &str) -> f32 {
    let input = read_string(prompt);
    return input.parse::<f32>().expect("Input er ikke et tal.");
}

// String-input
fn read_string(prompt: &str) -> String {
    // write prompt
    print!("{}: ", prompt);
    io::stdout().flush().unwrap();

    // Læs brugerinput
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
    println!("calc al - Andengradsligning");
    println!("");

    
    let choice = read_string("Input");
    match choice.as_str() {

        "calc py" => pythagoras(),
        "calc al" => andengradsligning(),
        _ => println!("Ugyldigt input"),
    }
    println!("");
}

// Basics
//fn basics() {
//    println!("+, -, *, /, ^, sqrt");
//    let basics_a = read_float("Input");

    //Calculate basics_a
    
//}




// Pythagoras
fn pythagoras() {
    println!("Indtast a, b & c-værdierne, men hold den ukendte værdi til 0");
    let py_a = read_float("a");
    let py_b = read_float("b");
    let py_c = read_float("c");

    //Udregn py_ans
    let py_ans = (py_a.powf(2.0) + py_b.powf(2.0) + py_c.powf(2.0)).sqrt();

    //Print svar afhængigt af 0
    if py_a == 0.0 {
        println!("");
        println!("b² + c² = a²");
        println!("{}² + {}² = {}²", py_b, py_c, py_ans);
    }
    if py_b == 0.0 {
        println!("");
        println!("a² + c² = b²");
        println!("{}² + {}² = {}²", py_a, py_c, py_ans);
    }
    if py_c == 0.0 {
        println!("");
        println!("a² + b² = c²");
        println!("{}² + {}² = {}²", py_a, py_b, py_ans);
    }
}

// Andengradsligning
fn andengradsligning() {
    println!("Indtast dine a, b & c-værdier");
    let al_a = read_float("a");
    let al_b = read_float("b");
    let al_c = read_float("c");

    // Udregn andengradsligning
    let al_d = (al_b.powf(2.0) - 4.0 * al_a * al_c).sqrt();
    println!("{}", al_d);

    // Match al_d
    match al_d.partial_cmp(&0.0).unwrap() {
        Ordering::Less => println!("Ingen løsning"),
        Ordering::Equal => println!("gaide2"),
        Ordering::Greater => println!("gaide3"),
    }
}
