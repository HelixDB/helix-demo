# Setup

## HelixCLI installation

```bash
curl -sSL https://install.helix-db.com | bash
```

## Verifying Installation

To verify that HelixCLI is installed correctly:

```bash
helix --version
```

## Install Helix

```bash
helix install
```

# Write Queries

> in `./helix-test/schema.hx`

```js
N::Patient {
    name: String
    age: I64
}

N::Doctor {
    name: String
    city: String
}

E::Visit {
    From: Patient
    To: Doctor
    Properties: {
        doctors_summary: String
        date: I64
    }
}
```

> in `./helix-test/queries.hx`
```js
// ------------------
// Create Data
// ------------------
QUERY createdoctor(name: String, city: String) =>
    doctor <- AddN<Doctor>({
        name: name,
        city: city
    })
    RETURN doctor

QUERY create_patient(name: String, age: I64) =>
    patient <- AddN<Patient>({
        name: name,
        age: age
    })
    RETURN patient

QUERY create_visit(patient_id: String, doctor_id: String, summary: String) =>
    visit <- AddE<Visit>({doctors_summary: summary})::From(patient_id)::To(doctor_id)
    RETURN visit

// ------------------
// Fetch Data
// ------------------
QUERY get_patient(patient_id: String) =>
    patient <- N<Patient>(patient_id)
    RETURN patient

QUERY get_patients_visits_in_previous_month(patient_id: String, date: I64) =>
    patient <- N<Patient>(patient_id)
    visits <- patient::OutE<Visit>::WHERE(_::{date}::GTE(date))
    RETURN visits

QUERY get_visit_by_date(patient_id: String, date: I64) =>
    patient <- N<Patient>(patient_id)
    visit <- patient::OutE<Visit>::WHERE(_::{date}::EQ(date))::RANGE(0, 1)
    RETURN visit

```


# Run Queries

> cd into `helix-demo` if you haven't already
```bash
helix deploy --local
```

# Send Requests

```bash
bash ./create_data.sh
```
> Run this one first

```bash
bash ./get_visit_by_date.sh
```

```bash
bash ./get_patients_visits_in_previous_month.sh
```

```bash
bash ./get_patient.sh
```



