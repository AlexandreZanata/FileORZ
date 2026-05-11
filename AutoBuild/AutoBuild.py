import os
import sys
import subprocess
import io
import shutil
import json
import stat
import time
from time import sleep

# Força o encoding do stdout para utf-8
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

# Adiciona o caminho raiz ao sys.path para importar utils
BASE_DIR = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, BASE_DIR)

from utils.model import load_config, save_config
from utils.RsaCryptography import generate_keys, sign_file, verify_file

# Pasta de saída para as builds
OUTPUT_DIR = "FileORZ"

def handle_remove_readonly(func, path, exc):
    """Trata erros de permissão ao remover arquivos (especialmente no Windows)."""
    excvalue = exc[1]
    if func in (os.rmdir, os.remove, os.unlink) and getattr(excvalue, 'errno', None) == 13: # Access Denied
        os.chmod(path, stat.S_IRWXU| stat.S_IRWXG| stat.S_IRWXO) # 0777
        func(path)
    else:
        raise

def robust_remove(path):
    """Remove um arquivo ou diretório de forma robusta com retentativas."""
    if not os.path.exists(path):
        return
    
    for i in range(3):
        try:
            if os.path.isdir(path):
                shutil.rmtree(path, onerror=handle_remove_readonly)
            else:
                os.chmod(path, stat.S_IWRITE)
                os.remove(path)
            return
        except Exception as e:
            if i == 2:
                print(f"  [AVISO] Não foi possível remover {path}: {e}")
            else:
                sleep(1)

def matar_processos():
    print("\nEncerrando processos que possam bloquear arquivos...")
    processos = ["FileORZ.exe", "FL_ORZ.exe", "index.exe", "FileORZ.bin"]
    for proc in processos:
        try:
            # Tenta encerrar o processo silenciosamente
            subprocess.run(["taskkill", "/F", "/IM", proc, "/T"], 
                         capture_output=True, text=True, check=False)
        except:
            pass
    sleep(1)

def limpar_builds_anteriores():
    print("\nLimpando builds anteriores...")
    deletar_dados = ['build', OUTPUT_DIR, '__pycache__', 'index.build', 'index.dist', 
                    'FileORZ.build', 'FileORZ.dist', 'FL_ORZ.build', 'FL_ORZ.dist', 'dist']
    
    for dado in deletar_dados:
        caminho = os.path.join(BASE_DIR, dado)
        if os.path.exists(caminho):
            robust_remove(caminho)
            if not os.path.exists(caminho):
                print(f"  [OK] {dado} removido")
    
    for arquivo in os.listdir(BASE_DIR):
        if arquivo.endswith('.cmd') or arquivo.endswith('.pyi'):
            try:
                os.remove(os.path.join(BASE_DIR, arquivo))
                print(f"  [OK] Arquivo {arquivo} removido")
            except:
                pass

def criar_pasta_build():
    print("\nCriando estrutura de pastas...")
    output_path = os.path.join(BASE_DIR, OUTPUT_DIR)
    dist_path = os.path.join(output_path, "dist")
    os.makedirs(output_path, exist_ok=True)
    os.makedirs(dist_path, exist_ok=True)
    print(f"  [OK] Estrutura {OUTPUT_DIR}/ e {OUTPUT_DIR}/dist/ criada")

def compilar_organizador():
    print("\nCompilando o organizador (FileORZ.py) com Nuitka...")
    os.chdir(BASE_DIR)
    dist_path = os.path.join(OUTPUT_DIR, "dist")
    
    comando = [
        sys.executable, '-m', 'nuitka',
        '--standalone',                          
        '--windows-console-mode=disable',
        f'--output-dir={dist_path}',
        '--output-filename=FileORZ.exe',         
        '--windows-icon-from-ico=ui/icon/IconApp.ico',
        '--assume-yes-for-downloads',
        '--show-progress',
        '--show-memory',
        '--plugin-enable=tk-inter',
        'FileORZ.py'
    ]
    
    print(f"Executando: {' '.join(comando)}")
    # Usamos shell=True no Windows às vezes ajuda com o PATH do compilador
    result = subprocess.run(comando, capture_output=False, text=True)
    
    if result.returncode == 0:
        print("  [OK] Organizador compilado com sucesso")
        sleep(2) 
        return True
    else:
        print("  [ERRO] Falha ao compilar organizador")
        return False

def compilar_ui():
    print("\nCompilando a UI (index.py) com Nuitka...")
    os.chdir(BASE_DIR)
    
    comando = [
        sys.executable, '-m', 'nuitka',
        '--standalone',                          
        '--windows-console-mode=disable',       
        f'--output-dir={OUTPUT_DIR}',            
        '--output-filename=FL_ORZ.exe',          
        '--windows-icon-from-ico=ui/icon/IconApp.ico',  
        '--enable-plugin=tk-inter',              
        '--include-package=customtkinter',       
        '--include-package=PIL',                 
        '--include-package=darkdetect',          
        '--include-package=utils',               
        '--include-data-dir=ui=ui',              
        '--assume-yes-for-downloads',
        '--show-progress',
        '--show-memory',
        'ui/index.py'
    ]
    
    print(f"Executando: {' '.join(comando)}")
    result = subprocess.run(comando, capture_output=False, text=True)
    
    if result.returncode == 0:
        print("  [OK] UI compilada com sucesso")
        sleep(2) 
        return True
    else:
        print("  [ERRO] Falha ao compilar UI")
        return False

def criar_keywords_padrao():
    print("\nCriando Key_Words.json padrão...")
    keywords = {
        "Nota Fiscal": [
            "Nota Fiscal", "NÚMERO DA NOTA", "NUMERO DA NOTA", "SIMPLES NACIONAL",
            "NOTA FISCAL DE SERVIÇOS ELETRÔNICA", "NOTA FISCAL DE SERVIÇOS ELETRONICA,",
            "Nota Fiscal Eletrônica", "Nota Fiscal Eletronica", "NFS-e", "NFS",
            "DANFE", "Série", "Serie", "PIS", "COFINS"
        ],
        "Contratos": [
            "Contrato", "contrato", "Assinatura", "Cláusula", "Documento",
            "documento", "Fiador", "fiador"
        ],
        "Boleto": [
            "Data de validade", "Código de barras", "nome do beneficiário",
            "Boleto", "Vencimento", "quantidade de parcela", "parcela"
        ]
    }
    
    # Salva na pasta da build/dist
    keywords_path = os.path.join(BASE_DIR, OUTPUT_DIR, 'dist', 'Key_Words.json')
    
    try:
        os.makedirs(os.path.dirname(keywords_path), exist_ok=True)
        with open(keywords_path, 'w', encoding='utf-8') as f:
            json.dump(keywords, f, indent=4, ensure_ascii=False)
        print(f"  [OK] Key_Words.json criado em: {keywords_path}")
    except Exception as e:
        print(f"  [ERRO] Falha ao criar Key_Words em {keywords_path}: {e}")

def reorganizar_estrutura():
    print("\nReorganizando estrutura de arquivos...")
    output_path = os.path.join(BASE_DIR, OUTPUT_DIR)
    
    # 1. Reorganizar index.dist (UI)
    index_dist = os.path.join(output_path, "index.dist")
    if os.path.exists(index_dist):
        print(f"  Movendo arquivos de index.dist/ para {OUTPUT_DIR}/")
        for item in os.listdir(index_dist):
            origem = os.path.join(index_dist, item)
            destino = os.path.join(output_path, item)
            
            # Tenta mover com retentativas se houver erro de acesso
            for i in range(5):
                try:
                    if os.path.exists(destino):
                        robust_remove(destino)
                    shutil.move(origem, destino)
                    break
                except Exception as e:
                    if i == 4:
                        print(f"  [ERRO] Falha ao mover {item}: {e}")
                        raise
                    sleep(1)
        robust_remove(index_dist)
    
    # 2. Reorganizar FileORZ.dist (Organizador)
    fileorz_dist_orig = os.path.join(output_path, "dist", "FileORZ.dist")
    dist_final = os.path.join(output_path, "dist")
    if os.path.exists(fileorz_dist_orig):
        print(f"  Movendo arquivos de dist/FileORZ.dist/ para {OUTPUT_DIR}/dist/")
        for item in os.listdir(fileorz_dist_orig):
            origem = os.path.join(fileorz_dist_orig, item)
            destino = os.path.join(dist_final, item)
            
            for i in range(5):
                try:
                    if os.path.exists(destino):
                        robust_remove(destino)
                    shutil.move(origem, destino)
                    break
                except Exception as e:
                    if i == 4:
                        print(f"  [ERRO] Falha ao mover {item}: {e}")
                        raise
                    sleep(1)
        robust_remove(fileorz_dist_orig)

def criar_config_padrao():
    print("\nCriando config.json padrão...")
    config = {
        "Desenvolvimento": {
            ".bin": True, ".htm": True, ".html": True, ".cfg": True, ".alg": True,
            ".md": True, ".ftl": True, ".json": True, ".py": True, ".bat": True,
            ".cmd": True, ".ps1": True, ".sh": True, ".ini": True, ".js": True,
            ".ts": True, ".css": True, ".java": True, ".cpp": True, ".cs": True,
            ".php": True, ".c": True, ".net": True, ".pyd": True, ".lexical": True,
            ".dll": True
        },
        "documentos": {
            ".pdf": True, ".doc": True, ".txt": True, ".pptx": True, ".docx": True,
            ".xlsx": True, ".xlsm": True, ".csv": True, ".xls": True, ".dotm": True,
            ".ponto": True, ".dotx": True, ".htm": True, ".html": True, ".cfg": True,
            ".alg": True, ".ftl": True, ".ppt": True, ".md": True
        },
        "videos": {
            ".mov": True, ".mp4": True, ".avi": True, ".av1": True, ".mpeg-2": True,
            ".avchd": True, ".aac": True, ".mkv": True, ".divx": True, ".h.264": True,
            ".mpeg-1": True, ".wmv": True
        },
        "audios": {
            ".mp3": True, ".wav": True, ".flac": True, ".3GP": True, ".M4A": True,
            ".ogg": True, ".wma": True, ".m4a": True, ".webm": True
        },
        "compactos": {
            ".rar": True, ".zip": True, ".zpix": True, ".7z": True, ".rar5": True,
            ".iso": True, ".gzip": True, ".7-zip": True, ".tar": True
        },
        "fontes": {
            ".ttf": True, ".eot": True, ".woff": True, ".woff2": True
        },
        "setups": {
            ".exe": True, ".msi": True, ".appx": True, ".appxbundle": True,
            ".msix": True, ".apk": True, ".Msixbundle": True
        },
        "imagens": {
            ".jpg": True, ".jpeg": True, ".png": True, ".bmp": True, ".tiff": True,
            ".gif": True, ".cr3": True, ".cr2": True, ".exif": True, ".psd": True,
            ".af": True, ".eps": True, ".ai": True, ".svg": True, ".webp": True,
            ".heic": True, ".heif": True, ".raw": True, ".img": True
        },
        "timeverification": "5",
        "Startup": False,
        "Folder": "pasta de organização",
        "AutoDelete": False,
        "Enviar Para Lixeira": False,
        "Excluir permanentemente": False,
        "AdvancedOrganize": False,
        "AutoDeleteConfig": {
            "Por Data de Criação": False,
            "Por Data de Modificação": False,
            "Dias para Auto Deletar": "15"
        }
    }
    
    # Salva na pasta do projeto para referência (opcional, mantido conforme original)
    dist_proj_config = os.path.join(BASE_DIR, 'dist', 'config.json')
    os.makedirs(os.path.dirname(dist_proj_config), exist_ok=True)
    
    # Salva na pasta da build
    build_config_path = os.path.join(BASE_DIR, OUTPUT_DIR, 'dist', 'config.json')

    for path in [dist_proj_config, build_config_path]:
        try:
            with open(path, 'w', encoding='utf-8') as f:
                json.dump(config, f, indent=4, ensure_ascii=False)
            print(f"  [OK] config.json criado em: {path}")
        except Exception as e:
            print(f"  [ERRO] Falha ao criar config em {path}: {e}")

def alterar_config_build():
    print("\nAjustando configurações da build...")
    try:
        # Carrega e salva usando as funções de utils para garantir consistência
        config_path = os.path.join(BASE_DIR, OUTPUT_DIR, 'dist', 'config.json')
        if os.path.exists(config_path):
            with open(config_path, 'r', encoding='utf-8') as f:
                config = json.load(f)
            
            config["timeverification"] = "5"
            config['Startup'] = False
            config['Folder'] = 'pasta de organização'
            
            with open(config_path, 'w', encoding='utf-8') as f:
                json.dump(config, f, indent=4, ensure_ascii=False)
            print("  [OK] Configurações da build ajustadas")
        else:
            print("  [AVISO] config.json não encontrado na build para ajuste")
    except Exception as e:
        print(f"  [ERRO] Erro ao ajustar config: {e}")

def limpar_temporarios():
    print("\nLimpando arquivos temporários...")
    pastas_temp = ['index.build', 'index.dist', 'index.onefile-build',
                   'FileORZ.build', 'FileORZ.dist', 'FileORZ.onefile-build']
    
    for pasta in pastas_temp:
        caminho = os.path.join(BASE_DIR, pasta)
        if os.path.exists(caminho):
            robust_remove(caminho)
            if not os.path.exists(caminho):
                print(f"  [OK] {pasta} removida")

def assinar_binarios():
    print("\nAssinando binários...")
    exe_path = os.path.join(BASE_DIR, OUTPUT_DIR, 'dist', 'FileORZ.exe')
    if os.path.exists(exe_path):
        try:
            generate_keys()
            sign_file(exe_path, 'private_key.pem')
            # verify_file precisa de 3 argumentos conforme definição
            verify_file(exe_path, exe_path + ".sig", 'public_key.pem')
            print("  [OK] Binário assinado com sucesso")
        except Exception as e:
            print(f"  [ERRO] Erro ao assinar binário: {e}")
    else:
        print("  [AVISO] Executável não encontrado para assinatura")

if __name__ == "__main__":
    print("\n" + "=" * 50)
    print("INICIANDO BUILD: FileORZ")
    print("=" * 50)
    sleep(1)

    ETAPAS = [
        ("Matar processos existentes", matar_processos),
        ("Limpar builds anteriores", limpar_builds_anteriores),
        ("Criar pasta de build", criar_pasta_build),
        ("Compilar UI", compilar_ui),
        ("Compilar Organizador", compilar_organizador),
        ("Reorganizar estrutura", reorganizar_estrutura),
        ("Criar Key_Words padrão", criar_keywords_padrao),
        ("Criar config padrão", criar_config_padrao),
        ("Ajustar configurações", alterar_config_build),
        ("Limpar arquivos temporários", limpar_temporarios),
        ("Assinar binários", assinar_binarios)
    ]

    for nome, func in ETAPAS:
        print(f"\n>>> {nome}")
        try:
            func()
            print(f"--- {nome} concluído ---")
        except Exception as e:
            print(f"🛑 ERRO FATAL em {nome}: {e}")
            sys.exit(1)
        sleep(1)

    print("\n" + "=" * 50)
    print("✅ COMPILAÇÃO CONCLUÍDA COM SUCESSO!")
    print("=" * 50)