// pub fn details () {
//     println!("hello from lib.rs");
// }

// pub mod front_house{
//     pub mod hosting{
//         pub fn add_to_waitlist(){

//         }
//     }
// }
// pub fn eat_at_restuarent(){
//     //absolute path
//     crate::front_house::hosting::add_to_waitlist();
//     //relative path
//     front_house::hosting::add_to_waitlist();
// }

//////////////////////////////////////////////////////////////////

//Super keyword

//////////////////////////////////////////////////////////////////

// fn serve_order(){}
// mod parent{
    

//     mod back_house{
//         fn hosting(){
//             cooking();
//             super::super::serve_order();
//         }
//         fn cooking(){

//         }
//     }

// }

//////////////////////////////////////////////////////////////////

// 19- Making struct and enum public

//////////////////////////////////////////////////////////////////

// mod front_house{
//     #[derive(Debug)]
//     pub struct Breakfast{
//         pub toast:String,
//         fruit:String,
//     }

//     impl Breakfast{
//        pub fn new(toast:String)->Breakfast{
//             Breakfast{
//                 toast,
//                 fruit:String::from("Apple")
//             }
//         }
//     }


// }
// pub fn take_breakfast(){

//     let mut breakfast=front_house::Breakfast::new(String::from("Dawn"));
//     breakfast.toast=String::from("Sun bread");
//     println!("{:#?}",breakfast);

// }

//////////////////////////////////////////////////////////////////

// 20- Making enum public

//////////////////////////////////////////////////////////////////

// #![allow(dead_code)]
// #![allow(unused_variables)]
// mod front_house{

// pub enum Appitizer{
//     Soup,
//     Salad
// }

// }

// fn take_breakfast(){
//     let meal1=front_house::Appitizer::Soup;
//     let meal2=front_house::Appitizer::Salad;
// }


//////////////////////////////////////////////////////////////////

// 21- Bringing path into scope using "use" keyword

//////////////////////////////////////////////////////////////////


// #![allow(dead_code)]
// #![allow(unused_variables)]
// mod front_house{

// pub enum Appitizer{
//     Soup,
//     Salad
// }

// }
// use front_house::Appitizer;
// fn take_breakfast(){
//     let meal1=Appitizer::Soup;
//     let meal2=Appitizer::Salad;
// }


//////////////////////////////////////////////////////////////////

// 22- Bring HashMap into scope

//////////////////////////////////////////////////////////////////

// use std::collections::HashMap;
// fn main(){
//     let mut contects=HashMap::new();
//     contects.insert("1","mohad.jillani@gmail.com");
    
// }

//////////////////////////////////////////////////////////////////

// 23-Bringing two types

//24-Providing new name using "as" keyword to remove ambigity

//////////////////////////////////////////////////////////////////
 
// use std::fmt::Result;
// use std::io::Result ;
// fn main(){
//     fn function1()->fmt::Result{}
//     fn function2()->io::Result{}
// } //not working this example right now

////////////////////////////////////////////////////////////////// 

// 26- Using external packages
// adding package from "crates.io" in "cargo.toml" file
//for exaple "rand = "0.7.3" under dependencies
// paste this in "main.rs" for result

//////////////////////////////////////////////////////////////////

// use rand::Rng;
// fn main(){
//    let secret_number=rand::thread_rng().gen_range(1,100);
//    println!("{}",secret_number);
// }

//////////////////////////////////////////////////////////////////

//27-Using module from different file
//divided module in to two files check
//front_house.rs and front_house/hosting.rs

//////////////////////////////////////////////////////////////////

pub mod front_house;
pub fn eat_at_restuarent(){
    //absolute path
    crate::front_house::hosting::add_to_waitlist();
    //relative path
    front_house::hosting::add_to_waitlist();
}
fn mai(){
    println!("whats the name of your services")
        select:String,
        select:str,
        
    }
}