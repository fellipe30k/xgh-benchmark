# Valor máximo
max_value = 100_000_000

# Marca os múltiplos de prime como compostos. Fica num método para que o
# YJIT (ruby --yjit) consiga compilá-lo, já que é chamado ~1.200 vezes.
# Laços while evitam a criação de Ranges/Enumerators e a chamada de
# um bloco por iteração, que dominavam o tempo da versão com step.
def mark_multiples(numbers, prime, max_value)
  j = prime * prime
  while j <= max_value
    numbers[j] = false
    j += prime
  end
end

# Inicializando a lista de números
numbers = Array.new(max_value + 1, true)
numbers[0] = numbers[1] = false

# Medindo o tempo de execução com relógio monotônico
limit = Integer.sqrt(max_value)
start_time = Process.clock_gettime(Process::CLOCK_MONOTONIC, :float_millisecond)

i = 2
while i <= limit
  mark_multiples(numbers, i, max_value) if numbers[i]
  i += 1
end

elapsed = Process.clock_gettime(Process::CLOCK_MONOTONIC, :float_millisecond) - start_time

# Contando os números primos (sem criar um array com todos eles)
prime_count = numbers.count(true)

# Imprimindo a quantidade de números primos encontrados e o tempo de execução
puts "Quantidade de números primos até #{max_value}: #{prime_count}"
puts format('Tempo de execução: %.2f milissegundos', elapsed)
