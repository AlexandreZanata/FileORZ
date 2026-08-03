# i18n catalog — message IDs

> Phase 04 inventory. Runtime: phase 10 (`docs/I18N-RUNTIME.md`).  
> Locales: [`crates/fileorz-i18n/locales/`](../crates/fileorz-i18n/locales/).  
> Strategy: `.local/I18N-STRATEGY.md` · ADR-0003.

**ID syntax:** Fluent files use hyphenated identifiers (`main-btn-start`). Dotted
names in strategy examples (`main.btn.start`) are the same logical ID.

**Screens:** Main · Settings hub · Extensions · Advanced (BETA) · Auto-delete · Tray · Errors/feedback · About

Icons/emoji in upstream CustomTkinter are decorative — Fluent strings are plain text.

## Main shell (`ui/index.py`, `header.py`, `btn.py`, `Select_Folder.py`, `Time_Select.py`)

| ID | Screen | Source (pt-BR today) | Notes |
|----|--------|----------------------|-------|
| `app-title` | Main | FileORZ | Brand |
| `app-tagline` | Main | Organizador de Arquivos | Header subtitle |
| `app-window-title` | Main | File ORZ - Organize seus arquivos | `root.title` |
| `header-changelog` | Main | Changelog | Link button |
| `header-github` | Main | GitHub | Link button |
| `header-autostart` | Main | Iniciar com Windows | Linux: login / system |
| `main-btn-settings` | Main | Configurações | Was emoji + label |
| `main-btn-start` | Main | Iniciar Organização | Was emoji + label |
| `main-btn-stop` | Main | Parar Organização | Start/Stop CTA |
| `folder-pick-label` | Main | Selecionar Pasta para Organizar | |
| `folder-pick-button` | Main | Selecionar | |
| `folder-pick-dialog` | Main | Selecione a pasta | filedialog title |
| `folder-pick-empty` | Main | Nenhuma pasta selecionada | Empty path label |
| `interval-label` | Main | Intervalo de Verificação (Minutos) | |
| `interval-help` | Main | Tempo em minutos entre cada verificação… | |

## Settings hub (`ui/config.py`)

| ID | Screen | Source (pt-BR today) |
|----|--------|----------------------|
| `settings-hub-window-title` | Hub | Configurações Gerais - FileORZ |
| `settings-hub-title` | Hub | Configuração - FileORZ |
| `settings-hub-subtitle` | Hub | Ajuste as preferências do organizador |
| `settings-card-extensions-title` | Hub | Categorias & Extensões |
| `settings-card-extensions-body` | Hub | Gerencie e customize as extensões… |
| `settings-card-advanced-title` | Hub | Organização Avançada (BETA) |
| `settings-card-advanced-body` | Hub | Regras inteligentes com palavras-chave… |
| `settings-card-autodelete-title` | Hub | Auto Deletar |
| `settings-card-autodelete-body` | Hub | Defina intervalos e regras automáticas… |
| `settings-card-configure` | Hub | Configurar |

## Extensions (`ui/ext_config.py`)

| ID | Screen | Source (pt-BR today) |
|----|--------|----------------------|
| `settings-ext-title` | Ext | Categorias e Extensões |
| `settings-ext-subtitle` | Ext | Gerencie as extensões por categoria |
| `settings-ext-select-all` | Ext | Todos |
| `settings-ext-select-none` | Ext | Nenhum |
| `settings-ext-save` | Ext | Salvar Categorias |
| `settings-ext-saved` | Ext | Configurações salvas com sucesso! |

## Advanced PDF keywords (`ui/Advanced_Config.py`)

| ID | Screen | Source (pt-BR today) |
|----|--------|----------------------|
| `settings-advanced-window-title` | Adv | Configurações Avançadas - FileORZ (BETA) |
| `settings-advanced-title` | Adv | Organização Avançada de Documentos (BETA) |
| `settings-advanced-enabled-on` | Adv | Modo Avançado Ativado |
| `settings-advanced-enabled-off` | Adv | Modo Avançado Desativado |
| `settings-advanced-add-group` | Adv | Adicionar Grupo |
| `settings-advanced-help` | Adv | Defina o nome do grupo e as palavras-chave… |
| `settings-advanced-empty` | Adv | Nenhum grupo cadastrado. Clique em + Adicionar Grupo. |
| `settings-advanced-group-name-placeholder` | Adv | Nome do Grupo (ex: Boletos) |
| `settings-advanced-keywords-placeholder` | Adv | Palavras ou frases separadas por vírgula… |
| `settings-advanced-save-group` | Adv | Salvar |

## Auto-delete (`ui/Config_AutoDell.py`)

| ID | Screen | Source (pt-BR today) |
|----|--------|----------------------|
| `settings-autodelete-window-title` | Auto | Configurações Auto Deletar |
| `settings-autodelete-title` | Auto | Configurar Auto Deletar |
| `settings-autodelete-enabled-on` | Auto | Auto Deletar Ativado |
| `settings-autodelete-enabled-off` | Auto | Auto Deletar Desativado |
| `settings-autodelete-filters-title` | Auto | Filtros de Exclusão |
| `settings-autodelete-filter-by-created` | Auto | Por Data de Criação |
| `settings-autodelete-filter-by-modified` | Auto | Por Data de Modificação |
| `settings-autodelete-deadline-title` | Auto | Prazo para Exclusão |
| `settings-autodelete-deadline-help` | Auto | Dias para excluir o arquivo. |
| `settings-autodelete-type-title` | Auto | Tipo de exclusão |
| `settings-autodelete-to-trash` | Auto | Enviar Para Lixeira |
| `settings-autodelete-permanent` | Auto | Excluir permanentemente |

## Tray (`utils/system_tray.py`)

| ID | Screen | Source (pt-BR today) |
|----|--------|----------------------|
| `tray-tooltip` | Tray | FileORZ |
| `tray-open` | Tray | Abrir |
| `tray-quit` | Tray | Fechar |

## Errors / feedback (`utils/StartTask.py`, organize validation)

| ID | Screen | Source (pt-BR today) |
|----|--------|----------------------|
| `error-folder-missing` | Main | Selecione uma pasta primeiro! |
| `error-organizer-start-failed` | Main | Erro ao iniciar o organizador! |
| `feedback-organize-started` | Main | Organização concluída com sucesso! |
| `error-folder-invalid` | Core | Diretório inválido ou não selecionado |

## About (Linux target — UI-LINUX)

| ID | Screen | Source |
|----|--------|--------|
| `about-title` | About | About FileORZ |
| `about-license` | About | Licensed under GNU GPL-3.0 |
| `about-upstream` | About | Upstream project |
| `about-notices` | About | Third-party notices |

## Category display names (folder ids stay Portuguese)

| ID | Default folder id | en | pt-BR |
|----|-------------------|----|-------|
| `category-documentos` | documentos | Documents | Documentos |
| `category-imagens` | imagens | Images | Imagens |
| `category-audios` | audios | Audio | Áudios |
| `category-videos` | videos | Videos | Vídeos |
| `category-compactos` | compactos | Archives | Compactos |
| `category-fontes` | fontes | Fonts | Fontes |
| `category-setups` | setups | Installers | Setups |
| `category-desenvolvimento` | Desenvolvimento | Development | Desenvolvimento |
| `category-outros` | OUTROS | Other | Outros |

## Non-translated / log-only

| Pattern | Decision |
|---------|----------|
| `print(...)` in FileORZ/AutoDelete/Alg | `log-*` — not in Fluent |
| AutoBuild installer strings | Out of scope |
| Keyword phrases in Key_Words.json | User data, not UI catalog |

## Review checklist (main + four dialogs)

- [x] Main shell
- [x] Settings hub
- [x] Extensions
- [x] Advanced (BETA)
- [x] Auto-delete
- [x] Tray + start feedback errors
