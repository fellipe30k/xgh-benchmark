#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

// Tempo de relógio (wall clock) em milissegundos, igual às outras linguagens.
// clock() mede tempo de CPU, o que não é comparável com Instant/time.Now.
static double now_ms(void) {
    struct timespec ts;
    clock_gettime(CLOCK_MONOTONIC, &ts);
    return ts.tv_sec * 1e3 + ts.tv_nsec / 1e6;
}

int main(void) {
    // Valor máximo
    const size_t maxValue = 100000000;

    // Inicializando o array de números na heap (1 byte por número)
    unsigned char *numbers = malloc(maxValue + 1);
    if (numbers == NULL) {
        fprintf(stderr, "Falha na alocação de memória\n");
        return 1;
    }
    memset(numbers, 1, maxValue + 1);
    numbers[0] = numbers[1] = 0;

    // Medindo o tempo de execução
    double startTime = now_ms();
    for (size_t i = 2; i * i <= maxValue; i++) {
        if (numbers[i]) {
            for (size_t j = i * i; j <= maxValue; j += i) {
                numbers[j] = 0;
            }
        }
    }
    double endTime = now_ms();

    // Contando os números primos (soma sem desvio, vetorizável pelo compilador)
    size_t primeCount = 0;
    for (size_t i = 0; i <= maxValue; i++) {
        primeCount += numbers[i];
    }

    // Liberando a memória alocada
    free(numbers);

    // Imprimindo a quantidade de números primos encontrados e o tempo de execução
    printf("Quantidade de números primos até %zu: %zu\n", maxValue, primeCount);
    printf("Tempo de execução: %.2f milissegundos\n", endTime - startTime);

    return 0;
}
