use std::collections::{HashMap, HashSet};
use std::cell::RefCell;
use std::sync::Arc;
use std::time::Instant;

use get_routes::handler;
use helixdb::helix_engine::vector_core::vector::HVector;
use helixdb::{
    node_matches,
    props,
    helix_engine::graph_core::ops::{
        g::G,
        in_::{in_::InAdapter, in_e::InEdgesAdapter, to_n::ToNAdapter},
        out::{from_n::FromNAdapter, out::OutAdapter, out_e::OutEdgesAdapter},
        vectors::{ insert::InsertVAdapter, search::SearchVAdapter},
        source::{add_e::{AddEAdapter, EdgeType}, add_n::AddNAdapter, e::EAdapter, e_from_id::EFromId, e_from_types::EFromTypes, n::NAdapter, n_from_id::NFromId, n_from_types::NFromTypesAdapter},
        tr_val::{TraversalVal, Traversable},
        util::{dedup::DedupAdapter, drop::DropAdapter, filter_mut::FilterMut, filter_ref::FilterRefAdapter, range::RangeAdapter, update::Update},
    },
    helix_engine::types::GraphError,
    helix_gateway::router::router::HandlerInput,
    protocol::count::Count,
    protocol::response::Response,
    protocol::traversal_value::TraversalValue,
    protocol::remapping::ResponseRemapping,
    protocol::{filterable::Filterable, value::Value, return_values::ReturnValue, remapping::Remapping},
};
use sonic_rs::{Deserialize, Serialize};

// Node Schema: Patient
#[derive(Serialize, Deserialize)]
struct Patient {
    name: String,
    age: i32,
}

// Node Schema: Doctor
#[derive(Serialize, Deserialize)]
struct Doctor {
    name: String,
    city: String,
}

// Edge Schema: Visit
#[derive(Serialize, Deserialize)]
struct Visit {
    doctors_summary: String,
    date: i32,
}

#[handler]
pub fn get_patients_visits_in_previous_month(input: &HandlerInput, response: &mut Response) -> Result<(), GraphError> {
    #[derive(Serialize, Deserialize)]
    struct get_patients_visits_in_previous_monthData {
        name: String,
        date: i32,
    }

    let data: get_patients_visits_in_previous_monthData = match sonic_rs::from_slice(&input.request.body) {
        Ok(data) => data,
        Err(err) => return Err(GraphError::from(err)),
    };

    let mut remapping_vals: RefCell<HashMap<u128, ResponseRemapping>> = RefCell::new(HashMap::new());
    let db = Arc::clone(&input.graph.storage);
    let txn = db.graph_env.read_txn().unwrap();

    let mut return_vals: HashMap<String, ReturnValue> = HashMap::with_capacity(1);

        let tr = G::new(Arc::clone(&db), &txn)
    .n_from_types(&["Patient"])
    .filter_ref(|val, _| {
    if let Ok(val) = val {
    val.check_property("name").map_or(false, |v| *v != data.name)    } else { false }
})
;    let patient = tr.collect_to::<Vec<_>>();

        let tr = G::new_from(Arc::clone(&db), &txn, patient.clone())
.out_e("Visit")
    .filter_ref(|val, _| {
    if let Ok(val) = val {
    val.check_property("date").map_or(false, |v| matches!(v, Value::I32(val) if *val >= data.date))    } else { false }
})
;    let visits = tr.collect_to::<Vec<_>>();

    return_vals.insert("visits".to_string(), ReturnValue::from_traversal_value_array_with_mixin(visits, remapping_vals.borrow_mut()));
    response.body = sonic_rs::to_vec(&return_vals).unwrap();

    Ok(())
}

#[handler]
pub fn get_patient(input: &HandlerInput, response: &mut Response) -> Result<(), GraphError> {
    #[derive(Serialize, Deserialize)]
    struct get_patientData {
        name: String,
    }

    let data: get_patientData = match sonic_rs::from_slice(&input.request.body) {
        Ok(data) => data,
        Err(err) => return Err(GraphError::from(err)),
    };

    let mut remapping_vals: RefCell<HashMap<u128, ResponseRemapping>> = RefCell::new(HashMap::new());
    let db = Arc::clone(&input.graph.storage);
    let txn = db.graph_env.read_txn().unwrap();

    let mut return_vals: HashMap<String, ReturnValue> = HashMap::with_capacity(1);

        let tr = G::new(Arc::clone(&db), &txn)
    .n_from_types(&["Patient"])
    .filter_ref(|val, _| {
    if let Ok(val) = val {
    val.check_property("name").map_or(false, |v| *v != data.name)    } else { false }
})
;    let patient = tr.collect_to::<Vec<_>>();

    return_vals.insert("patient".to_string(), ReturnValue::from_traversal_value_array_with_mixin(patient, remapping_vals.borrow_mut()));
    response.body = sonic_rs::to_vec(&return_vals).unwrap();

    Ok(())
}

#[handler]
pub fn get_visit_by_date(input: &HandlerInput, response: &mut Response) -> Result<(), GraphError> {
    #[derive(Serialize, Deserialize)]
    struct get_visit_by_dateData {
        name: String,
        date: i32,
    }

    let data: get_visit_by_dateData = match sonic_rs::from_slice(&input.request.body) {
        Ok(data) => data,
        Err(err) => return Err(GraphError::from(err)),
    };

    let mut remapping_vals: RefCell<HashMap<u128, ResponseRemapping>> = RefCell::new(HashMap::new());
    let db = Arc::clone(&input.graph.storage);
    let txn = db.graph_env.read_txn().unwrap();

    let mut return_vals: HashMap<String, ReturnValue> = HashMap::with_capacity(1);

        let tr = G::new(Arc::clone(&db), &txn)
    .n_from_types(&["Patient"])
    .filter_ref(|val, _| {
    if let Ok(val) = val {
    val.check_property("name").map_or(false, |v| *v != data.name)    } else { false }
})
;    let patient = tr.collect_to::<Vec<_>>();

        let tr = G::new_from(Arc::clone(&db), &txn, patient.clone())
.out_e("Visit")
    .filter_ref(|val, _| {
    if let Ok(val) = val {
    val.check_property("date").map_or(false, |v| *v != data.date)    } else { false }
})
    .range(0, 1)
;    let visit = tr.collect_to::<Vec<_>>();

    return_vals.insert("visit".to_string(), ReturnValue::from_traversal_value_array_with_mixin(visit, remapping_vals.borrow_mut()));
    response.body = sonic_rs::to_vec(&return_vals).unwrap();

    Ok(())
}

#[handler]
pub fn create_data(input: &HandlerInput, response: &mut Response) -> Result<(), GraphError> {
    #[derive(Serialize, Deserialize)]
    struct create_dataData {
        doctor_name: String,
        doctor_city: String,
        patient_name: String,
        patient_age: i32,
        summary: String,
    }

    let data: create_dataData = match sonic_rs::from_slice(&input.request.body) {
        Ok(data) => data,
        Err(err) => return Err(GraphError::from(err)),
    };

    let mut remapping_vals: RefCell<HashMap<u128, ResponseRemapping>> = RefCell::new(HashMap::new());
    let db = Arc::clone(&input.graph.storage);
    let mut txn = db.graph_env.write_txn().unwrap();

    let mut return_vals: HashMap<String, ReturnValue> = HashMap::with_capacity(1);

        let tr = G::new_mut(Arc::clone(&db), &mut txn)
    .add_n("Doctor", props!{ "name".to_string() => data.doctor_name, "city".to_string() => data.doctor_city }, None, None);    let doctor = tr.collect_to::<Vec<_>>();

        let tr = G::new_mut(Arc::clone(&db), &mut txn)
    .add_n("Patient", props!{ "name".to_string() => data.patient_name, "age".to_string() => data.patient_age }, None, None);    let patient = tr.collect_to::<Vec<_>>();

    let tr = G::new_mut(Arc::clone(&db), &mut txn)
    .add_e("Visit", props!{ "doctors_summary".to_string() => data.summary }, None, patient.id(), doctor.id(), true, EdgeType::Vec);
let _ = tr.collect_to::<Vec<_>>();

    return_vals.insert("patient".to_string(), ReturnValue::from_traversal_value_array_with_mixin(patient, remapping_vals.borrow_mut()));
    response.body = sonic_rs::to_vec(&return_vals).unwrap();

    txn.commit()?;
    Ok(())
}

