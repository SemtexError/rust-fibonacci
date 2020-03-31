use std::num::Wrapping;

use rand::prelude::*;

fn calc (index: i32)
{
    let mut rng = rand::thread_rng();
    let random: f64 = rng.gen();

    let mut x = Wrapping(0i64);
    let mut y = Wrapping(1i64);
    let mut z = Wrapping(0i64);
    let max = 100000000 + (random * 10.0).round() as i32;

    for _ in 0..max
    {
        z = x + y;
        x = y;
        y = z;
    }

    println!("{}, {}", index, max);
}

fn main()
{
    for i in 0..1000
    {
        calc(i);
    }
}