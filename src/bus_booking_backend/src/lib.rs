// Import necessary dependencies
#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use regex::Regex;
use std::{borrow::Cow, cell::RefCell};

// Use these types to store our canister's state and generate unique IDs
type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

// Define the Admin struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Admin {
    id: u64,
    name: String,
    email: String,
    created_at: u64,
}

impl Storable for Admin {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Admin {
    const MAX_SIZE: u32 = 2048;
    const IS_FIXED_SIZE: bool = false;
}

// Define the Route struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Route {
    id: u64,
    name: String,
    admin_id: u64,
    passengers: Vec<u64>,
    created_at: u64,
}

impl Storable for Route {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Route {
    const MAX_SIZE: u32 = 2048;
    const IS_FIXED_SIZE: bool = false;
}

// Define the Passenger struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Passenger {
    id: u64,
    name: String,
    email: String,
    points: u64, // New field for points
    created_at: u64,
}

impl Storable for Passenger {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Passenger {
    const MAX_SIZE: u32 = 2048;
    const IS_FIXED_SIZE: bool = false;
}

// Define the Booking struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Booking {
    id: u64,
    route_id: u64,
    passenger_id: u64,
    amount: f64,
    created_at: u64,
}

impl Storable for Booking {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Booking {
    const MAX_SIZE: u32 = 2048;
    const IS_FIXED_SIZE: bool = false;
}

// Define the Proposal struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Proposal {
    id: u64,
    route_id: u64,
    proposer_id: u64,
    description: String,
    votes_for: u64,
    votes_against: u64,
    created_at: u64,
}

impl Storable for Proposal {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Proposal {
    const MAX_SIZE: u32 = 2048;
    const IS_FIXED_SIZE: bool = false;
}

// Define payloads

// Admin Payload
#[derive(candid::CandidType, Serialize, Deserialize)]
struct AdminPayload {
    name: String,
    email: String,
}

// Route Payload
#[derive(candid::CandidType, Serialize, Deserialize)]
struct RoutePayload {
    name: String,
    admin_id: u64,
}

// Passenger Payload
#[derive(candid::CandidType, Serialize, Deserialize)]
struct PassengerPayload {
    name: String,
    email: String,
}

// Booking Payload
#[derive(candid::CandidType, Serialize, Deserialize)]
struct BookingPayload {
    route_id: u64,
    passenger_id: u64,
    amount: f64,
}

// Proposal Payload
#[derive(candid::CandidType, Serialize, Deserialize)]
struct ProposalPayload {
    route_id: u64,
    proposer_id: u64,
    description: String,
}

// Vote Payload
#[derive(candid::CandidType, Serialize, Deserialize)]
struct VotePayload {
    proposal_id: u64,
    passenger_id: u64,
    vote: bool, // true for 'for', false for 'against'
}

// Add Passenger to Route Payload
#[derive(candid::CandidType, Serialize, Deserialize)]
struct AddPassengerToRoutePayload {
    route_id: u64,
    passenger_id: u64,
}

// Thread-local variables that will hold our canister's state
thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static ADMIN_STORAGE: RefCell<StableBTreeMap<u64, Admin, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0)))
    ));

    static ROUTES_STORAGE: RefCell<StableBTreeMap<u64, Route, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));

    static PASSENGERS_STORAGE: RefCell<StableBTreeMap<u64, Passenger, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))
    ));

    static BOOKINGS_STORAGE: RefCell<StableBTreeMap<u64, Booking, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3)))
    ));

    static PROPOSALS_STORAGE: RefCell<StableBTreeMap<u64, Proposal, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(4)))
    ));
}

// Error handling enum
#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    UnAuthorized { msg: String },
    NotFound { msg: String },
    EmptyFields { msg: String },
    NotRoutePassenger { msg: String },
    AlreadyExists { msg: String },
    InvalidEmail { msg: String },
    InvalidName { msg: String },
}

// Utility function to validate email
fn validate_email(email: &str) -> Result<(), Error> {
    let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    if !email_regex.is_match(email) {
        Err(Error::InvalidEmail {
            msg: "Ensure the email address is of the correct format".to_string(),
        })
    } else {
        Ok(())
    }
}

// Utility function to validate name
fn validate_name(name: &str) -> Result<(), Error> {
    let name_regex = Regex::new(r"^[a-zA-Z]+(([',. -][a-zA-Z ])?[a-zA-Z]*)*$").unwrap();
    if !name_regex.is_match(name) {
        Err(Error::InvalidName {
            msg: "Invalid name".to_string(),
        })
    } else {
        Ok(())
    }
}

// Function to create a route ADMIN
#[ic_cdk::update]
fn create_admin(payload: AdminPayload) -> Result<Admin, Error> {
    if payload.name.is_empty() || payload.email.is_empty() {
        return Err(Error::EmptyFields {
            msg: "Name and email are required".to_string(),
        });
    }

    // Validate the email address
    validate_email(&payload.email)?;

    // Ensure the email address is unique
    let email_exists = ADMIN_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .any(|(_, admin)| admin.email == payload.email)
    });
    if email_exists {
        return Err(Error::AlreadyExists {
            msg: "Email address already in use".to_string(),
        });
    }

    // Validate the name
    validate_name(&payload.name)?;

    // Generate unique IDs for admins
    let id = generate_unique_id();

    // Create a new admin
    let admin = Admin {
        id,
        name: payload.name,
        email: payload.email,
        created_at: time(),
    };

    // Store the new admin
    ADMIN_STORAGE.with(|storage| {
        storage.borrow_mut().insert(id, admin.clone()).unwrap();
    });

    Ok(admin)
}

// Function to create a route
#[ic_cdk::update]
fn create_route(payload: RoutePayload) -> Result<Route, Error> {
    // Validate the route name
    validate_name(&payload.name)?;

    // Validate the admin ID
    let admin_exists = ADMIN_STORAGE.with(|storage| {
        storage.borrow().contains_key(&payload.admin_id)
    });
    if !admin_exists {
        return Err(Error::NotFound {
            msg: "Admin not found".to_string(),
        });
    }

    // Generate unique IDs for routes
    let id = generate_unique_id();

    // Create a new route
    let route = Route {
        id,
        name: payload.name,
        admin_id: payload.admin_id,
        passengers: vec![],
        created_at: time(),
    };

    // Store the new route
    ROUTES_STORAGE.with(|storage| {
        storage.borrow_mut().insert(id, route.clone()).unwrap();
    });

    Ok(route)
}

// Function to create a passenger
#[ic_cdk::update]
fn create_passenger(payload: PassengerPayload) -> Result<Passenger, Error> {
    if payload.name.is_empty() || payload.email.is_empty() {
        return Err(Error::EmptyFields {
            msg: "Name and email are required".to_string(),
        });
    }

    // Validate the email address
    validate_email(&payload.email)?;

    // Ensure the email address is unique
    let email_exists = PASSENGERS_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .any(|(_, passenger)| passenger.email == payload.email)
    });
    if email_exists {
        return Err(Error::AlreadyExists {
            msg: "Email address already in use".to_string(),
        });
    }

    // Validate the name
    validate_name(&payload.name)?;

    // Generate unique IDs for passengers
    let id = generate_unique_id();

    // Create a new passenger
    let passenger = Passenger {
        id,
        name: payload.name,
        email: payload.email,
        points: 0, // Initialize with 0 points
        created_at: time(),
    };

    // Store the new passenger
    PASSENGERS_STORAGE.with(|storage| {
        storage.borrow_mut().insert(id, passenger.clone()).unwrap();
    });

    Ok(passenger)
}

// Function to book a route for a passenger
#[ic_cdk::update]
fn book_route(payload: BookingPayload) -> Result<Booking, Error> {
    // Validate the route ID
    let route_exists = ROUTES_STORAGE.with(|storage| {
        storage.borrow().contains_key(&payload.route_id)
    });
    if !route_exists {
        return Err(Error::NotFound {
            msg: "Route not found".to_string(),
        });
    }

    // Validate the passenger ID
    let passenger_exists = PASSENGERS_STORAGE.with(|storage| {
        storage.borrow().contains_key(&payload.passenger_id)
    });
    if !passenger_exists {
        return Err(Error::NotFound {
            msg: "Passenger not found".to_string(),
        });
    }

    // Generate unique IDs for bookings
    let id = generate_unique_id();

    // Create a new booking
    let booking = Booking {
        id,
        route_id: payload.route_id,
        passenger_id: payload.passenger_id,
        amount: payload.amount,
        created_at: time(),
    };

    // Store the new booking
    BOOKINGS_STORAGE.with(|storage| {
        storage.borrow_mut().insert(id, booking.clone()).unwrap();
    });

    Ok(booking)
}

// Function to propose a new route
#[ic_cdk::update]
fn propose_route(payload: ProposalPayload) -> Result<Proposal, Error> {
    // Validate the route ID
    let route_exists = ROUTES_STORAGE.with(|storage| {
        storage.borrow().contains_key(&payload.route_id)
    });
    if !route_exists {
        return Err(Error::NotFound {
            msg: "Route not found".to_string(),
        });
    }

    // Validate the proposer ID
    let proposer_exists = PASSENGERS_STORAGE.with(|storage| {
        storage.borrow().contains_key(&payload.proposer_id)
    });
    if !proposer_exists {
        return Err(Error::NotFound {
            msg: "Proposer not found".to_string(),
        });
    }

    // Generate unique IDs for proposals
    let id = generate_unique_id();

    // Create a new proposal
    let proposal = Proposal {
        id,
        route_id: payload.route_id,
        proposer_id: payload.proposer_id,
        description: payload.description,
        votes_for: 0,
        votes_against: 0,
        created_at: time(),
    };

    // Store the new proposal
    PROPOSALS_STORAGE.with(|storage| {
        storage.borrow_mut().insert(id, proposal.clone()).unwrap();
    });

    Ok(proposal)
}

// Function to vote on a proposal
#[ic_cdk::update]
fn vote_on_proposal(payload: VotePayload) -> Result<(), Error> {
    // Validate the proposal ID
    let mut proposal = PROPOSALS_STORAGE.with(|storage| {
        storage.borrow_mut().get(&payload.proposal_id).ok_or(Error::NotFound {
            msg: "Proposal not found".to_string(),
        })
    })?;

    // Validate the passenger ID
    let passenger_exists = PASSENGERS_STORAGE.with(|storage| {
        storage.borrow().contains_key(&payload.passenger_id)
    });
    if !passenger_exists {
        return Err(Error::NotFound {
            msg: "Passenger not found".to_string(),
        });
    }

    // Update the proposal votes
    if payload.vote {
        proposal.votes_for += 1;
    } else {
        proposal.votes_against += 1;
    }

    // Store the updated proposal
    PROPOSALS_STORAGE.with(|storage| {
        storage.borrow_mut().insert(proposal.id, proposal.clone()).unwrap();
    });

    Ok(())
}

// Function to add a passenger to a route
#[ic_cdk::update]
fn add_passenger_to_route(payload: AddPassengerToRoutePayload) -> Result<(), Error> {
    // Validate the route ID
    let mut route = ROUTES_STORAGE.with(|storage| {
        storage.borrow_mut().get(&payload.route_id).ok_or(Error::NotFound {
            msg: "Route not found".to_string(),
        })
    })?;

    // Validate the passenger ID
    let passenger_exists = PASSENGERS_STORAGE.with(|storage| {
        storage.borrow().contains_key(&payload.passenger_id)
    });
    if !passenger_exists {
        return Err(Error::NotFound {
            msg: "Passenger not found".to_string(),
        });
    }

    // Ensure the passenger is not already on the route
    if route.passengers.contains(&payload.passenger_id) {
        return Err(Error::AlreadyExists {
            msg: "Passenger is already on this route".to_string(),
        });
    }

    // Add the passenger to the route
    route.passengers.push(payload.passenger_id);

    // Store the updated route
    ROUTES_STORAGE.with(|storage| {
        storage.borrow_mut().insert(route.id, route.clone()).unwrap();
    });

    Ok(())
}

// Utility function to generate unique IDs
fn generate_unique_id() -> u64 {
    ID_COUNTER.with(|counter| {
        let current_value = *counter.borrow().get();
        let _ = counter.borrow_mut().set(current_value + 1);
        current_value + 1
    })
}

ic_cdk::export_candid!();
