pub use num::* ;
pub struct QInteger {
    pub base: usize ,
    pub q : f32 ,
    pub number : f32
}

impl QInteger {
    pub fn integer_count(&mut self) {
        if self.q == 1.0 {
            self.number = self.base as f32 * 1.0 ;
        }
        else {
            self.number = (pow(self.q ,self.base)-1.0)/(self.q - 1.0) ;
        }
    }
    pub fn qfactorial(&self) -> f32 {
        let mut aiter = self.base ;
        let mut temp = 1.0 ;
        while aiter > 0 {
            temp = temp * ((pow(self.q ,aiter)-1.0)/(self.q - 1.0)) ;
            aiter = aiter - 1 ;
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
        }
        while biter > 0 {
            btemp = btemp * ((pow(self.q ,biter)-1.0)/(self.q - 1.0)) ;
            biter = biter - 1 ;
        }
        while citer > 0 {
            ctemp = ctemp * ((pow(self.q ,citer)-1.0)/(self.q - 1.0)) ;
            citer = citer - 1 ;
        }
        temp = (atemp / btemp) / ctemp ;
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
