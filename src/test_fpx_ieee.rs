


#[cfg(test)]
mod tests {
    use crate::fpx_ieee::FpxIEEE;

    use super::*;

    /**
     * Test to identify issues with the equals(...) function
     */
    #[test]
    fn equals_method() {
        assert!(FpxIEEE::equals(&FpxIEEE::new( false, [true; 8], [false; 23] ), &FpxIEEE::INFINITE));
        assert!(!FpxIEEE::equals(&FpxIEEE::new( false, [true; 8], [true; 23] ), &FpxIEEE::INFINITE));
    }

    /**
     *  Test to identify issues caused by borrowing, copying, referencing values within functions
     *  
     *  tested functions:
     */
    #[test]
    fn persistence() {
        let f: [bool; 23] = [false; 23];
        let fpx_: FpxIEEE = FpxIEEE::new(false, [true; 8], f);
        assert!(FpxIEEE::equals(&fpx_, &FpxIEEE::INFINITE));
        // do calculations

        assert!(FpxIEEE::equals(&fpx_, &FpxIEEE::INFINITE));

    }

}