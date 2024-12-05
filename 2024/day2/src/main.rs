use std::{error::Error, fs::File, io::{self, BufRead}};

#[derive(Debug)]
enum Order{
    Increasing,
    Decreasing
}

fn check_safety(l: Vec<i32>) -> Result<bool, Box<dyn Error>>{
    let mut is_safe: bool = true;
    let order = if l[0] < l[1] {Order::Increasing} else {Order::Decreasing};
    for (i, x) in l.iter().enumerate(){
        if i > 0{
            match order{
                Order::Increasing => {
                    if x < &l[i-1]{
                        is_safe = false;
                    }
                }
                Order::Decreasing => {
                    if x > &l[i-1]{
                        is_safe = false;
                    }
                }
            }
            if !(1..4).contains(&x.abs_diff(l[i-1])){
                is_safe=false
            } 
        }
    }
    Ok(is_safe)
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let mut count: i32 = 0;
    for line in io::BufReader::new(file).lines(){
        let line = line?;
        let l: Vec<i32> = line.split(" ").map(|x| x.parse::<i32>().unwrap()).collect();
        let mut pass: bool = false;
        for (i, _) in l.iter().enumerate(){
            let mut temp = l.clone();
            temp.remove(i as usize);
            if check_safety(temp)?{
                pass = true
            }
        }
        if pass{
            count += 1;
        }
    }
    println!("safety count: {}", &count);
    Ok(())
}
