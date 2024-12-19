#[derive(Debug, PartialEq, Clone)]
pub struct PuzzleInput<'a> {
    pub towel_patterns: Vec<&'a str>,
    pub target_patterns: Vec<&'a str>,
}

impl<'a> PuzzleInput<'a> {
    pub fn new(towel_patterns: Vec<&'a str>, target_patterns: Vec<&'a str>) -> Self {
        Self {
            towel_patterns,
            target_patterns,
        }
    }

    pub fn validate_assumptions(&self) -> Result<(), String> {
        if !self.towel_patterns.iter().all(|p| p.len() > 0) {
            return Err("Empty towel pattern".to_string());
        }
        if !self.target_patterns.iter().all(|p| p.len() > 0) {
            return Err("Empty target pattern".to_string());
        }

        // towel patterns should  be unique
        let mut unique_towel_patterns = self.towel_patterns.clone();
        unique_towel_patterns.sort();
        unique_towel_patterns.dedup();
        if unique_towel_patterns.len() != self.towel_patterns.len() {
            return Err("Towel patterns should be unique".to_string());
        }

        // // No towel pattern should be the prefix of another towel pattern
        // for i in 0..self.towel_patterns.len() {
        //     for j in 0..self.towel_patterns.len() {
        //         if i != j && self.towel_patterns[i].starts_with(&self.towel_patterns[j]) {
        //             return Err(
        //                 "No towel pattern should be the prefix of another towel pattern"
        //                     .to_string(),
        //             );
        //         }
        //     }
        // }

        Ok(())
    }
}
