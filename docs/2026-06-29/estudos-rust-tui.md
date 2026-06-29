# Estudo — Rust aplicado à TUI

Data: 29 de junho de 2026

Projeto: **He4rt Linux Tutor**

---

## **Visão geral do dia**

- Hoje fiz **dois estudos** complementares, ambos conectando Rust com a arquitetura do projeto.
- **Bloco 1 (`ex1` → `ex9`)** → fundamentos de Rust aplicados à **ideia** de UI, sem biblioteca gráfica.
- **Bloco 2 (`ex01` → `ex09`)** → começo da **interface real** no terminal, usando `crossterm` + `ratatui`.
- A progressão foi do conceito para a prática: primeiro modelei os painéis como dados, depois desenhei eles numa TUI de verdade.

---

## **Bloco 1 — Fundamentos de Rust aplicados à ideia de UI (`ex1` → `ex9`)**

Aqui o foco foi **modelar** os blocos da arquitetura como dados e funções em Rust puro (só `std`), sem desenhar nada na tela ainda.

| Arquivo | O que treinei | Evolução |
| ------- | ------------- | -------- |
| `ex1` | Criei uma função `mostrar_terminal(comando: &str)` para imprimir algo parecido com um Terminal Pane. | Primeiro contato com função e parâmetro `&str`. |
| `ex2` | Criei a struct `TerminalPane` com `comando: String`. | Comecei a representar um painel como dado estruturado. |
| `ex3` | Criei `render_terminal(&TerminalPane)`. | Separei dados da exibição. Aqui comecei a usar referência `&`. |
| `ex4` | Criei `TutorialPane` com título, instrução, dica e `mostrar_dica`. | Comecei a simular o painel de tutorial. |
| `ex5` | Criei `GlossaryPopup` com termo, explicação e `aberto`. | Comecei a simular o popup condicional. |
| `ex6` | Fiz repetição até o usuário digitar `pwd`. | Entrei na lógica de validação simples. |
| `ex7` | Criei histórico com `Vec<String>` e `for`. | Treinei o armazenamento de vários comandos. |
| `ex8` | Fiz uma mini lição interativa com resposta `pwd` e opção de dica. | Juntei loop, validação e dica. |
| `ex9` | Juntei Terminal Pane, Tutorial Pane, Glossary Popup, Status e menu de ajuda. | Primeiro mini app completo, sem Ratatui. |

**Destaques de Rust que apareceram aqui:**

- `struct` para representar cada painel como dado (`TerminalPane`, `TutorialPane`, `GlossaryPopup`).
- `enum StatusPasso` (`Aguardando`, `Concluido`, `Falhou`) — exatamente os estados que vi no Tutorial Pane.
- Referências (`&`) para separar **dados** da **renderização**.
- `match` para rotear opções do menu de ajuda (1 / 2 / 3 + curinga `_`).
- `loop` + `break` para o ciclo de tentativa do aluno.
- `.clone()` para reaproveitar o comando sem mover o valor.

O `ex9` já é, na prática, um **embrião do Lesson State**: um conjunto de dados (painéis + status) que muda conforme o aluno age e é re-renderizado a cada passo.

---

## **Bloco 2 — Interface real no terminal com bibliotecas (`ex01` → `ex09`)**

Aqui troquei o `println!` por uma TUI de verdade, lendo teclas com `crossterm` e desenhando painéis com `ratatui`.

| Arquivo | O que treinei | Evolução |
| ------- | ------------- | -------- |
| `ex01` | Usei `crossterm`, `raw_mode`, `event::read`, `KeyCode` e `KeyEventKind::Press`. | Comecei a ler teclas de verdade no terminal (loop até apertar `q`). |
| `ex02` | Criei `input: String`, digitei caracteres, apaguei com Backspace e enviei com Enter. | Simulei o input de um Terminal Pane. |
| `ex03` | Criei a primeira tela com Ratatui usando `Paragraph`, `Block` e `Borders`. | Primeiro contato com desenho de interface TUI. |
| `ex04` | Dividi a tela em duas colunas com `Layout`: Terminal Pane e Tutorial Pane. | Comecei a montar a estrutura visual dos painéis. |
| `ex05` | Juntei Ratatui + Crossterm com input, histórico e dois painéis. | Primeiro exercício interativo com UI desenhada. |
| `ex06` | Adicionei `StatusPasso` e validação de `pwd`. | Simulei o começo do Validator Engine. |
| `ex07` | Adicionei `mostrar_dica` e atalho `h`. | Tutorial Pane ficou mais dinâmico. |
| `ex08` | Adicionei foco com `Tab`, `Painel`, borda colorida e Glossary Popup centralizado. | Entrei em foco de UI e popup real. |
| `ex09` | Separei histórico embaixo do Tutorial Pane, mensagem no rodapé, bloqueei ações após concluir e mantive popup/dica/foco. | Ficou como uma mini lição TUI bem organizada. |

**Destaques de Rust + TUI que apareceram aqui:**

- **Padrão de estado central**: a `struct App` guarda tudo (`input`, `historico`, `status`, `mostrar_dica`, `foco`, `glossario_aberto`, `mensagem`, `sair`).
- **Event loop**: `event::poll` + `event::read` + `match tecla.code` — o coração da interação.
- `crossterm` para entrada bruta (`enable_raw_mode`, `KeyCode`, `KeyEventKind::Press`).
- `ratatui` para saída: `Frame`, `Layout`, `Constraint`, `Direction`, `Paragraph`, `Block`, `Borders`, `Wrap`.
- **Renderização derivada do estado**: a função `desenhar` lê o `App` e monta a tela — UI é função do estado.
- `#[derive(PartialEq)]` no `enum Painel` para comparar o foco atual.
- Popup centralizado com layout aninhado + `Clear` (o `area_centralizada`).
- `matches!(app.status, StatusPasso::Concluido)` para bloquear ações depois de concluir.
- Estilo com `Style`/`Color` (borda amarela no painel em foco, ciano no glossário).

Esse bloco mostrou na prática como **Terminal Pane, Tutorial Pane e Glossary Popup** se comportam juntos, e o `match tecla.code` é exatamente o ponto onde, no futuro, o **Event Bus** vai entrar.

---

## **Conceitos que fixei**

- `struct` e `enum` para modelar painéis e estados da lição.
- `enum StatusPasso` = os estados `Idle`/`Validating`/`Passed`/`Failed` que estudei na arquitetura.
- Referências `&` e `&mut`, separando dados de renderização e permitindo mutação controlada.
- `match` (com `_` e `matches!`) para rotear teclas, comandos e opções.
- `loop` / `while !app.sair` como ciclo principal de um app interativo.
- O padrão **"estado central + render do estado"** (a `struct App` + a função `desenhar`).
- Event loop com `crossterm`: `poll`, `read`, `KeyCode`, `KeyEventKind::Press`.
- Desenho de TUI com `ratatui`: `Layout`/`Constraint`/`Direction`, `Paragraph`, `Block`, `Borders`, `Wrap`, `Clear`.
- `#[derive(PartialEq)]` para comparar variantes de enum (foco entre painéis).
- Popup centralizado com layout aninhado e `Clear` para limpar a área.

---

## 🧾 **Resumo final**

- Saí da modelagem conceitual (Bloco 1) e cheguei a uma **mini lição TUI funcional** (Bloco 2).
- Entendi na prática que a UI do projeto é **função do estado**: muda o `App`, re-desenha a tela.
- Já conectei vários blocos da arquitetura ao código: Terminal Pane (input + histórico), Tutorial Pane (objetivo + status + dica), Glossary Popup (abrir/fechar com foco) e o início do Validator Engine (validação de `pwd`).
- O `enum StatusPasso` e a `struct App` são, na prática, o esqueleto do que será o **Lesson State**.
- Ficou claro onde o **Event Bus** vai se encaixar: no ponto em que hoje eu trato as teclas direto no `match`.

---

## 🚧 **Próximos passos**

- **Event Bus** → extrair o tratamento de teclas/comandos para **eventos** (criar um `enum Evento` e transportá-lo, evoluindo para canais/`channels` em Rust).
- **Validator Engine** → transformar o `if comando == "pwd"` em **asserts** de verdade (avaliar `command`, `exit`, `fs` e `script`).
- **Lesson State** → consolidar a `struct App` num **estado de lição** explícito, com passo atual, tentativas e progresso.
- **Content Load (TOML + schema)** → tirar a lição "hardcoded" do código e **carregar de um arquivo TOML** (crates `serde` / `toml`), com validação.
- **Progress Store** → salvar e carregar o progresso do aluno em disco.
- **Runtime do Shell** → conectar a um `bash` real via `portable-pty`, com sandbox e leitura de eventos via FIFO.
- Continuar **conectando cada exercício ao bloco correspondente** da arquitetura, para fortalecer linguagem e desenho do sistema ao mesmo tempo.
