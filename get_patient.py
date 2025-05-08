import helix
from helix.client import Query
from helix.types import Payload
from typing import List

class get_patient(Query):
    def __init__(self, name: str):
        super().__init__()
        self.name = name
    def query(self) -> List[Payload]:
        return [{ "name": self.name }]
    def response(self, response): return response

if __name__ == "__main__":
    db = helix.Client(local=True)
    res = db.query(get_patient("Owen Brooks"))
    print(res)