use crate::{
    decimal::is_decimal,
    div::{self, divide},
    multiply,
};

pub struct Operation {
    a: f32,
    b: f32,
}

impl Operation {
    pub fn multiplication(mut a: f32, mut b: f32) -> f32 {
        let mut result: f32 = 0.0;
        if a == 0.0 || b == 0.0 {
            result = 0.0;
        } else if b < 0.0 {
            result = Self::multiplication(a, -b);
            result = -result;
        } else if a < 0.0 {
            result = Self::multiplication(-a, b);
            result = -result;
        } else if b == 1.0 {
            result = a + Self::multiplication(a, b - 1.0);
        } else if is_decimal(a) == true && is_decimal(b) == false {
            let x = a;
            a = b;
            b = x;

            result = a + Self::multiplication(a, b - 1.0);
            return result;
        } else if is_decimal(a) == false && is_decimal(b) == true {
            result = a + Self::multiplication(a, b - 1.0);
            return result;
        } else if is_decimal(a) == false && is_decimal(b) == false {
            let d = 10.0;

            //convert b to whole number

            result = b + Self::multiplication(d - 1.0, b);

            //result =  Self::multiplication(a, result);

            // result = divide(result);
            return result;
        } else if is_decimal(a) == true && is_decimal(b) == true {
            result = a + Self::multiplication(a, b - 1.0);
            return result;
        }
        //else if b > 1.0 {
        //result = a + Self::multiplication(a, b - 1.0);
        // }
        else if a < 1.0 && a < b {
            result = b - 1.0;
        }

        result
    }
}
