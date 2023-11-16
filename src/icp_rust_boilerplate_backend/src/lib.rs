use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

#[warn(unused_must_use)]
type Result<T> = std::result::Result<T, Error>;

thread_local! {
    static EDUCATION_MEMORY: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static WORK_MEMORY: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static COMPANY_MEMORY: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static JOB_MEMORY: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static JOBAPPLICATION_MEMORY: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );
}

lazy_static::lazy_static! {
    static ref EDUCATION_STORAGE: Mutex<StableBTreeMap<u64, EducationHistory, Memory>> = {
        Mutex::new(StableBTreeMap::init(
            EDUCATION_MEMORY.with(|m| m.borrow().get(MemoryId::new(1)))
        ))
    };

    static ref EDUCATION_COUNTER: Mutex<IdCell> = {
        Mutex::new(IdCell::init(
            EDUCATION_MEMORY.with(|m| m.borrow().get(MemoryId::new(0))),
            0,
        ).expect("Cannot create a counter"))
    };
}

// ... (similar changes for other data types)

#[ic_cdk::query]
fn get_education_history() -> Result<Vec<EducationHistory>> {
    let user_id = ic_cdk::caller().to_string();
    let education_data = EDUCATION_STORAGE.lock().unwrap();

    Ok(education_data
        .iter()
        .filter(|(_, v)| v.user_id == user_id)
        .map(|(_, v)| v.clone())
        .collect())
}

#[ic_cdk::update]
fn add_education_history(payload: EducationHistoryPayload) -> Result<EducationHistory> {
    let user_id = ic_cdk::caller().to_string();
    let id = EDUCATION_COUNTER.lock().unwrap().get_and_increment().expect("Cannot increment counter");

    let education = EducationHistory {
        id,
        user_id: user_id.clone(),
        name: payload.name,
        degre: payload.degre,
        field_of_study: payload.field_of_study,
        start_year: payload.start_year,
        end_year: payload.end_year,
        create_at: time(),
        update_at: None,
    };

    _save_education(&education)?;
    Ok(education)
}

#[ic_cdk::update]
fn update_education_history(id: u64, payload: EducationHistoryPayload) -> Result<EducationHistory> {
    let user_id = ic_cdk::caller().to_string();
    let mut education_data = EDUCATION_STORAGE.lock().unwrap();

    match education_data.get_mut(&id) {
        Some(education) => {
            if education.user_id == user_id {
                education.name = payload.name;
                education.degre = payload.degre;
                education.field_of_study = payload.field_of_study;
                education.start_year = payload.start_year;
                education.end_year = payload.end_year;
                education.update_at = Some(time());

                _save_education(education)?;
                Ok(education.clone())
            } else {
                Err(Error::NotAuthorize {
                    msg: format!("{} not owner", user_id),
                })
            }
        }
        None => Err(Error::NotFound {
            msg: format!("Education with id {} not found", id),
        }),
    }
}

// ... (similar changes for other functions)

// Helper function

fn _save_education(data: &EducationHistory) -> Result<()> {
    EDUCATION_STORAGE.lock().unwrap().insert(data.id, data.clone());
    Ok(())
}

// ... (similar changes for other helper functions)

#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
    NotAuthorize { msg: String },
    StorageError { msg: String },  // New error variant for storage operations
}

ic_cdk::export_candid!();
