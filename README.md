## Introduction
<p>This project is developed as homework of Rust Bootcamp which is organized by <a href="https://www.risein.com/">RiseIn.</a>

<p>Aim of the project is to create an enum object named as Operation and create a function and implement on Operation object. </p>

<p>In main function, one of branches of the Operation is inherited and each branches return f64. </p>

<p>Each branches are functions in this case and expects two <b>float</b> parameters. </p>

<p>Selected branch of enum (as a function) returns the result.</p>

<p>Enum can be instantiated as: </p>
<code>
fn main()
{
    let number = Operation::Add(2.0, 4.5); //Assumed that 2.0 and 4.5 are received by keyboard.

}

enum Operation{
    Add(f64, f64),
}

impl Operation{ 
    fn calculate(&self) -> f64 //returns 64 bit float
    {
        match self{
            Operation::Add(n1, n2) => {
                return n1 + n2;
            }
        }
    }
}

</code>

<p>I want to thank to<a href="https://www.risein.com/">RiseIn.</a> and <a href="https://patika.dev/">PatikaDev for exclusive bootcamp.</a></p>