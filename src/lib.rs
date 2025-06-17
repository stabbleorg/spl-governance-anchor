#![allow(unused)]

use anchor_lang::prelude::*;

declare_id!("GovER5Lthms3bLBqWub97yVrMmEogzX7xNjdXpPPCVZw");

#[program]
pub mod spl_governance {
    use super::*;

    pub fn deposit_governing_tokens(ctx: Context<DepositGoverningTokens>, amount: u64) -> Result<()> {
        Ok(())
    }

    pub fn withdraw_governing_tokens(ctx: Context<WithdrawGoverningTokens>) -> Result<()> {
        Ok(())
    }

    pub fn set_governance_delegate(ctx: Context<SetGovernanceDelegate>, new_delegate: Option<Pubkey>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct DepositGoverningTokens<'info> {
    pub realm: AccountInfo<'info>,
    #[account(mut)]
    pub governing_token_holding: AccountInfo<'info>,
    #[account(mut)]
    pub governing_token_source: AccountInfo<'info>,
    pub governing_token_owner: AccountInfo<'info>,
    pub governing_token_source_authority: AccountInfo<'info>,
    #[account(mut)]
    pub token_owner_record: AccountInfo<'info>,
    #[account(mut)]
    pub payer: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
    pub realm_config: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct WithdrawGoverningTokens<'info> {
    pub realm: AccountInfo<'info>,
    #[account(mut)]
    pub governing_token_holding: AccountInfo<'info>,
    #[account(mut)]
    pub governing_token_destination: AccountInfo<'info>,
    pub governing_token_owner: AccountInfo<'info>,
    #[account(mut)]
    pub token_owner_record: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
    pub realm_config: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct SetGovernanceDelegate<'info> {
    pub governing_token_owner: AccountInfo<'info>,
    #[account(mut)]
    pub token_owner_record: AccountInfo<'info>,
}
