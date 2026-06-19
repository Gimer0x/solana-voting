use anchor_lang::prelude::*;

// Program ID: Public Key of the voting program
declare_id!("54rNndwQkumaVeqyJRh19MPDzeEUQLPspNj4wtc7FYAr");

// Macro 
#[program]
pub mod voting {
    use super::*;

    pub fn init_poll(
        ctx: Context<InitializePoll>, 
        _poll_id: u64,
        poll_name: String,
        poll_description: String,
        poll_voting_start: u64,
        poll_voting_end: u64,
        poll_option_index: u64,
    ) -> Result<()> {
        let poll = &mut ctx.accounts.poll_account;
        poll.poll_name = poll_name;
        poll.poll_description = poll_description;
        poll.poll_voting_start = poll_voting_start;
        poll.poll_voting_end = poll_voting_end;
        poll.poll_option_index = poll_option_index;
        Ok(())
    }

    pub fn initialize_candidate(
        ctx: Context<InitializeCandidate>, 
        _poll_id: u64, 
        candidate_name: String) -> Result<()> {
    
        ctx.accounts.candidate_account.candidate_name = candidate_name;
        ctx.accounts.poll_account.poll_option_index += 1;
        
        Ok(())
    }

    pub fn vote(ctx: Context<InitializeVote>, _poll_id: u64, _candidate: String) -> Result<()> {
        let candidate = &mut ctx.accounts.candidate_account;
   
        let current_timestamp = Clock::get()?.unix_timestamp;
    
        if current_timestamp < (ctx.accounts.poll_account.poll_voting_start as i64)
        {
            return Err(ErrorCode::VotingNotStarted.into());
        }
    
        if current_timestamp > (ctx.accounts.poll_account.poll_voting_end as i64)
        {
            return Err(ErrorCode::VotingEnded.into());
        }
    
        candidate.candidate_votes += 1;
        Ok(())
    }
}

// This is an instruction account. It defenes which accounts are required and 
//how to create/validate them when running the initialize_poll instruction.
#[derive(Accounts)]
#[instruction(poll_id: u64)]
pub struct InitializePoll<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init, 
        payer = signer, 
        space = 8 + PollAccount::INIT_SPACE,
        seeds = [b"poll".as_ref(), poll_id.to_le_bytes().as_ref()],
        bump,
    )]
    pub poll_account: Account<'info, PollAccount>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(poll_id: u64, candidate: String)]
pub struct InitializeCandidate<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"poll".as_ref(), poll_id.to_le_bytes().as_ref()],
        bump,
    )]
    pub poll_account: Account<'info, PollAccount>,

    #[account(
        init, 
        payer = signer, 
        space = 8 + CandidateAccount::INIT_SPACE,
        seeds = [poll_id.to_le_bytes().as_ref(), candidate.as_ref()],
        bump,
    )]
    pub candidate_account: Account<'info, CandidateAccount>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(poll_id: u64, candidate: String)]
pub struct InitializeVote<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"poll".as_ref(), poll_id.to_le_bytes().as_ref()],
        bump,
    )]
    pub poll_account: Account<'info, PollAccount>,

    #[account(
        mut,
        seeds = [poll_id.to_le_bytes().as_ref(), candidate.as_ref()],
        bump,
    )]
    pub candidate_account: Account<'info, CandidateAccount>,
}

// on-chain data models.
#[account]
#[derive(InitSpace)]
pub struct CandidateAccount {
    #[max_len(64)]
    pub candidate_name: String,
    pub candidate_votes: u64,
}

#[account]
#[derive(InitSpace)]
pub struct PollAccount {
    #[max_len(32)]
    pub poll_name: String,
    #[max_len(280)]
    pub poll_description: String,
    pub poll_voting_start: u64,
    pub poll_voting_end: u64,
    pub poll_option_index: u64,
}

// Error code macro
#[error_code]
pub enum ErrorCode {
    #[msg("Voting ended")]
    VotingEnded,
    #[msg("Voting not started")]
    VotingNotStarted,
}