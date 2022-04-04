// The problem link:
// https://codingcompetitions.withgoogle.com/codejam/round/0000000000876ff1/0000000000a46471
use std::io;
fn main() {

    let mut nums = String::new();
    io::stdin().read_line(&mut nums).unwrap();
    let mut num_s = nums.split_whitespace();
    let n: i32 = num_s.next().unwrap().parse().unwrap();
    let mut i = 1;
    while i <= n {
        let mut size =String::new();
        io::stdin().read_line(&mut size).unwrap();
        let mut size_s = size.split_whitespace();
        let size_: usize = size_s.next().unwrap().parse().unwrap();
        let mut vec: Vec<i32> = Vec::with_capacity(size_);
        
        // input the sides of the dices
        let mut sides = String::new();
        io::stdin().read_line(&mut sides).unwrap();
        let mut sides_s = sides.split_whitespace();
        let mut k = 1;
        while k <= size_ {
            let mut tmp = sides_s.next().unwrap().parse().unwrap();
            vec.push(tmp);
            k += 1;
        }
        vec.sort();
        let mut legal = 0;
        let mut k = 1;
        for j in 0..size_ {
            if k <= vec[j] {
                k += 1;
                legal += 1;
            }
        }
        println!("Case #{}: {}", i, legal);
        i += 1;
    }
}
