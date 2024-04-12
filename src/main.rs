fn main() {
    let first_name = "Tim";
    let last_name = "The";

    let mut name = format!("{} {}", first_name, last_name);
    name.push_str(" Enchanter ");
    //string concatenation is weird in Rust
    //I think I like name.push_str(); approach more than format!();

    let age = 32;

    let nothing_creative = true;
     
    println!("{name} is {age} years old. \nDeclared, assigned, and printed 3 datatypes: {nothing_creative}");
}