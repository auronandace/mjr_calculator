use std::io;

fn main() {
    instructions();
    loop {
        println!("Please input your calculation (q to quit):");
        let calc = user_input();
        let str_collection: Vec<&str> = calc.as_str().split_whitespace().collect();
        let (mut ops, mut nums, mut reason): (Vec<BinaryOp>, Vec<f64>, &str) = (Vec::new(), Vec::new(), "");
        let (mut num_count, mut op_count, mut correct_order_count, mut try_again) = (0, 0, 0, false);
        for element in &str_collection {
            match *element {
                "+" | "&" => {ops.push(BinaryOp::Add); op_count += 1;}
                "-" => {ops.push(BinaryOp::Sub); op_count += 1;}
                "/" => {ops.push(BinaryOp::Div); op_count += 1;}
                "*" | "x" => {ops.push(BinaryOp::Mul); op_count += 1;}
                "%" => {ops.push(BinaryOp::Mod); op_count += 1;}
                _ => {
                    if element.parse::<f64>().is_ok() {nums.push(element.parse().unwrap()); num_count += 1;}
                    else {continue;}
                }
            }
            if num_count == op_count || num_count == op_count + 1 {correct_order_count += 1;}
        }
        if str_collection.is_empty() {
            reason = ".\nYou need to input something.";
            try_again = true;
        } else if str_collection[0] == "q" {break;} else if str_collection.len() != ops.len() + nums.len() {
            reason = " value.\nOnly numbers and operators permitted.";
            try_again = true;
        } else if ops.len() + nums.len() < 3 {
            reason = " quantity.\nInput requires at least 2 numbers and an operator.";
            try_again = true;
        } else if ops.len() != nums.len() - 1 {
            reason = " quantity.\nInput included too many operators or numbers.";
            try_again = true;
        } else if correct_order_count != num_count + op_count {
            reason = " order. Input should start and end with a number and alternate between operator and number.";
            try_again = true;
        }
        if try_again {eprintln!("Invalid user input{reason}"); println!("Please try again:"); continue;}
        let mut result = perform_op(ops[0], nums[0], nums[1]);
        for index in 0..ops.len() {
            let phrase = if index == 0 && ops.len() > 1 {"1st Result: "}
                         else if index == ops.len() - 1 || ops.len() == 1 {"End Result: "} else {"Next Result: "};
            if index != 0 {result = perform_op(ops[index], result, nums[index + 1]);};
            println!("{phrase}{result}");
        }
    }
}

#[derive(Copy, Clone)]
enum BinaryOp {Add, Sub, Mul, Div, Mod}

fn instructions() {
    println!("Please ensure all numbers and operating symbols are separated with a space.");
    println!("Fractions and parenthesis are not supported.");
    println!("Do not type = at the end. Just press enter after the last number.");
    println!("\nValid Operating Symbols Key:\n");
    println!("+ means add\n- means subtract\n/ means divide\n* means multiply\n% means remainder");
    println!("\nControl Flow Example:");
    println!("Input in this calculator: 1 + 5 * 3 / 2\nMathematical intention: ((1 + 5) * 3) / 2");
    println!("How this calculator works: 1+5=(Result:6) -> 6*3=(Result:18) -> 18/2=(Result:9) End Result:9\n");
}

fn user_input() -> String {
    let mut calc = String::new();
    io::stdin().read_line(&mut calc).expect("Failed to read line.");
    calc.trim().to_string()
}

fn perform_op(current_op: BinaryOp, first_num: f64, second_num: f64) -> f64 {
    match current_op {
        BinaryOp::Add => first_num + second_num,
        BinaryOp::Sub => first_num - second_num,
        BinaryOp::Div => first_num / second_num,
        BinaryOp::Mul => first_num * second_num,
        BinaryOp::Mod => first_num % second_num,
    }
}
