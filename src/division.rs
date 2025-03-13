use crate::division;

pub struct Ooperation {
    a: f32,
    b: f32,
}

impl Ooperation {
    //fn division(&self){

    pub fn division(a: f32, b: f32) -> f32 {
        let mut result: f32 = 0.0;

        if a == 0.0 || b == 0.0 {
            result = 0.0;
        } else if a > b {
            result = 1.0 + Self::division(a - b, b);
            return result;
        } else if a < 0.0 && b > a {
            result = 1.0 + Self::division(-a - b, b);
            result = -result;
        } else if a == b {
            result = 1.0;
            return result;
        }
        result
    }
}
