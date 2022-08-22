use std::io::{self, Write};
use rand::Rng;

pub fn guess() {
    let mut rng = rand::thread_rng();
    let guessed_number:i32 = rng.gen_range(0..1000);
    let mut ans = String::new();
    loop{
      println!("guess a number between 0 and 1000:");
      ans.clear();
      io::stdout().flush();
      io::stdin().read_line(&mut ans).expect("failed to get result");
      let int_response: i32= ans.trim().parse().expect("please give me correct string number!");
      if  int_response == guessed_number {
          println!("Congrats, youve guessed it, the number is {}",guessed_number);
          break;
      }else if int_response>guessed_number {
          println!("Lower");
      }else{
          println!("Higher");
      }

    }
}
