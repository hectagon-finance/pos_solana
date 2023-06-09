use anchor_lang::prelude::*;

#[cfg(feature = "mainnet")]
declare_id!("wormDTUJ6AWPNvk59vGQbDvGJmqbDTdgWgAqcLBCgUb");

#[cfg(feature = "solana-devnet")]
declare_id!("DZnkkTmCiFWfYTfT41X3Rd1kDgozqzxWaHqsw6W4x2oe");

declare_id!("B6RHG3mfcckmrYN1UhmJzyS1XX3fZKbkeUcpJe9Sy3FE");

#[derive(Debug, Clone)]
pub struct TokenBridge;

impl Id for TokenBridge {
    fn id() -> Pubkey {
        ID
    }
}
