/**========================================================================
 * ?                                ABOUT
 * @author         :  Md. Minhaz
 * @email          :  mdm047767@gmail.com
 * @repo           :  https://github.com/mdminhaz2003/RUST-Programming/
 * @createdOn      :  10 December, 2021 at 11:00:00 AM
 * @description    :  This project can receive an user input from console and compare with a random number and print a result it such as Guessing Game.
 *========================================================================**/

//* Lib Section 
use rand::Rng;
use std::io;
use std::cmp::Ordering;

//* Main Function 
fn main() {
    //* create a string type variable 
    let mut guess = String::new();
    
    //* genaret a random number from external lib such as rand::Rng 
    let random_number = rand::thread_rng().gen_range(1..101);

    //* Print to console Guessing number 
    println!("Guess The Number ! : ");
    println!("Please Input Your Guess : ");

    //* Input number read and convert to string type variable 
    io::stdin().read_line(&mut guess).expect("Failed to read line");

    //* print to console Guessing number and genarated random number with string type variable
    println!("You Enterd {} and secret number is {}", guess, random_number);

    //* converting variable type string type to integer type
    //* u32 indecate that this variable can store unsigned integer type and 32 bit integer number
    //* trim() function remove all weate space from string variable 
    //* parse() function can do string type variable to integer type variable
    //* expect() function can handle error to convert string type variable to integer type
    let guess: u32 = guess.trim().parse().expect("Invalid guess !! Please try again !");

    //* Compareing value which is input a user and genarated by random 
    match guess.cmp(&random_number) {
        Ordering::Equal => println!("Congrats ! Number Matched."),
        Ordering::Greater => println!("Your number is too big."),
        Ordering::Less => println!("Your number is too smalll.")
    }


    //* for run this program go to console and write this command for windows or mac.
    //?  cargo run
}
