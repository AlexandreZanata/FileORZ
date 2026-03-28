from utils.model import load_config, save_config

CONFIG = load_config()

class TimeVerification:
    def __init__(self, time: str):
        self._time = time

    def Gettime(self):
        return CONFIG["timeverification"]

    def time(self):
        CONFIG["timeverification"] = self._time
        save_config(CONFIG)

class DaysAutoDelete:
    def __init__(self, days: str):
        self._days = days

    def Setdays(self):
        CONFIG["AutoDeleteConfig"]["Dias para Auto Deletar"] = self._days
        save_config(CONFIG)

    def GetDays(self):
        return CONFIG["AutoDeleteConfig"]["Dias para Auto Deletar"]

if __name__ == "__main__":
    DaysAutoDelete = DaysAutoDelete("20")
    DaysAutoDelete.Setdays()
    print(f"Dias: {DaysAutoDelete.GetDays()}")