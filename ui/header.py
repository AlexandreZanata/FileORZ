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

import webbrowser

import customtkinter

from utils import StartUp
from utils.model import toggle_startup as toggle_startup_registry

COLORS = {
    "header_gradient_start": "#667eea",
    "header_gradient_end": "#764ba2",
    "header_bg": "#1E1E3F",
    "button_bg": "#2D2D44",
    "button_hover": "#3D3D54",
    "button_border": "#4D4D64",
    "text_primary": "#FFFFFF",
    "accent": "#9D4EDD",
    "accent_hover": "#7B2CBF",
    "switch_progress": "#9D4EDD",
    "switch_bg": "#2D2D44",
}

def changelog_button(parent):
    btn = customtkinter.CTkButton(
        parent, 
        text="Changelog",
        font=customtkinter.CTkFont(family="Segoe UI", size=12, weight="bold"),
        fg_color=COLORS["button_bg"], 
        border_width=1,
        border_color=COLORS["button_border"],
        corner_radius=8,
        height=32,
        width=90,
        hover_color=COLORS["button_hover"],
        text_color=COLORS["text_primary"]
    )
    btn.bind("<Button-1>", lambda event:
    webbrowser.open("https://thainanviniciuskatchan.github.io/FileORZ/changelog.html"))
    return btn

def git_button(parent):
    btn = customtkinter.CTkButton(
        parent, 
        text="GitHub",
        font=customtkinter.CTkFont(family="Segoe UI", size=12, weight="bold"),
        fg_color=COLORS["button_bg"], 
        border_width=1,
        border_color=COLORS["button_border"],
        corner_radius=8,
        height=32,
        width=90,
        hover_color=COLORS["button_hover"],
        text_color=COLORS["text_primary"]
    )
    btn.bind("<Button-1>", lambda event: webbrowser.open("https://github.com/ThainanViniciusKatchan/FileORZ"))
    return btn

# Essa Função Controla a Inicialização da Aplicação no Windows, Alterando o Json de Configuração
# e Criando o Registro de StartUp
def startup_button(parent):
    Start = StartUp.StartUpSys()
    startup_var = customtkinter.BooleanVar(value=Start.GetEnabled)
    config_obj = StartUp.StartUpSys()
    print(f"config StartUp: {config_obj.GetEnabled}")

    def toggle_startup():
        new_value = startup_var.get()
        print(f"Novo valor: {new_value}")
        config_obj.enabled = new_value
        toggle_startup_registry(new_value)

    startup_switch = customtkinter.CTkSwitch(
        parent,
        text="Iniciar com Windows",
        command= toggle_startup,
        variable=startup_var,
        font=customtkinter.CTkFont(family="Segoe UI", size=11),
        text_color=COLORS["text_primary"],
        fg_color=COLORS["switch_bg"],
        progress_color=COLORS["switch_progress"],
        button_color=COLORS["text_primary"],
        button_hover_color="#E0E0E0"
    )
    startup_switch.pack(side="left", padx=(0, 28))

# Função Principal que cria o header na aplicação
def header(root):
    header_frame = customtkinter.CTkFrame(
        root,
        fg_color=COLORS["header_bg"],
        corner_radius=0,
        height=60
    )
    header_frame.pack(fill="x", side="top")
    header_frame.pack_propagate(False)

    inner_container = customtkinter.CTkFrame(header_frame, fg_color="transparent")
    inner_container.pack(fill="both", expand=True, padx=20, pady=10)

    logo_frame = customtkinter.CTkFrame(inner_container, fg_color="transparent")
    logo_frame.pack(side="left", anchor="center")

    icon_label = customtkinter.CTkLabel(
        logo_frame,
        text="🗂️",
        font=customtkinter.CTkFont(size=24)
    )
    icon_label.pack(side="left", padx=(0, 5))

    # Nome da aplicação
    title_label = customtkinter.CTkLabel(
        logo_frame,
        text="FileORZ",
        font=customtkinter.CTkFont(family="Segoe UI", size=22, weight="bold"),
        text_color=COLORS["text_primary"]
    )
    title_label.pack(side="left")

    # Subtítulo
    subtitle_label = customtkinter.CTkLabel(
        logo_frame,
        text="Organizador de Arquivos",
        font=customtkinter.CTkFont(family="Segoe UI", size=10),
        text_color="#A0A0A0"
    )
    subtitle_label.pack(side="left", padx=(10, 0))

    controls_frame = customtkinter.CTkFrame(inner_container, fg_color="transparent")
    controls_frame.pack(side="right", anchor="center")

    startup_button(controls_frame)

    # Botão GitHub
    git = git_button(controls_frame)
    git.pack(side="left")

    # Botão Changelog
    changelog = changelog_button(controls_frame)
    changelog.pack(side="left", padx=(15, 0))

    return header_frame


if __name__ == "__main__":
    root = customtkinter.CTk()
    root.geometry("700x50")
    root.title("FileORZ")
    root.configure(bg=COLORS["header_bg"])
    header(root)
    root.mainloop()