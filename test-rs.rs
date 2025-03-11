use std::time::Instant;

fn main() {
    // Valor máximo
    let max_value = 100_000_000;
    
    // Inicializando o vetor apenas para números ímpares
    // is_prime[i] corresponde ao número 2*i + 1
    let mut is_prime = vec![true; (max_value + 1) / 2];
    is_prime[0] = false; // 1 não é primo
    
    // Medindo o tempo de execução
    let start_time = Instant::now();
    
    // Aplicando o Crivo de Eratóstenes apenas para números ímpares
    let sqrt_max = (max_value as f64).sqrt() as usize;
    for i in 1..=((sqrt_max - 1) / 2) {
        if is_prime[i] {
            // O número real é 2*i + 1
            let p = 2 * i + 1;
            
            // Marcar todos os múltiplos ímpares de p como não primos
            // Começamos com p*p e calculamos o índice correspondente
            let mut idx = (p * p - 1) / 2;
            while idx < is_prime.len() {
                is_prime[idx] = false;
                idx += p; // Incrementamos o índice por p
            }
        }
    }
    
    // Contagem rápida sem recrear vetores
    // Começamos com 1 para contar o número 2
    let count = 1 + is_prime.iter().skip(1).filter(|&&is_p| is_p).count();
    
    let duration = start_time.elapsed();
    
    // Imprimindo a quantidade de números primos encontrados e o tempo de execução
    println!("Quantidade de números primos até {}: {}", max_value, count);
    println!("Tempo de execução: {} milissegundos", duration.as_millis());
}