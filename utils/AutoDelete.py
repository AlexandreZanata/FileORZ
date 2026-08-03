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

import os
import sys
from datetime import datetime
from os import path, remove, scandir

from send2trash import send2trash

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from utils.model import load_config

CONFIG = load_config("dist", "config")
CONFIG_AUTO_DELETE = CONFIG["AutoDelete"]


def AutoDelete():
    def GetConfig() -> tuple[bool, bool, bool, str]:
        data = load_config("dist", "config")

        cfg = data.get("AutoDeleteConfig", {})

        by_create_date = cfg.get("Por Data de Criação", False)
        by_last_modified_date = cfg.get("Por Data de Modificação", False)
        days_to_auto_delete = cfg.get("Dias para Auto Deletar", "0")

        return by_create_date, by_last_modified_date, days_to_auto_delete

    def scan_files(
        PATH_FILES,
    ):  # Escaneia os arquivos da pasta e trás as datas de criação e modificação
        Dias_Config = int(GetConfig()[2])
        File_Name = ""


        CreateDate: datetime | None = None
        ModifyDate: datetime | None = None
        with scandir(PATH_FILES) as entries:
            for entry in entries:
                if entry.is_file():
                    File_Name = entry.name
                    CreateDate = datetime.fromtimestamp(entry.stat().st_birthtime)
                    ModifyDate = datetime.fromtimestamp(entry.stat().st_mtime)
                    # Validação de exclusão
                    if CONFIG_AUTO_DELETE:
                        if (datetime.now() - CreateDate).days > Dias_Config:
                            if GetConfig()[0]:
                                if CONFIG["Enviar Para Lixeira"]:
                                    send2trash(entry.path)
                                    print(
                                        f"O arquivo {File_Name} foi enviado para a lixeira"
                                    )
                                elif CONFIG["Excluir permanentemente"]:
                                    remove(entry.path)
                                    print(
                                        f"O arquivo {File_Name} foi excluído permanentemente"
                                    )
                            else:
                                print("Configuração de data de criação desativada")
                        else:
                            print(
                                f"Nenhum arquivo está a mais de {Dias_Config} dias "
                                "para ser excluído\n Por data de criação"
                            )

                    if GetConfig()[1]:
                        if (datetime.now() - ModifyDate).days > Dias_Config:
                            if CONFIG["Enviar Para Lixeira"]:
                                send2trash(entry.path)
                                print(
                                    f"O arquivo {File_Name} foi enviado para a lixeira"
                                )
                            elif CONFIG["Excluir permanentemente"]:
                                remove(entry.path)
                                print(
                                    f"O arquivo {File_Name} foi excluído permanentemente"
                                )
                        else:
                            print(
                                f"Nenhum arquivo está a mais de {Dias_Config} dias "
                                "para ser excluído\n Por data de modificação"
                            )
                    else:
                        print("Configuração de data de modificação desativada")
                else:
                    print("Configuração de auto-delete desativada")

    Ignore_Config = [
        "Folder",
        "AutoDelete",
        "AutoDeleteConfig",
        "Startup",
        "timeverification",
        "Enviar Para Lixeira",
        "Excluir permanentemente",
        "AdvancedOrganize",
    ]

    if CONFIG_AUTO_DELETE:
        for key in CONFIG:
            if key in Ignore_Config:
                pass
            else:
                if not path.exists(CONFIG["Folder"] + "\\" + key):
                    print(
                        f"A pasta {CONFIG['Folder']}\\ {key} não foi encontrada ou não existe"
                    )
                else:
                    for subfolder in os.listdir(CONFIG["Folder"] + "\\" + key):
                        subfolder = subfolder.upper().replace(".", "")
                        Absolute_Path = CONFIG["Folder"] + "\\" + key + "\\" + subfolder
                        if path.exists(Absolute_Path):
                            scan_files(Absolute_Path)
                            print(f"A pasta {Absolute_Path} foi Encontrada")
                        else:
                            print(f"Pasta {Absolute_Path} não encontrada ou não existe")
    else:
        print("Configuração de auto-delete desativada")


if __name__ == "__main__":
    AutoDelete()
