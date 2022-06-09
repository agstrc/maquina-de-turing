# Rust Turing Machine

Uma Máquina de Turing implementada em Rust. Essa máquina foi implementada ara uma trabalho
da matéria **Teoria da Computação** no curso de graduação em Ciência da Computação.

A implementação contida nesse repositório possui dois objetivos:

- Aderir aos requisitos da definição do trabalho
- Providenciar aprendizado na linguagem Rust

## Definição do Trabalho

1. Utilizando definição e exemplos de MTs vistos em aula, o grupo deverá implementar um
algoritmo que execute funcionamento de MTs de maneira genérica, isto é, o algoritmo não deve ser
construído para uma MT específico.

2. As entradas serão: a) sétupla que define a MT e b) cadeias pertencentes ou não à linguagem da
MT dada. Assim, o algoritmo deve suportar a entrada de diferentes MTs e de diferentes cadeias para
teste de cada MT. A sétupla deve ser armazenada em um arquivo e este será entrada para o
algoritmo e as cadeias serão fornecidas durante a execução do algoritmo.

3. A saída deve conter: a) a sétupla fornecida na entrada e para cada cadeia testada b) a cadeia
testada, c) resultado compatível com o problema fornecido: se for reconhecimento de cadeia,
ACEITA ou REJEITA, se for cálculo de função, o resultado do cálculo (lembrando que neste caso, o
resultado deve estar escrito na fita) e d) passo a passo do funcionamento da MT na cadeia de
entrada, mostrando a fita a cada passo da execução.

4. O grupo deverá preparar apresentação contendo a) descrição geral do algoritmo, b) estruturas de
dados utilizadas para representar a sétupla, c) MTs testadas durante a implementação e d) o que é
apresentado como saída do algoritmo.

## Executando o projeto

Para executar o projeto, é necessário uma instalação funcional de uma toolchain `Rust` com o
gerenciador de pacotes `Cargo`.  
O processo de execução consiste em simplesmente executar o programa, via `cargo run` ou diretamente
por meio do executável, informando como parâmetro o *path* para um arquivo JSON que contenha a
definição da sétupla da Máquina de Turing. O formato do arquivo JSON segue o exemplo à seguir:

```json
{
  "alphabet": ["0", "1", "X", "Y", "B"],
  "blank_symbol": "B",
  "input_symbols": ["0", "1"],
  "states": ["q0", "q1", "q2", "q3", "q4"],
  "initial_state": "q0",
  "final_states":["q3"],
  "transitions":[
    {
      "from_state":"q0",
      "read_symbol":"0",
      "write_symbol":"X",
      "move_to":"R",
      "next_state":"q1"
    },
    // ...
  ]
}
```

Esse exemplo espera que os dados sigam a definição formal de uma Máquina de Turing, como
definido na [página de Wikipedia](https://en.wikipedia.org/wiki/Turing_machine#Formal_definition).

## Screenshots

![Menu da fita](https://i.imgur.com/E8l3Ukp.png)
![Processamento da fita](https://i.imgur.com/8tWaI10.png)
