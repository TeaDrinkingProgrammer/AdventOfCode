use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;

fn main() {
    // excersice1();
    excersice1b();
}

fn readExample(){
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
}
fn excersice1a(){
    let filename = "excersice1.txt";
    let mut previous_num: u32  = 0;
    let mut num_of_increases: u32 = 0;
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (index, line) in reader.lines().enumerate() {
        let current_num = line.unwrap().parse::<u32>().unwrap(); // Ignore errors.
        if index > 0 {
            if previous_num < current_num {
                num_of_increases += 1;
            }   
        }
        previous_num = current_num;
    }
    println!("Number of increases: {}",num_of_increases)
}
fn excersice1b(){
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

    //actual excersice:
    let mut num_of_increases: u32 = 0;
    for index in 2..num_vector.len()-1 {
        let mut previous_sum = 0; 
        if index > 2 {
            previous_sum = num_vector[index-3] + num_vector[index-2] + num_vector[index-1];
        }
        let current_sum = num_vector[index-2] + num_vector[index-1] + num_vector[index];
        if previous_sum < current_sum {
            num_of_increases +=1;
        }
    }
    println!("Number of increases: {}",num_of_increases);
}