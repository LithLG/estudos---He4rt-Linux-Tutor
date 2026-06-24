# Estudo — Tutorial Pane, Glossary Popup, Núcleo de Aplicação & Persistência

Data: 18 de junho de 2026

Projeto: **He4rt Linux Tutor**

---

## 📋 **Tutorial Pane — onde o aluno entende o que fazer**

- Estudei que, se o Terminal Pane é onde o aluno **age**, o Tutorial Pane é onde o aluno **entende** o que precisa fazer.
- Entendi que ele responde perguntas como: qual lição estou fazendo? qual passo estou agora? o que preciso fazer? já acertei? errei? existe uma dica? posso avançar?
- Fixei que cada `step` no Gist pode ter campos como `id`, `instruction`, `hint` e `asserts`.
- Compreendi que o Tutorial Pane renderiza principalmente `instruction`, `hint`, `status` e o progresso do passo atual — os `asserts` ficam por conta do Validator Engine.


## 🔄 **Tutorial Pane é mais "estado" do que "lógica"**

- Abstraí que o Tutorial Pane é mais **estado** do que **lógica**: ele reflete o que o sistema decidiu, não decide sozinho.
- `Idle` → estado inicial, quando o aluno ainda não fez nada ou ainda não voltou para validação.
- `Validating` → o sistema está verificando a mensagem/ação do aluno.
- `Passed` → o aluno concluiu o passo.
- `Failed` → o aluno tentou, mas os asserts não passaram (podendo mostrar uma dica/hint).


## 🧭 **Fluxo do Tutorial Pane**

- Tutorial Pane mostra a instrução: "Crie um diretório chamado projeto usando mkdir."
- Aluno digita `mkdir projeto` no Terminal Pane.
- Terminal Pane envia bytes para o PTY → PTY entrega ao bash → bash executa o `mkdir`.
- Shell hook captura eventos estruturados: comando, exit code, cwd, timestamp.
- Event Bus recebe os eventos → Validator Engine roda os asserts → Lesson State muda (`Idle → Validating → Passed/Failed`).
- Tutorial Pane renderiza o novo estado: "Passo concluído" ou "Tente novamente".


## 📖 **Glossary Popup — apoio rápido para termos técnicos**

- Entendi que o Glossary Popup é uma **janela temporária** que aparece por cima da interface para explicar um termo (ex.: o que é `pwd`).
- Fixei que ele conversa principalmente com o Tutorial Pane, o Lesson State, o Content Load e o Event Bus.
- Aprendi que o glossário **não fica escrito dentro do componente visual** — ele vem do conteúdo da lição (TOML).
- Vi que o exemplo do Gist usa os campos: `term` (nome do termo), `short` (explicação curta) e `man` (explicação completa), além de marcações como `@pwd`, `@diretório` e `@mkdir` na instrução.


## 🪟 **O que o Glossary Popup consegue fazer**

- Abrir uma janela temporária e mostrar o termo selecionado.
- Mostrar explicação curta, explicação longa, exemplo de uso e comando relacionado.
- Permitir rolagem se o texto for grande e fechar com `Esc`.
- Devolver o foco para onde estava antes (Tutorial Pane ou Terminal Pane).
- Permitir navegar entre termos da lição, destacar o termo atual e **ajudar o aluno sem avançar a lição**.


## 🔁 **Fluxo completo do Glossary Popup**

- Content Load lê o TOML e encontra a instrução com `@pwd` e o glossário com `term = "pwd"`.
- Lesson State guarda o passo atual e seus termos; o Tutorial Pane renderiza o termo destacado.
- Aluno pressiona `F1`, `?` ou seleciona o termo → Event Bus recebe `OpenGlossary("pwd")`.
- App State marca `glossary_open = true` e `glossary_term = "pwd"` → o popup aparece.
- Aluno lê e aperta `Esc` → popup fecha → foco volta para o Tutorial Pane ou Terminal Pane.


## 🧠 **Núcleo de Aplicação — quem decide as coisas**

- Entendi que, enquanto a UI **mostra**, o núcleo **decide**: o que aconteceu? isso importa para a lição? o aluno acertou? qual o estado atual? o que mostrar agora?
- Fixei a ideia central: o Event Bus transporta acontecimentos, o Validator Engine verifica se o passo foi cumprido, e o Lesson State guarda o estado atual.
- De forma abstrata: **Event Bus = mensageiro**, **Validator Engine = validador**, **Lesson State = memória da aula**.


## 📡 **Event Bus — o transportador de eventos**

- Aprendi que o Event Bus **apenas transporta eventos**: ele não executa comando, não valida resposta, não mostra interface e não guarda progresso.
- Entendi que ele carrega acontecimentos como: o usuário digitou um comando, o comando terminou, retornou exit code 0, o diretório mudou, um arquivo foi criado.
- Também transporta eventos de interface/estado: terminal entrou/saiu de fullscreen, o usuário pediu para abrir o glossário, a validação começou, falhou ou passou.


## ✅ **Validator Engine — o corretor da lição**

- Entendi que o Validator Engine verifica se o aluno cumpriu o objetivo do passo.
- Exemplo: para "Crie uma pasta chamada projeto", ele checa se o comando faz sentido, se terminou com sucesso, se a pasta `projeto` existe e se ela é mesmo um diretório.
- Se tudo estiver certo → retorna `Passed`; se algo falhar → retorna `Failed`.
- Fixei que ele decide a partir de **eventos estruturados** + **asserts**, e não olhando o texto da tela (que seria frágil).


## 🧩 **Asserts — as regras de validação**

- Aprendi que um **assert** é uma regra/condição que precisa ser verdadeira para o passo passar.
- Exemplo com `mkdir projeto`, os asserts possíveis são:
  - O comando digitado precisa parecer `mkdir projeto`.
  - O comando precisa terminar com exit code `0`.
  - O caminho `projeto` precisa existir.
  - Esse caminho precisa ser um diretório.
- Se todos passarem → o passo passa. Se um falhar → o passo falha.


## 🗃️ **Lesson State — a memória viva da lição**

- Entendi que o Lesson State é a **memória atual da lição enquanto o programa roda**.
- Ele guarda: qual lição está aberta, qual passo está ativo, status do passo, se a dica está visível, quantas tentativas o aluno fez, se a lição terminou.
- Fixei os estados principais: `Idle` (espera), `Validating` (verificação), `Passed` (sucesso) e `Failed` (falha).
- Estudei a máquina de estados: `Idle → Validating → Passed → Next Step → Idle`, ou em caso de erro: `Idle → Validating → Failed → Idle` (nova tentativa).
- Vi o exemplo conceitual em Rust com a `struct LessonState` (campos como `lesson_id`, `current_step_index`, `status`, `attempts`, `show_hint`, `feedback`, `fullscreen_active`).


## 📦 **Content Load — carregando a lição (TOML + schema)**

- Entendi que o Content Load lê um arquivo (ex.: `navegacao-basica.toml`) e transforma em dados que o programa usa.
- Fixei o fluxo: `Arquivo TOML → Content Load → validação do schema → Lesson Definition em memória → Lesson State → Tutorial Pane`.
- Aprendi a diferença: **TOML = o formato do arquivo**; **Schema = as regras que o arquivo precisa seguir** (o "contrato" da lição).
- O Content Load responde: essa lição existe? está bem escrita? tem passos válidos? os asserts fazem sentido? os termos de glossário existem? pode ser carregada com segurança?
- A decisão técnica do projeto é usar **TOML + validação no load + lint** para a comunidade escrever lições sem mexer no código Rust.


## 🏷️ **Campos do arquivo TOML da lição**

- `[lesson]` → cabeçalho com `id`, `title`, `locale`, `schema_version`, `requires`, `sandbox_binaries`.
- `[[glossary]]` → termos da lição com `term`, `short` e `man`.
- `[[seed]]` → arquivos/pastas que precisam existir antes da lição começar (`path`, `content`).
- `[[step]]` → cada etapa da lição, com `id`, `instruction` (texto do Tutorial Pane, com `@termo`) e `hint`.
- `[[step.assert]]` → regras de validação com `type` (`command`, `exit`, `fs`, `script`) e campos como `match`, `equals`, `path`, `is_dir`, `run`, `expect_exit`.


## 🔍 **lint — validar a lição sem rodar tudo**

- Aprendi que o Content Load **não carrega qualquer coisa**: ele precisa validar.
- `lint` é o comando para verificar se o TOML está correto sem rodar a lição inteira (ex.: `lint navegacao-basica.toml`).
- Resultado bom: `OK: lição válida`.
- Resultado ruim: aponta o erro, ex.: assert `script` usando um binário fora da allowlist.


## 💾 **Progress Store — salvando o progresso em disco**

- Entendi que o Progress Store salva o **progresso local do aluno** em um arquivo TOML (ex.: `~/.local/share/he4rt-tutor/progress.toml`).
- O modelo inclui `schema_version`, uma seção por lição, `status`, `current_step`, `completed_at` e telemetria local reservada (`attempts_per_step`, `time_seconds`).
- Fixei a diferença chave: **Lesson State** = estado vivo em memória (app aberto); **Progress Store** = arquivo salvo em disco.
- Fluxo: `abre app → Progress Store carrega arquivo → Lesson State inicializa com o progresso → aluno joga → Lesson State muda → Progress Store salva no arquivo`.


## ⚖️ **Diferenças que precisei separar bem**

- **Content Load ≠ Tutorial Pane**: o Content Load carrega o conteúdo do TOML; o Tutorial Pane apenas mostra.
- **Content Load ≠ Validator Engine**: o Content Load valida o **arquivo da lição**; o Validator Engine valida a **ação do aluno**.
- **Content Load ≠ Progress Store**: o Content Load lê o conteúdo da aula; o Progress Store lê e salva o progresso do aluno.


## 💡 **Conceitos que fixei**

- O Tutorial Pane mostra estado; ele não contém a lógica de validação.
- Os estados do passo são `Idle`, `Validating`, `Passed` e `Failed`.
- O Glossary Popup é temporário e vem do conteúdo da lição, não do componente visual.
- O Glossary Popup ajuda o aluno **sem avançar** a lição.
- Event Bus = mensageiro, Validator Engine = validador, Lesson State = memória da aula.
- O Event Bus só transporta eventos; não executa, não valida, não desenha, não persiste.
- O Validator Engine decide `Passed`/`Failed` usando eventos estruturados + asserts.
- Assert é uma condição que precisa ser verdadeira para o passo passar.
- O Lesson State é memória **viva** (RAM); o Progress Store é memória **persistida** (disco).
- TOML é o formato; o Schema é o contrato de regras da lição.
- O `lint` valida a lição sem precisar rodá-la inteira.
- Content Load, Validator Engine e Progress Store têm responsabilidades distintas.


## 🧾 **Resumo final**

- Hoje entendi a camada que **decide e exibe o estado da lição**, acima do Terminal Pane.
- O Tutorial Pane e o Glossary Popup são a face visível; o Event Bus, o Validator Engine e o Lesson State são o núcleo que decide.
- O conteúdo da lição mora em arquivos TOML validados por um schema (Content Load + lint), e o progresso do aluno é persistido em disco pelo Progress Store.
- Ficou claro o ciclo completo: o aluno age → eventos viajam pelo Event Bus → o Validator Engine corrige → o Lesson State muda → o Tutorial Pane mostra o resultado → o Progress Store guarda o avanço.


## 🚧 **Próximos passos**

- Estudar o **Runtime do Shell**.
- Entender qual é o papel do runtime dentro da arquitetura, abaixo do Terminal Pane.
- Ver como o shell é iniciado e mantido vivo durante a lição (processo, PTY, ciclo de vida).
- Entender como os **shell hooks** capturam eventos estruturados (comando, exit code, cwd, timestamp) e os entregam ao Event Bus.
- Estudar como o ambiente é isolado/sandbox e como a allowlist de binários (`sandbox_binaries`) é aplicada na prática.
- Ver como o `[[seed]]` prepara arquivos e pastas iniciais antes da lição começar.
- Entender como o runtime se comunica com o Validator Engine para asserts do tipo `script` (`run`, `expect_exit`).
- Comparar as responsabilidades do Runtime do Shell com as do Terminal Pane e do Validator Engine.
