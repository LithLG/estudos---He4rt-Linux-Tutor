# Estudo — Runtime do Shell

Data: 22 de junho de 2026

Projeto: **He4rt Linux Tutor** 

---

## 🐚 **Runtime do Shell — o ambiente real de comandos**

- Estudei que o Runtime do Shell é a camada que **cria e controla o ambiente real** onde o aluno digita comandos Linux.
- Entendi que o objetivo dele é garantir três coisas: **isolamento**, **previsibilidade** e **validação segura**.
- Fixei que, no Gist, essa camada roda um **bash real**, dentro de um **sandbox isolado**, usando um **PTY real**.
- Compreendi que os comandos são **limitados por lição** e que existe um **canal paralelo** dedicado à validação.


## 🧱 **Componentes principais**

- Aprendi que o Runtime do Shell se divide em três caixas principais:
  - **PTY Host** → `portable-pty`
  - **Sandbox** → `bwrap` + PATH
  - **Assert Channel** → `bwrap exec`
- Entendi que cada caixa tem uma responsabilidade distinta: criar o terminal real, isolar o ambiente e validar por fora sem poluir o terminal do aluno.


## 🔌 **PTY Host — criando e gerenciando o PTY**

- Entendi que o PTY Host é a parte que **cria e gerencia o PTY** (pseudo-terminal).
- Fixei que o PTY faz o bash **acreditar que está conectado a um terminal real**.
- Aprendi que o `portable-pty` é a biblioteca usada para criar esse terminal dentro da aplicação.
- Vi que a decisão técnica do Gist é usar `portable-pty` justamente porque o objetivo é um **shell real, não uma emulação**.


## 🔁 **Fluxo do PTY Host**

- O fluxo é: `Terminal Pane → (envia bytes) → PTY Host → (entrega ao bash) → bash → (gera saída) → PTY Host → (devolve bytes) → Terminal Pane`.
- Exemplo: o aluno digita `pwd`, o Terminal Pane envia `p`, `w`, `d`, `Enter`, o PTY Host entrega ao bash, o bash responde `/home/aluno` e o PTY Host devolve essa saída para a tela.
- Fixei as responsabilidades do PTY Host: criar o PTY, iniciar o bash, enviar os bytes digitados, receber stdout/stderr, informar o tamanho do terminal, lidar com resize e encerrar o shell ao fim da lição.
- Reforcei que o shell usado no projeto é o **bash**, que é quem interpreta comandos como `pwd`, `ls`, `cd projeto`, `mkdir aula` e `cat arquivo.txt`.


## 📦 **Sandbox — área isolada de prática**

- Entendi que o Sandbox é uma **área isolada** onde o aluno pode digitar comandos reais sem mexer livremente no sistema da máquina.
- Aprendi que a decisão do Gist é usar **`bwrap` (Bubblewrap)** para criar namespace e isolamento de PATH/mount **sem o peso de um container completo**.
- Fixei que o `bwrap` cria um ambiente isolado para cada lição.


## 🫧 **Fluxo do bwrap (ciclo de vida do ambiente)**

- O `bwrap` cria o ambiente isolado.
- O sistema cria os arquivos iniciais da lição (seed).
- É tirado um **snapshot** do estado limpo.
- O bash nasce dentro desse ambiente.
- O aluno interage.
- O **reset** restaura o snapshot.
- O fim da lição destrói o ambiente.


## 🛣️ **PATH e PATH restrito**

- Aprendi que, no Linux, o `PATH` é a variável que diz **onde o shell procura comandos**.
- Exemplo: ao digitar `ls`, o bash procura um executável chamado `ls` nos diretórios do PATH.
- Entendi que **PATH restrito** significa que o sistema controla quais comandos ficam disponíveis dentro da lição.
- Fixei o uso do `sandbox_binaries`, ex.: `["pwd", "ls", "cd", "mkdir", "cat"]` — só esses comandos ficam acessíveis.


## ✅ **Allowlist — a lista do que é permitido**

- Entendi que **allowlist** é a lista do que é permitido, definida pelo `sandbox_binaries` no schema da lição.
- Fixei que a decisão do Gist é usar **bash com PATH restrito** para manter uma allowlist **por lição**.
- Aprendi o porquê de restringir o PATH: **evitar que o aluno use comandos fora da proposta** da lição, mantendo tudo previsível.


## 🧩 **Builtins do bash**

- Aprendi que alguns comandos **não são programas separados** — eles já existem dentro do próprio bash.
- Esses comandos são chamados de **builtins**.
- Exemplos: `cd`, `echo`, `export`, `alias`, `exec`, `eval`.
- Entendi que isso importa para o runtime, porque builtins não passam pelo PATH como executáveis externos.


## 🪝 **Shell hook — observando o que acontece no bash**

- Entendi que o shell hook é uma forma de **observar o que acontece dentro do bash**.
- Ele permite ao sistema saber: qual comando foi digitado, se terminou com sucesso, qual o exit code, em qual pasta o aluno estava e quando o comando terminou.
- Fixei por que isso é necessário: `echo "mkdir projeto"` mostra `mkdir projeto` na tela, mas **não cria pasta nenhuma** — com o hook, o sistema sabe que o comando real foi o `echo`, e não o `mkdir`.
- Aprendi que, no Gist, o hook usa **`trap DEBUG`** para capturar o comando **antes** de executar e **`PROMPT_COMMAND`** para capturar exit code, diretório e timestamp **depois** da execução.


## 📨 **FIFO — o canal que entrega os eventos**

- Entendi o fluxo do hook: `bash executa → shell hook captura → FIFO recebe → aplicação lê o FIFO → Event Bus recebe → Validator Engine`.
- Aprendi que **FIFO = First In, First Out** (o primeiro que entra é o primeiro que sai).
- Fixei o exemplo de informação enviada:
  - `PRE  mkdir projeto` → antes do comando executar, mostrando o comando.
  - `POST 0 /home/aluno` → depois de terminar, com exit code `0` (sucesso) e a pasta onde o aluno estava.


## 🛡️ **Assert Channel — validar sem poluir o terminal**

- Entendi que o Assert Channel é um **canal separado** usado pelo sistema para fazer verificações.
- Ele existe para o Validator Engine conferir se o passo foi cumprido **sem digitar comandos no terminal do aluno**.
- Exemplo: para validar "Crie uma pasta chamada projeto", o sistema precisa rodar `test -d projeto`, mas esse teste **não deve aparecer** no terminal do aluno.
- Fixei a ideia central: o Assert Channel permite **validar a lição sem poluir o terminal** do aluno, rodando por fora.


## ⏱️ **Race condition — validar na hora certa**

- Aprendi que **race condition** é quando duas coisas acontecem quase ao mesmo tempo e a **ordem** pode causar erro.
- Exemplo: ao digitar `mkdir projeto`, quase ao mesmo tempo a pasta é criada, o evento do comando chega e o validador tenta verificar.
- Entendi o risco: se o Validator Engine verificar **cedo demais**, pode ainda não enxergar a pasta e dizer que falhou, mesmo o aluno tendo acertado.
- Fixei a solução: o sistema deve **validar depois que o comando terminou** (usando o `POST` do hook como gatilho).


## 💡 **Conceitos que fixei**

- O Runtime do Shell garante isolamento, previsibilidade e validação segura.
- Ele roda um **bash real**, não uma emulação.
- O PTY Host conecta o bash ao Terminal Pane usando um PTY real (`portable-pty`).
- O Sandbox usa `bwrap` para isolar sem o peso de um container completo.
- O ciclo do ambiente é: cria → seed → snapshot → bash → interação → reset → destrói.
- O PATH restrito + `sandbox_binaries` formam a allowlist de comandos por lição.
- Builtins (`cd`, `echo`, `export`...) vivem dentro do próprio bash.
- O shell hook usa `trap DEBUG` (antes) e `PROMPT_COMMAND` (depois) para capturar o comando real.
- Os eventos viajam pelo FIFO (First In, First Out) até o Event Bus.
- O Assert Channel valida por fora, sem sujar o terminal do aluno.
- A validação deve ocorrer **depois** do comando terminar, para evitar race condition.


## 🧾 **Resumo**

- Hoje entendi a camada que **cria e sustenta o ambiente real** onde o aluno pratica.
- O PTY Host dá o terminal real, o Sandbox (`bwrap` + PATH restrito) dá o isolamento, e o Assert Channel dá a validação limpa.
- O shell hook (`trap DEBUG` + `PROMPT_COMMAND`) captura o que de fato aconteceu e envia pelo FIFO até o Event Bus e o Validator Engine.
- Ficou claro o ciclo completo do Runtime do Shell: abrir o bash → conectar via PTY → isolar no sandbox → limitar comandos → preparar o seed → guardar snapshot para reset → capturar comandos reais → enviar eventos pelo FIFO → validar por fora pelo Assert Channel → pausar validação em programas fullscreen → limpar o ambiente ao final.


## 🚧 **Próximos passos**

- Estudar **Rust** de forma mais aprofundada, já que é a linguagem do projeto.
- Elaborar **exercícios práticos para cada parte da arquitetura**, para fixar tanto a linguagem quanto cada bloco em si.
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

