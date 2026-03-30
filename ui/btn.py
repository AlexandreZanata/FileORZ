import customtkinter
from customtkinter import filedialog
from ui.config import open_config_window
from ui.Config_AutoDell import open_Windows_CFG_autoDell


def config_btn(COLORS, actions_frame, root):
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

from utils.StartTask import start_organizer
def start_btn(COLORS, actions_frame):
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