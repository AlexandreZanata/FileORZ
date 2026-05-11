"""
    Copyright (C) 2026 Thainan Vinicius Katchan

    This file is part of FileORZ.

    FileORZ is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    FileORZ is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with FileORZ.  If not, see <https://www.gnu.org/licenses/
"""

import json
import os
from utils.model import load_config, save_config, script_dir

class AdvancedConfig:
    def __init__(self):
        self.config = load_config("dist", "config")
        self.keywords_path = os.path.join(script_dir(), "AdvancedAlg", "Key_Words.json")

    def get_enabled(self):
        return self.config.get("AdvancedOrganize", False)

    def set_enabled(self, enabled):
        self.config["AdvancedOrganize"] = enabled
        save_config("dist", "config", self.config)

    def load_keywords(self):
        try:
            if not os.path.exists(self.keywords_path):
                # Se não existir, cria um arquivo vazio ou com estrutura básica
                with open(self.keywords_path, 'w', encoding='utf-8') as f:
                    json.dump({}, f, indent=2, ensure_ascii=False)
                return {}
            with open(self.keywords_path, 'r', encoding='utf-8') as f:
                return json.load(f)
        except Exception as e:
            print(f"Erro ao carregar Key_Words.json: {e}")
            return {}

    def save_keywords(self, keywords_data):
        try:
            with open(self.keywords_path, 'w', encoding='utf-8') as f:
                json.dump(keywords_data, f, indent=4, ensure_ascii=False)
            return True
        except Exception as e:
            print(f"Erro ao salvar Key_Words.json: {e}")
            return False
