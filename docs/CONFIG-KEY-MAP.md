# Config key map — legacy → stable

> Phase 04. Migration runs on first load (ADR-0004).  
> I18N strategy table expanded with every key read under `utils/`.

Stable schema uses English identifiers. Category **maps** keep Portuguese
folder **ids** (`documentos`, …); UI labels use Fluent `category.*`.

## Top-level keys

| Legacy key | Stable key | Readers (`utils/`) |
|------------|------------|--------------------|
| `Folder` | `folder` | `folder.py`, `AutoDelete.py` |
| `timeverification` | `interval_minutes` | `folder.py`, `timeVerification.py` |
| `Startup` | `autostart` | `StartUp.py` |
| `AutoDelete` | `auto_delete.enabled` | `delete.py`, `AutoDelete.py` |
| `Enviar Para Lixeira` | `auto_delete.to_trash` | `delete.py`, `AutoDelete.py` |
| `Excluir permanentemente` | `auto_delete.permanent` | `delete.py`, `AutoDelete.py` |
| `AdvancedOrganize` | `advanced_organize` | `AdvancedConfig.py` |
| `AutoDeleteConfig` | `auto_delete` (object) | `delete.py`, `AutoDelete.py`, `timeVerification.py` |
| `folder_delete` | `folder_delete` | `delete.py` (`Folder_Delete`) |
| *(new)* | `locale` | Rust only until phase 10 |

## Nested — `AutoDeleteConfig`

| Legacy key | Stable key |
|------------|------------|
| `Por Data de Criação` | `auto_delete.by_created` |
| `Por Data de Modificação` | `auto_delete.by_modified` |
| `Dias para Auto Deletar` | `auto_delete.max_age_days` |

## Nested — `folder_delete`

| Legacy key | Stable key |
|------------|------------|
| `ativado` | `folder_delete.enabled` |
| `lixeira` | `folder_delete.to_trash` |
| `excluir_permanentemente` | `folder_delete.permanent` |
| `pastas_ORZ` | `folder_delete.orz_folders_only` |
| `tudo` | `folder_delete.everything` |

## Category maps (dynamic)

Any other top-level object whose values are `{ ".ext": bool }` is a category:

| Legacy | Stable |
|--------|--------|
| `documentos`, `imagens`, `audios`, `videos`, `compactos`, `fontes`, `setups`, `Desenvolvimento`, … | `categories.<id>` with same ext toggles |

Ids are **not** renamed on disk. Display via `category.<id>` Fluent IDs.

## Keywords file

| Legacy path / name | Stable |
|--------------------|--------|
| `dist/Key_Words.json` | `$XDG_CONFIG_HOME/fileorz/keywords.json` |
| Group names (`Nota Fiscal`, …) | User data folder names — not schema keys |

## Placeholder values

| Legacy value | Stable |
|--------------|--------|
| `Folder` == `"pasta de organização"` | empty / unset `folder` |

## Validation

```bash
python3 scripts/check_config_key_map.py
```

Every string literal key read from config in `utils/*.py` must appear in this
document (legacy column or nested section).
