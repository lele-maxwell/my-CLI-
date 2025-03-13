
use decimal::is_decimal;
use multiply::Operation;
use division::Ooperation;
mod multiply;
mod division;
mod div;
mod decimal;
fn main() {
  //  println!("Result: {}", is_decimal(1.0));
        
    println!("Result multiplication:{}", Operation::multiplication(2.956, 1.685));
    println!("Result division :{}", Ooperation::division(40.5, 2.0));


}


//
//fn main() {
//
//
//    println!("{}", multiply(-0.2, 2.2));
//
//}

//fn multiply(a: f32, b: f32) -> f32 {
//    let mut result:f32 = 0.0;
//  if a == 0.0 || b == 0.0 {
//      result = 0.0;
//  }
//
//   else if b < 0.0 {
//     result = multiply(a, -b);
//     result = - result
//  }
//  else if a < 0.0 {
//      result = multiply(-a, b);
//      result = - result
//   }
//
//
//  else if b > 1.0 {
//      result = (a + multiply(a, (b - 1.0)));
//
//  }
//  else if a < 1.0 && a < b{
//    result = b - a ;
//  }
//
//  result
//}
//

//mod division;
//
//fn main() {
//    println!("{}", division(-4.0, 02.0));
//}
//
//fn division(a: f32, b: f32) -> f32 {
//    let mut result: f32 = 0.0;
//
//    if a == 0.0 || b == 0.0 {
//        result = 0.0;
//        
//    } else if a > b {
//        result = 1.0 + division(a - b, b);
//        return result;
//
//    } else if a < 0.0 && b > a {
//        result = 1.0 + division(-a - b, b);
//        result = -result;
//
//    } else if a == b {
//        result = 1.0;
//        return result;
//    }
//    result
//}







/*
- crate.io 
-docs


*/
