/*
 *This program implements Hamming Code, which is an error-detecting and error-correcting code used for transmitting data over noisy communication channels. Specifically, Hamming Code adds additional parity bits to the original data to detect and correct errors that might occur during transmission.
 *
 * Fixing The Problem using Hamming Code
 *
 * Author: Hasan Zaidi
 */

use std::io::Read;

fn main() {
  
  let mut file = std::fs::File::open("Decoding Text.txt").unwrap();
  let mut file2 = std::fs::File::open("Encoding Text.txt").unwrap();
  let mut contents = String::new();
  let mut contents2 = String::new();
  file.read_to_string(&mut contents).unwrap();
  file2.read_to_string(&mut contents2).unwrap();
  let mut array_new_parity = [0,0,0,0];


  for words in contents2.split_whitespace(){  //Printing letters into bits
    for chars in words.as_bytes() {
      println!("{}", chars);
    }
  }

  
  for word in contents.split_whitespace() {
    
    if word.contains('='){
      println!("=========== End Of Line ===========\n");
      continue; //Change
    }
    
    let num_string = word; // numeric string
    
    let num = num_string.parse::<u32>().unwrap(); // parsing input string to u32
    
    let binary = format!("{:b}", num); // converting u32 to binary string
    
    let binary_vec: Vec<char> = binary.chars().collect(); // converting binary string to vector of chars

    
    let mut _frst = binary_vec[binary_vec.len()-1]; //vars for holding parity bits
    let mut _sec = binary_vec[binary_vec.len()-2];
    let mut _frth = binary_vec[binary_vec.len()-4];
    let mut _etth = binary_vec[binary_vec.len()-8];
    println!("+---------------+");
    println!("\n{} -- {}\n", num, binary);
    
    
    
    // Printing the binary vector
    print!("\nBinary Vector: ");
    for bit in &binary_vec {
      print!(" {}", bit);
    } //For loop print each bit of the binary vector
    
    
    let binary_vec_copy = binary_vec.clone();
    let binary_vec_copy2 = binary_vec.clone();
    let binary_vec_copy3 = binary_vec.clone();
    hamming_calculation_frst(_frst, binary_vec, &mut array_new_parity);
    hamming_calculation_sec(_sec, binary_vec_copy, &mut array_new_parity);
    hamming_calculation_frth(_frth, binary_vec_copy2,&mut array_new_parity);
    hamming_calculation_egth(_etth, binary_vec_copy3,&mut array_new_parity);

    print!("\nBinary Index Value - ");
    for elem in array_new_parity.iter() {
      print!("{}", elem);
    }

    
    
    println!("\n\n");
    
  } //------big for loop ended--------
  
}



//These functions perform the parity checks with the given parity bits
fn hamming_calculation_frst(mut parity_bit_frst: char, mut bin_vec: Vec<char>, array_2b_updates: &mut [i32]){
  let mut bin_char_array = ['0','0','0','0','0','0'];
  
  print!("\n\nStarting check at p1:");
  let mut index = 0;
  //updating the bin_char_array with the new values 
  for bin_char in bin_vec.iter().rev().step_by(2){
    if index <= 5{
      bin_char_array[index] = *bin_char;
    }
    index += 1;
  }
  
  println!();
  let mut num_ones = 0;
  bin_char_array[0] = 'p'; //making sure the parity bit is 0
  for &i in bin_char_array.iter() {
    if i == '1'{
      num_ones += 1;
    }
    println!("Redundant Bits = {}",i);
  }
  println!("Amount of ones: {} ", num_ones);

  
  let mut p1 = '0';
  if num_ones % 2 == 0 {
    p1 = '0';
  } else {
    p1 = '1';
  }

  bin_vec.reverse();
  if p1 == bin_vec[0] {
    println!("No Error Detected at R1 bit");
  } else {
    println!("Error Detected at R1");
    p1 = '1';
    array_2b_updates[0] = if p1 == '1' {1} else {0};
  }
  println!("\nReversed Bin:");
  for corrected_bit in bin_vec.iter() {
    print!("{}",corrected_bit);
  }
} // first parity check function ended



fn hamming_calculation_sec(mut parity_bit_sec: char,mut bin_vec: Vec<char>, array_2b_updates: &mut [i32]){
  bin_vec.reverse(); // #REVERSED 
  let mut p2;
  p2 = bin_vec[1];
  let mut bin_char_array = ['0','0','0','0','0','0'];
  
  bin_char_array[0] = bin_vec[1];
  bin_char_array[1] = bin_vec[2];
  bin_char_array[2] = bin_vec[5];
  bin_char_array[3] = bin_vec[6];
  bin_char_array[4] = bin_vec[9];
  if bin_vec.len() > 10 {
    bin_char_array[5] = bin_vec[10];
  }
  

  bin_char_array[0] = 'p'; //making sure place of p2 is 0
  println!("\n\nStarting check at p2: ");
  let mut j = 0;
  let mut amnt_ones = 0;
  for i in bin_char_array{       //Iterating over bin_char_array and counting amount of 1's
    println!("Redundant Bits @ {}: {}", j, i);
    if i == '1' {
      amnt_ones += 1;
    }
    j += 1;
  }
  println!("Amount of ones: {}",amnt_ones);

  if amnt_ones % 2 == 0 {
    p2 = '0';
  } else {
    p2 = '1';
  }

  if p2 == parity_bit_sec {
    println!("No Error Detected at R2 bit");
  } else {
    println!("Error Detected at R2 bit");
    p2 = '1';
    array_2b_updates[1] = if p2 == '1' {1} else {0};
  }
  
} //Second parity calculation function ended


fn hamming_calculation_frth(mut parity_bit_frth: char,mut bin_vec: Vec<char>, array_2b_updates: &mut [i32]){
  bin_vec.reverse(); // #REVERSED 
  let mut p4;
  p4 = bin_vec[3];
  let mut bin_char_array = ['0','0','0','0','0'];
  
  bin_char_array[0] = bin_vec[3];
  bin_char_array[1] = bin_vec[4];
  bin_char_array[2] = bin_vec[5];
  bin_char_array[3] = bin_vec[6];
  if bin_vec.len() > 10 {
    bin_char_array[4] = bin_vec[10];
  }
  

  bin_char_array[0] = 'p'; //making sure place of p4 is 0
  println!("\n\nStarting check at p4: ");
  let mut j = 0;
  let mut amnt_ones = 0;
  for i in bin_char_array{   //Iterating over bin_char_array and counting amount of 1's
    println!("Redundant Bits @ {}: {}", j, i);
    if i == '1' {
      amnt_ones += 1;
    }
    j += 1;
  }
  println!("Amount of ones: {}",amnt_ones);

  if amnt_ones % 2 == 0 {
    p4 = '0';
  } else {
    p4 = '1';
  }
  
  if p4 == bin_vec[3] {
    println!("No Error Detected at R4 bit");
  } else {
    println!("Error Detected at R4 bit");
    p4 = '1';
    array_2b_updates[2] = if p4 == '1' {1} else {0};
  }
  
}


fn hamming_calculation_egth(mut parity_bit_egth: char,mut bin_vec: Vec<char>, array_2b_updates: &mut [i32]){
  bin_vec.reverse(); // #REVERSED 
  let mut p8;
  
  let mut bin_char_array = ['0','0','0','0','0'];
  bin_char_array[0] = bin_vec[7];
  bin_char_array[1] = bin_vec[8];
  bin_char_array[2] = bin_vec[9];
  if bin_vec.len() > 10 {
    bin_char_array[3] = bin_vec[10];
  }
  
  bin_char_array[0] = 'p'; //making sure place of p8 is 0
  println!("\n\nStarting check at p8: ");
  let mut j = 0;
  let mut amnt_ones = 0;
  for i in bin_char_array{   //Iterating over bin_char_array and counting amount of 1's
    println!("Redundant Bits @ {}: {}", j, i);
    if i == '1' {
      amnt_ones += 1;
    }
    j += 1;
  }
  println!("Amount of ones: {}",amnt_ones);

  if amnt_ones % 2 == 0 {
    p8 = '0';
  } else {
    p8 = '1';
  }
  if bin_vec[7] == p8 {
    println!("No Error Detected at R8 bit");
  } else {
    println!("Error Detected at R8 bit");
    p8 = '1';
    array_2b_updates[3] = if p8 == '1' {1} else {0};
  }
}


