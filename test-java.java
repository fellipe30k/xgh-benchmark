public class Primes {

    public static void main(String[] args) {
        // Valor máximo
        final int maxValue = 100000000;

        // Inicializando o array de números
        boolean[] numbers = new boolean[maxValue + 1];
        for (int i = 2; i <= maxValue; i++) {
            numbers[i] = true;
        }
        numbers[0] = numbers[1] = false;

        // Medindo o tempo de execução
        long startTime = System.currentTimeMillis();
        for (int i = 2; i <= Math.sqrt(maxValue); i++) {
            if (numbers[i]) {
                for (int j = i * i; j <= maxValue; j += i) {
                    numbers[j] = false;
                }
            }
        }
        long endTime = System.currentTimeMillis();

        // Coletando os números primos
        int primeCount = 0;
        for (int i = 2; i <= maxValue; i++) {
            if (numbers[i]) {
                primeCount++;
            }
        }

        // Calculando o tempo de execução em milissegundos
        long executionTimeMs = endTime - startTime;

        // Imprimindo a quantidade de números primos encontrados e o tempo de execução
        System.out.println("Quantidade de números primos até " + maxValue + ": " + primeCount);
        System.out.println("Tempo de execução: " + executionTimeMs + " milissegundos");
    }
}
