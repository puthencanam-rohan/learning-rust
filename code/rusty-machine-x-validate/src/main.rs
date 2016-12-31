extern crate rusty_machine;

use std::fmt;
use rusty_machine::analysis::cross_validation::k_fold_validate;
use rusty_machine::analysis::score::row_accuracy;
use rusty_machine::learning::naive_bayes::{NaiveBayes, Bernoulli};
use rusty_machine::linalg::{BaseMatrix, Matrix};

fn main() {
    let inputs = Matrix::new(3, 2, vec![1.0, 1.1,
                                        5.2, 4.3,
                                        6.2, 7.3]);

    let targets = Matrix::new(3, 3, vec![1.0, 0.0, 0.0,
                                         0.0, 0.0, 1.0,
                                         0.0, 0.0, 1.0]);

    let mut model = NaiveBayes::<Bernoulli>::new();

    let accuracy_per_fold: Vec<f64> = k_fold_validate(
        &mut model,
        &inputs,
        &targets,
        3,
        // Score each fold by the fraction of test samples where
        // the model's prediction equals the target.
        row_accuracy
    ).unwrap();

    // Set precision in format
    let prec = 3;

    for value in accuracy_per_fold.into_iter() {
        let value = format!("{:.*}", prec, value);
        println!("{}", value);
    }
}

