use candid::{CandidType, Decode, Deserialize, Encode};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;

const MAX_VALUE_SIZE: u32 = 5000;

impl Storable {
   fn to_bytes(&self) -> Cow<[u8]> {
       Cow::Owned(Encode!(self).unwrap())
  }

  fn from_bytes(bytes: Cow<[u8]>) -> Self {
       Decode!(bytes.as_ref(), Self).unwrap()
  }
}

impl BoundedStorable {
    const MAX_SIZE: u32 = MAX_VALUE_SIZE;
    const IS_FIXED_SIZE: bool = false;
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

}

use candid::{CandidType, Deserialize, Encode};

#[derive(CandidType, Deserialize, Encode)]
struct Proposal {
    proposal_id: u64,
    title: String,
    description: String,
    creator: String,
}

// Example CreateProposal struct
#[derive(CandidType, Deserialize, Encode)]
struct CreateProposal {
    title: String,
    description: String,
    creator: String,
}

// Example Choice enum
#[derive(CandidType, Deserialize, Encode)]
enum Choice {
    Approve,
    Reject,
    Pass
}

// Example VoteError enum
#[derive(Debug)]
enum VoteError {
    ProposalNotFound,
    // Add other potential error variants as needed
}


#[ic_cdk::query]
fn get_proposal(key: u64) -> Option<Proposal> {
    // Ensure that the storage is initialized
    unsafe {
        if PROPOSAL_STORAGE.is_none() {
            PROPOSAL_STORAGE = Some(ProposalStorage::new());
        }

        // Access the proposal storage and retrieve the proposal
        if let Some(storage) = PROPOSAL_STORAGE.as_ref() {
            return storage.get_proposal(&key).cloned();
        }
    }

    None
}
#[ic_cdk::query]
fn get_proposal_count() -> u64 {
    // Ensure that the storage is initialized
    unsafe {
        if PROPOSAL_STORAGE.is_none() {
            PROPOSAL_STORAGE = Some(ProposalStorage::new());
        }

        // Access the proposal storage and retrieve the count
        if let Some(storage) = PROPOSAL_STORAGE.as_ref() {
            return storage.get_proposal_count();
        }
    }

    0 // Return 0 if the storage is not initialized
}

#[ic_cdk::update]
fn create_proposal(key: u64, proposal: CreateProposal) -> Option<Proposal> {
    // Ensure that the storage is initialized
    unsafe {
        if PROPOSAL_STORAGE.is_none() {
            PROPOSAL_STORAGE = Some(ProposalStorage::new());
        }

        // Access the proposal storage and create a new proposal
        if let Some(storage) = PROPOSAL_STORAGE.as_mut() {
            // Create a new Proposal instance based on the CreateProposal input
            let new_proposal = Proposal {
                // Initialize fields based on the input proposal
                // For example:
                // field1: proposal.field1,
                // field2: proposal.field2,
            };

            // Call the create_proposal method of ProposalStorage
            return storage.create_proposal(key, new_proposal);
        }
    }

    None
}

##[ic_cdk::update]
fn edit_proposal(key: u64, proposal: CreateProposal) -> Result<(), VoteError> {
    // Access the memory manager and edit an existing proposal
    MEMORY_MANAGER.with(|memory_manager| {
        // Try to get the existing proposals storage
        if let Some(storage) = memory_manager.borrow_mut().get_mut::<BTreeMap<u64, Proposal>>() {
            // Check if the proposal exists
            if let Some(existing_proposal) = storage.get_mut(&key) {
                // Update the existing proposal fields with the new data
                existing_proposal.title = proposal.title;
                existing_proposal.description = proposal.description;
                existing_proposal.creator = proposal.creator;

                return Ok(());
            }
            
            // Return an error if the proposal does not exist
            return Err(VoteError::ProposalNotFound);
        }

        // Return an error if the storage is not initialized
        Err(VoteError::ProposalNotFound)
    })
}


#[ic_cdk::update]
fn end_proposal(key: u64) -> Result<(), VoteError> {
    // Access the memory manager and end an existing proposal
    MEMORY_MANAGER.with(|memory_manager| {
        // Try to get the existing proposals storage
        if let Some(storage) = memory_manager.borrow_mut().get_mut::<BTreeMap<u64, Proposal>>() {
            // Check if the proposal exists
            if let Some(existing_proposal) = storage.remove(&key) {
                // Proposal removed successfully
                // You can perform additional cleanup or logging if needed
                return Ok(());
            }
            
            // Return an error if the proposal does not exist
            return Err(VoteError::ProposalNotFound);
        }

        // Return an error if the storage is not initialized
        Err(VoteError::ProposalNotFound)
    })
}
#[ic_cdk::update]
fn vote(key: u64, choice: Choice) -> Result<(), VoteError> {
    // Access the memory manager and vote on an existing proposal
    MEMORY_MANAGER.with(|memory_manager| {
        // Try to get the existing proposals storage
        if let Some(storage) = memory_manager.borrow_mut().get_mut::<BTreeMap<u64, Proposal>>() {
            // Check if the proposal exists
            if let Some(existing_proposal) = storage.get_mut(&key) {
                // Update the vote for the proposal based on the choice
                match choice {
                    Choice::Option1 => existing_proposal.vote_option1 += 1,
                    Choice::Option2 => existing_proposal.vote_option2 += 1,
                }

                // You can perform additional logic or validation here

                return Ok(());
            }
            
            // Return an error if the proposal does not exist
            return Err(VoteError::ProposalNotFound);
        }

        // Return an error if the storage is not initialized
        Err(VoteError::ProposalNotFound)
    })
}