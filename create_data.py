import helix
from helix.client import Query
from helix.types import Payload
import csv
from typing import List, Tuple

class create_doctor(Query):
    def __init__(self, doctors: List[Tuple[str, str]]):
        super().__init__()
        self.doctors = doctors
    def query(self) -> List[Payload]:
        docs_payload = []
        for name, city in self.doctors:
            docs_payload.append({ "name": name, "city": city })
        return [{ "doctors": docs_payload }]
    def response(self, response): return response

class create_patient(Query):
    def __init__(self, name: str, age: int):
        super().__init__()
        self.name = name
        self.age = age
    def query(self) -> List[Payload]:
        return [{ "name": self.name, "age": self.age }]
    def response(self, response): return response

class create_visit(Query):
    def __init__(self, pat_id: str, doc_id: str, summary: str):
        super().__init__()
        self.pat_id = pat_id
        self.doc_id = doc_id
        self.summary = summary
    def query(self) -> List[Payload]:
        return [{ "patient_id": self.pat_id, "doctor_id": self.doc_id, "summary": self.summary }]
    def response(self, response): return response

def read_patient_records(file_path):
    patient_data = []
    try:
        with open(file_path, mode='r', encoding='utf-8') as file:
            csv_reader = csv.DictReader(file)
            for row in csv_reader:
                patient_tuple = (
                    row['Patient_Name'],
                    row['Hospital_City'],
                    row['Doctor_Name'],
                    int(row['Age']),
                    row['Doctor_Consultation_Desc']
                )
                patient_data.append(patient_tuple)
        return patient_data
    except FileNotFoundError:
        print(f"Error: The file '{file_path}' was not found.")
        return []
    except Exception as e:
        print(f"Error: An unexpected error occurred: {e}")
        return []

if __name__ == "__main__":
    db = helix.Client(local=True)

    file_path = "patients.csv"
    patients = read_patient_records(file_path)
    doctors = [] # (name, city)
    for patient in patients:
        print(f"Patient Name: {patient[0]}")
        print(f"Hospital City: {patient[1]}")
        print(f"Doctor Name: {patient[2]}")
        print(f"Age: {patient[3]}")
        print(f"Consultation Description: {patient[4]}")
        print("-" * 50)

        doctors.append((patient[2],patient[1]))

        db.query(create_patient(patient[0], patient[3]))
        db.query(create_visit(patient[0], patient[2], patient[4]))

    db.query(create_doctor(doctors))