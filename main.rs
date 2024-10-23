/*
 * This program implements Hamming Code, which is an error-detecting and error-correcting code used for transmitting data over noisy communication channels.
 * Author: Hasan Zaidi
 */

 use std::fs::File;
 use std::io::{self, Read};
 
 const PARITY_BITS: [usize; 4] = [0, 1, 3, 7]; // Positions of parity bits in binary representation
 
 fn main() -> io::Result<()> {
     let (contents, contents2) = read_files("Decoding Text.txt", "Encoding Text.txt")?;
     
     for word in contents2.split_whitespace() {
         for char in word.chars() {
             println!("{}", char);
         }
     }
 
     for word in contents.split_whitespace() {
         if word.contains('=') {
             println!("=========== End Of Line ===========\n");
             continue; 
         }
 
         match word.parse::<u32>() {
             Ok(num) => {
                 let binary = format!("{:b}", num);
                 let binary_vec: Vec<char> = binary.chars().collect();
                 let binary_length = binary_vec.len(); // Determine the length dynamically
 
                 println!("+---------------+");
                 println!("\n{} -- {}\n", num, binary);
                 print_binary_vector(&binary_vec);
 
                 let mut array_new_parity = [0; 4];
                 for (i, &parity_bit) in PARITY_BITS.iter().enumerate() {
                     if parity_bit < binary_length {
                         check_parity_bit(&binary_vec, parity_bit, &mut array_new_parity, i);
                     }
                 }
 
                 print!("\nBinary Index Value - ");
                 for elem in array_new_parity.iter() {
                     print!("{}", elem);
                 }
                 println!("\n\n");
             },
             Err(e) => {
                 println!("Error parsing number: {}", e);
             }
         }
     }
     Ok(())
 }
 
 // Function to read content from two files
 fn read_files(file_path1: &str, file_path2: &str) -> io::Result<(String, String)> {
     let mut file1 = File::open(file_path1)?;
     let mut file2 = File::open(file_path2)?;
 
     let mut contents1 = String::new();
     let mut contents2 = String::new();
 
     file1.read_to_string(&mut contents1)?;
     file2.read_to_string(&mut contents2)?;
 
     Ok((contents1, contents2))
 }
 
 // Function to print binary vector
 fn print_binary_vector(binary_vec: &Vec<char>) {
     print!("\nBinary Vector: ");
     for &bit in binary_vec {
         print!(" {}", bit);
     }
 }
 
 // Function to check parity bits
 fn check_parity_bit(bin_vec: &[char], parity_bit_index: usize, array_updates: &mut [i32; 4], array_index: usize) {
     let mut num_ones = 0;
     let mut bin_char_array = ['0'; 6];
 
     // Assign the parity position to 'p'
     bin_char_array[0] = 'p';
 
     // Populate bin_char_array with relevant bits
     for i in 0..6 {
         if (i + 1) & (parity_bit_index + 1) != 0 {
             bin_char_array[i] = bin_vec[i];
         }
     }
 
     // Count number of '1's in the selected bits
     for &bit in &bin_char_array {
         if bit == '1' {
             num_ones += 1;
         }
     }
 
     println!("\n\nStarting check at p{}: ", parity_bit_index + 1);
     for (j, &bit) in bin_char_array.iter().enumerate() {
         println!("Redundant Bits @ {}: {}", j, bit);
     }
 
     let calculated_parity = if num_ones % 2 == 0 { '0' } else { '1' };
     if calculated_parity == bin_vec[parity_bit_index] {
         println!("No Error Detected at R{} bit", parity_bit_index + 1);
     } else {
         println!("Error Detected at R{}", parity_bit_index + 1);
         array_updates[array_index] = 1;
     }
 }
 