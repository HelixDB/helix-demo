import helix
from helix.client import Query
from helix.types import Payload
import csv
from typing import List

class create_data(Query):
    def __init__(
        self,
        doctor_name: str,
        doctor_city: str,
        patient_name: str,
        patient_age: int,
        summary: str,
    ):
        super().__init__()
        self.doctor_name = doctor_name
        self.doctor_city = doctor_city
        self.patient_name = patient_name
        self.patient_age = patient_age
        self.summary = summary
    def query(self) -> List[Payload]:
        return [{
            "doctor_name": self.doctor_name,
            "doctor_city": self.doctor_city,
            "patient_name": self.patient_name,
            "patient_age": self.patient_age,
            "summary": self.summary
        }]
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
    for patient in patients:
        db.query(create_data(patient[2], patient[1], patient[0], patient[3], patient[4]))

        print(f"Patient Name: {patient[0]}")
        print(f"Hospital City: {patient[1]}")
        print(f"Doctor Name: {patient[2]}")
        print(f"Age: {patient[3]}")
        print(f"Consultation Description: {patient[4]}")
        print("-" * 50)