use std::time::Instant;

fn main() {
    // Valor máximo
    let max_value = 100_000_000;

    // Inicializando o vetor de números
    let mut numbers = vec![true; max_value + 1];
    numbers[0] = false;
    numbers[1] = false;

    // Medindo o tempo de execução
    let start_time = Instant::now();
    let max_iter = max_value.isqrt();
    (2..=max_iter).for_each(|i| {
        if numbers[i] {
            numbers
                .iter_mut()
                .skip(i * i)
                .step_by(i)
                .for_each(|is_prime| *is_prime = false);
        }
    });
    let duration = start_time.elapsed();

    // Coletando os números primos
    let primes_count = numbers.iter().filter(|&&is_prime| is_prime).count();

    // Imprimindo a quantidade de números primos encontrados e o tempo de execução
    println!(
        "Quantidade de números primos até {}: {}",
        max_value, primes_count
    );
    println!(
        "Tempo de execução: {:?} milissegundos",
        duration.as_millis()
    );
}
