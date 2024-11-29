
/**
 *  Floating-point value in Standard IEEE-754.
 * 
 *  Allows saving numbers:
 *      normalized:
 *      (+-) 2.0^(-126) -> (+-) (1-2^-24)^(128)
 *      subnormalized:
 *      (+-) 2.0^(149) -> (+-) 2.0^(149)
 * 
 *  in the following format (numbers in bits):
 *  scf - s(1)c(7)f(23)
 *  total of 32 bits
 * 
 *  `s` - sign ('0' - positive value; '1' - negative value)
 *  `c` - format for the exponent of f ( c = Exponent - Bias-constant(=127) )
 *  `f` - f
 * 
 * 
 */
pub struct FpxIEEE {
    s: bool,
    c: [bool; 8],
    f: [bool; 23]
    
}
#[allow(dead_code)]
impl FpxIEEE {
    pub const INFINITE: FpxIEEE = FpxIEEE{ s: (false), c: ([true; 8]), f: ([false; 23]) };
    pub const INFINITE_NEGATIVE: FpxIEEE = FpxIEEE{ s: (true), c: ([true; 8]), f: ([false; 23]) };
    // NaN-values are not strictly in this format. f must simply be not equal to f_00...
    pub const NAN: FpxIEEE  = FpxIEEE{ s: (true), c: ([true; 8]), f: ([true; 23]) };

    /**
     * create a new FpxIEEE directly with s,c,f separated
     * 
     * s, c, f get destroyed in this operation.
     * 
     * @return new FpxIEEE{s, c, f}
     */
    pub fn new(s: bool, c:[bool; 8], f:[bool; 23]) -> FpxIEEE {
        return FpxIEEE{ s: (s), c: (c), f: (f) }
    }
    /**
     * equal function does not return true, if both values are NaN of a different format
     * 
     * 
     * examples: (
     * fpx_..f01... != fpx_..f10...)
     */
    pub fn equals(fpx_self : &FpxIEEE, fpx_other : &FpxIEEE) -> bool {
        if fpx_self.s != fpx_other.s {
            return false
        }
        for i in 0.. 7 {
            if fpx_self.c[i] != fpx_other.c[i] {
                return false
            }
        }
        for i in 0.. 23 {
            if fpx_self.f[i] != fpx_other.f[i] {
                return false
            }
        }
        return true

    }
    pub fn is_infinite(fpx_ : &FpxIEEE) -> bool {
        if fpx_.c == FpxIEEE::INFINITE.c && fpx_.f == FpxIEEE::INFINITE.f {
            true
        }
        else {
            false
        }
    }

    pub fn is_nan(fpx_ : &FpxIEEE) -> bool {
        if fpx_.c == FpxIEEE::NAN.c && fpx_.f != [false; 23] {
            true
        }
        else {
            false
        }
    }

    pub fn float_to_fpx(fvalue : &f32) -> FpxIEEE  {
        let s: bool;

        if fvalue.is_nan() {
            return FpxIEEE::NAN
        }
        if fvalue.is_sign_positive() {
            s = false; 
        } 
        else {
            s = true;
        }

        if fvalue.is_infinite() {
            // needs to be created to keep sign
            return FpxIEEE { s: (s), c: (FpxIEEE::INFINITE.c), f: (FpxIEEE::INFINITE.f) }
        }

        // continue calculation
        let c: [bool; 8] = [false; 8];
        let f: [bool; 23] = [false; 23];

        //TODO continue

        return FpxIEEE { s: (s), c: (c), f: (f) }
    }

    /**
     * String of the given FpxIEEE value
     * 
     * @return {String} of the format:
     * "fpx_s0c00000000f00000000000000000000000"
     */
    pub fn to_string(fpx_ : FpxIEEE) -> String {
        let mut string : String = String::from("fpx_s");
        if fpx_.s {
            string.push('0');
        }
        else {
            string.push('1');
        }
        string.push('c');
        for i in 0..8 {
            if fpx_.c[i] {
                string.push('1');
            }
            else {
                string.push('0');
            }
        }
        string.push('f');
        for i in 0..23 {
            if fpx_.f[i] {
                string.push('1');
            }
            else {
                string.push('0');
            }
        }
        string
    }
}
