use super::errors::ErrorCode;
use super::*;

pub fn leaving_tournament(
    ctx: Context<LeaveTournament>,
    _team_name: String,
    _team_id: u64,
    vote_type: VoteType,
) -> Result<()> {
    let team = &mut ctx.accounts.team_account;

    require!(
        team.active_tournament != Pubkey::default(),
        ErrorCode::NoActiveTournamentError
    );

    require!(
        team.members.contains(ctx.accounts.signer.key),
        ErrorCode::MemberNotInTeamError
    );

    require!(
        !team.voted_players.contains(ctx.accounts.signer.key),
        ErrorCode::AlreadyVotedError
    );

    match vote_type {
        VoteType::Yes => {
            team.leave_voted_players.push(*ctx.accounts.signer.key);
            team.leave_votes += 1;
        }
        VoteType::No => {
            team.voted_players.push(*ctx.accounts.signer.key);
        }
    }

    if team.leave_votes > 3 {
        // if yes votes are more than half of the team members
        // remove the tournament from the team's active tournament
        team.active_tournament = Pubkey::default();
        team.leave_votes = 0;
        team.leave_voted_players = vec![];
        team.voted_players = vec![];
        team.voting_result = false;
        team.yes_votes = 0;

        msg!(
            "{} is successfully left the tournament {}",
            team.name,
            team.name
        );
    }

    Ok(())
}

// derive macro to vote for leaving the tournament
#[derive(Accounts)]
#[instruction(_team_name: String, _team_id: u64)]
pub struct LeaveTournament<'info> {
    #[account(mut, seeds=[_team_name.as_bytes(), &_team_id.to_ne_bytes()], bump = team_account.bump)]
    pub team_account: Account<'info, TeamAccount>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}
