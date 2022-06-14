use std::collections::HashMap;

use ic_cdk::export::{candid::CandidType, Principal};
use serde::{Deserialize, Serialize};
use async_trait::async_trait;
use ic_cdk::api;


pub type Equities = u64;

#[derive(CandidType, Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Votes{
    Yes(Equities),
    No(Equities),
}


/// You need to use the basic methods implemented by the party
#[async_trait]
pub trait DaoCustomFn {
    // It is used to determine whether you are DAO member of Organization A
    async fn is_member(&self, member: Principal) -> Result<bool, String>;

    // // Implement specific voting methods
    // async fn get_equities(&self, member: Principal) -> Result<u64, String>;

    // Implement process completed proposals
    async fn handle_prposal(&self) -> Result<(), String>;
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct DaoBasic<T:DaoCustomFn> {
    prposal_list: HashMap<u64, Prposal>,
    next_prposal_id: u64,
    custom_fn: T,
}

// The state of a Proposal
#[derive(CandidType, Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum ProposalState {
    // The proposal is open for voting
    Open,

    // Enough "yes" votes have been cast to accept the proposal, and it will soon be executed
    Accepted,

    // Enough "no" votes have been cast to reject the proposal, and it will not be executed
    Rejected,

    // The proposal is currently being executed
    Executing,

    // The proposal has been successfully executed
    Succeeded,

    // A failure occurred while executing the proposal
    Failed(String),
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct Prposal {
    id: u64,
    proposer: Principal,
    title: String,
    content: String,
    proposal_state: ProposalState,
    vote_data: Vec<(Principal, Votes)>,
    end_time: u64,
    timestemp: u64,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct PrposalArg {
    proposer: Principal,
    title: String,
    content: String,
    end_time: u64,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct VotesArg {
    id: u64,
    caller: Principal,
    vote: Votes,
}


impl <T> DaoBasic<T> 
where
    T: DaoCustomFn,
{
    /// Instantiate the underlying DAO
    pub fn new(custom_fn: T) -> Self {
        DaoBasic {
            prposal_list: HashMap::default(),
            next_prposal_id: 1,
            custom_fn: custom_fn,
        }
    }

    /// Submit the proposal
    pub async fn proposal(&mut self, arg: PrposalArg) -> Result<(), String> {
        self.custom_fn.is_member(arg.proposer.clone()).await?;
        let proposal = Prposal {
            id: self.next_prposal_id,
            proposer: arg.proposer,
            title: arg.title,
            content: arg.content,
            proposal_state: ProposalState::Open,
            vote_data: Vec::new(),
            end_time: arg.end_time,
            timestemp: api::time(),
        };
        self.prposal_list.insert(self.next_prposal_id, proposal);
        self.next_prposal_id += 1;
        Ok(())
    }

    pub fn get_prposal(&self, id: u64) -> Result<Prposal, String>{
        match self.prposal_list.get(&id) {
            None => return Err(String::from("no prposal")),
            Some(prposal) => return Ok(prposal.clone()),
        }
    }

    pub fn proposal_list(&self) -> HashMap<u64,Prposal> {
        self.prposal_list.clone()
    }

    pub async fn vote(&mut self, arg: VotesArg) -> Result<(), String> {
        self.custom_fn.is_member(arg.caller.clone()).await?;
        if let Some(prposal) = self.prposal_list.get_mut(&arg.id) {
            for data in prposal.vote_data.iter() {
                if data.0 == arg.caller {
                    return Err(String::from("Users have voted"));
                }
            }
            prposal.vote_data.push((arg.caller.clone(), arg.vote))
        } else {
            return Err(String::from("The proposal does not exist"));
        }
        Ok(())
    }

    pub async fn handle_prposal(&self) -> Result<(), String>{
        self.custom_fn.handle_prposal().await?;
        Ok(())
    }

    pub fn change_prposal_state() -> Result<(), String> {
        todo!();
        Ok(())
    }
}

