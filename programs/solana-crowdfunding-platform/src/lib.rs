use anchor_lang::prelude::*;

declare_id!("Dd461ihexmrygC9Xr6qY7hJPVWDmEURcg1GPKzg8BZF");

#[program]
pub mod solana_crowdfunding_platform {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
