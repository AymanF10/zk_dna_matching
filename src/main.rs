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

fn generate_proof(prover_profile: &DnaProfile, target_profile: &DnaProfile) -> Option<bool> {
    // Simulate proof generation
    if prover_profile.matches(target_profile) {
        Some(true) // Proof of match
    } else {
        Some(false) // Proof of non-match
    }
}

fn verify_proof(proof: Option<bool>) -> bool {
    proof.unwrap_or(false)
}

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




