use std::collections::HashMap;
use std::fs::File;
use std::io::{ self, BufRead };

fn main() {
    // let mut strategy = HashMap::new();
    // strategy.insert("A", ("Y", 1));
    // strategy.insert("B", ("X", 2));
    // strategy.insert("C", ("Z", 3));

    // let val = strategy.get("A").copied().unwrap();
    // println!("{:?}", val);
    // println!("{} -> {}", val.0, val.1);

    let mut selection_points = HashMap::new();
    selection_points.insert("X", 1);
    selection_points.insert("Y", 2);
    selection_points.insert("Z", 3);

    let mut result_points = HashMap::new();
    result_points.insert("win", 6);
    result_points.insert("draw", 3);
    result_points.insert("loss", 0);
    
    // Open the file in read-only mode.
    let file = File::open("./guide.txt").unwrap(); 
    // Read the file line by line, and return an iterator of the lines of the file.
    let lines = io::BufReader::new(file).lines();

    let mut sum = 0;
    // Iterate over the lines of the file
    for line in lines {
        let txt_line = line.unwrap().to_owned();
        let bytes: Vec<&str> = txt_line.split_whitespace().collect();
        match bytes[0] {
            "A" => match bytes[1] {
                "X" => sum += selection_points.get("X").unwrap() + result_points.get("draw").unwrap(),
                "Y" => sum += selection_points.get("Y").unwrap() + result_points.get("win").unwrap(),
                "Z" => sum += selection_points.get("Z").unwrap() + result_points.get("loss").unwrap(),
                _ => println!("FAIL")
            },
            "B" => match bytes[1] {
                "X" => sum += selection_points.get("X").unwrap() + result_points.get("loss").unwrap(),
                "Y" => sum += selection_points.get("Y").unwrap() + result_points.get("draw").unwrap(),
                "Z" => sum += selection_points.get("Z").unwrap() + result_points.get("win").unwrap(),
                _ => println!("FAIL")
            },
            "C" => match bytes[1] {
                "X" => sum += selection_points.get("X").unwrap() + result_points.get("win").unwrap(),
                "Y" => sum += selection_points.get("Y").unwrap() + result_points.get("loss").unwrap(),
                "Z" => sum += selection_points.get("Z").unwrap() + result_points.get("draw").unwrap(),
                _ => println!("FAIL")
            },
            _ => println!("FAIL")
        }
    }
    println!("PART ONE SUM: {}", sum);


    // PART TWO

    // Open the file in read-only mode.
    let file = File::open("./guide.txt").unwrap(); 
    // Read the file line by line, and return an iterator of the lines of the file.
    let lines = io::BufReader::new(file).lines();
        
    let mut sum = 0;
    // Iterate over the lines of the file
    for line in lines {
        let txt_line = line.unwrap().to_owned();
        let bytes: Vec<&str> = txt_line.split_whitespace().collect();
        match bytes[0] {
            "A" => match bytes[1] {
                "X" => sum += selection_points.get("Z").unwrap() + result_points.get("loss").unwrap(),
                "Y" => sum += selection_points.get("X").unwrap() + result_points.get("draw").unwrap(),
                "Z" => sum += selection_points.get("Y").unwrap() + result_points.get("win").unwrap(),
                _ => println!("FAIL")
            },
            "B" => match bytes[1] {
                "X" => sum += selection_points.get("X").unwrap() + result_points.get("loss").unwrap(),
                "Y" => sum += selection_points.get("Y").unwrap() + result_points.get("draw").unwrap(),
                "Z" => sum += selection_points.get("Z").unwrap() + result_points.get("win").unwrap(),
                _ => println!("FAIL")
            },
            "C" => match bytes[1] {
                "X" => sum += selection_points.get("Y").unwrap() + result_points.get("loss").unwrap(),
                "Y" => sum += selection_points.get("Z").unwrap() + result_points.get("draw").unwrap(),
                "Z" => sum += selection_points.get("X").unwrap() + result_points.get("win").unwrap(),
                _ => println!("FAIL")
            },
            _ => println!("FAIL")
        }
    }
    println!("PART TWO SUM: {}", sum);

}
