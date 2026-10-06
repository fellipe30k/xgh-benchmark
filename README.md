
# 🚀 xgh-benchmark

Um comparativo simples de desempenho entre diferentes linguagens de programação calculando números primos até 100.000.000.

---

## 🛠️ Como compilar e executar os testes

### 🖥️ **C**
```bash
gcc -O3 -o test-c test-c.c
./test-c
```

### ⚙️ **Rust**
```bash
rustc -C opt-level=3 test-rs.rs
./test-rs
```

### 🖥️ **C++**
```bash
g++ -o test-cpp test-cpp.cpp
./test-cpp
```

### 🌟 **Go**
```bash
go build test-go.go
./test-go
```

### ☕ **Java**
```bash
cp test-java.java Primes.java
java Primes.java
```

### 🟨 **JavaScript**
```bash
node test-js.js
```

### 🐍 **Python**
```bash
python3 test-py.py
```

### 💎 **Ruby**
```bash
ruby --yjit test-rb.rb
```

---

## 💻 Máquina de teste

| Componente | Especificação |
|---|---|
| **CPU** | AMD Ryzen 5 3600 (6 núcleos / 12 threads, até 4,2 GHz, 32 MiB L3) |
| **RAM** | 32 GB |
| **SO** | Debian GNU/Linux 12 (bookworm), kernel 6.12.95 |
| **C / C++** | GCC 12.2.0 |
| **Rust** | rustc 1.92.0 |
| **Go** | go 1.24.5 |
| **Java** | OpenJDK 17.0.19 |
| **JavaScript** | Node.js 21.7.3 |
| **Ruby** | Ruby 4.0.6 (YJIT) |
| **Python** | CPython 3.11.2 |

---

## 📊 Resultados de desempenho

Cada linguagem foi executada 3 vezes; o valor mostrado é a mediana. O tempo mede apenas o crivo (sem alocação e sem contagem).

| 🏆 **Posição** | 💻 **Linguagem** | 🔢 **Quantidade de primos** | ⏱️ **Tempo de execução** |
|----------------|-----------------|-----------------------------|--------------------------|
| 🥇 **1º**      | Go              | 5.761.455                  | **572,25 ms**            |
| 🥈 **2º**      | Rust            | 5.761.455                  | **591,10 ms**            |
| 🥉 **3º**      | Java            | 5.761.455                  | **592 ms**               |
| 4º             | C               | 5.761.455                  | **593,95 ms**            |
| 5º             | JavaScript      | 5.761.455                  | **1.146 ms**             |
| 6º             | C++             | 5.761.455                  | **6.326 ms**             |
| 7º             | Ruby            | 5.761.455                  | **6.583,97 ms**          |
| 8º             | Python          | 5.761.455                  | **17.696,72 ms**         |

---

## 📝 Observações
- ⚖️ Go, Rust, Java e C ficaram em **empate técnico** (diferença menor que 4%, dentro da variação entre execuções). Nesse ponto o gargalo é o acesso à memória (cache misses), não a linguagem.
- 🔧 C saiu de ~1.100 ms para ~590 ms: antes era compilado **sem otimização** (`-O3` agora) e media tempo de CPU com `clock()` em vez de tempo real.
- 🟨 JavaScript agora conclui sem erro de memória no Node.js 21 e fica em cerca do dobro do tempo dos compilados.
- 🧐 C++ ainda é compilado sem `-O` e usa `std::vector<bool>` (1 bit por número), o que explica o tempo alto. (Refactor??)
- 💎 Ruby caiu de ~13,5 s para ~6,6 s trocando `Range#step` com bloco por laços `while` e rodando com `--yjit`.
- 🐢 Python segue o mais lento, refletindo o custo do interpretador em laços puros.

---

✨ **Contribuições são bem-vindas!** Caso deseje adicionar mais linguagens ou otimizações, sinta-se à vontade para fazer um PR.  
