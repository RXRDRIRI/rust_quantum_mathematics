pub use std::io ;
pub fn verifyq() -> f32 {
    let mut sentinel = true ; // this is to control the program 
    let mut temp = 0.0 ; // temporary f32 value
    while sentinel == true { // looping program until user input correctly
    println!("Please input the q variable(32 datatype) : "); 
    let mut q = String::new(); 
    io::stdin().read_line(&mut q)
        .expect("Failed to Read line :v "); // read the q variable from user input
    let q = q.trim().parse::<f32>(); // shadow and parse input string into a f32
        match q {
            Ok(num) => {temp = num ; sentinel = false ; } // sentinel become false and ends the loop if input is correct
            // then the parsed number is stored in temp
            Err(error) => {println!("There is an error in retrieving q variable. Please try again. Error is {}" , error) ;
                            sentinel = true ;} // if error then program keeps looping 
        }
    }
        temp // return the temp value 
    }

pub fn verifybase() -> usize {
    let mut sentinel = true ; // this is for better program control
    let mut temp = 0 ; // temporary usize value
    while sentinel == true { // as long as sentinel is true, user have to keep input until they input the correct input
    println!("Please input the base number(u32 data type) : "); 
    let mut base = String::new(); 
    io::stdin().read_line(&mut base)
        .expect("Failed to Read line :v "); // read base number from input
    let n = base.trim().parse::<usize>(); // shadow the input string and convert it into usize data type
        match n {
            Ok(num) => {temp = num ; sentinel = false ; } // when user input correct input type , sentinel is false thus ending the while loop
            // parsed number also is then stored into temp 
            Err(error) => {println!("There is an error in retrieving base number. Please try again. Error is {}" , error) ; 
                            sentinel = true ;} // if incorrect then sentinel is true and program keeps looping to ask for correct input
        }
    }
        temp // return the temporary value
    }