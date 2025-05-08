import helix
from helix.client import Query
from helix.types import Payload
from typing import List
import time

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
    res = db.query(get_visit_by_date("Owen Brooks", int(time.time() * 1000)))
    print(res)