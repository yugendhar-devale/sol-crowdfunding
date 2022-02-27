use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod crowdfundingplatform {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, amount: u64, description: String) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        base_account.campaign_owner = *ctx.accounts.user.owner;
        base_account.campaign_amount = amount;
        base_account.campaign_descriptions = description;
        base_account.campaign_fulfilled = 0;
        Ok(())
    }
    pub fn get_campaign_status(ctx: Context<CampaignStatus>) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        if (base_account.campaign_fulfilled < base_account.campaign_amount) {
            base_account.campaign_status = "ACTIVE".to_string();
        } else {
            base_account.campaign_status = "FULFILLED".to_string();
        }
        Ok(())
    }
    pub fn fund_campaign(ctx: Context<FundCampaign>, amount: u64) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        base_account.campaign_fulfilled += amount;
        Ok(())
    }
    pub fn collect_funds_and_close_campaign(
        ctx: Context<CollectFundsAndCloseCampaign>,
    ) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        base_account.campaign_status = "CLOSED".to_string();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CampaignStatus<'info> {
    #[account(mut)]
    base_account: Account<'info, AccountDetails>,
}

#[derive(Accounts)]
pub struct FundCampaign<'info> {
    #[account(mut)]
    base_account: Account<'info, AccountDetails>,
}

#[derive(Accounts)]
pub struct FundsLeft<'info> {
    #[account(mut)]
    base_account: Account<'info, AccountDetails>,
}

#[derive(Accounts)]
pub struct CollectFundsAndCloseCampaign<'info> {
    #[account(mut)]
    base_account: Account<'info, AccountDetails>,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init,payer=user,space=64*64)]
    pub base_account: Account<'info, AccountDetails>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct AccountDetails {
    pub campaign_owner: Pubkey,
    pub campaign_amount: u64,
    pub campaign_descriptions: String,
    pub campaign_fulfilled: u64,
    pub campaign_status: String,
}
