import os
import ctypes
import psutil
import customtkinter as ctk
import customtkinter
from tkinter import messagebox
from utils.model import load_config

def check_if_running(TaskName):
    for proc in psutil.process_iter(['name']):
        if proc.info['name'] == TaskName:
            return True
    return False

def start_task():
    config = load_config()
    Startup = config["Startup"]

    STATUS = check_if_running("FileORZ.exe")

    if Startup == False:
       SCRIPT_DIR = os.path.join(os.getcwd(), "dist", "FileORZ.exe")
    elif Startup == True:
        SCRIPT_DIR = os.path.join(os.getenv('LOCALAPPDATA'), 'FileORZ', 'FileORZ.exe')

    if STATUS == False:
        if os.path.exists(SCRIPT_DIR):
            ctypes.windll.shell32.ShellExecuteW(
                None,
                'open',
                SCRIPT_DIR,
            None,
            None,
            1
            )
            return True
    else:
        messagebox.showinfo("Erro", "FileORZ.exe ja esta em execução")
        return False

# Iniciar a organização
def start_organizer(main_container, root, folder, feedback_label):
    # Remove label anterior se existir
    if feedback_label is not None:
        feedback_label.destroy()

    # verifica se a pasta foi selecionada
    if not folder or folder == "pasta de organização":
        feedback_label = customtkinter.CTkLabel(
            main_container,
            text="Selecione uma pasta primeiro!",
            font=customtkinter.CTkFont(family="Segoe UI", size=13, weight="bold"),
            text_color="red"
        )
        feedback_label.pack(pady=(15, 0))
        root.after(3000, lambda: feedback_label.destroy() if feedback_label.winfo_exists() else None)
        return
    else:
        if start_task():
            feedback_label = customtkinter.CTkLabel(
                main_container,
                text="Organização concluída com sucesso!",
                font=customtkinter.CTkFont(family="Segoe UI", size=13, weight="bold"),
                text_color="green"
            )
            feedback_label.pack(pady=(15, 0))
            root.after(3000, lambda: feedback_label.destroy() if feedback_label.winfo_exists() else None)
        else:
            feedback_label = customtkinter.CTkLabel(
                main_container,
                text="Erro ao iniciar o organizador!",
                font=customtkinter.CTkFont(family="Segoe UI", size=13, weight="bold"),
                text_color="red"
            )
            feedback_label.pack(pady=(15, 0))
            root.after(3000, lambda: feedback_label.destroy() if feedback_label.winfo_exists() else None)

    # verifica se o processo do organizador já está funcionando