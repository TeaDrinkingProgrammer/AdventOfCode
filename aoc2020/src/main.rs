use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;

fn main() {
    // exercise1();
    exercise2();
}

fn exercise1(){
    let filename = "excersice1.txt";
    let mut num_vector: Vec<u32> = Vec::new();
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        // Show the line and its number.
        num_vector.push(line.parse::<u32>().unwrap())
    }
    for number1 in &num_vector {
        for number2 in &num_vector {
            for number3 in &num_vector {
                if number1 + number2 + number3 == 2020{
                    println!("num1: {}, num2: {}, num3: {}",number1,number2,number3);
                    println!("answer: {}", number1 * number2 * number3);
                }
        }
        }
    }
}

fn exercise2(){
    //failed excercise, use split next time
    // let filename = "excersice2.txt";
    // // let mut line_vector: Vec<String> = Vec::new();
    // // Open the file in read-only mode (ignoring errors).
    // let file = File::open(filename).unwrap();
    // let reader = BufReader::new(file);
    // // Read the file line by line using the lines() iterator from std::io::BufRead.
    // for (_index, line) in reader.lines().enumerate() {
    //     let line = line.unwrap(); // Ignore errors.
    //     // Show the line and its number.
    //     let char_vec: Vec<char> = line.chars().collect();
    //     let mut min : i32;
    //     let max = char_vec[2] as i32 - 0x30;
    //     let base_letter = char_vec[4];

        
        
    //     for index in 7..char_vec.len() {

    //         if index == 1 && char_vec[index] != '-' {
    //             &mut min = char_vec[0] as i32 - 0x30;
    //         }

    //     }
    //     println!("");
    // }
}
fn exercise3(){

    let filename = "excersice3.txt";
    let mut slope_vector: Vec<Vec<bool>> = Vec::new();
    let mut amount_of_trees : Vec<i16> = Vec::new();
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (index, line) in reader.lines().enumerate() {
        let line_vector : Vec<char> = line.unwrap().chars().collect();
        for value in line_vector {
            //Is there a tree? push true
            if value == '.' {
                slope_vector[index].push(false);
            } else {
                slope_vector[index].push(true);
            }
        }
    }
    let mut x : isize = 0;
    let mut y : isize = 0;
    //Loop doesn't work yet
    // loop {
    //     if &x + 3 < slope_vector.len().try_into().unwrap() && &y + 1 < slope_vector[x].len().try_into().unwrap() {
    //         &mut x += 3;
    //         &mut y += 1;
    //        if slope_vector[&x][&y] {
    //             amount_of_trees += 1;
    //        }
    //     }
    // }
}