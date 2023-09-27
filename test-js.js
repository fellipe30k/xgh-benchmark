// Valor máximo
const maxValue = 100000000;

// Inicializando o array de números
const numbers = new Array(maxValue + 1).fill(true);
numbers[0] = numbers[1] = false;

// Medindo o tempo de execução
const startTime = Date.now();
for (let i = 2; i <= Math.sqrt(maxValue); i++) {
    if (numbers[i]) {
        for (let j = i * i; j <= maxValue; j += i) {
            numbers[j] = false;
        }
    }
}
const endTime = Date.now();

// Coletando os números primos
const primes = numbers.reduce((count, isPrime) => isPrime ? count + 1 : count, 0);

// Calculando o tempo de execução em milissegundos
const executionTimeMs = endTime - startTime;

// Imprimindo a quantidade de números primos encontrados e o tempo de execução
console.log(`Quantidade de números primos até ${maxValue}: ${primes}`);
console.log(`Tempo de execução: ${executionTimeMs} milissegundos`);
