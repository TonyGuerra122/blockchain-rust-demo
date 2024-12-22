# Blockchain Rust Demo

Este é um projeto de demonstração de uma blockchain extremamente simples escrita em Rust. O objetivo é facilitar o aprendizado de como funcionam os conceitos básicos de uma blockchain, como armazenamento de blocos, adição de novos blocos e exibição da cadeia.

## Requisitos

Para executar o projeto, você precisa ter:

-   Rust instalado em sua máquina.

## Funcionalidades

-   Adicionar Blocos: Adicione novos blocos à blockchain com dados personalizados.

-   Exibir Blockchain: Liste todos os blocos armazenados na blockchain.

-   Limpar Blockchain: Limpe todos os blocos existentes na cadeia.

## Instalação

Clone o repositório para sua máquina local:

```bash
git clone https://github.com/TonyGuerra122/blockchain-rust-demo.git
cd blockchain-rust-demo
```
### Compile o projeto:
```bash
cargo build --release
```
## Uso

### Rodando o programa

Execute o binário:
```bash
cargo run --release
```
Você verá as opções de subcomandos:
```out
Blockchain-rust-demo 0.1

USAGE:
    blockchain-rust-demo <SUBCOMMAND>

SUBCOMMANDS:
    printchain    Exibe todos os blocos na blockchain
    clear         Limpa a blockchain
    addblocks     Adiciona um bloco na blockchain com os dados fornecidos
```
### Subcomandos

#### 1. Adicionar um bloco

Adicione um novo bloco à blockchain:
```bash
cargo run --release addblocks "<SEUS DADOS>"
```
Exemplo:
```bash
cargo run --release addblocks "Bloco de exemplo"
```
#### 2. Exibir a blockchain

Liste todos os blocos armazenados:
```bash
cargo run --release printchain
```
#### 3. Limpar a blockchain

Apague todos os blocos da cadeia:
```bash
cargo run --release clear
```
## Licença

Este projeto está licenciado sob a MIT License.

## Autor

**Anthony Guerra [E-mail](guerra.anthony122@gmail.com)**