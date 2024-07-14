use super::errors::ErrorCode;
use super::*;

pub fn handle_distribute_proposal(
    ctx: Context<DistributionProposalHandler>,
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

    require!(
        !team
            .distribution_voted_players
            .contains(ctx.accounts.signer.key),
        ErrorCode::AlreadyVotedError
    );

    match vote_type {
        VoteType::Yes => {
            team.distribution_voted_players
                .push(*ctx.accounts.signer.key);
            team.distribution_yes_votes += 1;
        }
        VoteType::No => {
            team.voted_players.push(*ctx.accounts.signer.key);
        }
    }

    // react for a successful vote
    if team.distribution_voted_players.len() > 3 && team.distribution_yes_votes > 3 {
        team.distribution_voting_result = true;
    }

    // react for a failed vote
    if team.distribution_voted_players.len() > 3 && team.distribution_yes_votes < 3 {
        team.distribution_voting_result = false;
    }

    Ok(())
}

// derive macro to vote for distribution
#[derive(Accounts)]
#[instruction(_team_name: String, _team_id: u64)]
pub struct DistributionProposalHandler<'info> {
    #[account(mut, seeds=[_team_name.as_bytes(), &_team_id.to_ne_bytes()], bump = team_account.bump)]
    pub team_account: Account<'info, TeamAccount>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}
