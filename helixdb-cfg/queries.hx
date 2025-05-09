// ------------------
// Create Data
// ------------------
QUERY create_data( doctor_name: String, doctor_city: String, patient_name: String, patient_age: I64, summary: String, date: I64) =>
    doctor <- AddN<Doctor>({
        name: doctor_name,
        city: doctor_city
    })
    patient <- AddN<Patient>({
        name: patient_name,
        age: patient_age
    })
    AddE<Visit>({doctors_summary: summary, date: date})::From(patient)::To(doctor)
    RETURN patient

// ------------------
// Fetch Data
// ------------------
QUERY get_patient(name: String) =>
    patient <- N<Patient>::WHERE(_::{name}::EQ(name))
    RETURN patient

QUERY get_patients_visits_in_previous_month(name: String, date: I64) =>
    patient <- N<Patient>::WHERE(_::{name}::EQ(name))
    visits <- patient::OutE<Visit>::WHERE(_::{date}::GTE(date))
    RETURN visits

QUERY get_visit_by_date(name: String, date: I64) =>
    patients <- N<Patient>
    patient <- patients::WHERE(_::{name}::EQ(name))
    visit <- patient::OutE<Visit>::WHERE(_::{date}::EQ(date))::RANGE(0, 1)
    RETURN patient, visit