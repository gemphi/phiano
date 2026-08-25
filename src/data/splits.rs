/// Train/validation/test data splits for reproducible evaluation.
/// Uses deterministic hashing for reproducibility (no random seed needed).

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// DataSplits — partitions sentences into train (80%), validation (10%), test (10%).
#[derive(Debug, Clone)]
pub struct DataSplits {
    pub train: Vec<String>,
    pub validation: Vec<String>,
    pub test: Vec<String>,
}

impl DataSplits {
    /// Splits a corpus into train/val/test using deterministic hashing.
    /// Each sentence is hashed and assigned to a split based on the hash value.
    pub fn from_corpus(corpus: &[String]) -> Self {
        let mut train = Vec::new();
        let mut validation = Vec::new();
        let mut test = Vec::new();

        for sentence in corpus {
            let bucket = Self::hash_bucket(sentence);
            match bucket {
                0..=7 => train.push(sentence.clone()),
                8 => validation.push(sentence.clone()),
                9 => test.push(sentence.clone()),
                _ => train.push(sentence.clone()),
            }
        }

        Self { train, validation, test }
    }

    /// Splits with custom ratios (e.g., 0.8, 0.1, 0.1).
    pub fn with_ratios(corpus: &[String], train_ratio: f64, val_ratio: f64) -> Self {
        let mut train = Vec::new();
        let mut validation = Vec::new();
        let mut test = Vec::new();

        let val_threshold = (train_ratio + val_ratio) * 10.0;

        for sentence in corpus {
            let bucket = Self::hash_bucket(sentence);
            let bucket_f = bucket as f64 / 10.0;
            if bucket_f < train_ratio {
                train.push(sentence.clone());
            } else if bucket_f < val_threshold {
                validation.push(sentence.clone());
            } else {
                test.push(sentence.clone());
            }
        }

        Self { train, validation, test }
    }

    /// Returns the training sentences.
    pub fn train_iter(&self) -> impl Iterator<Item = &String> {
        self.train.iter()
    }

    /// Returns the validation sentences.
    pub fn val_iter(&self) -> impl Iterator<Item = &String> {
        self.validation.iter()
    }

    /// Returns the test sentences.
    pub fn test_iter(&self) -> impl Iterator<Item = &String> {
        self.test.iter()
    }

    /// Returns counts for each split.
    pub fn counts(&self) -> (usize, usize, usize) {
        (self.train.len(), self.validation.len(), self.test.len())
    }

    /// Deterministic hash bucket [0, 9] for a sentence.
    fn hash_bucket(sentence: &str) -> u64 {
        let mut hasher = DefaultHasher::new();
        sentence.hash(&mut hasher);
        hasher.finish() % 10
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_splits_deterministic() {
        let corpus: Vec<String> = (0..100)
            .map(|i| format!("sentence number {}", i))
            .collect();

        let s1 = DataSplits::from_corpus(&corpus);
        let s2 = DataSplits::from_corpus(&corpus);

        assert_eq!(s1.train, s2.train);
        assert_eq!(s1.validation, s2.validation);
        assert_eq!(s1.test, s2.test);
    }

    #[test]
    fn test_split_counts() {
        let corpus: Vec<String> = (0..1000)
            .map(|i| format!("unique sentence {}", i))
            .collect();

        let splits = DataSplits::from_corpus(&corpus);
        let (train, val, test) = splits.counts();

        assert!(train > 700, "train should be ~80%: got {}", train);
        assert!(val > 50, "val should be ~10%: got {}", val);
        assert!(test > 50, "test should be ~10%: got {}", test);
    }
}
