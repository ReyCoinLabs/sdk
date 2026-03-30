# 📁 programs/rey_staking/src/lib.rs

```rust
use anchor_lang::prelude::*;

declare_id!("Rey111111111111111111111111111111111111111");

#[program]
pub mod rey_staking {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.staking_account.total_staked = 0;
        Ok(())
    }

    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        let user = &mut ctx.accounts.user_account;
        let staking = &mut ctx.accounts.staking_account;

        user.amount += amount;
        staking.total_staked += amount;

        Ok(())
    }

    pub fn unstake(ctx: Context<Unstake>, amount: u64) -> Result<()> {
        let user = &mut ctx.accounts.user_account;
        let staking = &mut ctx.accounts.staking_account;

        require!(user.amount >= amount, ErrorCode::InsufficientFunds);

        user.amount -= amount;
        staking.total_staked -= amount;

        Ok(())
    }
}

#[account]
pub struct StakingAccount {
    pub total_staked: u64,
}

#[account]
pub struct UserAccount {
    pub amount: u64,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub staking_account: Account<'info, StakingAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub staking_account: Account<'info, StakingAccount>,
    #[account(mut)]
    pub user_account: Account<'info, UserAccount>,
}

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(mut)]
    pub staking_account: Account<'info, StakingAccount>,
    #[account(mut)]
    pub user_account: Account<'info, UserAccount>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient funds")]
    InsufficientFunds,
}
```
