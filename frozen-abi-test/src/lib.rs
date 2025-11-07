use rand::distributions::{Distribution, Standard};
use rand::Rng;
use serde::{Deserialize, Serialize};
use solana_frozen_abi_macro::frozen_abi;
use solana_frozen_abi_macro::AbiExample;

#[cfg_attr(
    feature = "frozen-abi",
    // derive(AbiExample),
    // derive(AbiEnumVisitor),
    frozen_abi(
        api_digest = "fFcqrZWZX4WcorTUxfMCVWeh2QcwamXKdLTzsDj58Kn",
        abi_digest = "5RoDSUMyqu38JmXiZBKXzAGRuku7JfHtKamyGqnYN7ND"
    )
)]
#[derive(Serialize, Deserialize)]
pub struct BlockhashQueue {
    pub last_hash_index: u64,
    pub a: u16,
}

impl Distribution<BlockhashQueue> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> BlockhashQueue {
        BlockhashQueue {
            last_hash_index: rng.gen(),
            a: rng.gen(),
        }
    }
}
