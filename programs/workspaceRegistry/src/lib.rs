use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod workspace_registry {
    use super::*;
    
    pub fn create_workspace(ctx: Context<Create>,id: u64, _metadata_hash: String) -> ProgramResult {
        let workspace = ctx.accounts.workspace;
        //How to get the signer/owner account data?
        workspace.id = id;
        workspace.metadata_hash = _metadata_hash;
    }
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer=user, space= 8+64+64+64)]
    pub workspace: Account<'info, Workspace>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Workspace {
    id:u64,
    owner: String,
    metadata_hash: String,
}
