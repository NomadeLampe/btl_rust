// source http://www.ameyalokare.com/rust/2017/10/12/rust-str-vs-String.html
fn main() {
    let mon_str: &str = "Aloha str";
    println!("{}", mon_str);


    let mon_string = String::from("Aloha String !");
    println!("{}", mon_string);

    println!("{}", mon_string.to_string());
}