use std::io;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    
    let mut lock: i32 = 50;
    let mut password: i32 = 0;
    let mut i: i32 = 0;
    println!("I wonder what the password might be");
    let lines = io::stdin().lines();
    for mut line in lines.map_while(Result::ok) {
        i+=1;
        let rotation: char =line.remove(0);
        password += lockrotation(rotation,&mut lock, line);
    }
    let elapsed_time = now.elapsed();
    println!("the password is....{}!!!", password);
    println!("read {} lines", i);
    println!("took {} nanoseconds to solve", elapsed_time.as_nanos());    
}

fn lockrotation(rotation: char,lock: &mut i32, line: String) -> i32 {
    let value: i32 = line.parse().expect("failed to parse into integer value");
    let mut result: i32 = 0;
    if rotation == 'L' {
        if *lock==0 {
            result-=1;
        }
        *lock = *lock-value;
        while *lock < 0{
            *lock = 100+*lock;
            result+=1;
        }
        if *lock==0 {
            result+=1;
        }
    }
    else {
        *lock = *lock+value;
        while *lock > 99 {
            *lock = *lock-100;
            result+=1;
        }
    }
    if *lock == 0 {
        return result;
    }
    else {
        return result;
    }
}
