use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pymodule]
fn eratosthenes(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(get_prime_numbers))?;
    Ok(())
}

#[pyfunction]
fn get_prime_numbers(n: i32) -> PyResult<Vec<i32>> {
    let mut flags = Vec::new();
    for _ in 0..n+1 {
        flags.push(true);
    }

    let upper = (n as f32).sqrt().floor() as i32;
    for i in 2..upper+1 {
        if !flags[i as usize] {
            continue;
        }

        let prime = i;

        let mut j = prime * 2;
        while j <= n {
            flags[j as usize] = false;
            j += prime;
        }
    }

    let mut primes = Vec::new();
    for i in 2..n+1 {
        if flags[i as usize] {
            primes.push(i);
        }
    }

    Ok(primes)
}