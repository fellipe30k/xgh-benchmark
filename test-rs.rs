use std::time::Instant;

fn main() {
    // Valor máximo
    let max_value: usize = 100_000_000;

    // Inicializando o vetor de números
    let mut numbers = vec![true; max_value + 1];
    numbers[0] = false;
    numbers[1] = false;

    // Medindo o tempo de execução
    let start_time = Instant::now();
    for i in 2..=max_value.isqrt() {
        if numbers[i] {
            // Fatiar a partir de i*i evita percorrer o início do vetor
            // e elimina a checagem de limites no laço interno
            for is_prime in numbers[i * i..].iter_mut().step_by(i) {
                *is_prime = false;
            }
        }
    }
    let duration = start_time.elapsed();

    // Contando os números primos
    let primes_count = numbers.iter().filter(|&&is_prime| is_prime).count();

    // Imprimindo a quantidade de números primos encontrados e o tempo de execução
    println!(
        "Quantidade de números primos até {}: {}",
        max_value, primes_count
    );
    println!(
        "Tempo de execução: {:.2} milissegundos",
        duration.as_secs_f64() * 1000.0
    );
}
