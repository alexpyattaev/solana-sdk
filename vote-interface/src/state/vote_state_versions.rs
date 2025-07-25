#[cfg(test)]
use arbitrary::{Arbitrary, Unstructured};
use {
    crate::{
        authorized_voters::AuthorizedVoters,
        state::{
            vote_state_0_23_5::VoteState0_23_5, vote_state_1_14_11::VoteState1_14_11, CircBuf,
            LandedVote, Lockout, VoteStateV3,
        },
    },
    solana_pubkey::Pubkey,
    std::collections::VecDeque,
};

#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Deserialize, serde_derive::Serialize)
)]
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum VoteStateVersions {
    V0_23_5(Box<VoteState0_23_5>),
    V1_14_11(Box<VoteState1_14_11>),
    Current(Box<VoteStateV3>),
}

impl VoteStateVersions {
    pub fn new_current(vote_state: VoteStateV3) -> Self {
        Self::Current(Box::new(vote_state))
    }

    pub fn convert_to_current(self) -> VoteStateV3 {
        match self {
            VoteStateVersions::V0_23_5(state) => {
                let authorized_voters =
                    AuthorizedVoters::new(state.authorized_voter_epoch, state.authorized_voter);

                VoteStateV3 {
                    node_pubkey: state.node_pubkey,

                    authorized_withdrawer: state.authorized_withdrawer,

                    commission: state.commission,

                    votes: Self::landed_votes_from_lockouts(state.votes),

                    root_slot: state.root_slot,

                    authorized_voters,

                    prior_voters: CircBuf::default(),

                    epoch_credits: state.epoch_credits.clone(),

                    last_timestamp: state.last_timestamp.clone(),
                }
            }

            VoteStateVersions::V1_14_11(state) => VoteStateV3 {
                node_pubkey: state.node_pubkey,
                authorized_withdrawer: state.authorized_withdrawer,
                commission: state.commission,

                votes: Self::landed_votes_from_lockouts(state.votes),

                root_slot: state.root_slot,

                authorized_voters: state.authorized_voters.clone(),

                prior_voters: state.prior_voters,

                epoch_credits: state.epoch_credits,

                last_timestamp: state.last_timestamp,
            },

            VoteStateVersions::Current(state) => *state,
        }
    }

    fn landed_votes_from_lockouts(lockouts: VecDeque<Lockout>) -> VecDeque<LandedVote> {
        lockouts.into_iter().map(|lockout| lockout.into()).collect()
    }

    pub fn is_uninitialized(&self) -> bool {
        match self {
            VoteStateVersions::V0_23_5(vote_state) => {
                vote_state.authorized_voter == Pubkey::default()
            }

            VoteStateVersions::V1_14_11(vote_state) => vote_state.authorized_voters.is_empty(),

            VoteStateVersions::Current(vote_state) => vote_state.authorized_voters.is_empty(),
        }
    }

    pub fn vote_state_size_of(is_current: bool) -> usize {
        if is_current {
            VoteStateV3::size_of()
        } else {
            VoteState1_14_11::size_of()
        }
    }

    pub fn is_correct_size_and_initialized(data: &[u8]) -> bool {
        VoteStateV3::is_correct_size_and_initialized(data)
            || VoteState1_14_11::is_correct_size_and_initialized(data)
    }
}

#[cfg(test)]
impl Arbitrary<'_> for VoteStateVersions {
    fn arbitrary(u: &mut Unstructured<'_>) -> arbitrary::Result<Self> {
        let variant = u.choose_index(2)?;
        match variant {
            0 => Ok(Self::Current(Box::new(VoteStateV3::arbitrary(u)?))),
            1 => Ok(Self::V1_14_11(Box::new(VoteState1_14_11::arbitrary(u)?))),
            _ => unreachable!(),
        }
    }
}
