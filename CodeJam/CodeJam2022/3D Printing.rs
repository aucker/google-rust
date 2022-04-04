// The problem link:
// https://codingcompetitions.withgoogle.com/codejam/round/0000000000876ff1/0000000000a4672b
use std::io;
fn main() {
   
    let mut num = String::new();
    io::stdin().read_line(&mut num).unwrap();
    let mut s = num.split_whitespace();
    let n: i32 = s.next().unwrap().parse().unwrap();
    let mut i = 1;
    while i <= n {
        let mut input1 = String::new();
        io::stdin().read_line(&mut input1).unwrap();
        let mut s1 = input1.split_whitespace();
        let a1: i32 = s1.next().unwrap().parse().unwrap();
        let a2: i32 = s1.next().unwrap().parse().unwrap();
        let a3: i32 = s1.next().unwrap().parse().unwrap();
        let a4: i32 = s1.next().unwrap().parse().unwrap();
        let mut a: Vec<i32> = Vec::with_capacity(4);
        a.push(a1);
        a.push(a2);
        a.push(a3);
        a.push(a4);

        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).unwrap();
        let mut s2 = input2.split_whitespace();
        let b1: i32 = s2.next().unwrap().parse().unwrap();
        let b2: i32 = s2.next().unwrap().parse().unwrap();
        let b3: i32 = s2.next().unwrap().parse().unwrap();
        let b4: i32 = s2.next().unwrap().parse().unwrap();
        let mut b: Vec<i32> = Vec::with_capacity(4);
        b.push(b1);
        b.push(b2);
        b.push(b3);
        b.push(b4);

        let mut input3 = String::new();
        io::stdin().read_line(&mut input3).unwrap();
        let mut s3 = input3.split_whitespace();
        let c1: i32 = s3.next().unwrap().parse().unwrap();
        let c2: i32 = s3.next().unwrap().parse().unwrap();
        let c3: i32 = s3.next().unwrap().parse().unwrap();
        let c4: i32 = s3.next().unwrap().parse().unwrap();
        let mut c: Vec<i32> = Vec::with_capacity(4);
        c.push(c1);
        c.push(c2);
        c.push(c3);
        c.push(c4);
        
        let d1 = min(a[0], b[0], c[0]);
        let d2 = min(a[1], b[1], c[1]);
        let d3 = min(a[2], b[2], c[2]);
        let d4 = min(a[3], b[3], c[3]);
        let sum = d1 + d2 + d3 + d4; 
        if sum < 1000000 {
            println!("Case #{}: IMPOSSIBLE", i);
        } else {
            let mut remain = sum - 1000000;
            // let mut d: Vec<i32> = Vec::with_capacity(4);
            let mut r1 = d1;
            let mut r2 = d2;
            let mut r3 = d3;
            let mut r4 = d4;
            if remain == 0 {
                // println!("Case #{}: {} {} {} {}", i, r1, r2, r3, r4);
                r1 = d1;
                r2 = d2;
                r3 = d3;
                r4 = d4;

            } else if remain > d1 {
                r1 = 0;
                remain -= d1;
                if remain > d2 {
                    r2 = 0;
                    remain -= d2;
                    if remain > d3 {
                        r3 = 0;
                        remain -= d3;
                        r4 = d4;
                    } else {
                        r3 = d3 - remain;
                    }
                } else {
                    r2 = d2 - remain;
                };
            } else {
                r1 = d1 - remain;
            }
            println!("Case #{}: {} {} {} {}", i, r1, r2, r3, r4);
        }

        
        i += 1;
    }
}
fn min(a: i32, b: i32, c: i32) -> i32 {
    let mut tmp = a;
    tmp = if tmp < b {
        tmp
    } else {
        b
    };
    tmp = if tmp < c {
        tmp
    } else {
        c
    };
    tmp
}