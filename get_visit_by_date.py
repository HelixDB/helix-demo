import helix
from helix.client import Query
from helix.types import Payload
from typing import List
from datetime import datetime, timedelta

class get_visit_by_date(Query):
    def __init__(self, name: str, date: int):
        super().__init__()
        self.name = name
        self.date = date
    def query(self) -> List[Payload]:
        return [{ "name": self.name, "date": self.date }]
    def response(self, response): return response

if __name__ == "__main__":
    db = helix.Client(local=True)

    date = "2025-04-18 10:45:00"
    dt = datetime.strptime(date, "%Y-%m-%d %H:%M:%S")

    res = db.query(get_visit_by_date("Owen Brooks", int(dt.timestamp())))
    print(res)