#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <math.h>
#include <time.h>

int main() {
    // Valor máximo
    const int maxValue = 100000000;

    // Inicializando o array de números na heap
    bool *numbers = malloc((maxValue + 1) * sizeof(bool));
    if (numbers == NULL) {
        fprintf(stderr, "Falha na alocação de memória\n");
        return 1;
    }
    for (int i = 0; i <= maxValue; i++) {
        numbers[i] = true;
    }
    numbers[0] = numbers[1] = false;

    // Medindo o tempo de execução
    clock_t startTime = clock();
    for (int i = 2; i <= (int)sqrt(maxValue); i++) {
        if (numbers[i]) {
            for (int j = i * i; j <= maxValue; j += i) {
                numbers[j] = false;
            }
        }
    }
    clock_t endTime = clock();

    // Coletando os números primos
    int primeCount = 0;
    for (int i = 2; i <= maxValue; i++) {
        if (numbers[i]) {
            primeCount++;
        }
    }

    // Liberando a memória alocada
    free(numbers);

    // Calculando o tempo de execução em milissegundos
    double executionTimeMs = (double)(endTime - startTime) / CLOCKS_PER_SEC * 1000;

    // Imprimindo a quantidade de números primos encontrados e o tempo de execução
    printf("Quantidade de números primos até %d: %d\n", maxValue, primeCount);
    printf("Tempo de execução: %.2f milissegundos\n", executionTimeMs);

    return 0;
}
