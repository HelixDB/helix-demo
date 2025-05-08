N::Patient {
    name: String,
    age: I32
}

N::Doctor {
    name: String,
    city: String
}

E::Visit {
    From: Patient,
    To: Doctor,
    Properties: {
        doctors_summary: String,
        date: I32
    }
}