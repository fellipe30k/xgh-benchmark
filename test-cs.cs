using System;
using System.Collections.Generic;
using System.Diagnostics;

class Program
{
    static void Main()
    {
        // Valor máximo
        const int maxValue = 1000000;

        // Inicializando a lista de números
        bool[] numbers = new bool[maxValue + 1];
        Array.Fill(numbers, true);
        numbers[0] = numbers[1] = false;

        // Medindo o tempo de execução
        Stopwatch stopwatch = Stopwatch.StartNew();
        for (int i = 2; i <= Math.Sqrt(maxValue); i++)
        {
            if (numbers[i])
            {
                for (int j = i * i; j <= maxValue; j += i)
                {
                    numbers[j] = false;
                }
            }
        }
        stopwatch.Stop();

        // Coletando os números primos
        int primeCount = 0;
        for (int i = 2; i <= maxValue; i++)
        {
            if (numbers[i])
            {
                primeCount++;
            }
        }

        // Calculando o tempo de execução em milissegundos
        long executionTimeMs = stopwatch.ElapsedMilliseconds;

        // Imprimindo a quantidade de números primos encontrados e o tempo de execução
        Console.WriteLine($"Quantidade de números primos até {maxValue}: {primeCount}");
        Console.WriteLine($"Tempo de execução: {executionTimeMs} milissegundos");
    }
}
