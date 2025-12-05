use std::io;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let stdin = io::stdin();
    let mut buffer = String::new();
    stdin.read_line(&mut buffer).expect("failed to read line");
    let ranges: Vec<String> = buffer.trim().split(',').map(|s| s.to_string()).collect();
    let mut sum: u64 = 0;
    for range in ranges {
        let bounds : (u64, u64) = get_bounds(range);
        println!("bounds are.. {}-{}", bounds.0, bounds.1);
        if bounds.0 == 0 && bounds.1 == 0 {
            continue;
        }
        sum += calculate_palindromes(bounds);
    }
    let elapsed_time = now.elapsed();
    println!("the password should be... {}, took {}", sum, elapsed_time.as_secs());
}


fn get_bounds(boundary: String) -> (u64, u64) {
    let boundsstring: Vec<String> = boundary.split('-').map(|s| s.to_string()).collect();
    if boundsstring[0].chars().nth(0) == Some('0') || boundsstring[1].chars().nth(0) == Some('0') {
        (0,0)
    }
    else {
        return (boundsstring[0].parse().expect("failed to parse first num"),boundsstring[1].parse().expect("failed to parse second num"));
    }
}

fn calculate_palindromes(bounds: (u64, u64)) -> u64 {
    let mut sum: u64 = 0;
    for i in bounds.0..(bounds.1+1) {
        let strnum : String = i.to_string();
        if pattern(strnum) {
            println!("found palindrome {}", i);
            sum+=i;
        }
    }
    sum
}

fn pattern(strnum: String) -> bool {
    let mut result: bool = false;
    let mut k: usize;
    let strnumvec: Vec<char> = strnum.chars().collect();
    let mut pattern: String = String::new();
    let mut loopedpattern: String = String::new();
    for i in 0..strnum.len()/2 {
        pattern.push(strnumvec[i]);

        k=1*(i+1);
        while k < strnum.len() {
            let mut bounds: usize = pattern.len()+k;
            if bounds > strnum.len() {bounds = strnum.len();}
            for j in k..bounds {
                loopedpattern.push(strnumvec[j]);

            }
            if pattern == loopedpattern {
                result = true;
            }
            else {
                result = false;
                break;
            }

            k+=1*(i+1);
            loopedpattern.clear();
        }
        loopedpattern.clear();
        if result {
            break;
        }    
    }
    return result;
}

