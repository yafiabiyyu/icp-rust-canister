#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

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

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Company {
    id:u64,
    name:String,
    email:String,
    phone:String,
    address:String,
    description:String,
    admin: String,
    created_at:u64
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Job {
    id: u64,
    company_id: u64,
    position: String,
    requirements:String,
    location:String,
    salary: String,
    description:String,
    posted_by: String,
    created_at:u64,
    updated_at:Option<u64>
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

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct CompanyPayload {
    name:String,
    email:String,
    phone:String,
    address:String,
    description:String
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct JobPayload {
    position: String,
    requirements:String,
    location:String,
    salary: String,
    description:String,
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

impl Storable for Company {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Company {
    const MAX_SIZE: u32 = 2048;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Job {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Job {
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
            EDUCATION_MEMORY.with(|m| m.borrow().get(MemoryId::new(1)))
        )
    );

    static WORK_MEMORY: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static WORK_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(WORK_MEMORY.with(|m| m.borrow().get(MemoryId::new(2))), 0)
        .expect("Connot create a counter")
    );

    static WORK_STORAGE: RefCell<StableBTreeMap<u64, WorkHistory, Memory>> = RefCell::new(
        StableBTreeMap::init(
            WORK_MEMORY.with(|m| m.borrow().get(MemoryId::new(3)))
        )
    );

    static COMPANY_MEMORY: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static COMPANY_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(COMPANY_MEMORY.with(|m| m.borrow().get(MemoryId::new(4))), 0)
        .expect("Connot create a counter")
    );

    static COMPANY_STORAGE: RefCell<StableBTreeMap<u64, Company, Memory>> = RefCell::new(
        StableBTreeMap::init(
            COMPANY_MEMORY.with(|m| m.borrow().get(MemoryId::new(5)))
        )
    );

    static JOB_MEMORY: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static JOB_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(JOB_MEMORY.with(|m| m.borrow().get(MemoryId::new(6))), 0)
        .expect("Connot create a counter")
    );

    static JOB_STORAGE: RefCell<StableBTreeMap<u64, Job, Memory>> = RefCell::new(
        StableBTreeMap::init(
            JOB_MEMORY.with(|m| m.borrow().get(MemoryId::new(7)))
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
    _save_education(&education);
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
                _save_education(&education);
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
    _save_work(&work);
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
                _save_work(&work);
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

#[ic_cdk::query]
fn get_all_company() -> Result<Vec<Company>> {
    let company: Vec<(u64, Company)> = COMPANY_STORAGE.with(|service| service.borrow().iter().collect());
    let total_company = company.len();
    let mut comp: Vec<Company> = Vec::new();
    for key in 0..total_company {
        comp.push(company.get(key).unwrap().clone().1);
    }
    if comp.len() > 0 {
        Ok(comp)
    }else {
        Err(Error::NotFound { msg: format!(" There are currently no company") })
    }
}

#[ic_cdk::update]
fn register_company(payload: CompanyPayload) -> Result<Company> {
    let id = COMPANY_COUNTER.with(|counter| {
        let current_value = *counter.borrow().get();
        counter.borrow_mut().set(current_value + 1)
    }).expect("Cannot increment counter");

    let company = Company {
        id:id,
        name:payload.name,
        email:payload.email,
        phone:payload.phone,
        address: payload.address,
        description:payload.description,
        admin:ic_cdk::caller().to_string(),
        created_at: time()
    };
    _save_company(&company);
    Ok(company)
}

#[ic_cdk::update]
fn update_company(id:u64, payload:CompanyPayload) -> Result<Company> {
    match _get_company(&id) {
        Some(mut company) => {
            if company.admin == ic_cdk::caller().to_string() {
                company.name = payload.name;
                company.email = payload.email;
                company.phone = payload.phone;
                company.address = payload.address;
                company.description = payload.description;
                _save_company(&company);
                Ok(company)
            }else {
                Err(Error::NotAuthorize {
                    msg: format!("{} not company admin", ic_cdk::caller().to_string()),
                })
            }
        },
        None => {
            Err(Error::NotFound { msg: format!("Company with id {} not found", id) })
        }
    }
}

#[ic_cdk::update]
fn delete_company(id:u64) -> Result<Company> {
    match _get_company(&id) {
        Some(mut company) => {
            if company.admin == ic_cdk::caller().to_string() {
                match COMPANY_STORAGE.with(|service|service.borrow_mut().remove(&id)) {
                    Some(comp) => Ok(comp),
                    None => {
                        Err(Error::NotFound { msg: format!("Company with id {} not found", id) })
                    }
                }
            }else {
                Err(Error::NotAuthorize {
                    msg: format!("{} not company admin", ic_cdk::caller().to_string()),
                })
            }
        },
        None => {
            Err(Error::NotFound { msg: format!("Company with id {} not found", id) })
        }
    }
}

#[ic_cdk::query]
fn get_all_job() -> Result<Vec<Job>> {
    let job: Vec<(u64, Job)> = JOB_STORAGE.with(|service| service.borrow().iter().collect());
    let total_job = job.len();
    let mut jobs: Vec<Job> = Vec::new();
    for key in 0..total_job {
        jobs.push(job.get(key).unwrap().clone().1);
    }
    if jobs.len() > 0 {
        Ok(jobs)
    }else {
        Err(Error::NotFound { msg: format!(" There are currently no job") })
    }
}

#[ic_cdk::query]
fn get_job_by_company(id:u64) -> Result<Vec<Job>> {
    JOB_STORAGE.with(|job| {
        Ok(job
            .borrow()
            .iter()
            .filter(|(_, v)| v.company_id == id)
            .map(|(_, v)| v.clone())
            .collect()
        )
    })
}


#[ic_cdk::update]
fn post_new_job(companys_id:u64, payload:JobPayload) -> Result<Job> {
    match _get_company(&companys_id) {
        Some(mut company) => {
            if company.admin == ic_cdk::caller().to_string() {
                let id = JOB_COUNTER.with(|counter| {
                    let current_value = *counter.borrow().get();
                    counter.borrow_mut().set(current_value + 1)
                })
                .expect("Cannot increment counter");

                let job = Job {
                    id: id,
                    company_id: companys_id,
                    position: payload.position,
                    requirements: payload.requirements,
                    location: payload.location,
                    salary: payload.salary,
                    description: payload.description,
                    posted_by: ic_cdk::caller().to_string(),
                    created_at: time(),
                    updated_at: None
                };
                _save_job_post(&job);
                Ok(job)
            }else {
                Err(Error::NotAuthorize {
                    msg: format!("{} not owner", ic_cdk::caller().to_string()),
                })
            }
        },
        None => Err(Error::NotFound {
            msg: format!("Company with id {} not found", companys_id),
        }),
    }
}

#[ic_cdk::update]
fn update_job_post(id:u64, payload:JobPayload) -> Result<Job> {
    match _get_job_post(&id) {
        Some(mut job) => {
            if job.posted_by == ic_cdk::caller().to_string() {
                job.position = payload.position;
                job.requirements = payload.requirements;
                job.location = payload.location;
                job.salary = payload.salary;
                job.description = payload.description;
                job.updated_at = Some(time());
                _save_job_post(&job);
                Ok(job)
            }else {
                Err(Error::NotAuthorize {
                    msg: format!("{} not owner", ic_cdk::caller().to_string()),
                })
            }
        },
        None => Err(Error::NotFound {
            msg: format!("Job with id {} not found", id),
        }),
    }
}

#[ic_cdk::update]
fn delete_job_post(id:u64) -> Result<Job> {
    match _get_job_post(&id) {
        Some(job) => {
            if job.posted_by == ic_cdk::caller().to_string() {
                match JOB_STORAGE.with(|service| service.borrow_mut().remove(&id)) {
                    Some(jobs) => Ok(jobs),
                    None => {
                        Err(Error::NotFound {
                            msg: format!("Job Post with id {} not found", id),
                        })
                    }
                }
            }else {
                Err(Error::NotAuthorize {
                    msg: String::from("Not authorized to delete this job post"),
                })
            }
        },
        None => {
            Err(Error::NotFound {
                msg: format!("Education with id {} not found", id),
            })
        }
    }
}

// Helper function

fn _save_education(data: &EducationHistory) {
    EDUCATION_STORAGE.with(|service| service.borrow_mut().insert(data.id, data.clone()));
}

fn _get_education(id: &u64) -> Option<EducationHistory> {
    EDUCATION_STORAGE.with(|service| service.borrow().get(&id))
}

fn _save_work(data: &WorkHistory) {
    WORK_STORAGE.with(|service| service.borrow_mut().insert(data.id, data.clone()));
}

fn _get_work(id: &u64) -> Option<WorkHistory> {
    WORK_STORAGE.with(|service| service.borrow().get(&id))
}

fn _save_company(data: &Company) {
    COMPANY_STORAGE.with(|service| service.borrow_mut().insert(data.id, data.clone()));
}

fn _get_company(id:&u64) -> Option<Company>{
    COMPANY_STORAGE.with(|service| service.borrow().get(&id))
}

fn _save_job_post(data: &Job) {
    JOB_STORAGE.with(|service| service.borrow_mut().insert(data.id, data.clone()));
}

fn _get_job_post(id:&u64) -> Option<Job> {
    JOB_STORAGE.with(|service| service.borrow().get(&id))
}

#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
    NotAuthorize { msg: String }, // Other error types can be added here
}

// Need this to generate candid
ic_cdk::export_candid!();
