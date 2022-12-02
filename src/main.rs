use std::io ;

fn quantum_integer_count(n : u16 , q : f64) -> f64 {
    (q.powi(n.try_into().unwrap()) - 1.0)/(q - 1.0) // return this mathematics expression
}
fn main() {

    println!("Welcome to our Program\n\n\n");

    // step 1 : initialize the q variable 

    println!("Please input the q number (q number is a 64 bit floating number) : "); 
    let mut q = String::new(); 
    io::stdin().read_line(&mut q)
        .expect("Failed to Read line :v "); // initialize the q variable that will hold the q value
    let q: f64 = q.trim().parse()
        .expect("Please type a number >_<") ; // input is in the form of string. So we have to shadow the q variable and make it into a f64 data
    println!("The q number is {}" , q);
    //step 1 is finished 
    // step 2 : Initialize the n variable
    println!("Please input the n number (n number is a 64 bit unisgned integer number) : "); 
    let mut n = String::new(); 
    io::stdin().read_line(&mut n)
        .expect("Failed to Read The Input (T_T) "); // initialize the n variable that will hold the n value
    let mut quantum_number: f64 = n.trim().parse() // initialize quantum number with the same value as n but in the form of floating number 
        .expect("Please type a number >_<") ; // input is in the form of string and we have to parse it into f64
    let n: u16 = n.trim().parse()
        .expect("Please type a number >_<") ; // After quantum number initialized we shadow the original n and make it into u16 data
    println!("The n number is {}" , n);
    //step 2 is finished

    //step 3 count quantum variable 
    // if q is one then quantum number is the same as n
    if q != 1.0 {
        quantum_number = quantum_integer_count(n , q); // since q is not one then we must compute the q integer 
    }
    println!("The quantum integer number is {}" , quantum_number); // I want to be a cat
}

// this is all for now
//Goodbye everyone
// See you later
// contact me through email 
// My email is RXRD@tutanota.com
