use candid::{CandidType, Deserialize};
use ic_cdk::export::Principal;
use ic_cdk_macros::*;
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct User {
    pub name: String,          // Nama pengguna
    pub roles: Vec<UserRole>,  // Peran yang dimiliki pengguna (bisa lebih dari satu)
    pub active_role: UserRole, // Peran yang sedang aktif
    pub registered_date: u64,  // Tanggal registrasi
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq)]
pub enum UserRole {
    Patient,
    Doctor,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct MedicalRecord {
    patient_id: Principal,     // ID pasien
    doctor_id: Principal,      // ID dokter yang membuat
    diagnosis: String,         // Diagnosis penyakit
    medication: String,        // Obat yang diberikan
    notes: String,            // Catatan tambahan
    created_at: u64,          // Waktu pembuatan
}

thread_local! {
    static USERS: RefCell<HashMap<Principal, User>> = RefCell::new(HashMap::new());
    static MEDICAL_RECORDS: RefCell<HashMap<Principal, Vec<MedicalRecord>>> = 
        RefCell::new(HashMap::new());
}

fn caller() -> Principal {
    ic_cdk::caller()
}

fn now() -> u64 {
    ic_cdk::api::time()
}

#[update]
fn register_user(name: String, role: UserRole) -> Result<Principal, String> {
    let user_id = caller();
    
    USERS.with(|users| {
        let mut users = users.borrow_mut();

        if let Some(user) = users.get_mut(&user_id) {
            if !user.roles.contains(&role) {
                user.roles.push(role.clone());
            }
            return Ok(user_id);
        }

        users.insert(
            user_id,
            User {
                name,
                roles: vec![role.clone()],
                active_role: role,
                registered_date: now(),
            },
        );
        Ok(user_id)
    })
}

#[update]
fn switch_role(role: UserRole) -> Result<(), String> {
    let user_id = caller();
    
    USERS.with(|users| {
        let mut users = users.borrow_mut();
        
        if let Some(user) = users.get_mut(&user_id) {
            if user.roles.contains(&role) {
                user.active_role = role;
                Ok(())
            } else {
                Err("Peran tidak tersedia untuk pengguna ini".to_string())
            }
        } else {
            Err("User tidak ditemukan".to_string())
        }
    })
}

#[update]
fn create_medical_record(
    patient_id: Principal,
    diagnosis: String,
    medication: String,
    notes: String,
) -> Result<(), String> {
    let doctor_id = caller();
    
    USERS.with(|users| {
        let users = users.borrow();
        if let Some(user) = users.get(&doctor_id) {
            if user.active_role != UserRole::Doctor {
                return Err("Hanya dokter yang dapat membuat rekam medis".to_string());
            }
        } else {
            return Err("Dokter tidak ditemukan".to_string());
        }
        Ok(())
    })?;

    let record = MedicalRecord {
        patient_id,
        doctor_id,
        diagnosis,
        medication,
        notes,
        created_at: now(),
    };

    MEDICAL_RECORDS.with(|records| {
        records
            .borrow_mut()
            .entry(patient_id)
            .or_insert_with(Vec::new)
            .push(record);
    });

    Ok(())
}

#[query]
fn get_medical_records(patient_id: Principal) -> Result<Vec<MedicalRecord>, String> {
    let caller_id = caller();
    
    USERS.with(|users| {
        let users = users.borrow();
        if let Some(user) = users.get(&caller_id) {
            match user.active_role {
                UserRole::Doctor => (),
                UserRole::Patient if caller_id == patient_id => (),
                _ => return Err("Akses ditolak".to_string()),
            }
        } else {
            return Err("User tidak ditemukan".to_string());
        }
        Ok(())
    })?;

    MEDICAL_RECORDS.with(|records| {
        Ok(records
            .borrow()
            .get(&patient_id)
            .cloned()
            .unwrap_or_default())
    })
}

#[query]
fn get_user_info(user_id: Principal) -> Result<(Principal, User), String> {
    USERS.with(|users| {
        users
            .borrow()
            .get(&user_id)
            .map(|user| (user_id, user.clone()))
            .ok_or("User tidak ditemukan".to_string())
    })
}

#[query]
fn get_all_users() -> Vec<(Principal, User)> {
    USERS.with(|users| {
        users.borrow()
            .iter()
            .map(|(k, v)| (*k, v.clone()))
            .collect()
    })
}

#[query]
fn get_active_role() -> Result<UserRole, String> {
    let user_id = caller();
    USERS.with(|users| {
        users
            .borrow()
            .get(&user_id)
            .map(|user| user.active_role.clone())
            .ok_or("User tidak ditemukan".to_string())
    })
}