use std::io;
use std::time::Instant;

fn main() {
    let mut sum = 0;
    let lines = io::stdin().lines();
    for  line in lines.map_while(Result::ok) {
        sum+=maxjolt(line);
    }
    println!("max jolt found {}", sum);
}

fn maxjolt_recursive(line_vector: Vec<char>, mut left_bound: usize, right_bound: usize,current_max_number: &mut String, line_vector_size: usize) {
    if left_bound >= line_vector_size || right_bound <= 0 {
         return;
    }
    else {
        let mut max_digit = '0';
        for index in left_bound..line_vector_size-right_bound+1 {
            if line_vector[index] > max_digit {
                max_digit = line_vector[index];
                left_bound = index+1;
            }
        }
        current_max_number.push(max_digit);
        maxjolt_recursive(line_vector, left_bound, right_bound-1,  current_max_number, line_vector_size)
    }
}

//rewrite this into a DP where we solve for max in each case(WRONG)
// greedy works just fine
// keep in mind this does work for numbers lesser than 12 digits, simply slap a smaller initial right_bound
fn maxjolt(line: String) -> u64 {
    let line_vector: Vec<char> = line.chars().collect();
    let n: usize =  line_vector.len();
    let mut result_string = String::new();
    maxjolt_recursive(line_vector, 0, 12,  &mut result_string, n);
    println!("{}", result_string);
    result_string.parse().expect("failed to parse into an integer")
}
    
