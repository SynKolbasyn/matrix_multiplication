use anyhow::{bail, Result};
use cli_table::Table;
use indicatif::{ProgressBar, ProgressStyle};
use rand::rngs::ThreadRng;
use rand::{Rng, thread_rng};
use rayon::prelude::*;


fn main() -> Result<()> {
    let m1: Vec<Vec<f64>> = gen_random_matrix(3, 3)?;
    let m2: Vec<Vec<f64>> = gen_random_matrix(3, 3)?;

    let result: Vec<Vec<f64>> = matrix_multiplication(&m1, &m2)?;

    print_matrices(&m1, &m2, &result)?;

    return Ok(());
}


fn gen_random_matrix(cols: u128, rows: u128) -> Result<Vec<Vec<f64>>> {
    let mut matrix: Vec<Vec<f64>> = Vec::new();

    let mut rng: ThreadRng = thread_rng();

    for _ in 0..cols {
        let mut row: Vec<f64> = Vec::new();

        for _ in 0..rows {
            let mut from: i8 = rng.gen();
            let mut to: i8 = rng.gen();

            while from >= to {
                from = rng.gen();
                to = rng.gen();
            }

            row.push((rng.gen_range((from as f64)..=(to as f64)) * 10.0).round() / 10.0);
        }

        matrix.push(row);
    }

    return Ok(matrix);
}


fn print_matrices(m1: &Vec<Vec<f64>>, m2: &Vec<Vec<f64>>, result: &Vec<Vec<f64>>) -> Result<()> {
    println!("\nMatrix 1:\n{}", m1.table().display()?);
    println!("\nMatrix 2:\n{}", m2.table().display()?);
    println!("\nResult:\n{}", result.table().display()?);

    return Ok(());
}


fn matrix_multiplication(m1: &Vec<Vec<f64>>, m2: &Vec<Vec<f64>>) -> Result<Vec<Vec<f64>>> {
    check_matrix(&m1, &m2)?;

    let pb: ProgressBar = ProgressBar::new((m1.len() * m2[0].len() * m1[0].len()) as u64);
    pb.set_style(ProgressStyle::default_bar().template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})").unwrap());
    pb.set_message(format!("Multiplication..."));

    let result: Vec<Vec<f64>> = (0..m1.len()).into_par_iter().map(|i: usize| {
        let mut row: Vec<f64> = Vec::new();

            for j in 0..m2[0].len() {
                let mut cell: f64 = 0.0;

                for k in 0..m1[0].len() {
                    cell += m1[i][k] * m2[k][j];
                    pb.inc(1);
                }

                row.push((cell * 10.0).round() / 10.0);
            }

        row
    }).collect();

    pb.finish_with_message("Finished: ");

    return Ok(result);
}


fn check_matrix(m1: &Vec<Vec<f64>>, m2: &Vec<Vec<f64>>) -> Result<()> {
    if m1.len() == 0 || m2.len() == 0 {
        bail!("The dimensions of the matrices are incorrect");
    }

    let m1_len: usize = m1[0].len();
    let m2_len: usize = m2[0].len();

    if m1_len != m2.len() || m2_len != m1.len() {
        bail!("The dimensions of the matrices are incorrect");
    }

    if m1_len == 0 || m2_len == 0 {
        bail!("The dimensions of the matrices are incorrect");
    }

    for i in 0..m1.len() {
        if m1_len != m1[i].len() || m2_len != m2[i].len() {
            bail!("The dimensions of the matrices are incorrect");
        }
    }

    return Ok(());
}
