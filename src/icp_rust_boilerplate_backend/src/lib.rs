#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::fmt::format;
use std::{borrow::Cow, cell::RefCell, collections::HashMap};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

#[warn(unused_must_use)]
type Result<T> = std::result::Result<T, Error>;

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct EducationHistory {
    id: u64,
    user_id: String,
    name: String,
    degre: String,
    field_of_study: String,
    start_year: u64,
    end_year: u64,
    create_at: u64,
    update_at: Option<u64>,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct WorkHistory {
    id: u64,
    user_id: String,
    name: String,
    position: String,
    start_year: u64,
    end_year: Option<u64>,
    salary: u64,
    description: String,
    created_at: u64,
    update_at: Option<u64>,
}

// Payload
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct EducationHistoryPayload {
    name: String,
    degre: String,
    field_of_study: String,
    start_year: u64,
    end_year: u64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct WorkHistoryPayload {
    name: String,
    position: String,
    start_year: u64,
    end_year: Option<u64>,
    salary: u64,
    description: String
}

impl Storable for EducationHistory {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for EducationHistory {
    const MAX_SIZE: u32 = 2048;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for WorkHistory {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for WorkHistory {
    const MAX_SIZE: u32 = 2048;
    const IS_FIXED_SIZE: bool = false;
}

thread_local! {
    static EDUCATION_MEMORY: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static EDUCATION_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(EDUCATION_MEMORY.with(|m| m.borrow().get(MemoryId::new(0))), 0)
        .expect("Connot create a counter")
    );

    static EDUCATION_STORAGE: RefCell<StableBTreeMap<u64, EducationHistory, Memory>> = RefCell::new(
        StableBTreeMap::init(
            EDUCATION_MEMORY.with(|m| m.borrow().get(MemoryId::new(0)))
        )
    );

    static WORK_MEMORY: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static WORK_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(WORK_MEMORY.with(|m| m.borrow().get(MemoryId::new(0))), 0)
        .expect("Connot create a counter")
    );

    static WORK_STORAGE: RefCell<StableBTreeMap<u64, WorkHistory, Memory>> = RefCell::new(
        StableBTreeMap::init(
            WORK_MEMORY.with(|m| m.borrow().get(MemoryId::new(1)))
        )
    );
}

#[ic_cdk::query]
fn get_education_history() -> Result<Vec<EducationHistory>> {
    EDUCATION_STORAGE.with(|education| {
        Ok(education
            .borrow()
            .iter()
            .filter(|(_, v)| v.user_id == ic_cdk::caller().to_string())
            .map(|(_, v)| v.clone())
            .collect())
    })
}

#[ic_cdk::update]
fn add_education_history(payload: EducationHistoryPayload) -> Result<EducationHistory> {
    let id = EDUCATION_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment counter");

    let education = EducationHistory {
        id: id,
        user_id: ic_cdk::caller().to_string(),
        name: payload.name,
        degre: payload.degre,
        field_of_study: payload.field_of_study,
        start_year: payload.start_year,
        end_year: payload.end_year,
        create_at: time(),
        update_at: None,
    };
    save_education(&education);
    Ok(education)
}

#[ic_cdk::update]
fn update_education_history(id: u64, payload: EducationHistoryPayload) -> Result<EducationHistory> {
    match _get_education(&id) {
        Some(mut education) => {
            if education.user_id == ic_cdk::caller().to_string() {
                // Perbaiki nama field yang salah diubah
                education.name = payload.name;
                education.degre = payload.degre;
                education.field_of_study = payload.field_of_study;
                education.start_year = payload.start_year;
                education.end_year = payload.end_year;
                education.update_at = Some(time());
                save_education(&education);
                Ok(education)
            } else {
                Err(Error::NotAuthorize {
                    msg: format!("{} not owner", ic_cdk::caller().to_string()),
                })
            }
        }
        None => Err(Error::NotFound {
            msg: format!("Education with id {} not found", id),
        }),
    }
}

#[ic_cdk::update]
fn delete_education_history(id: u64) -> Result<EducationHistory> {
    match _get_education(&id) {
        Some(education) => {
            if education.user_id == ic_cdk::caller().to_string() {
                match EDUCATION_STORAGE.with(|service| service.borrow_mut().remove(&id)) {
                    Some(edu) => Ok(edu),
                    None => {
                        // Kembalikan Error jika EDUCATION_STORAGE tidak dapat menghapus data
                        Err(Error::NotFound {
                            msg: format!("Education with id {} not found", id),
                        })
                    }
                }
            } else {
                // Kembalikan Error jika pemanggil tidak memiliki izin
                Err(Error::NotAuthorize {
                    msg: String::from("Not authorized to delete this education history"),
                })
            }
        }
        None => {
            // Kembalikan Error jika tidak ada pendidikan dengan ID yang diberikan
            Err(Error::NotFound {
                msg: format!("Education with id {} not found", id),
            })
        }
    }
}

#[ic_cdk::query]
fn get_work_history() -> Result<Vec<WorkHistory>> {
    WORK_STORAGE.with(|work| {
        Ok(work
            .borrow()
            .iter()
            .filter(|(_, v)| v.user_id == ic_cdk::caller().to_string())
            .map(|(_, v)| v.clone())
            .collect())
    })
}

#[ic_cdk::update]
fn add_work_history(payload: WorkHistoryPayload) -> Result<WorkHistory> {
    let id = WORK_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment counter");

    let work = WorkHistory {
        id: id,
        user_id: ic_cdk::caller().to_string(),
        name: payload.name,
        position: payload.position,
        start_year: payload.start_year,
        end_year: payload.end_year,
        salary: payload.salary,
        description: payload.description,
        created_at: time(),
        update_at: None,
    };
    save_work(&work);
    Ok(work)
}

#[ic_cdk::update]
fn update_work_history(id: u64, payload: WorkHistoryPayload) -> Result<WorkHistory> {
    match _get_work(&id) {
        Some(mut work) => {
            if work.user_id == ic_cdk::caller().to_string() {
                work.name = payload.name;
                work.position = payload.position;
                work.start_year = payload.start_year;
                work.end_year = payload.end_year;
                work.salary = payload.salary;
                work.description = payload.description;
                work.update_at = Some(time());
                save_work(&work);
                Ok(work)
            } else {
                Err(Error::NotAuthorize {
                    msg: format!("{} not owner", ic_cdk::caller().to_string()),
                })
            }
        }
        None => Err(Error::NotFound {
            msg: format!("Education with id {} not found", id),
        }),
    }
}

#[ic_cdk::update]
fn delete_work_history(id: u64) -> Result<WorkHistory> {
    match _get_work(&id) {
        Some(work) => {
            if work.user_id == ic_cdk::caller().to_string() {
                match WORK_STORAGE.with(|service| service.borrow_mut().remove(&id)) {
                    Some(wrk) => Ok(wrk),
                    None => {
                        Err(Error::NotFound { msg: format!("Work with id {} not found", id) })
                    }
                }
            }else {
                Err(Error::NotAuthorize { msg: String::from("Not authorized to delet this work history") })
            }
        },
        None => {
            Err(Error::NotFound { msg: format!("Work with id {} not found", id) })
        }
    }
}

// Helper function

fn save_education(data: &EducationHistory) {
    EDUCATION_STORAGE.with(|service| service.borrow_mut().insert(data.id, data.clone()));
}

fn _get_education(id: &u64) -> Option<EducationHistory> {
    EDUCATION_STORAGE.with(|service| service.borrow().get(&id))
}

fn save_work(data: &WorkHistory) {
    WORK_STORAGE.with(|service| service.borrow_mut().insert(data.id, data.clone()));
}

fn _get_work(id: &u64) -> Option<WorkHistory> {
    WORK_STORAGE.with(|service| service.borrow().get(&id))
}

#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
    NotAuthorize { msg: String }, // Other error types can be added here
}

// Need this to generate candid
ic_cdk::export_candid!();
