use super::*;

pub fn view_vote_results(
    ctx: Context<ViewVoteResults>,
    _team_name: String,
    _team_id: u64,
) -> Result<()> {
    let team = &mut ctx.accounts.team_account;

    msg!("Vote results");
    msg!("Yes votes: {}", team.yes_votes);
    msg!("No votes: {}", team.voted_players.len());

    Ok(())
}

// derive macro to view vote results instruction
#[derive(Accounts)]
#[instruction(_team_name: String, _team_id: u64)]
pub struct ViewVoteResults<'info> {
    #[account(mut, seeds=[_team_name.as_bytes(), &_team_id.to_ne_bytes()], bump = team_account.bump)]
    pub team_account: Account<'info, TeamAccount>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}
