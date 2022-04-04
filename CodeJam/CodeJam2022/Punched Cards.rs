// The problem link:
// https://codingcompetitions.withgoogle.com/codejam/round/0000000000876ff1/0000000000a4621b
use std::io;
fn main() {
   

    let mut num = String::new();
    io::stdin().read_line(&mut num).unwrap();
    let mut s = num.split_whitespace();
    let n: i32 = s.next().unwrap().parse().unwrap();
    let mut i = 1;
    while i <= n {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut s = input.split_whitespace();
        let m: i32 = s.next().unwrap().parse().unwrap();
        let n: i32 = s.next().unwrap().parse().unwrap();
        println!("Case #{}:", i);
        for a in 1..=(2*m+1) {
            for b in 1..=(2*n+1) {
                if (a == 1 && b == 1) || (a == 1 && b == 2) || (a == 2 && b == 1) {
                    print!(".");
                } else if a % 2 == 0 && b % 2 == 0 {
                    print!(".");
                } else if a % 2 == 0 && b % 2 == 1 {
                    print!("|");
                } else if a % 2 == 1 && b % 2 == 0 {
                    print!("-");
                } else {
                    print!("+");
                }
            }
            println!();
        }
        i += 1;
    }
}