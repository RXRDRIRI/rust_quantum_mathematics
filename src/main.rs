use rust_quantum_mathematics::QInteger ; //q integer is a strcuture here that consist of unsigned integer base , q and the q integer which is a floating point number
mod verify ; // this is a module to read input and make sure that the input is correct
fn main() {
    let mut casa = QInteger{base : 0 , q : 0.0 , number : 0.0} ; // initialize the initial structure
    println!("Welcome to our Program\n\n\n");
    // step 1 : initialize the q variable 
    casa.q = verify::verifyq() ; //after that put the q value to the structure 
    println!("The q variable is {}" , casa.q);
    //step 1 is finished 
    // step 2 : initialize the base number 
    casa.base = verify::verifybase() ; //after that put the base value to the structure 
    println!("The base number is {}" , casa.base);
    // step 3 : calculate the q number 
    casa.integer_count();
    println!("The q integer is {}" , casa.number);

}

// this is all for now
//Goodbye everyone
// See you later
// contact me through email 
// My email is RXRD@tutanota.com
