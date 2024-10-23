# Hamming Code Implementation

## Overview

The Hamming Code Implementation project is a demonstration of error detection and correction techniques used in data communication systems. By employing the Hamming algorithm, this project effectively encodes and decodes binary data, ensuring reliable data transmission over noisy channels. This implementation showcases my proficiency in Rust and understanding of fundamental concepts in error correction, making it a valuable addition to my technical portfolio.

## Features

- **Error Detection and Correction**: Implements the Hamming Code to identify and correct single-bit errors in transmitted data.
- **User-Friendly Input/Output**: Reads data from text files and outputs results, providing a clear and understandable interaction for users.
- **Detailed Debugging Information**: Offers comprehensive output for parity checks and error status, aiding in understanding the encoding and decoding processes.
- **Efficient Data Handling**: Processes binary data efficiently, demonstrating effective use of Rust's performance capabilities.

## How It Works

The Hamming Code algorithm adds redundancy to data by incorporating parity bits, which allows for the detection and correction of errors. The key steps involved in this project are:

1. **Input Processing**: The program reads numeric data from the `Encoding Text.txt` file and prepares it for encoding.
2. **Binary Conversion**: Converts the numeric input into its binary representation.
3. **Parity Bit Calculation**: Calculates parity bits based on the data bits and their positions, ensuring that the overall parity conforms to the specified error detection strategy.
4. **Error Simulation**: The program can simulate errors in the encoded data for testing purposes.
5. **Error Detection and Correction**: During decoding, the program checks the parity bits against the received data, identifies any errors, and corrects them if possible.
6. **Output**: Displays the encoded data, calculated parity bits, and error correction results clearly in the console.

### Example Flow

- **Input**: Given a number (e.g., `5`), the program will output its binary representation, the calculated parity bits, and any detected errors after processing an encoded string.
- **Output**: The results will indicate whether errors were detected, the corrected data, and the corresponding binary values.

## Usage

1. **Setup**: Clone this repository and navigate to the project directory.
   ```bash
   git clone https://github.com/hasanzaidi1/error_detection.git
   cd error_detection

2. Input Files: Prepare the input files:
   
   `Encoding Text.txt`: Contains the numeric data to be encoded.
   
   `Decoding Text.txt`: Contains the encoded data to be decoded.

4. Compile the Code: Ensure Rust is installed on your machine and compile the project using:

    ```bash
    rustc main.rs
    
5. Run the Program:
    ```bash
    ./main
    
6. Results: The console will display the results of the encoding and decoding processes, including any errors detected.


## Technical Skills Demonstrated

- **Programming Language**: Proficient in Rust, focusing on systems programming and memory safety.
  - **Situation**: Required to implement a reliable data transmission solution.
  - **Task**: Develop a program that effectively encodes and decodes data using error correction techniques.
  - **Action**: Leveraged Rust's features to ensure memory safety and efficiency while coding the Hamming algorithm.
  - **Result**: Successfully created a robust program that performs reliably in error-prone environments.

- **Algorithms and Data Structures**: Implemented the Hamming Code algorithm, showcasing understanding of data integrity principles.
  - **Situation**: Faced with the challenge of ensuring data integrity during transmission.
  - **Task**: Identify and correct single-bit errors in transmitted data.
  - **Action**: Integrated parity bits into the encoding process, enabling error detection and correction.
  - **Result**: Achieved a reliable encoding and decoding mechanism that minimizes data loss.

- **File I/O Operations**: Managed file input/output in Rust, demonstrating skills in data manipulation.
  - **Situation**: Needed to process external data files for encoding and decoding.
  - **Task**: Read numeric data from files and output results to the console.
  - **Action**: Implemented effective file handling to streamline data processing.
  - **Result**: Enhanced user experience through seamless data input and output.

- **Debugging and Testing**: Developed features for error simulation and validation.
  - **Situation**: Required to ensure the accuracy of the encoding and decoding processes.
  - **Task**: Create a testing framework to simulate errors in transmitted data.
  - **Action**: Built-in error simulation and detailed output for comprehensive testing.
  - **Result**: Increased confidence in the program's reliability through thorough testing.

- **Problem Solving**: Utilized logical reasoning to address challenges related to error detection and correction.
  - **Situation**: Encountered difficulties in managing parity bits during encoding.
  - **Task**: Optimize the calculation of parity bits to ensure accurate error detection.
  - **Action**: Analyzed the algorithm and adjusted the logic for parity bit calculations.
  - **Result**: Improved the overall efficiency and reliability of the error correction mechanism.


