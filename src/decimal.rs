//use crate::decimal;

pub fn is_decimal(num:f32) -> bool{
    let num_split=format!("{:.1}",num);
    let num_split = num_split.to_string();
    let num_splited:Vec<&str>= num_split.split(".").collect();
    
    
    if num_splited[1] == "0" {
        return true
    } 
    else if num_splited[1] == "9" {
        return true;
        
    }
    else {
        return false 
    }
}