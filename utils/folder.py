from utils.model import load_config, save_config


class Folder:
    def __init__(self, folder: str = ""):
        self._folderOrz = folder

    @property
    def folder(self) -> str:
        return self._folderOrz

    @folder.getter
    def Getfolder(self) -> str:
        CONFIG = load_config("dist", "config")
        return CONFIG["Folder"]

    @folder.setter
    def folder(self, folder: str):
        CONFIG = load_config("dist", "config")
        if CONFIG["timeverification"]:
            pass
        if folder != "":
            if folder != Folder.Getfolder:
                CONFIG["Folder"] = folder.replace("/", "\\")
                save_config("dist", "config", CONFIG)
            else:
                pass
        else:
            pass

if __name__ == "__main__":
    # folder = Folder()
    # folder.folder = "c:/users/Thayn/downloads"
    # print(folder.Getfolder)
    ...