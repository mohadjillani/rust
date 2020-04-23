
///////////////////////////////////////////////////////////

//01-enum
//02-enum example 1
//03-enum example 2(same as 1)
///////////////////////////////////////////////////////////

// #[derive(Debug)]
// enum Students{
//     Online,
//     Onsite,
// }
// fn main() {
//     let student = Students::Online;
//     println!("{:?}",student);

//     let student2 = Students::Onsite;
//     println!("{:?}",student2);
// }

// //#![allow(unused_variables)]
// fn main() {
// enum IpAddrKind {
//     V4,
//     V6,
// }

// struct IpAddr {
//     kind: String,
//     address: String,
// }
// let loopback = IpAddr {
//     kind: String::from("127.0.0.1")
//     address: String::from("::1"),
// };
// println!("{}",loopback);
// }

///////////////////////////////////////////////////////////

//5-A concise way to use enum

///////////////////////////////////////////////////////////
// fn main(){
//     let ip=IPkind::V4(String::from("v4 ip address"));
//     let ip_2=IPkind::V6(192,168,0,1);
//     println!("{:?}",ip);
//     println!("{:?}",ip_2);
// }
// #[derive(Debug)]
// enum IPkind{
//     V4(String),
//     V6(u32,u32,u32,u32),
// }

///////////////////////////////////////////////////////////

//6-Create enum for message

///////////////////////////////////////////////////////////

// fn main(){

//    let mesg1=Message::Quit;
//    let mesg2=Message::Write(String::from("message writed"));
//    let mesg3=Message::Move{x:10,y:20};
//    let mesg4=Message::ChangeColor(22,11,33);
//    println!("{:?}\n{:?}\n{:?}\n{:?}",mesg1,mesg2,mesg3,mesg4);
// }
// #[derive(Debug)]
// enum Message{
//     Quit,
//     Write(String),
//     Move{x:u32,y:u32},
//     ChangeColor(u32,u32,u32),
// }

///////////////////////////////////////////////////////////

//7-Defining methods for enum

///////////////////////////////////////////////////////////

// fn main(){

//    let mesg1=Message::Quit;
//    let mesg2=Message::Write(String::from("message writed"));
//    let mesg3=Message::Move{x:10,y:20};
//    let mesg4=Message::ChangeColor(22,11,33);
//    println!("{:?}\n{:?}\n{:?}\n{:?}",mesg1,mesg2,mesg3,mesg4);
//    mesg2.call();
//    mesg4.call();
// }
// #[derive(Debug)]
// enum Message{
//     Quit,
//     Write(String),
//     Move{x:u32,y:u32},
//     ChangeColor(u32,u32,u32),
// }
// impl Message{
//     fn call(&self){
//         println!("\n{:?}",&self);
//     }
// }

///////////////////////////////////////////////////////////

//8-Defining and calling a function

///////////////////////////////////////////////////////////

// fn main(){
//     let four=IPkind::V4;
//     let six =IPkind::V6;

//     route(six);
//     route(four);
// }
// #[derive(Debug)]
// enum IPkind{
//     V4,
//     V6
// }
// fn route(x:IPkind){
//  println!("{:?}",x);
// }


///////////////////////////////////////////////////////////

//9-The "Option" enum

///////////////////////////////////////////////////////////

// fn main(){

//     let number=Option::Some(4);
//     let string=Option::Some(String::from("I am a String"));
    
//     println!("{:?}\n{:?}",number,string);
// }
// #[derive(Debug)]
// enum Option <T> {
//     Some(T),
//     None
// }

///////////////////////////////////////////////////////////

//10-Create instancefrom predefine Option enum

///////////////////////////////////////////////////////////

// fn main(){
//     #[derive(Debug)]
//     let some_number=Some(21);
//     let some_string=Some(String::from("Mohad Jillani"));
//     let empty:Option<u32>=None;
//     println!("{:?}",some_number);
//     println!("\n{:?}\n{:?}\n{:?}",some_number,some_string,empty);
// }

///////////////////////////////////////////////////////////

//11-Does rust have nulls?

//we can't perform operations on differnt variable types

///////////////////////////////////////////////////////////

///////////////////////////////////////////////////////////

//12-The match control flow operator

///////////////////////////////////////////////////////////

// fn main(){
//     let x=3;
//     match x {
//         1=>println!("one"),
//         2=>println!("two"),
//         3=>println!("three"),
//         _=>println!("none"),
//     }
// }

///////////////////////////////////////////////////////////

//13-Coin sorting machine

//14-Coin sorting machine code

///////////////////////////////////////////////////////////

// fn main(){
//     let mycoin=Coin::Five;
//     let x=chak_value(mycoin);
//     println!("{}",x);
//     fn chak_value(coin:Coin)->u8{
//         match coin{
//             Coin::One=>1,
//             Coin::Two=>2,
//             Coin::Five=>5,
//             Coin::Ten=>10,
//         }
//     }
// }
// enum Coin{
//     One,
//     Two,
//     Five,
//     Ten,
// }

///////////////////////////////////////////////////////////

//15-Curly braces in match arm

//use to execute block of code on match

///////////////////////////////////////////////////////////

///////////////////////////////////////////////////////////

//16-Patterns that bind the values

///////////////////////////////////////////////////////////

// fn main(){
//     let mycoin=Coin::Five(CoinType::Brown);
//     let x=chak_value(mycoin);
//     println!("{}",x);
//     fn chak_value(coin:Coin)->u8{
//         match coin{
//             Coin::One=>1,
//             Coin::Two=>2,
//             Coin::Five(coin_type)=>{
//                 println!("{:?}",coin_type);
//                 5
//             },
//             Coin::Ten=>10,
//         }
//     }
// }
// #[derive(Debug)]
// enum CoinType{
//     White,
//     Brown,
// }
// enum Coin{
//     One,
//     Two,
//     Five(CoinType),
//     Ten,
// }

///////////////////////////////////////////////////////////

//17-Matching with Option<T>

///////////////////////////////////////////////////////////

// fn main(){

//     let val=Some(4);
//     let now_val=plus_one(val);
//     println!("{:?}",now_val);
// }
// fn plus_one(x:Option<i32>)->Option<i32>{
//     match x{
//         None=>None,
//         Some(i)=>Some(i+1),
//     }
// }