use super::errors::ErrorCode;
use super::*;

pub fn removing_member(
    ctx: Context<RemoveMember>,
    _team_name: String,
    _team_id: u64,
    member: Pubkey,
) -> Result<()> {
    let team = &mut ctx.accounts.team_account;

    require!(team.members.len() > 1, ErrorCode::TeamCapacityLowError);

    require!(team.captain != member, ErrorCode::NotCaptainError);

    require!(
        team.members.contains(&member),
        ErrorCode::MemberNotInTeamError
    );

    require!(
        team.members.contains(&member),
        ErrorCode::MemberNotInTeamError
    );

    team.members.retain(|&x| x != member);

    msg!(
        "{} is successfully removed from the team {}",
        member,
        team.name
    );

    Ok(())
}

// derive macro to remove member from team
#[derive(Accounts)]
#[instruction(team_name: String, team_id: u64)]
pub struct RemoveMember<'info> {
    #[account(mut, seeds=[team_name.as_bytes(), &team_id.to_ne_bytes()], bump = team_account.bump)]
    pub team_account: Account<'info, TeamAccount>,

    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
