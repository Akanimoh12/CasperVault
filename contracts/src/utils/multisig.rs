use odra::prelude::*;
use odra::{Address, Event, Mapping, Var};
use odra::casper_types::U512;

#[derive(Debug, PartialEq, Eq, odra::OdraType)]
pub struct Proposal {
    pub proposer: Address,
    pub target_contract: Address,
    pub execution_time: u64,
    pub executed: bool,
    pub signatures: Vec<Address>,
}

#[odra::module]
pub struct MultiSig {
    signers: Mapping<Address, bool>,
    signer_count: Var<u8>,
    required_signatures: Var<u8>,
    proposals: Mapping<u64, Proposal>,
    proposal_count: Var<u64>,
    timelock_duration: Var<u64>,
}

#[odra::module]
impl MultiSig {
    pub fn init(&mut self, initial_signers: Vec<Address>) {
        self.required_signatures.set(3);
        self.signer_count.set(initial_signers.len() as u8);
        self.timelock_duration.set(86400);
        
        for signer in initial_signers {
            self.signers.set(&signer, true);
        }
    }
    
    pub fn propose(&mut self, target_contract: Address) -> u64 {
        let caller = self.env().caller();
        if !self.is_signer(caller) {
            self.env().revert(crate::types::VaultError::Unauthorized);
        }
        
        let proposal_id = self.proposal_count.get_or_default();
        let current_time = self.env().get_block_time();
        let timelock = self.timelock_duration.get_or_default();
        
        let proposal = Proposal {
            proposer: caller,
            target_contract,
            execution_time: current_time + timelock,
            executed: false,
            signatures: vec![caller],
        };
        
        self.proposals.set(&proposal_id, proposal);
        self.proposal_count.set(proposal_id + 1);
        
        self.env().emit_event(ProposalCreated {
            proposal_id,
            proposer: caller,
            target: target_contract,
            execution_time: current_time + timelock,
        });
        
        proposal_id
    }
    
    pub fn sign_proposal(&mut self, proposal_id: u64) {
        let caller = self.env().caller();
        if !self.is_signer(caller) {
            self.env().revert(crate::types::VaultError::Unauthorized);
        }
        
        let mut proposal = self.proposals.get(&proposal_id)
            .unwrap_or_else(|| self.env().revert(crate::types::VaultError::InvalidRequest));
        
        if proposal.executed {
            self.env().revert(crate::types::VaultError::InvalidRequest);
        }
        
        if proposal.signatures.contains(&caller) {
            return;
        }
        
        proposal.signatures.push(caller);
        self.proposals.set(&proposal_id, proposal.clone());
        
        self.env().emit_event(ProposalSigned {
            proposal_id,
            signer: caller,
            signature_count: proposal.signatures.len() as u8,
        });
    }
    
    pub fn execute_proposal(&mut self, proposal_id: u64) -> bool {
        let mut proposal = self.proposals.get(&proposal_id)
            .unwrap_or_else(|| self.env().revert(crate::types::VaultError::InvalidRequest));
        
        if proposal.executed {
            self.env().revert(crate::types::VaultError::InvalidRequest);
        }
        
        let required = self.required_signatures.get_or_default();
        if (proposal.signatures.len() as u8) < required {
            self.env().revert(crate::types::VaultError::Unauthorized);
        }
        
        let current_time = self.env().get_block_time();
        if current_time < proposal.execution_time {
            self.env().revert(crate::types::VaultError::TimelockActive);
        }
        
        proposal.executed = true;
        self.proposals.set(&proposal_id, proposal.clone());
        
        self.env().emit_event(ProposalExecuted {
            proposal_id,
            executor: self.env().caller(),
            timestamp: current_time,
        });
        
        true
    }
    
    pub fn is_signer(&self, address: Address) -> bool {
        self.signers.get(&address).unwrap_or(false)
    }
    
    pub fn add_signer(&mut self, signer: Address) {
        self.signers.set(&signer, true);
        let count = self.signer_count.get_or_default();
        self.signer_count.set(count + 1);
    }
    
    pub fn remove_signer(&mut self, signer: Address) {
        self.signers.set(&signer, false);
        let count = self.signer_count.get_or_default();
        if count > 0 {
            self.signer_count.set(count - 1);
        }
    }
    
    pub fn get_proposal(&self, proposal_id: u64) -> Option<Proposal> {
        self.proposals.get(&proposal_id)
    }
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct ProposalCreated {
    pub proposal_id: u64,
    pub proposer: Address,
    pub target: Address,
    pub execution_time: u64,
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct ProposalSigned {
    pub proposal_id: u64,
    pub signer: Address,
    pub signature_count: u8,
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct ProposalExecuted {
    pub proposal_id: u64,
    pub executor: Address,
    pub timestamp: u64,
}
