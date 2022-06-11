use ic_cdk::export::{
    candid::{CandidType, Deserialize},
    Principal,
};

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub type Equities = u64;

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub enum Votes{
    Yes(Equities),
    No(Equities),
}


/// You need to use the basic methods implemented by the party
pub trait DaoTrait {
    // It is used to determine whether you are DAO member of Organization A
    async fn is_member(member: Principal) -> Result<bool, String>;

    // Implement specific voting methods
    async fn get_weight(member: Principal) -> Result<u64, String>;

    // Implement process completed proposals
    async fn handle_prposal();
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct DaoBasic {
    prposal: Prposal,
    next_prposal_id: u64,
}

// The state of a Proposal
#[derive(Clone, Debug, CandidType, Deserialize, PartialEq)]
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

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
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

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct PrposalArg {
    proposer: Principal,
    title: String,
    content: String,
    end_time: u64,
}


impl DaoBasic {
    pub fn new() -> DaoBasic {
        todo!()
    }

    pub fn proposal(&self, arg: PrposalArg) -> Result<bool, String> {
        todo!()
    }

    pub fn get_prposal(&self, id: u64) -> Result<Prposal, String> {
        todo!()
    }

    pub fn proposal_list(&self) -> Result<Vec<Prposal>, String> {
        todo!()
    }

    pub async fn votes(&self, caller: Principal, id: u64, dao_trait: &impl DaoTrait) -> Result<bool, String> {
        dao_trait.is_member(&caller).await?;
        let weight = dao_trait.get_weight(&ic::caller).await?;
        todo!()
    }
}