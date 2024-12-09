use std::{error::Error, fs::read_to_string};

fn main() -> Result<(), Box<dyn Error>> {
    let s = read_to_string("input.txt")?;
    let mut iter = s.char_indices().peekable();
    let mut res = 0;
    while let Some((i, c)) = iter.next() {
        if let (Some((_, c1)), Some((_, c2)), Some((_, c3))) = (
            iter.peek().copied(),
            iter.clone().nth(1),
            iter.clone().nth(2),
        ) {
            if c == 'm' && c1 == 'u' && c2 == 'l' && c3 == '(' {
                for j in 4..=8 {
                    if let Some(a) = iter.clone().nth(j) {
                        if a.1 == ')' {
                            let tmp = String::from(&s[i..i+j+1]).replace("mul(", "");
                            let l = tmp.split(",").map(|num| num.parse().unwrap()).collect::<Vec<i32>>();
                            println!("multiplying {} X {}", &l[0], &l[1]);
                            res += l[0] * l[1];
                            break;
                        }
                    }
                }
            }
        }
    }
    println!("res: {}", &res);
    Ok(())
}
