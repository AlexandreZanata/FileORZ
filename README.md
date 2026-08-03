> **Fork notice / Aviso de fork**
>
> This repository is a **fork** of the original FileORZ project for agent-harness
> and quality-gate work. Application code is **third-party** upstream software
> (GNU GPL v3.0). See [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md).
>
> - **Original (upstream):** https://github.com/ThainanViniciusKatchan/FileORZ
> - **This fork:** https://github.com/AlexandreZanata/FileORZ
>
> Este repositório é um **fork** do FileORZ original. O código da aplicação é de
> **terceiros** (upstream). Não apresente este fork como o projeto original.

## Linux product (this fork)

The **supported Linux ship path** is the Rust binary (iced + Fluent), not the
Python CustomTkinter UI. Install guide: [`docs/INSTALL-LINUX.md`](docs/INSTALL-LINUX.md).
Parity / regression gate: [`docs/PARITY-REPORT.md`](docs/PARITY-REPORT.md).

```bash
cargo build -p fileorz --release
./target/release/fileorz --version
./target/release/fileorz            # iced main shell
# or package:
bash scripts/package-linux.sh
sudo apt install ./dist/linux/fileorz_*_amd64.deb
```

Python sources (`ui/`, `FileORZ.py`, `utils/`, …) remain **legacy upstream
reference / characterization oracle** (ADR-0001). They are **not** launched by
the `.deb` or the documented Linux install path.

Locales: **en** + **pt-BR** via Fluent (`crates/fileorz-i18n`).

<div align="center">

# 📁 FileORZ

![FileORZ](assets/File_ORZ.png)

### 🚀 Organizador Automático de Arquivos

[![Python](https://img.shields.io/badge/Python-3.12+-3776AB?style=for-the-badge&logo=python&logoColor=white)](https://python.org)
[![License](https://img.shields.io/badge/Licença-GNU%20GPL-green?style=for-the-badge)](LICENSE)
![OpenSource](https://img.shields.io/badge/OpenSource%20-20B2AA?style=for-the-badge)


**Desenvolvido por Thainan Vinicius Katchan** (upstream) · fork mantido em AlexandreZanata/FileORZ
# DESENVOLVIDO NO BRASIL 💚💛

### 🌐 Idiomas | Languages

[🇧🇷 Português](#-português) • [🇺🇸 English](#-english)

</div>

> ⚠️ **Nota (fork Linux):** o caminho de produto é o binário Rust — ver
> [docs/INSTALL-LINUX.md](docs/INSTALL-LINUX.md). A UI Python abaixo é
> **legado / referência upstream**.

> ⚠️ **Note (Linux fork):** the product path is the Rust binary — see
> [docs/INSTALL-LINUX.md](docs/INSTALL-LINUX.md). The Python UI below is
> **legacy / upstream reference**.

# 🇧🇷 Português

### 📋 Descrição

O **FileORZ** surgiu para solucionar um problema comum: a desorganização de arquivos em pastas. Quantas vezes você teve dificuldade para encontrar um arquivo em meio a dezenas de arquivos soltos? O FileORZ resolve isso automaticamente!

## ⚙️ Como Executar

### Linux (produto — Rust)

```bash
git clone https://github.com/AlexandreZanata/FileORZ.git
cd FileORZ
cargo build -p fileorz --release
./target/release/fileorz
```

Detalhes e `.deb`: [docs/INSTALL-LINUX.md](docs/INSTALL-LINUX.md).

### Legacy — Python UI (upstream / oracle)

```bash
# Referência histórica — não é o caminho de release Linux deste fork
pip install -r requirements.txt
python ui/index.py
```

## ✨ Funcionalidades

### 📂 Organização Inteligente
| Recurso          | Descrição                                              |
|------------------|--------------------------------------------------------|
| 🎨 Interface Gráfica | Feita com CustomTkinter, moderna e intuitiva           |
| 📁 Pasta Customizável | Escolha qualquer pasta para organizar                  |
| 🗑️ auto deletar | Ative o auto deltar e decida como e quanto irá excluír |
| 🏷️ Por Extensão | Arquivos organizados automaticamente por tipo          |
| ⏱️ Tempo Configurável | Defina o intervalo de verificação                      |
| 🚀 Inicialização Automática | Inicie com o Windows automaticamente                   |
| 💾 Baixo Consumo | Menos de 10MB de memória RAM                           |

### 📊 Performance
> Em testes com **200 arquivos** totalizando **200GB**:
> - 💾 Memória: apenas **15MB**
> - 🖥️ CPU: menos de **1%**
> - ⏱️ Tempo: menos de **20 segundos**

### 🔄 Inicialização com o Sistema
- ✅ Não requer instalação
- ✅ Copia arquivos para `AppData` automaticamente
- ✅ Cria uma chave no Registro do Windows

### 💚 Open Source
- ✅ Código 100% aberto e auditável
- ✅ **Sem** cobrança mensal ou anual
- ✅ **Sem** venda de dados
- ✅ **Sem** anúncios ou publicidade
- ✅ **Sem** limitações

---

<div align="center">

### 💚 Agradeço a todos que apoiam este projeto! 💛

</div>

---

# 🇺🇸 English

## 📋 Description

**FileORZ** was created to solve a common problem: file disorganization in folders. How many times have you struggled to find a file among dozens of loose files? FileORZ solves this automatically!

---

## ⚙️ How to Run

### Linux (product — Rust)

```bash
git clone https://github.com/AlexandreZanata/FileORZ.git
cd FileORZ
cargo build -p fileorz --release
./target/release/fileorz
```

Details and `.deb`: [docs/INSTALL-LINUX.md](docs/INSTALL-LINUX.md).

### Legacy — Python UI (upstream / oracle)

```bash
# Historical reference — not this fork's Linux release path
pip install -r requirements.txt
python ui/index.py
```

---

## ✨ Features

### 📂 Smart Organization
| Feature          | Description                                              |
|------------------|----------------------------------------------------------|
| 🎨 Graphical Interface | Built with CustomTkinter, modern and intuitive           |
| 📁 Custom Folder | Choose any folder to organize                            |
| 🗑️ Auto Delete   | Enable auto-delete and decide how and how much to delete |
| 🏷️ By Extension  | Files automatically organized by type                    |
| ⏱️ Configurable Time | Set the verification interval                            |
| 🚀 Auto Start    | Start with Windows automatically                         |
| 💾 Low Consumption | Less than 10MB of RAM memory                             |

### 📊 Performance
> In tests with **200 files** totaling **200GB**:
> - 💾 Memory: only **15MB**
> - 🖥️ CPU: less than **1%**
> - ⏱️ Time: less than **20 seconds**

### 🔄 System Startup
- ✅ No installation required
- ✅ Automatically copies files to `AppData`
- ✅ Creates an entry in Windows Registry

### 💚 Open Source
- ✅ 100% open and auditable code
- ✅ **No** monthly or annual fees
- ✅ **No** data selling
- ✅ **No** ads or publicity
- ✅ **No** limitations

---

<div align="center">

### 💚 Thank you to everyone who supports this project! 💛

---

**[⬆ Back to top](#-fileorz)**

</div>

<p style="text-align: center"> LICENSE: <br>
PT-BR: Este projeto está licenciado sob a GNU General Public License v3.0 - veja o arquivo LICENSE para detalhes. <br>
EN: This project is licensed under the GNU General Public License v3.0 - see the LICENSE file for details.
</p>
