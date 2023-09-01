use std::io;
enum Operation{
    Add (f64, f64),
    Substract (f64, f64),
    Multiply(f64, f64),
    Divide (f64, f64),
}

fn calculate(operation: Operation) -> f64 {
    let result;
    match operation{
        Operation::Add(num1, num2) => result = num1 + num2,
        Operation::Substract(num1, num2) => result = num1 - num2,
        Operation::Multiply(num1, num2) => result = num1 * num2,
        Operation::Divide(num1, num2) => result = num1 / num2,
    }
    return result;
}

fn main(){
    loop {
        println!("Enter the first number:");
        let mut input1 = String::new();
        io::stdin().read_line(&mut input1).expect("Failed to read input.");
        let input1 = input1.trim().parse::<f64>().expect("Invalid input.");
        //println!("{:?}", input1);


        println!("Enter the second number:");
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).expect("Failed to read input.");
        let input2 = input2.trim().parse::<f64>().expect("Invalid input.");
        //println!("{:?}", input2);

        println!("Enter the operation number:
        1) Add
        2) Subtract
        3) Multiply
        4) Divide
        5) Quit");
        let mut op_input = String::new();
        io::stdin().read_line(&mut op_input).expect("Failed to read input.");
        let op_input = op_input.trim().parse::<u8>().expect("Invalid input.");

        let operator;
        match op_input{
            1 => operator = Operation::Add(input1, input2),
            2 => operator = Operation::Substract(input1, input2),
            3 => operator = Operation::Multiply(input1, input2),
            4 => operator = Operation::Divide(input1, input2),
            5 => break,
            _ => panic!("Invalid input."),
        }
        let result = calculate(operator);
        println!("The result is: {}", result);
    }

}

