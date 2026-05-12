import pypdf
from pathlib import Path
import shutil
import os
from utils.model import load_config
from utils import folder

CONFIG = load_config("dist", "Key_Words")

diretorio_base: str = folder.Folder().Getfolder + "\\"
print(diretorio_base)

def processar_texto():
    # 2. Procura as palavras-chave
    for file in os.listdir(diretorio_base):
        texto_completo = ""
        caminho_completo = os.path.join(diretorio_base, file)
        if not file.endswith(".pdf"):
            continue
        with pypdf.PdfReader(caminho_completo) as pdf:
            try:
                for pagina in pdf.pages:
                    # Pega o texto e joga pra maiúsculo pra não ter erro de case
                    texto_completo = pagina.extract_text()
                    texto_completo = texto_completo.upper()
            except Exception as e:
                print(f"Não foi possivel ler o arquivo PDF {e}")

        movido = False
        for tipo, palavras in CONFIG.items():
            if movido:
                break
            for palavra in palavras:
                if palavra.upper() in texto_completo:
                    print(f"\nEncontrado: {palavra} ({tipo})")
                    pasta_destino = os.path.join(diretorio_base, tipo)
                    Path(diretorio_base+tipo).mkdir(parents=True, exist_ok=True)
                    destino_arquivo = os.path.join(pasta_destino, file)
                    if os.path.exists(destino_arquivo):
                        print(f"Arquivo {file} não movido pois já existe na pasta {tipo}")
                    else:
                        try:
                            shutil.move(str(caminho_completo), str(destino_arquivo))  # Move o arquivo para a pasta nova
                            movido = True
                            print(f"Sucesso: Arquivo {file} movido para {pasta_destino}")
                        except Exception as e:
                            print(f"Erro! não foi possível mover o arquivo {file}: {e}")
                else:
                    print(f"Não foi possível ler a palavra {palavra} no arquivo {file}")

if __name__ == "__main__":
    processar_texto()
