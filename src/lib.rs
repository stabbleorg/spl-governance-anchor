#![allow(unused)]

use anchor_lang::{
    prelude::{
        borsh::{BorshDeserialize, BorshSchema, BorshSerialize},
        *,
    },
    solana_program::{instruction::Instruction, program::invoke_signed},
};
use spl_governance::instruction::{
    deposit_governing_tokens as create_deposit_governing_tokens_ix,
    set_governance_delegate as create_set_governance_delegate_ix,
    withdraw_governing_tokens as create_withdraw_governing_tokens_ix,
};

declare_id!("GovER5Lthms3bLBqWub97yVrMmEogzX7xNjdXpPPCVZw");

pub fn deposit_governing_tokens<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, DepositGoverningTokens<'info>>,
    amount: u64,
    governing_token_mint: Pubkey,
) -> Result<()> {
    let ix: Instruction = create_deposit_governing_tokens_ix(
        &ID,
        ctx.accounts.realm.key,
        ctx.accounts.governing_token_source.key,
        ctx.accounts.governing_token_owner.key,
        ctx.accounts.governing_token_source_authority.key,
        ctx.accounts.payer.key,
        amount,
        &governing_token_mint,
    );

    invoke_signed(&ix, &ToAccountInfos::to_account_infos(&ctx), ctx.signer_seeds).map_err(Into::into)
}

pub fn withdraw_governing_tokens<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, WithdrawGoverningTokens<'info>>,
    governing_token_mint: Pubkey,
) -> Result<()> {
    let ix: Instruction = create_withdraw_governing_tokens_ix(
        &ID,
        ctx.accounts.realm.key,
        ctx.accounts.governing_token_destination.key,
        ctx.accounts.governing_token_owner.key,
        &governing_token_mint,
    );

    invoke_signed(&ix, &ToAccountInfos::to_account_infos(&ctx), ctx.signer_seeds).map_err(Into::into)
}

pub fn set_governance_delegate<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, SetGovernanceDelegate<'info>>,
    realm: Pubkey,
    governing_token_mint: Pubkey,
    new_governance_delegate: Option<Pubkey>,
) -> Result<()> {
    let ix: Instruction = create_set_governance_delegate_ix(
        &ID,
        ctx.accounts.governing_token_owner.key,
        &realm,
        &governing_token_mint,
        ctx.accounts.governing_token_owner.key,
        &new_governance_delegate,
    );

    invoke_signed(&ix, &ToAccountInfos::to_account_infos(&ctx), ctx.signer_seeds).map_err(Into::into)
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

#[derive(Clone)]
pub struct SplGovernance;

impl anchor_lang::Id for SplGovernance {
    fn id() -> Pubkey {
        ID
    }
}
