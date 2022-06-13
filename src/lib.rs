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

    // Implement specific voting methods
    async fn get_equities(&self, member: Principal) -> Result<u64, String>;

    // Implement process completed proposals
    async fn handle_prposal(&self) -> Result<(), String>;
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct DaoBasic<T:DaoCustomFn> {
    prposal_list: Vec<Prposal>,
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
            prposal_list: Vec::new(),
            next_prposal_id: 1,
            custom_fn: custom_fn,
        }
    }

    /// Submit the proposal
    pub async fn proposal(&mut self, arg: PrposalArg) -> Result<(), String> {
        self.custom_fn.is_member(arg.proposer.clone()).await?;
        let propsal = Prposal {
            id: self.next_prposal_id,
            proposer: arg.proposer,
            title: arg.title,
            content: arg.content,
            proposal_state: ProposalState::Open,
            vote_data: Vec::new(),
            end_time: arg.end_time,
            timestemp: api::time(),
        };
        self.prposal_list.push(propsal);
        self.next_prposal_id += 1;
        Ok(())
    }

    pub fn get_prposal(&self, id: u64) -> Result<Prposal, String>{
        for proposal in self.prposal_list.iter() {
            if proposal.id == id {
                return Ok(proposal.clone());
            }
        }
        Err(String::from("no prposal"))
    }

    pub fn proposal_list(&self) -> Vec<Prposal> {
        self.prposal_list.clone()
    }

    pub async fn vote(&self, arg: VotesArg) -> Result<bool, String> {
        self.custom_fn.is_member(arg.caller.clone()).await?;
        let weight = self.custom_fn.get_equities(arg.caller.clone()).await?;
        todo!()
    }

    pub async fn handle_prposal(&self) -> Result<(), String>{
        self.custom_fn.handle_prposal().await?;
        Ok(())
    }
}
