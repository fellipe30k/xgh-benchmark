require 'benchmark'

# Valor máximo
max_value = 1_000_000_00

# Inicializando a lista de números
numbers = [true] * (max_value + 1)
numbers[0] = numbers[1] = false

# Medindo o tempo de execução com a gem benchmark
time = Benchmark.measure do
  (2..Math.sqrt(max_value)).each do |i|
    if numbers[i]
      (i*i..max_value).step(i) do |j|
        numbers[j] = false
      end
    end
  end
end

# Coletando os números primos
primes = numbers.each_index.select { |i| numbers[i] }

# Imprimindo a quantidade de números primos encontrados
puts "Quantidade de números primos até #{max_value}: #{primes.length}"

# Imprimindo o tempo de execução em milissegundos
puts "Tempo de execução: #{time.real * 1000} milissegundos"
