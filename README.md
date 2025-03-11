
# 🚀 xgh-benchmark

Um comparativo simples de desempenho entre diferentes linguagens de programação calculando números primos até 100.000.000.

---

## 🛠️ Como compilar e executar os testes

### 🖥️ **C**
```bash
gcc -o test-c test-c.c -lm
./test-c
```

### ⚙️ **Rust**
```bash
rustc -O test-rs.rs
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

---

## 📊 Resultados de desempenho

| 🏆 **Posição** | 💻 **Linguagem** | 🔢 **Quantidade de primos** | ⏱️ **Tempo de execução**       |
|----------------|-----------------|-----------------------------|--------------------------------|
| 🥇 **1º**      | Rust            | 5.761.455                  | **765 ms**                 |
| 🥈 **2º**      | Go              | 5.761.455                  | **936 ms**                   |
| 🥉 **3º**      | Java            | 5.761.455                  | **950 ms**                   |
| 4º             | C               | 5.761.455                  | **1.070,51 ms**              |
| 5º             | C++             | 5.761.455                  | **9.110 ms**                 |
| 6º             | Ruby            | 5.761.455                  | **13.727,47 ms**             |
| 7º             | Python          | 5.761.455                  | **19.760,35 ms**             |
| ❌ **Erro**    | JavaScript      | -                          | **Heap out of memory** 🛑    |

---

## 📝 Observações
- JavaScript enfrentou um problema de limite de memória no cálculo de números primos.
- ⚡ Go se destacou como a linguagem mais rápida neste benchmark.
- 💡 Rust e C++ apresentaram tempos mais altos comparados a C e Go.

---

✨ **Contribuições são bem-vindas!** Caso deseje adicionar mais linguagens ou otimizações, sinta-se à vontade para fazer um PR.  
