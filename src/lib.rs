pub use num::* ;
pub struct QInteger {
    pub base: usize ,
    pub q : f32 ,
    pub number : f32
} // q integer is made into a structure here

impl QInteger {
    pub fn integer_count(&mut self) { // implement a function to count the q integer from q variable and base number 
        if self.q == 1.0 {
            self.number = self.base as f32 * 1.0 ; // if q is one then q integer is the same as base number 
        }
        else {
            self.number = (pow(self.q ,self.base)-1.0)/(self.q - 1.0) ; // or else then q integer is (q ^n - 1) / (q-1)
        }
    }
    pub fn qfactorial(&self) -> f32 {
        let mut aiter = self.base ; // initialize an iterative variable
        let mut temp = 1.0 ; // temporary storage 
        while aiter > 0 { // program will keep looping until aiter reaches 1 
            temp = temp * ((pow(self.q ,aiter)-1.0)/(self.q - 1.0)) ; // calculating the q integer of current aiter and multiply it with current temp
            aiter = aiter - 1 ; // aiter is decremented so the loop can end
        }
        temp
    }
    pub fn qbinomial(&self , j : usize) -> f32 {
        let mut aiter = self.base ;
        let mut biter = j ;
        let mut citer = self.base - j ;
        let mut temp = 1.0 ;
        let mut atemp = 1.0 ;
        let mut btemp = 1.0 ;
        let mut ctemp = 1.0 ;
        while aiter > 0 {
            atemp = atemp * ((pow(self.q ,aiter)-1.0)/(self.q - 1.0)) ;
            aiter = aiter - 1 ;
        } //basically the factorial of self
        while biter > 0 {
            btemp = btemp * ((pow(self.q ,biter)-1.0)/(self.q - 1.0)) ;
            biter = biter - 1 ;
        } // the factorial of j 
        while citer > 0 {
            ctemp = ctemp * ((pow(self.q ,citer)-1.0)/(self.q - 1.0)) ;
            citer = citer - 1 ;
        } // the factorial of base of self subtracted by j 
        temp = (atemp / btemp) / ctemp ; // factorial of self divided by (factorial of j multiplied by factorial of (base of self - j ))
        temp 
    }
}

#[cfg(test)]
mod tests_module1 {
    use super::* ;
    #[test] 
    fn q_integer_test1() {
        let mut case = QInteger {base : 5 , q : 3.0 , number : 0.0} ;
        case.integer_count() ;
        assert_eq!(case.number , 121.0);
    }
    #[test] 
    fn q_factorial_test1() {
        let case = QInteger {base : 5 , q : 3.0 , number : 121.0} ;
        let magictest = case.qfactorial() ;
        assert_eq!(magictest , 251680.0);
    }
}
