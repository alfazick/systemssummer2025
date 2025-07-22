
fn isAdult(age:u8) -> String {
    
    match age > 21 {
        true => "Adult".to_string(),
        false => "Kid".to_string(),
    }

}

fn main() {
    
    println!("{:?}", isAdult(15));
    println!("{:?}", isAdult(20));
}
