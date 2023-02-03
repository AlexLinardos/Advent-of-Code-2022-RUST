use std::fs::File;
use std::io::{ self, BufRead };

fn main() {
    // PART 1
    let file = File::open("./calories.txt").unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut sum = 0;
    let mut max_sum = 0;
    for line in lines {
        let num = line.unwrap();

        if !num.trim().is_empty() {
            let num: u32 = num.trim().parse().expect("Not a number...");
            sum += num;
        } else {
            if sum > max_sum {
                max_sum = sum;
            }
            sum = 0; // reset sum on empty line (moving to next elf)
        }
    }

    // last sum check
    if sum > max_sum {
        max_sum = sum;
    }
    println!("MAX: {}", max_sum);


    // PART 2
    let mut sum = 0;
    let mut v_sums = Vec::new();
    let file = File::open("./calories.txt").unwrap();
    let lines = io::BufReader::new(file).lines();

    for line in lines {
        let num = line.unwrap();

        if !num.trim().is_empty() {
            let num: u32 = num.trim().parse().expect("Not a number...");
            sum += num;
        } else {
            v_sums.push(sum);
            sum = 0; // reset sum on empty line (moving to next elf)
        }
    }
    v_sums.sort_by(|a, b| b.cmp(a));
    println!("MAX 3: {:?}", v_sums[0..3].to_vec());
    let total: u32= v_sums[0..3].to_vec().iter().sum();
    println!("TOTAL: {}", total);

}
