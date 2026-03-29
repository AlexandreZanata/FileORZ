import customtkinter
from customtkinter import filedialog
import os
import sys
from ui.config import open_config_window
from ui.Config_AutoDell import open_Windows_CFG_autoDell
from ui.header import header
from ui.Centralizar_Janela import Centralizar_Janela
from ui.Select_Folder import folder_select
from ui.Time_Select import time_select
import ctypes

# TODO:
#   Descobrir por que quando mudo o valor do tempo de verificação o caminho da pasta no config volta para último.
#   Descobrir por que o tempo não muda no arquivo de config.
#   Descobrir por que quando desativo o auto deletar a incialização com o sistema também desativa.
#   Finalizar a refatoração do código melhorando a escrita do front-end

from newUtils import timeVerification, folder
from utils.StartTask import start_organizer

# Padrão de cores
COLORS = {
    "bg_primary": "#0D0D0D",
    "bg_secondary": "#1A1A2E",
    "bg_card": "#16213E",
    "accent_primary": "#9D4EDD",
    "accent_hover": "#7B2CBF",
    "accent_success": "#06D6A0",
    "accent_success_hover": "#05B88A",
    "text_primary": "#FFFFFF",
    "text_secondary": "#A0A0A0",
    "text_muted": "#6C6C6C",
    "border": "#2D2D44",
    "button_secondary": "#2D2D44",
    "button_secondary_hover": "#3D3D54",
    "dropdown_bg": "#1A1A2E",
}

ORZ = 'FLORZ'
ctypes.windll.shell32.SetCurrentProcessExplicitAppUserModelID(ORZ)

Time = timeVerification
Folder = folder.Folder()

# Busca a pasta de execução da aplicação
if getattr(sys, "frozen", False):
    base_path = getattr(sys, "_MEIPASS", os.path.dirname(sys.executable))
else:
    base_path = os.path.dirname(os.path.abspath(__file__))

icon_path = os.path.join(base_path, "icon", "IconApp.ico")

root = customtkinter.CTk()
root.title("File ORZ")

# Busca o Icone da aplicação
if os.path.exists(icon_path):
    root.iconbitmap(default=icon_path)
else:
    print(f"[AVISO] Ícone não encontrado em: {icon_path}")

root.geometry("700x420")
root.configure(fg_color=COLORS["bg_primary"])
root.resizable(False, False)

# Header
header(root)
# Centralize window
Centralizar_Janela(root, 700, 420)
main_container = customtkinter.CTkFrame(root, fg_color="transparent")
main_container.pack(fill="both", expand=True, padx=30, pady=20)
folder_select(main_container, COLORS)
time_select(main_container, COLORS)

actions_frame = customtkinter.CTkFrame(main_container, fg_color="transparent")
actions_frame.pack(fill="x", pady=(10, 0))

# Label de feedback
feedback_label = None

# Botão de configurações (esquerda)
btn_config = customtkinter.CTkButton(
    actions_frame,
    text="⚙️  Configurar Organizador",
    command=lambda: open_config_window(root),
    fg_color=COLORS["button_secondary"],
    hover_color=COLORS["button_secondary_hover"],
    border_width=0,
    corner_radius=10,
    font=customtkinter.CTkFont(family="Segoe UI", size=13, weight="bold"),
    width=160,
    height=48
)
btn_config.pack(side="left")

btn_config_autoDell = customtkinter.CTkButton(
    actions_frame,
    text="⚙️  Configurar AutoDeletar",
    command=lambda: open_Windows_CFG_autoDell(root),
    fg_color=COLORS["button_secondary"],
    hover_color=COLORS["button_secondary_hover"],
    border_width=0,
    corner_radius=10,
    font=customtkinter.CTkFont(family="Segoe UI", size=13, weight="bold"),
    width=160,
    height=48
)
btn_config_autoDell.pack(side="left", padx=(29, 0))

# Botão para iniciar a organização
btn_Start_Organizer = customtkinter.CTkButton(
    actions_frame,
    text="🚀  Iniciar Organização",
    command=lambda:start_organizer,
    fg_color=COLORS["accent_success"],
    hover_color=COLORS["accent_success_hover"],
    corner_radius=10,
    border_width=0,
    font=customtkinter.CTkFont(family="Segoe UI", size=14, weight="bold"),
    width=200,
    height=48
)
btn_Start_Organizer.pack(side="right")

footer = customtkinter.CTkLabel(
    root,
    text="File ORZ - Organize seus arquivos",
    font=customtkinter.CTkFont(family="Segoe UI", size=10),
    text_color=COLORS["text_muted"]
)
footer.pack(side="bottom", pady=10)

root.resizable(False, False)
root.mainloop()

# if __name__ == "__main__":
#     start_organizer()