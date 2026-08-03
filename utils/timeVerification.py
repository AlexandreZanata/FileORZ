from utils.model import load_config, save_config


class TimeVerification:
    def __init__(self, time: str):
        self._time = time

    def time(self):
        CONFIG = load_config("dist", "config")
        if CONFIG["timeverification"] != self._time:
            CONFIG["timeverification"] = self._time
            save_config("dist", "config", CONFIG)
        else:
            pass

    def Gettime(self):
        CONFIG = load_config("dist", "config")
        return CONFIG["timeverification"]

class DaysAutoDelete:
    def __init__(self, days: str):
        self._days = days

    def Setdays(self):
        CONFIG = load_config("dist", "config")
        CONFIG["AutoDeleteConfig"]["Dias para Auto Deletar"] = self._days
        save_config("dist", "config", CONFIG)

    def GetDays(self):
        CONFIG = load_config("dist", "config")
        return CONFIG["AutoDeleteConfig"]["Dias para Auto Deletar"]

# if __name__ == "__main__":
#     Time = TimeVerification("5")
#     Time.time()