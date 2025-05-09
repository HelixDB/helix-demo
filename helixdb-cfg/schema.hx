N::Patient {
    name: String,
    age: I64
}

N::Doctor {
    name: String,
    city: String
}

// Visit Edge between Patient and Doctor
E::Visit {
    From: Patient,
    To: Doctor,
    Properties: {
        doctors_summary: String,
        date: I64
    }
}