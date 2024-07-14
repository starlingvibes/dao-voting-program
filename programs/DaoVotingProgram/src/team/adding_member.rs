use super::errors::ErrorCode;
use super::*;

pub fn adding_member(
    ctx: Context<AddMember>,
    _team_name: String,
    _team_id: u64,
    member: Pubkey,
) -> Result<()> {
    let team = &mut ctx.accounts.team_account;

    require!(team.members.len() < 5, ErrorCode::TeamCapacityFullError);
    require!(
        !team.members.contains(&member),
        ErrorCode::MemberAlreadyInTeamError
    );
    // ensure that the signer is a captain
    require!(
        team.captain == *ctx.accounts.signer.key,
        ErrorCode::NotCaptainError
    );

    team.members.push(member);

    msg!("{} is successfully added to the team {}", member, team.name);

    Ok(())
}

// derive macro for adding member instruction
#[derive(Accounts)]
#[instruction(team_name: String, team_id: u64)]
pub struct AddMember<'info> {
    #[account(mut, seeds=[team_name.as_bytes(), &team_id.to_ne_bytes()], bump = team_account.bump)]
    pub team_account: Account<'info, TeamAccount>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}
