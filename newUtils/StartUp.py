from utils.model import json_path, load_config, save_config

JSON_PATH = json_path()
CONFIG = load_config()

class StartUpSys:
    def __init__(self, value: bool = False):
        self._value = value

    @property
    def enabled(self):
        return self._value

    @enabled.getter
    def GetEnabled(self):
        return CONFIG["Startup"]

    @enabled.setter
    def enabled(self, value: bool):
        CONFIG["Startup"] = value
        save_config(CONFIG)

if __name__ == "__main__":
    # StartUp = StartUpSys()
    # StartUp.enabled = True
    # print(StartUp.GetEnabled)
    ...