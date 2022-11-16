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
    println!("1 - Basics (+, -, *, /, ^, sqrt)");
    println!("2 - Pythagoras");
    println!("");

    
    let choice = read_integer("Vælg (1-2)");
    match choice {
//        1 => basics(),
        2 => pythagoras(),
        _ => println!("Ugyldigt input"),
    }
    println!("");
}



// Plus, minus, gange & division
//fn basics() {
//   let operators = vec!["+", "-", "*", "/", "^", "sqrt"];
//   let basics_a = read_input("Input (+, -, *, /, ^, sqrt)");
//   let basics_a = basics_a.to_string();
//
//   // Split operator (eg. +) from basics_num (eg. 1 & 1)
//   for operator in operators {
//       if operator == "+" {
//           let basics_num: Vec<&str> = basics_a.split(operator).collect();
//           println!("{:?}", basics_num);
//            
//           let num1 = basics_num[0].parse::<i64>();
//           let num2 = basics_num[1].parse::<i64>();
//
//           let basics_ans = num1 + num2;
//           println!("{}", basics_ans);
//        }
//    }
//}



// Pythagoras
fn pythagoras() {
    let py_a = read_integer("a");
    let py_b = read_integer("b");
    let py_c = read_integer("c");

    //Calculate py_ans
    let py_ans = (py_a + py_b + py_c) * (py_a + py_b + py_c);

    //Print answer depending on what value is 0
    if py_a == 0 {
        println!("");
        println!("a² = {}", py_ans);
    }
    if py_b == 0 {
        println!("");
        println!("b² = {}", py_ans);
    }
    if py_c == 0 {
        println!("");
        println!("c² = {}", py_ans);
    }
}



