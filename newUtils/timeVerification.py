from utils.model import json_path, load_config, save_config

JSON_PATH = json_path()
CONFIG = load_config()

class TimeVerification:
    def __init__(self, time: str = ""):
        self._time = time

    @property
    def time(self):
        return self._time

    @time.getter
    def Gettime(self):
        return CONFIG["timeverification"]

    @time.setter
    def time(self, time: str):
        CONFIG["timeverification"] = time
        save_config(CONFIG)

class DaysAutoDelete:
    def __init__(self, days: int = 0):
        self._days = days

    @property
    def days(self):
        return self._days

    @days.getter
    def GetDays(self):
        return CONFIG["AutoDeleteConfig"]["Dias para Auto Deletar"]

    @days.setter
    def days(self, days: int):
        CONFIG["AutoDeleteConfig"]["Dias para Auto Deletar"] = days
        save_config(CONFIG)

if __name__ == "__main__":
    TimeVerification = TimeVerification()
    TimeVerification.time = "20"
    print(f"Time: {TimeVerification.Gettime}")
    print()

    DaysAutoDelete = DaysAutoDelete()
    DaysAutoDelete.days = "60"
    print(f"Dias: {DaysAutoDelete.GetDays}")