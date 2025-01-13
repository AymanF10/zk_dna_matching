This project implements a basic zero-knowledge proof (ZKP) system that allows users to prove they possess a specific DNA profile without revealing the actual data. The implementation uses Short Tandem Repeats (STRs) as a model for DNA profiles.
Overview-- 
- Define DNA profiles using STR counts.
- Generate zero-knowledge proofs for matching profiles.
- Verify proofs without revealing sensitive information.
- Simulate matching and non-matching scenarios through unit tests.

- Steps:
 1: Define DNA Profile Model- Created a Rust structure to represent an STR profile
 2: Implement ZKP Protocol- Generated Proofs: Created a function that generates a proof based on the user's STR counts compared to the target profile.
 3:Verify Proofs: Implement a verification function that checks if the proof is valid without revealing any data.
 4: Testing- Write tests to simulate matching and non-matching scenarios:
 5:Putting It All Together in main- Now, combine everything in the main function to simulate matching and non-matching scenarios:
 6:Running Your Project- Compile and Run

