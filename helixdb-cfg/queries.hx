// ------------------
// Create Data
// ------------------
QUERY createdoctor(name: String, city: String) =>
    doctor <- AddN<Doctor>({
        name: name,
        city: city
    })
    RETURN doctor

QUERY create_patient(name: String, age: I32) =>
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

QUERY get_patients_visits_in_previous_month(patient_id: String, date: I32) =>
    patient <- N<Patient>(patient_id)
    visits <- patient::OutE<Visit>::WHERE(_::{date}::GTE(date))
    RETURN visits

QUERY get_visit_by_date(patient_id: String, date: I32) =>
    patient <- N<Patient>(patient_id)
    visit <- patient::OutE<Visit>::WHERE(_::{date}::EQ(date))::RANGE(0, 1)
    RETURN visit
