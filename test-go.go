package main

import (
	"fmt"
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
	for i := 2; i*i <= maxValue; i++ {
		if numbers[i] {
			// Fatiar a partir de i*i permite ao compilador eliminar
			// a checagem de limites no laço interno
			multiples := numbers[i*i:]
			for j := 0; j < len(multiples); j += i {
				multiples[j] = false
			}
		}
	}
	elapsed := time.Since(startTime)

	// Contando os números primos (sem alocar uma lista com todos eles)
	primeCount := 0
	for _, isPrime := range numbers {
		if isPrime {
			primeCount++
		}
	}

	// Imprimindo a quantidade de números primos encontrados e o tempo de execução
	fmt.Printf("Quantidade de números primos até %d: %d\n", maxValue, primeCount)
	fmt.Printf("Tempo de execução: %.2f milissegundos\n", float64(elapsed.Nanoseconds())/1e6)
}
