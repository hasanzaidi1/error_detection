# Hamming Code Error Detection and Correction

This program implements Hamming Code, an error-detecting and error-correcting code used for transmitting data over noisy communication channels. Hamming Code adds additional parity bits to the original data to detect and correct errors that might occur during transmission.

## Table of Contents

- [Introduction](#introduction)
- [Installation](#installation)
- [Usage](#usage)
- [File Structure](#file-structure)
- [Author](#author)


## Introduction

In data communication, ensuring the integrity of transmitted information is crucial. Hamming Code is a widely used technique that not only detects errors but corrects them, providing robustness in data transmission. This Rust program demonstrates the implementation and application of Hamming Code for error detection and correction.

## Installation

1. Make sure you have Rust installed. If not, install Rust from the [official website](https://www.rust-lang.org/tools/install).

2. Clone the repository to your local machine using:

    ```bash
    git clone https://github.com/your-username/hamming-code.git
    ```

3. Navigate to the project directory:

    ```bash
    cd hamming-code
    ```

4. Run the program using:

    ```bash
    cargo run
    ```

## Usage

1. The program reads data from input files (`Decoding Text.txt` and `Encoding Text.txt`).

2. It converts the data into binary and calculates Hamming Code parity bits.

3. The program detects and corrects errors based on the Hamming Code algorithm.

## File Structure
- `main.rs` - Source code for Hamming Code algorithm.
- `Decoding Text.txt`: Input file for decoding text.
- `Encoding Text.txt`: Input file for encoding text.

## Author

- [Hasan Zaidi](https://github.com/hasanzaidi1)

