import helix
from helix.client import Query
from helix.types import Payload
from typing import List
import time
from datetime import datetime, timedelta

class get_patients_visits_in_previous_month(Query):
    def __init__(self, name: str, date: int):
        super().__init__()
        self.name = name
        self.date = date
    def query(self) -> List[Payload]:
        return [{ "name": self.name, "date": self.date}]
    def response(self, response): return response

if __name__ == "__main__":
    db = helix.Client(local=True)

    now = datetime.now()
    yesterday = now - timedelta(days=30)
    timestamp = yesterday.timestamp()

    res = db.query(get_patients_visits_in_previous_month("Owen Brooks", int(timestamp)))
    print(res)