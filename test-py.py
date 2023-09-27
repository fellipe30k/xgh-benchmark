import time
import math

# Valor máximo
max_value = 1_000_000_00

# Inicializando a lista de números
numbers = [True] * (max_value + 1)
numbers[0] = numbers[1] = False

# Medindo o tempo de execução
start_time = time.time()
for i in range(2, int(math.sqrt(max_value)) + 1):
    if numbers[i]:
        for j in range(i * i, max_value + 1, i):
            numbers[j] = False
end_time = time.time()

# Coletando os números primos
primes = [i for i, is_prime in enumerate(numbers) if is_prime]

# Calculando o tempo de execução em milissegundos
execution_time_ms = (end_time - start_time) * 1000

print(f"Quantidade de números primos até {max_value}: {len(primes)}")
print(f"Tempo de execução: {execution_time_ms} milissegundos")