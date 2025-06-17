#![allow(unused)]

use anchor_lang::{
    prelude::{
        borsh::{BorshDeserialize, BorshSchema, BorshSerialize},
        *,
    },
    solana_program::{instruction::Instruction, program::invoke_signed},
};

declare_id!("GovER5Lthms3bLBqWub97yVrMmEogzX7xNjdXpPPCVZw");

pub fn deposit_governing_tokens<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, DepositGoverningTokens<'info>>,
    amount: u64,
) -> Result<()> {
    invoke_signed(
        &Instruction {
            program_id: ID,
            accounts: vec![
                AccountMeta::new_readonly(ctx.accounts.realm.key(), false),
                AccountMeta::new(ctx.accounts.governing_token_holding.key(), false),
                AccountMeta::new(ctx.accounts.governing_token_source.key(), false),
                AccountMeta::new_readonly(ctx.accounts.governing_token_owner.key(), true),
                AccountMeta::new_readonly(ctx.accounts.governing_token_source_authority.key(), true),
                AccountMeta::new(ctx.accounts.token_owner_record.key(), false),
                AccountMeta::new(ctx.accounts.payer.key(), true),
                AccountMeta::new_readonly(ctx.accounts.system_program.key(), false),
                AccountMeta::new_readonly(ctx.accounts.token_program.key(), false),
                AccountMeta::new_readonly(ctx.accounts.realm_config.key(), false),
            ],
            data: borsh::to_vec(&GovernanceInstruction::DepositGoverningTokens { amount }).unwrap(),
        },
        &ctx.accounts.to_account_infos(),
        ctx.signer_seeds,
    )
    .map_err(Into::into)
}

pub fn withdraw_governing_tokens<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, WithdrawGoverningTokens<'info>>,
) -> Result<()> {
    invoke_signed(
        &Instruction {
            program_id: ID,
            accounts: vec![
                AccountMeta::new_readonly(ctx.accounts.realm.key(), false),
                AccountMeta::new(ctx.accounts.governing_token_holding.key(), false),
                AccountMeta::new(ctx.accounts.governing_token_destination.key(), false),
                AccountMeta::new_readonly(ctx.accounts.governing_token_owner.key(), true),
                AccountMeta::new(ctx.accounts.token_owner_record.key(), false),
                AccountMeta::new_readonly(ctx.accounts.token_program.key(), false),
                AccountMeta::new_readonly(ctx.accounts.realm_config.key(), false),
            ],
            data: borsh::to_vec(&GovernanceInstruction::WithdrawGoverningTokens {}).unwrap(),
        },
        &ctx.accounts.to_account_infos(),
        ctx.signer_seeds,
    )
    .map_err(Into::into)
}

pub fn set_governance_delegate<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, SetGovernanceDelegate<'info>>,
    new_governance_delegate: Option<Pubkey>,
) -> Result<()> {
    invoke_signed(
        &Instruction {
            program_id: ID,
            accounts: vec![
                AccountMeta::new_readonly(ctx.accounts.governing_token_owner.key(), true),
                AccountMeta::new(ctx.accounts.token_owner_record.key(), false),
            ],
            data: borsh::to_vec(&GovernanceInstruction::SetGovernanceDelegate {
                new_governance_delegate,
            })
            .unwrap(),
        },
        &ctx.accounts.to_account_infos(),
        ctx.signer_seeds,
    )
    .map_err(Into::into)
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

/// Instructions supported by the Governance program
#[derive(Clone, Debug, PartialEq, Eq, BorshDeserialize, BorshSerialize, BorshSchema)]
#[allow(clippy::large_enum_variant)]
pub enum GovernanceInstruction {
    /// Deposits governing tokens (Community or Council) to Governance Realm and
    /// establishes your voter weight to be used for voting within the Realm
    /// Note: If subsequent (top up) deposit is made and there are active votes
    /// for the Voter then the vote weights won't be updated automatically
    /// It can be done by relinquishing votes on active Proposals and voting
    /// again with the new weight
    ///
    ///  0. `[]` Realm account
    ///  1. `[writable]` Governing Token Holding account.
    ///     * PDA seeds: ['governance',realm, governing_token_mint]
    ///  2. `[writable]` Governing Token Source account. It can be either
    ///     spl-token TokenAccount or MintAccount Tokens will be transferred or
    ///     minted to the Holding account
    ///  3. `[signer]` Governing Token Owner account
    ///  4. `[signer]` Governing Token Source account authority It should be
    ///     owner for TokenAccount and mint_authority for MintAccount
    ///  5. `[writable]` TokenOwnerRecord account.
    ///     * PDA seeds: ['governance',realm, governing_token_mint,
    ///       governing_token_owner]
    ///  6. `[signer]` Payer
    ///  7. `[]` System
    ///  8. `[]` SPL Token program
    ///  9. `[]` RealmConfig account.
    ///     * PDA seeds: ['realm-config', realm]
    DepositGoverningTokens {
        /// The amount to deposit into the realm
        #[allow(dead_code)]
        amount: u64,
    },

    /// Withdraws governing tokens (Community or Council) from Governance Realm
    /// and downgrades your voter weight within the Realm.
    /// Note: It's only possible to withdraw tokens if the Voter doesn't have
    /// any outstanding active votes.
    /// If there are any outstanding votes then they must be relinquished
    /// before tokens could be withdrawn
    ///
    ///  0. `[]` Realm account
    ///  1. `[writable]` Governing Token Holding account.
    ///     * PDA seeds: ['governance',realm, governing_token_mint]
    ///  2. `[writable]` Governing Token Destination account. All tokens will be
    ///     transferred to this account
    ///  3. `[signer]` Governing Token Owner account
    ///  4. `[writable]` TokenOwnerRecord account.
    ///     * PDA seeds: ['governance',realm, governing_token_mint,
    ///       governing_token_owner]
    ///  5. `[]` SPL Token program
    ///  6. `[]` RealmConfig account.
    ///     * PDA seeds: ['realm-config', realm]
    WithdrawGoverningTokens {},

    /// Sets Governance Delegate for the given Realm and Governing Token Mint
    /// (Community or Council). The Delegate would have voting rights and
    /// could vote on behalf of the Governing Token Owner. The Delegate would
    /// also be able to create Proposals on behalf of the Governing Token
    /// Owner.
    /// Note: This doesn't take voting rights from the Token Owner who still can
    /// vote and change governance_delegate
    ///
    /// 0. `[signer]` Current Governance Delegate or Governing Token owner
    /// 1. `[writable]` Token Owner  Record
    SetGovernanceDelegate {
        #[allow(dead_code)]
        /// New Governance Delegate
        new_governance_delegate: Option<Pubkey>,
    },
}

#[derive(Clone)]
pub struct SplGovernance;

impl anchor_lang::Id for SplGovernance {
    fn id() -> Pubkey {
        ID
    }
}
