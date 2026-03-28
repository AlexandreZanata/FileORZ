from utils.model import json_path, load_config, save_config

JSON_PATH = json_path()
CONFIG = load_config()


class StartUp:
    def __init__(self, enabled: bool = False):
        self._enabled = enabled

    @property
    def enabled(self):
        return self._enabled

    @enabled.getter
    def GetEnabled(self):
        return CONFIG["Startup"]

    @enabled.setter
    def enabled(self, enabled: bool):
        CONFIG["Startup"] = enabled
        save_config(CONFIG)

if __name__ == "__main__":
    StartUp = StartUp()
    StartUp.enabled = True
    print(StartUp.GetEnabled)