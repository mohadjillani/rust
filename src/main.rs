//Rust Programming part 1
//lesson 5-2
//////////////////////////////////
// #[derive(Debug)]
// struct Book{
//     name:String,
//     auther:String,
//     price:u16,
//     avalible:bool,
// }//template

// fn main(){

//     let book_1=Book{
//         name:String::from("Rust"),
//         auther:String::from("MJ"),
//         price:500,
//         avalible:true,
//     };
//     let book_2=Book{
//         name:String::from("Rust"),
//         auther:String::from("MJ"),
//         price:600,
//         avalible:true,
//     };
//     println!("{:#?}",book_1);
//     println!("{:#?}",book_2);
// }
//////////////////////////////////////////


//3-Mutable Struct
// #[derive(Debug)]
// struct Book{
//     name:String,
//     auther:String,
//     price:u16,
//     avalible:bool,
// }//template
// fn main(){

//     let mut book_1=Book{
//         name:String::from("Rust"),
//         auther:String::from("MJ"),
//         price:500,
//         avalible:true,
//     };
//     book_1.name=String::from("Rust Programming");
//     println!("{:#?}",book_1);
// }

//////////////////////////////////////////////////////////////////////////////////////////////

//4-field init shorthand

// #[derive(Debug)]
// struct Book{
//     name:String,
//     auther:String,
//     price:u16,
//     avalible:bool,
// }//template
// fn build(name:String,auther:String)->Book{
//     Book{
//         name,//shortHand
//         auther,//shortHand
//         price:500,//static value
//         avalible:true,//static value
//     }
// }

// fn main(){
//     let book_1=build(String::from("Rust"),String::from("Mohad Jillani"));
//     println!("{:#?}",book_1);
// }

///////////////////////////////////////////////////////////////////////////////////////////

//5-Struct update syntax

// #[derive(Debug)]
// struct Book{
//     name:String,
//     auther:String,
//     price:u16,
//     avalible:bool,
// }//template
// fn main(){

//     let book_1=Book{
//         name:String::from("Rust"),
//         auther:String::from("MJ"),
//         price:500,
//         avalible:true,
//     };
//     let book_2=Book{
//         name:String::from("Rust Programming"),
//         ..book_1  //never use coma(,) here

//     };
//     println!("{:#?}",book_2);
// }

///////////////////////////////////////////////////////////////////////////////

//6-Tuple Structs

// #[derive(Debug)]

// struct Colour(i32,i32,i32);

// fn print_color(x:Colour){
    
//     println!("{:#?}",x);

// }

// fn main(){
//     let red=Colour(6,3,4);

//     print_color(red);
// }

///////////////////////////////////////////////////////////////////

//7-Ownership of struct data

//expected lifetime error

//////////////////////////////////////////////////////////////////////

//8-An example how refector an example using function,tuple and struct

///////////////////////////////////////////////////////////////////////

//Example using function

// fn area(heigth:u32,width:u32)->u32{
//     heigth*width
// }

// fn main(){
//     let heigth:u32=100;
//     let width:u32=50;
//     println!("Area is {}",area(heigth,width));
    
// }

///////////////////////////////////////////////////////////////////////////////

//9-Example Using tuple

///////////////////////////////////////////////////////////////////////////////

// fn area(dimention:(u32,u32))->u32{
//     dimention.0*dimention.1
// }

// fn main(){
//   let rectangle=(100,50);
//     println!("Area of rectangle is {}",area(rectangle));
// }

//////////////////////////////////////////////////////////////////////////////////

//10-Refactoring this example using struct

//////////////////////////////////////////////////////////////////////////////////

// #[derive(Debug)]

// struct Dimention{
//     heigth:u32,
//     width:u32,
// }//template returning
// fn area(dimention:&Dimention)->u32{
//     dimention.heigth*dimention.width
// }
// fn main(){
//     let rectangle= Dimention{
//         heigth:100,
//         width:50,
//     };//now use semi-coln
//     println!("Area of rectangle:\n{:#?} is :{}",rectangle,area(&rectangle));
//     //borrowing using refferance operator
// }

//////////////////////////////////////////////////////////////////////////////////

//11-Adding usefull functionality using derived traits

//reading topics
//display trade is default trait
//we use Debug trait for print custom data type
//differance between {:?} and {:#?}
//more details in chapter 10

//////////////////////////////////////////////////////////////////////////////////

//12-Method syntax

//differance between variable and object
//variable stores the builtin data types for example integer float etc.
//objects stores the custom data type for exmple struct.
//diff b/w functions and methods

//////////////////////////////////////////////////////////////////////////////////

//13-defining method

//////////////////////////////////////////////////////////////////////////////////

// #[derive(Debug)]
// struct Dimention{
//     heigth:u32,
//     width:u32,
// }//template returning
// impl Dimention{

//     fn area(&self)->u32{
//         self.heigth*self.width
//     }

// }
// fn main(){
//     let rectangle= Dimention{
//         heigth:100,
//         width:50,
//     };//now use semi-coln
//     println!("Area of rectangle:\n{:?} is :{}",rectangle,rectangle.area());
//     //borrowing using refferance operator
// }

//////////////////////////////////////////////////////////////////////////////////

//Methods with more parameters

////////////////////////////////////////////////////////////////////////////////////

// struct Dimention{
//     heigth:u32,
//     width:u32,
// }//template returning
// impl Dimention{

//     fn store(&self,other:&Dimention)->bool{
//         self.heigth>other.heigth && self.width>other.width
//     }

// }
// fn main(){
//     let rectangle1= Dimention{heigth:100,width:50};//now use semi-coln
//     let rectangle2= Dimention{heigth:80,width:40};
//     let rectangle3= Dimention{heigth:50,width:50};
//     println!("Rectangle1 is store Rectangle2: {}",rectangle1.store(&rectangle2));

//     println!("Rectangle1 is store Rectangle2: {}",rectangle2.store(&rectangle3));
//     //borrowing using refferance operator
// }

////////////////////////////////////////////////////////////////////////////////////

//15-Associated Function

////////////////////////////////////////////////////////////////////////////////////

// #[derive(Debug)]
// struct Dimention{
//     heigth:u32,
//     width:u32
// }
// impl Dimention{
//     fn square(size:u32)->Dimention{
//         Dimention{
//             heigth:size,
//             width:size,
//         }
//     }

// }

// fn main(){
//     let square=Dimention::square(5);
    
//     println!("{:#?}",square);

// }

////////////////////////////////////////////////////////////////////////////////////

//16-Multiple impl blocks

//we can use multiple impl blocks but with the same name(not recommended)

////////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////////

//17-Question and feedback

//struct vs enum

////////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////////

//18-User input

////////////////////////////////////////////////////////////////////////////////////

// use std::io;
// fn main(){
//     let mut s=String::new();

//     io::stdin().read_line(&mut s).expect("failed to read line.");
//     println!("{}",s);
//     //converting string to integer
//     //trim use for trim extra spaces from start and end of string
//     //parse  use for convert
//     let mut s_int:u32=s.trim().parse().expect("failed to convert.");
//     s_int=s_int+1;
//     println!("{}",s_int);
// }

