from utils.model import json_path, load_config, save_config

JSON_PATH = json_path()
CONFIG = load_config()


class AutoDelete:
    def __init__(self, AutoDell: bool = bool, Trash: bool = bool, Permanently: bool = bool):
        self._AutoDell = AutoDell
        self._Trash = Trash
        self._Permanently = Permanently

    @property
    def AutoDelete(self):
        return self._AutoDell

    @property
    def Lixeira(self):
        return self._Trash

    @property
    def Permanente(self):
        return self._Permanently

    def GetFilters(self):
        items = dict()
        for k, v in CONFIG.items():
            if k == "Enviar Para Lixeira" or k == "Excluir permanentemente":
                items[k] = v
        return items

    @AutoDelete.getter
    def GetAutoDelete(self):
        return CONFIG["AutoDelete"]

    @AutoDelete.setter
    def AutoDelete(self, AutoDell: bool):
        CONFIG["AutoDelete"] = AutoDell
        save_config(CONFIG)

    @Lixeira.getter
    def GetLixeira(self):
        return CONFIG["Enviar Para Lixeira"]

    @Lixeira.setter
    def Lixeira(self, Trash: bool):
        CONFIG["Enviar Para Lixeira"] = Trash
        save_config(CONFIG)

    @Permanente.getter
    def GetPermanente(self):
        return CONFIG["Excluir permanentemente"]

    @Permanente.setter
    def Permanente(self, Permanently: bool):
        CONFIG["Excluir permanentemente"] = Permanently
        save_config(CONFIG)

    def SetFilters(self, filter: str, value: bool):
        for C in  CONFIG:
            if C in AutoDelete.GetFilters(None).keys():
                if filter == C:
                    CONFIG[filter] = value
                else:
                    CONFIG[C] = False
                if filter == True and C == True:
                    raise ValueError("Ambos os filtros não podem ser ativados ao mesmo tempo")
        save_config(CONFIG)

class AutoDeleFilter:
    def __init__(self, datacriacao: bool = False, datamodificacao: bool = False):
        self._DataCriacao = datacriacao
        self._DataModificacao = datamodificacao

    @property
    def DataCriacao(self):
        return self._DataCriacao

    @property
    def DataModificacao(self):
        return self._datamodificacao

    @DataCriacao.getter
    def GetDataCriacao(self):
        return CONFIG["AutoDeleteConfig"]["Por Data de Criação"]

    @DataCriacao.setter
    def DataCriacao(self, datacriacao: bool):
        CONFIG["AutoDeleteConfig"]["Por Data de Criação"] = datacriacao
        save_config(CONFIG)

    @DataModificacao.getter
    def GetDataModificacao(self):
        return CONFIG["AutoDeleteConfig"]["Por Data de Modificação"]

    @DataModificacao.setter
    def DataModificacao(self, datamodificacao: bool):
        CONFIG["AutoDeleteConfig"]["Por Data de Modificação"] = datamodificacao
        save_config(CONFIG)

    def GetFilters(self):
        items = dict()
        for k, v in CONFIG["AutoDeleteConfig"].items():
            if k == "Por Data de Criação" or k == "Por Data de Modificação":
                items[k] = v
        return items

    def SetFilters(self, filter: str, value: bool):
        for F in  CONFIG["AutoDeleteConfig"]:
            if F in AutoDeleFilter.GetFilters(None).keys():
                if filter == F:
                    CONFIG["AutoDeleteConfig"][filter] = value
                else:
                    CONFIG["AutoDeleteConfig"][F] = False
                if filter == True and F == True:
                    raise ValueError("Ambos os filtros não podem ser ativados ao mesmo tempo")
        save_config(CONFIG)

if __name__ == "__main__":
    filters = AutoDeleFilter()
    filters.SetFilters("Por Data de Modificação", True)