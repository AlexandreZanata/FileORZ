from utils.model import load_config, save_config

CONFIG = load_config()

class TimeVerification:
    def __init__(self, time: str):
        self._time = time

    def time(self):
        if CONFIG["timeverification"] != self._time:
            CONFIG["timeverification"] = self._time
            save_config(CONFIG)
        else:
            pass

    def Gettime(self):
        return CONFIG["timeverification"]

class DaysAutoDelete:
    def __init__(self, days: str):
        self._days = days

    def Setdays(self):
        CONFIG["AutoDeleteConfig"]["Dias para Auto Deletar"] = self._days
        save_config(CONFIG)

    def GetDays(self):
        return CONFIG["AutoDeleteConfig"]["Dias para Auto Deletar"]

# if __name__ == "__main__":
#     Time = TimeVerification("5")
#     Time.time()