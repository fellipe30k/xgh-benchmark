use std::time::Instant;

fn main() {
    // Valor máximo
    let max_value = 1_000_000_00;
    
    // Inicializando o vetor de números
    let mut numbers = vec![true; max_value + 1];
    numbers[0] = false;
    numbers[1] = false;
    
    // Medindo o tempo de execução
    let start_time = Instant::now();
    for i in 2..=((max_value as f64).sqrt() as usize) {
        if numbers[i] {
            for j in (i*i..=max_value).step_by(i) {
                numbers[j] = false;
            }
        }
    }
    let duration = start_time.elapsed();
    
    // Coletando os números primos
    let primes: Vec<_> = numbers.iter().enumerate().filter_map(|(i, &is_prime)| {
        if is_prime { Some(i) } else { None }
    }).collect();
    
    // Imprimindo a quantidade de números primos encontrados e o tempo de execução
    println!("Quantidade de números primos até {}: {}", max_value, primes.len());
    println!("Tempo de execução: {:?} milissegundos", duration.as_millis());
}
