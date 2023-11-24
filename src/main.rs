fn main() {
    //elect.calculate();
    // CONSOLE INPUT-OUTPUT
    let mut line = String::new();
    let mut line2 = String::new();
    println!("Enter the first number: ");
    let b1 = std::io::stdin().read_line(&mut line).unwrap();
    let n1: f64 = line.trim().parse().unwrap();
    println!("{}", n1);
    println!("Please enter the second number: ");
    let b2 = std::io::stdin().read_line(&mut line2).unwrap();
    let n2: f64 = line2.trim().parse().unwrap();
    println!("{}", n2);
    let elect = Operation::Divide(n1, n2);
    println!("{}", elect.calculate());
}   

enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64)
}

impl Operation {
    fn calculate(&self) -> f64
    {
        match self {
            Operation::Add(n1, n2) => n1 + n2,
            Operation::Subtract(n1, n2) => n1 - n2,
            Operation::Multiply(n1, n2) => n1 * n2,
            Operation::Divide(n1, n2) => {
                if *n2 == 0.0 { // * is used for to reach actual data, because type of n2 is &f64 and i need to point 
                    panic!("Dividing to zero is not possible for now");
                }
                return n1 / n2;
            }
        }
    }
}
