use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token};

declare_id!("NNETxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx");

#[program]
pub mod neuralnet {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let market = &mut ctx.accounts.market;
        market.authority = ctx.accounts.authority.key();
        market.total_providers = 0;
        market.total_consumers = 0;
        market.total_tasks = 0;
        Ok(())
    }

    pub fn register_provider(ctx: Context<RegisterProvider>, specs: ProviderSpecs) -> Result<()> {
        let provider = &mut ctx.accounts.provider;
        let market = &mut ctx.accounts.market;

        provider.authority = ctx.accounts.authority.key();
        provider.specs = specs;
        provider.reputation = 0;
        provider.total_tasks = 0;
        provider.active = true;

        market.total_providers = market.total_providers.checked_add(1).unwrap();
        Ok(())
    }

    pub fn create_task(ctx: Context<CreateTask>, config: TaskConfig) -> Result<()> {
        let task = &mut ctx.accounts.task;
        let market = &mut ctx.accounts.market;

        task.authority = ctx.accounts.authority.key();
        task.provider = ctx.accounts.provider.key();
        task.config = config;
        task.status = TaskStatus::Created;
        task.created_at = Clock::get()?.unix_timestamp;

        market.total_tasks = market.total_tasks.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum TaskStatus {
    Created,
    InProgress,
    Completed,
    Failed,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct ProviderSpecs {
    pub gpu_model: String,
    pub vram_gb: u8,
    pub cuda_cores: u32,
    pub bandwidth_mbps: u32,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct TaskConfig {
    pub min_vram_gb: u8,
    pub min_cuda_cores: u32,
    pub expected_duration: i64,
    pub max_price: u64,
}

#[account]
pub struct Market {
    pub authority: Pubkey,
    pub total_providers: u64,
    pub total_consumers: u64,
    pub total_tasks: u64,
}

#[account]
pub struct Provider {
    pub authority: Pubkey,
    pub specs: ProviderSpecs,
    pub reputation: u32,
    pub total_tasks: u64,
    pub active: bool,
}

#[account]
pub struct Task {
    pub authority: Pubkey,
    pub provider: Pubkey,
    pub config: TaskConfig,
    pub status: TaskStatus,
    pub created_at: i64,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 8 + 32 + 8 + 8 + 8)]
    pub market: Account<'info, Market>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RegisterProvider<'info> {
    #[account(init, payer = authority, space = 8 + 32 + 64 + 4 + 8 + 1)]
    pub provider: Account<'info, Provider>,
    #[account(mut)]
    pub market: Account<'info, Market>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateTask<'info> {
    #[account(init, payer = authority, space = 8 + 32 + 32 + 64 + 1 + 8)]
    pub task: Account<'info, Task>,
    #[account(mut)]
    pub market: Account<'info, Market>,
    pub provider: Account<'info, Provider>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
} 