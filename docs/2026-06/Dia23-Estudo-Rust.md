# Estudo — Primeiro Contato com Rust (Exercícios Práticos)

Data: 23 de junho de 2026

Projeto: **He4rt Linux Tutor** 

---

## **Visão geral do dia**

- Hoje foi meu **primeiro contato com Rust**, a linguagem do projeto.
- Para sair da teoria, fiz **8 exercícios práticos** focados no básico da linguagem.
- O objetivo foi pegar familiaridade com a sintaxe, entrada/saída, tipos, controle de fluxo e funções — pilares que vou reusar ao implementar os blocos da arquitetura.


## 1️⃣ **ex1 — Variáveis e `println!`**

- Aprendi a declarar variáveis com `let` (`nome`, `idade`).
- Vi que strings ficam entre aspas (`"Lucas"`) e números inteiros são escritos direto (`20`).
- Pratiquei a macro `println!` usando `{}` como espaço reservado, passando os valores na ordem.
- Fixei que, por padrão, as variáveis em Rust são **imutáveis**.

```rust
let nome = "Lucas";
let idade = 23;
println!("Meu nome é {} e eu tenho {} anos.", nome, idade);
```


## 2️⃣ **ex2 — Operações aritméticas**

- Pratiquei os operadores básicos: `+`, `-`, `*` e `/`.
- Guardei cada resultado em uma variável (`soma`, `sub`, `mult`, `div`).
- Aprendi a **interpolação inline** com o nome da variável dentro das chaves (`{a}`, `{soma}`).
- Percebi que a divisão entre inteiros (`10 / 5`) resulta em inteiro.


## 3️⃣ **ex3 — Entrada do usuário + condicional**

- Usei `std::io` para ler entrada do teclado com `io::stdin().read_line(&mut idade)`.
- Tive meu primeiro contato com o **`&mut`** (referência mutável), passando a variável para ser preenchida.
- Aprendi a converter texto em número com `.trim().parse()`.
- Usei `.expect("...")` para tratar erro de leitura e de conversão.
- Apliquei `if / else` para decidir entre maior e menor de idade.


## 4️⃣ **ex4 — Funções**

- Criei minhas primeiras funções: `somar(a, b)` e `leitura()`.
- Aprendi a declarar parâmetros com tipo (`a: i32`) e o tipo de retorno com `-> i32`.
- Entendi o **retorno implícito**: a última expressão sem `;` é o valor retornado.
- Reaproveitei a função `leitura()` para ler dois números, deixando o `main` mais limpo.
- Comecei a perceber o valor de **separar responsabilidades** em funções.


## 5️⃣ **ex5 — Lendo e usando uma string**

- Reforcei o fluxo de leitura: `String::new()` → `read_line(&mut nome)` → `.trim()`.
- Tive contato com **shadowing**: reusei o nome `nome` criando uma nova versão já tratada (`let nome = nome.trim();`).
- Pratiquei a interpolação inline em uma saudação (`"Olá, {nome}!"`).


## 6️⃣ **ex6 — Calculadora com escolha de operação**

- Montei uma calculadora que lê dois números e uma operação.
- Usei `f64` para trabalhar com números decimais (`.parse()` para `f64`).
- Apliquei uma cadeia de `if / else if / else` comparando a string da operação (`"+"`, `"-"`, `"*"`, `"/"`).
- Tratei o caso de operação inválida no `else` final.


## 7️⃣ **ex7 — `match` com comandos**

- Aprendi a usar a expressão **`match`** para tratar diferentes comandos de texto.
- Mapeei `"iniciar"`, `"pausar"` e `"sair"` para mensagens específicas.
- Usei o curinga `_` para capturar qualquer comando desconhecido.
- 💡 Esse exercício conversa diretamente com a arquitetura: é exatamente o tipo de roteamento que o **Validator Engine** e o **Event Bus** vão fazer ao interpretar comandos e eventos.


## 8️⃣ **ex8 — Vetores e laços**

- Aprendi a criar uma lista com `Vec<String>` e `Vec::new()`.
- Usei `for i in 1..=5` para repetir a leitura cinco vezes (range inclusivo).
- Adicionei itens à lista com `.push(nome)`.
- Percorri a lista usando uma **referência** (`for nome in &nomes`) para não consumir o vetor.
- Reforcei o conceito de **ownership/borrowing** ao iterar sem mover os dados.


## 💡 **Conceitos que fixei**

- Variáveis com `let` e imutabilidade por padrão.
- Tipos básicos: `i32`, `f64`, `String`, `&str` e `Vec<T>`.
- Macro `println!` com placeholders `{}` e interpolação inline `{var}`.
- Entrada de dados com `io::stdin().read_line(&mut ...)`.
- Conversão de texto em número com `.trim().parse()`.
- Tratamento de erro inicial com `.expect()` e `.unwrap()`.
- Controle de fluxo: `if / else`, `if / else if` e `match` (com curinga `_`).
- Funções com parâmetros tipados, tipo de retorno (`-> T`) e retorno implícito.
- **Shadowing** (reusar o nome de uma variável criando uma nova).
- Primeiro contato com **referências** (`&mut`, `&`) e a ideia de ownership/borrowing.
- Laços com `for` e ranges (`1..=5`), além de manipular `Vec` com `.push()`.


## 🧾 **Resumo final**

- Saí da teoria e tive meu primeiro contato **prático** com Rust.
- Cobri os fundamentos que mais vou usar nos blocos do projeto: entrada/saída, tipos, conversões, condicionais, `match`, funções, vetores e laços.
- Já vi pontes claras com a arquitetura — o `match` do ex7 é a base do roteamento de comandos, e o tratamento de entrada/parse vai aparecer em quase todos os blocos.


## 🚧 **Próximos passos**

- **Elaborar exercícios práticos para cada parte da arquitetura**, para fixar tanto a linguagem quanto cada bloco em si.
- **Terminal Pane** → exercícios envolvendo entrada/saída de bytes, parsing de sequências ANSI e roteamento de teclas.
- **Tutorial Pane** → exercícios modelando os estados `Idle`, `Validating`, `Passed` e `Failed` (máquina de estados em Rust).
- **Glossary Popup** → exercícios de abrir/fechar popup, controle de foco e leitura de termos vindos do conteúdo.
- **Event Bus** → exercícios criando e transportando eventos (enums, canais/`channels` em Rust).
- **Validator Engine** → exercícios escrevendo asserts e avaliando `command`, `exit`, `fs` e `script`.
- **Lesson State** → exercícios com `struct` e `enum` para representar o estado vivo da lição.
- **Content Load (TOML + schema)** → exercícios de leitura e validação de arquivos TOML (ex.: crate `serde` / `toml`).
- **Progress Store** → exercícios de salvar e carregar progresso em disco.
- **Runtime do Shell** → exercícios com PTY (`portable-pty`), sandbox e leitura de eventos via FIFO.
- Conectar cada exercício ao bloco correspondente para ter um **entendimento mais sólido da linguagem e da arquitetura ao mesmo tempo**.
