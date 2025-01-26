## Overview
This project implements a simple DNA profile matching system in Rust. It defines a structure for DNA profiles, generates proofs of matches between profiles, and verifies these proofs. The project simulates the process of comparing DNA profiles based on Short Tandem Repeats (STR) counts.

## Table of Contents
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Project Structure](#project-structure)
- [Program Logic](#program-logic)
  - [DNA Profile Structure](#dna-profile-structure)
  - [Proof Generation](#proof-generation)
  - [Proof Verification](#proof-verification)
- [Usage](#usage)
- [Testing](#testing)
- [Contributing](#contributing)
- [License](#license)

## Installation
1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/dna-profile-matching.git
   cd dna-profile-matching
   ```

2. Build the project:
   ```bash
   cargo build
   ```

3. Run the project:
   ```bash
   cargo run
   ```

## Project Structure
The project consists of the following key files:

| File                  | Description                                                   |
|-----------------------|---------------------------------------------------------------|
| `main.rs`             | Contains the main logic for DNA profile creation, proof generation, and verification. |
| `function.rs`         | Contains functions related to proof generation and verification. |

## Program Logic

### DNA Profile Structure
The `DnaProfile` struct holds the count of each STR in a vector. It provides methods to create a new profile and check if two profiles match.

```rust
#[derive(Debug)]
struct DnaProfile {
    str_counts: Vec<u32>, // Count of each STR
}

impl DnaProfile {
    fn new(str_counts: Vec<u32>) -> Self {
        DnaProfile { str_counts }
    }

    fn matches(&self, other: &DnaProfile) -> bool {
        self.str_counts == other.str_counts
    }
}
```

### Proof Generation
The `generate_proof` function simulates the process of generating a proof that indicates whether two DNA profiles match.

```rust
fn generate_proof(prover_profile: &DnaProfile, target_profile: &DnaProfile) -> Option<bool> {
    if prover_profile.matches(target_profile) {
        Some(true) // Proof of match
    } else {
        Some(false) // Proof of non-match
    }
}
```

### Proof Verification
The `verify_proof` function checks the validity of the generated proof.

```rust
fn verify_proof(proof: Option<bool>) -> bool {
    proof.unwrap_or(false)
}
```

## Usage
1. **Creating Profiles**: You can create DNA profiles by initializing `DnaProfile` with a vector of STR counts.
2. **Generating Proofs**: Use `generate_proof` to create proofs between two profiles.
3. **Verifying Proofs**: Use `verify_proof` to check if the generated proof indicates a match.

### Example Usage in `main.rs`
```rust
fn main() {
    let target_profile = DnaProfile::new(vec![10, 5, 7]); // Example target profile
    let matching_profile = DnaProfile::new(vec![10, 5, 7]); // Matching profile
    let non_matching_profile = DnaProfile::new(vec![9, 6, 8]); // Non-matching profile

    // Test matching scenario
    let proof = generate_proof(&matching_profile, &target_profile);
    let result = verify_proof(proof);
    println!("Matching Profile Proof Verified: {}", result); // Should print true

    // Test non-matching scenario
    let proof = generate_proof(&non_matching_profile, &target_profile);
    let result = verify_proof(proof);
    println!("Non-Matching Profile Proof Verified: {}", result); // Should print false
}
```

## Testing
You can add tests to ensure that your functions work as expected. Create a new file named `tests.rs` in the `src` directory and write your test cases there.

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matching_profiles() {
        let profile1 = DnaProfile::new(vec![10, 5, 7]);
        let profile2 = DnaProfile::new(vec![10, 5, 7]);
        assert!(profile1.matches(&profile2));
    }

    #[test]
    fn test_non_matching_profiles() {
        let profile1 = DnaProfile::new(vec![10, 5, 7]);
        let profile2 = DnaProfile::new(vec![9, 6, 8]);
        assert!(!profile1.matches(&profile2));
    }
}
```

Run tests using:
```bash
cargo test
```
This project is licensed under the MIT License - see the LICENSE file for details.
