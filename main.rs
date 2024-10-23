/*
 * This program implements Hamming Code, which is an error-detecting and error-correcting code used for transmitting data over noisy communication channels.
 * Author: Hasan Zaidi
 */

 use std::fs::File;
 use std::io::{self, Read};
 
 const PARITY_BITS: [usize; 4] = [0, 1, 3, 7]; // Positions of parity bits in binary representation
 
 fn main() -> io::Result<()> {
     let mut file = File::open("Decoding Text.txt")?;
     let mut file2 = File::open("Encoding Text.txt")?;
     
     let mut contents = String::new();
     let mut contents2 = String::new();
     
     file.read_to_string(&mut contents)?;
     file2.read_to_string(&mut contents2)?;
     
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
 
         let num: u32 = word.parse().expect("Invalid number format");
         let binary = format!("{:b}", num);
         let binary_vec: Vec<char> = binary.chars().collect();
 
         println!("+---------------+");
         println!("\n{} -- {}\n", num, binary);
         print_binary_vector(&binary_vec);
 
         let mut array_new_parity = [0; 4];
         for (i, &parity_bit) in PARITY_BITS.iter().enumerate() {
             check_parity_bit(binary_vec.clone(), parity_bit, &mut array_new_parity, i);
         }
 
         print!("\nBinary Index Value - ");
         for elem in array_new_parity.iter() {
             print!("{}", elem);
         }
         println!("\n\n");
     }
     Ok(())
 }
 
 fn print_binary_vector(binary_vec: &Vec<char>) {
     print!("\nBinary Vector: ");
     for &bit in binary_vec {
         print!(" {}", bit);
     }
 }
 
 fn check_parity_bit(bin_vec: Vec<char>, parity_bit_index: usize, array_updates: &mut [i32; 4], array_index: usize) {
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
 