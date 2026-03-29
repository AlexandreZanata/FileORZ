from utils.model import json_path, load_config, save_config

JSON_PATH = json_path()
CONFIG = load_config()

class Folder:
    def __init__(self, folder: str = ""):
        self._folderOrz = folder

    @property
    def folder(self) -> str:
        return self._folderOrz

    @folder.getter
    def Getfolder(self) -> str:
        return CONFIG["Folder"]

    @folder.setter
    def folder(self, folder: str):
        if CONFIG["timeverification"]:
            pass
        if folder != "":
            if folder != Folder.Getfolder:
                CONFIG["Folder"] = folder
                save_config(CONFIG)
            else:
                pass
        else:
            pass
if __name__ == "__main__":
    folder = Folder()
    folder.folder = "c:/users/Thayn/downloads"
    print(folder.Getfolder)