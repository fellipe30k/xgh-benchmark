#include <iostream>
#include <vector>
#include <cmath>
#include <chrono>

int main() {
    // Valor máximo
    const int maxValue = 100000000;

    // Inicializando o vetor de números
    std::vector<bool> numbers(maxValue + 1, true);
    numbers[0] = numbers[1] = false;

    // Medindo o tempo de execução
    auto startTime = std::chrono::high_resolution_clock::now();
    for (int i = 2; i <= static_cast<int>(std::sqrt(maxValue)); ++i) {
        if (numbers[i]) {
            for (int j = i * i; j <= maxValue; j += i) {
                numbers[j] = false;
            }
        }
    }
    auto endTime = std::chrono::high_resolution_clock::now();

    // Coletando os números primos
    int primeCount = 0;
    for (int i = 2; i <= maxValue; ++i) {
        if (numbers[i]) {
            ++primeCount;
        }
    }

    // Calculando o tempo de execução em milissegundos
    auto duration = std::chrono::duration_cast<std::chrono::milliseconds>(endTime - startTime).count();

    // Imprimindo a quantidade de números primos encontrados e o tempo de execução
    std::cout << "Quantidade de números primos até " << maxValue << ": " << primeCount << std::endl;
    std::cout << "Tempo de execução: " << duration << " milissegundos" << std::endl;

    return 0;
}
