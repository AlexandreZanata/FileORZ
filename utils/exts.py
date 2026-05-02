from utils.model import json_path, load_config, save_config

JSON_PATH = json_path("dist", "config")
CONFIG = load_config("dist", "config")

class Extensions:
    def __init__(self, category: str = "" ,name: str = "", value: bool = bool):
        self._name = name
        self._value = value
        self._category = category

    @property
    def name(self):
        return f"EXT Json: {self._name}"

    @property
    def value(self):
        self._value = CONFIG[self._category][self._name]
        return f"Value Json: {self._value}"

    @property
    def category(self):
        return f"Category Json: {self._category}"

    @name.setter
    def name(self, name: str):
        self._name = name

    @category.setter
    def category(self, category: str):
        self._category = category

    @value.setter
    def value(self, value: bool):
            CONFIG[self._category][self._name] = value
            save_config("dist", "config", CONFIG)
            return f" [✔] {self._name} foi {self._value} com sucesso!"

    # def add_ext(self, name: str, category: str, value: bool = bool):
    #     self._name = name
    #     self._category = category
    #     self._value = value
    #     if self._name not in CONFIG[self._category]:
    #         CONFIG[self._category] = {self._name: self._value}

    def replace_all_value(self, value: bool, category: str):
        for ext in CONFIG[category]:
            try:
                CONFIG[category][ext] = value
                save_config("dist", "config", CONFIG)
            except Exception as e:
                print(f"[X] Erro ao mudar a Chave {ext}: {e}")
        return f"Todas as chaves Foram Alteradas com Sucesso!"

if __name__ == "__main__":
    EXT = Extensions()
    print(EXT.replace_all_value(True, "documentos"))