import fitz
from pathlib import Path
import shutil
import os
from utils.model import load_config

CONFIG = load_config("AdvancedAlg", "Key_Words")

diretorio_base: str ="C:\\Users\\Thayn\\OneDrive\\Área de Trabalho\\Teste\\"

def processar_texto():
    # 2. Procura as palavras-chave
    for file in os.listdir(diretorio_base):
        caminho_completo = os.path.join(diretorio_base, file)
        if not file.endswith(".pdf"):
            continue
        with fitz.open(diretorio_base+file) as pdf:
            for pagina in pdf:
                # Pega o texto e joga pra maiúsculo pra não ter erro de case
                texto_completo = pagina.get_text().upper()

        movido = False
        for tipo, palavras in CONFIG.items():
            if movido:
                break
            for palavra in palavras:
                if palavra.upper() in texto_completo:
                    print(f"\nEncontrado: {palavra} ({tipo})")
                    pasta_destino = os.path.join(diretorio_base, tipo)
                    Path(pasta_destino).mkdir(parents=True, exist_ok=True)
                    destino_arquivo = os.path.join(pasta_destino, file)
                    Path(diretorio_base+tipo).mkdir(parents=True, exist_ok=True)
                    if os.path.exists(destino_arquivo):
                        print(f"Arquivo {file} não movido pois já existe na pasta {tipo}")
                    else:
                        try:
                            shutil.move(str(diretorio_base + file), str(diretorio_base+tipo))  # Move o arquivo para a pasta nova
                            print(f"Sucesso: Arquivo {file} movido para {pasta_destino}")
                        except Exception as e:
                            print(f"Erro! não foi possível mover o arquivo {file}: {e}")
                else:
                    print(f"Não foi possível ler a palavra f{palavra} no arquivo {file}")

if __name__ == "__main__":
    processar_texto()
