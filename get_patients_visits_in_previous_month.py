import helix
from helix.client import Query
from helix.types import Payload
from typing import List

class get_patients_visits_in_previous_month(Query):
    def __init__(self, patient_id: str):
        super().__init__()
        self.patient_id = patient_id
    def query(self) -> List[Payload]:
        return [{ "patient_id": self.patient_id }]
    def response(self, response): return response

if __name__ == "__main__":
    db = helix.Client(local=True)
    res = db.query(get_patients_visits_in_previous_month("Owen Brooks"))
    print(res)