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
pub trait DaoCustomFn {
    // It is used to determine whether you are DAO member of Organization A
    pub async fn is_member(member: Principal) -> Result<bool, String>;

    // Implement specific voting methods
    pub async fn get_equities(member: Principal) -> Result<u64, String>;

    // Implement process completed proposals
    pub async fn handle_prposal();
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct DaoBasic<T:DaoCustomFn> {
    prposal: Vec<Prposal>,
    next_prposal_id: u64,
    custom_fn: T,
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

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
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
    pub fn new(custom_fn: T) -> DaoBasic {
        DaoBasic {
            prposal: Vec::new(),
            next_prposal_id: 1,
            custom_fn: custom_fn,
        }
    }

    /// Submit the proposal
    pub async fn proposal(&mut self, arg: PrposalArg) -> Result<bool, String> {
        self.custom_fn.is_member(&caller).await?;
        let propsal = Prposal {
            id: self.next_prposal_id,
            proposer: arg.proposer,
            title: arg.title,
            content: arg.content,
            proposal_state: ProposalState::Open,
            vote_data: Vec::new(),
            end_time: arg.end_time,
            timestemp: ic_cdk::timestemp,
        };
        self.next_prposal_id += 1;
        true
    }

    pub fn get_prposal(&self, id: u64) -> Result<Prposal, String> {
        self.prposal[id]
    }

    pub fn proposal_list(&self) -> Result<Vec<Prposal>, String> {
        self.prposal
    }

    pub async fn vote(&self, arg: VotesArg) -> Result<bool, String> {
        self.custom_fn.is_member(&arg.caller).await?;
        let weight = self.custom_fn.get_equities(&arg.caller).await?;
        todo!()
    }

    pub async fn handle_prposal(&self) {
        self.custom_fn.handle_prposal().await?;
    }
}