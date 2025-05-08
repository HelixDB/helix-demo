import helix
from helix.client import Query
from helix.types import Payload
from typing import List

class get_visit_by_date(Query):
    def __init__(self, patient_id: str, date: int):
        super().__init__()
        self.patient_id = patient_id
        self.date = date
    def query(self) -> List[Payload]:
        return [{ "patient_id": self.patient_id, "date": self.date }]
    def response(self, response): return response

if __name__ == "__main__":
    db = helix.Client(local=True)
    res = db.query(get_visit_by_date("Owen Brooks", ))
    print(res)