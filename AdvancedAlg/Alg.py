import fitz
from pathlib import Path
import shutil
import os
from utils.model import load_config

CONFIG = load_config("AdvancedAlg", "Key_Words")

diretorio_base: str ="C:\\Users\\Thayn\\OneDrive\\Área de Trabalho\\Teste\\"
caminho: str = ""

def path_to_pdf():
    caminho_arquivo = ""
    for file in os.listdir(diretorio_base):
        if file.endswith(".pdf"):
            file_name = file.replace('.pdf', '')
            caminho_arquivo = f"C:\\Users\\Thayn\\OneDrive\\Área de Trabalho\\Teste\\{file_name + ".pdf"}"
    return caminho_arquivo

def path_to_destiny():
    distino: str = ""
    for tipo in CONFIG.keys():
        if tipo in CONFIG:
            distino: str = f"C:\\Users\\Thayn\\OneDrive\\Área de Trabalho\\Teste\\{tipo}"
    return distino

def organizar_documento(distino=path_to_destiny()):

    global caminho
    caminho = path_to_pdf()
    global base
    base = Path(distino)

    with fitz.open(caminho) as pdf:
        for pagina in pdf:
            # Pega o texto e joga pra maiúsculo pra não ter erro de case
            texto_completo = pagina.get_text().upper()
            return texto_completo

texto = organizar_documento()

def processar_texto(texto_completo=texto):
    # 2. Procura as palavras-chave
    tipo_doc = None

    for tipo, palavras in CONFIG.items():
        for palavra in palavras:
            if palavra in texto_completo:
                print(f"Encontrado: {palavra} ({tipo})")
                if tipo == "Boleto":
                    tipo_doc = "Boleto"
            elif palavra not in texto_completo:
                print(f"Não foi possível encontrar a palavra-chave {palavra} do tipo {tipo}.")

    # 3. Se achou os dois, cria a pasta e move o arquivo
    if tipo_doc:
        nome_pasta = f"{tipo_doc}"
        caminho_nova_pasta = base / nome_pasta

        # Cria a pasta
        caminho_nova_pasta.mkdir(parents=True, exist_ok=True)

        # Move o arquivo para a pasta nova
        destino_arquivo = caminho_nova_pasta / distino / nome_pasta
        for file in os.listdir(diretorio_base):
            if file.endswith(".pdf"):
                shutil.move(str(caminho), str(destino_arquivo))
                break
            else:
                print("Pasta Sem arquivos!")
        print(f"Sucesso: Arquivo {file} movido para {destino_arquivo}")
    else:
        print("Faltou encontrar o tipo do documento.")

if __name__ == "__main__":
    processar_texto()