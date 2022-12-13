pub use std::io ;
pub fn verifyq() -> f32 {
    let mut sentinel = true ;
    let mut temp = 0.0 ;
    while sentinel == true {
    println!("Please input the q variable(32 datatype) : "); 
    let mut q = String::new(); 
    io::stdin().read_line(&mut q)
        .expect("Failed to Read line :v "); // initialize the q variable that will hold the q value
    let q = q.trim().parse::<f32>(); // input is in the form of string. So we have to shadow the q variable and parse it into a f64 data
        match q {
            Ok(num) => {temp = num ; sentinel = false ; }
            Err(error) => {println!("There is an error in retrieving q variable. Please try again. Error is {}" , error) ;
                            sentinel = true ;}
        }
    }
        temp
    }

pub fn verifybase() -> usize {
    let mut sentinel = true ;
    let mut temp = 0 ;
    while sentinel == true {
    println!("Please input the base number(u32 data type) : "); 
    let mut base = String::new(); 
    io::stdin().read_line(&mut base)
        .expect("Failed to Read line :v "); // initialize the q variable that will hold the q value
    let n = base.trim().parse::<usize>(); // input is in the form of string. So we have to shadow the q variable and parse it into a f64 data
        match n {
            Ok(num) => {temp = num ; sentinel = false ; }
            Err(error) => {println!("There is an error in retrieving base number. Please try again. Error is {}" , error) ;
                            sentinel = true ;}
        }
    }
        temp
    }