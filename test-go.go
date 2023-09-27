package main

import (
	"fmt"
	"math"
	"time"
)

func main() {
	// Valor máximo
	const maxValue = 100000000

	// Inicializando a fatia de números
	numbers := make([]bool, maxValue+1)
	for i := range numbers {
		numbers[i] = true
	}
	numbers[0], numbers[1] = false, false

	// Medindo o tempo de execução
	startTime := time.Now()
	for i := 2; i <= int(math.Sqrt(maxValue)); i++ {
		if numbers[i] {
			for j := i * i; j <= maxValue; j += i {
				numbers[j] = false
			}
		}
	}
	endTime := time.Now()

	// Coletando os números primos
	var primes []int
	for i, isPrime := range numbers {
		if isPrime {
			primes = append(primes, i)
		}
	}

	// Calculando o tempo de execução em milissegundos
	executionTimeMs := endTime.Sub(startTime).Milliseconds()

	// Imprimindo a quantidade de números primos encontrados e o tempo de execução
	fmt.Printf("Quantidade de números primos até %d: %d\n", maxValue, len(primes))
	fmt.Printf("Tempo de execução: %d milissegundos\n", executionTimeMs)
}
