# Estudo — Terminal Pane

Data: 17 de junho de 2026

Projeto: **He4rt Linux Tutor**

🖥️ **Terminal Pane — janela visual do terminal**

* Estudei o papel do **Terminal Pane** dentro da arquitetura da aplicação.
* Entendi que ele é a parte visual onde o usuário digita comandos e vê respostas.
* Fixei que ele **não executa comandos**, apenas mostra e encaminha informações.
* Compreendi que ele funciona como um espelho interativo do shell real.

🔌 **PTY — ponte com o terminal real**

* Aprendi que PTY significa **Pseudo-Terminal**.
* Entendi que ele serve como uma ponte entre a aplicação e o `bash`.
* O PTY faz o `bash` acreditar que está conectado a um terminal real.
* Fixei o fluxo: `Terminal Pane → PTY → bash`.

🐚 **bash — quem interpreta os comandos**

* Entendi que o `bash` é quem interpreta a linha digitada pelo usuário.
* No caso de `mkdir projeto`, o Terminal Pane não cria a pasta.
* O Terminal Pane envia bytes, o PTY entrega ao bash, e o bash chama o comando `mkdir`.
* Fixei que alguns comandos são programas externos chamados pelo bash.

⌨️ **Entrada do usuário — bytes de teclado**

* Aprendi que o Terminal Pane não envia comandos prontos para o PTY.
* Ele envia bytes correspondentes às teclas pressionadas.
* Exemplo: ao digitar `mkdir projeto`, ele envia tecla por tecla até o `Enter`.
* Entendi que o Terminal Pane precisa saber se está focado antes de encaminhar a tecla.

📤 **Saída do terminal — bytes de saída**

* Aprendi que o Terminal Pane recebe bytes vindos do PTY.
* Esses bytes podem representar texto comum, erro, cores, cursor ou comandos invisíveis.
* Entendi que a saída do terminal não é apenas uma string simples.
* Fixei que o Terminal Pane precisa interpretar o que recebe antes de mostrar na tela.

🎨 **Sequências ANSI — controle visual do terminal**

* Estudei que a saída do terminal pode conter sequências ANSI.
* Essas sequências podem mudar cor, mover cursor, limpar tela ou alterar o modo de exibição.
* Exemplo: `\x1b[31mErro\x1b[0m` representa texto vermelho e reset de estilo.
* Entendi que, se tratar tudo como texto puro, o Terminal Pane pode mostrar lixo na tela.

🧹 **Comando ****`clear`**

* Entendi que `clear` não é simplesmente imprimir várias linhas vazias.
* Ele envia comandos de controle para limpar a tela visível.
* Também pode reposicionar o cursor.
* Fixei que o Terminal Pane precisa interpretar essa saída corretamente.

🧩 **vim, less, nano e programas fullscreen**

* Aprendi que programas como `vim`, `less`, `nano`, `top` e `htop` precisam de tratamento especial.
* Eles podem usar uma tela alternativa, como se fosse um modo fullscreen.
* Entendi que eles controlam a tela inteira, movem cursor e redesenham constantemente.
* Fixei que o Terminal Pane precisa detectar entrada e saída desse modo.

🖼️ **Alternate screen / fullscreen**

* Estudei que alguns programas usam uma tela separada da tela normal do terminal.
* Ao abrir `vim`, por exemplo, ele entra em uma tela alternativa.
* Ao sair, a tela normal volta como estava antes.
* Entendi que isso pode ser detectado por sequências como `\x1b[?1049h` e `\x1b[?1049l`.

⚠️ **Ctrl+C em uma TUI**

* Entendi que `Ctrl+C` é uma tecla ambígua.
* No terminal, geralmente ela interrompe um processo em execução.
* Na aplicação, poderia ser interpretada como atalho para sair ou cancelar algo.
* Fixei que a decisão depende do foco e do estado atual da interface.

🎮 **Roteamento de teclas**

* Aprendi que a aplicação precisa decidir para onde cada tecla vai.
* Se um popup está aberto, a tecla pode ser tratada pelo popup.
* Se o Terminal Pane está focado, a tecla pode ser enviada ao PTY.
* Entendi que teclas como `Esc`, `Tab`, setas, `Ctrl+C` e `Enter` exigem cuidado.

✅ **Validator Engine — validação fora do Terminal Pane**

* Entendi que o Terminal Pane não deve validar se o aluno acertou.
* A validação pertence ao **Validator Engine**.
* Fixei que validar apenas olhando o texto da tela seria frágil.
* Exemplo: o aluno poderia usar `echo "/home/aluno/projeto"` sem realmente entrar na pasta.

🧱 **Screen Buffer — tela como matriz de células**

* Aprendi que a tela do terminal não deve ser pensada como uma string gigante.
* O modelo correto é imaginar uma matriz de células.
* Cada célula pode guardar caractere, cor, fundo, negrito, sublinhado e posição.
* Entendi que isso facilita lidar com cursor, cores, limpeza e movimentação.

🧠 **Chunks quebrados e parser com memória**

* Estudei que o PTY pode entregar a saída em pedaços.
* Uma sequência ANSI pode chegar quebrada em vários chunks.
* Exemplo: `\x1b[31mErro` pode chegar como `\x1b[`, depois `31`, depois `mErro`.
* Entendi que o parser precisa guardar estado até a sequência estar completa.

🧰 **ratatui + crossterm**

* Estudei a diferença entre `ratatui` e `crossterm`.
* `crossterm` controla o terminal: teclas, raw mode, resize, cursor e alternate screen.
* `ratatui` desenha a interface: painéis, bordas, textos, listas e widgets.
* Fixei o modelo: `crossterm = controle do terminal` e `ratatui = desenho da interface`.

📚 **Material de apoio**

* Também utilizei este artefato como apoio visual para firmar os conceitos estudados:
* https://claude.ai/public/artifacts/41b9d9d6-ac78-4b5c-9103-172c7c03d963

💡 **Conceitos que fixei**

* Terminal Pane não executa comandos.
* PTY não executa comandos.
* bash interpreta a linha digitada.
* Terminal Pane envia bytes de teclado.
* Terminal Pane recebe bytes de saída.
* Nem todo byte recebido é texto visível.
* Sequências ANSI controlam aspectos visuais do terminal.
* `vim` e `less` usam tela alternativa/fullscreen.
* `Ctrl+C` pode ser tecla do shell ou atalho da aplicação.
* A validação do aprendizado deve ficar fora do Terminal Pane.
* A tela do terminal deve ser pensada como uma matriz de células.
* O parser precisa ter memória para lidar com sequências quebradas.

🧾 **Resumo final**

* Hoje entendi que o **Terminal Pane** é uma camada visual e interativa.
* Ele recebe teclas, envia bytes ao PTY, recebe bytes de saída, interpreta essa saída e atualiza a interface.
* O shell real fica abaixo, conectado pelo PTY.
* A execução fica com o bash e a validação fica fora do Terminal Pane.

## 🚧 Próximos passos

* Estudar o **Tutorial Pane**
* Entender qual é o papel dele dentro da UI
* Ver como ele mostra o passo atual da lição
* Entender como ele exibe dicas, objetivos e feedback para o aluno
* Estudar como ele conversa com o **Lesson State**
* Entender como ele reage quando o **Validator Engine** confirma que uma etapa foi concluída
* Estudar o **Glossary Popup**
* Entender como ele ajuda o aluno com termos técnicos durante a lição
* Ver quando o popup deve aparecer e quando deve fechar
* Entender como o foco muda entre Terminal Pane, Tutorial Pane e Glossary Popup
* Estudar como atalhos como `F1`, `Esc` ou `?` podem abrir ou fechar o glossário
* Comparar as responsabilidades do Terminal Pane, Tutorial Pane e Glossary Popup

