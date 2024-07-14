use super::errors::ErrorCode;
use super::*;

pub fn initiating_tournament(
    ctx: Context<InitTournament>,
    _team_name: String,
    _team_id: u64,
    tournament_address: Pubkey,
    tournament_prize: u64,
) -> Result<()> {
    let team = &mut ctx.accounts.team_account;

    require!(
        team.captain == *ctx.accounts.signer.key,
        ErrorCode::NotCaptainError
    );

    require!(
        team.active_tournament == Pubkey::default(),
        ErrorCode::AlreadyActiveTournamentError
    );

    team.active_tournament = tournament_address;
    team.prize = tournament_prize;

    Ok(())
}

// derive macro for init tournament instruction
#[derive(Accounts)]
#[instruction(_team_name: String, _team_id: u64)]
pub struct InitTournament<'info> {
    #[account(mut, seeds=[_team_name.as_bytes(), &_team_id.to_ne_bytes()], bump = team_account.bump)]
    pub team_account: Account<'info, TeamAccount>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}
