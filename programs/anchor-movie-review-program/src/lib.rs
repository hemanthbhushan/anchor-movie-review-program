use anchor_lang::prelude::*;

declare_id!("5KPYjQj6BqDRnCiHzEBXWbQENgxStDX8AKdSvCZS3GX5");

#[program]
pub mod anchor_movie_review_program {
    use super::*;

    pub fn create_movie_review(ctx: Context<InitMovieReview>, title :String , review : String , rating : u8) -> Result<()> {

        let movie_review = &mut ctx.accounts.movie_review_pda;
        movie_review.title = title;
        movie_review.review = review;
        movie_review.rating = rating;
        Ok(())
    }

    pub fn update_movie_review(ctx: Context<UpdateMovieReview>, title :String , review : String , rating: u8 ) -> Result<()> {
        let movie_review_pda = &mut ctx.accounts.movie_review_pda;
        movie_review_pda.review = review;
        movie_review_pda.rating = rating;
        Ok(())
    }

    pub fn delete_movie_review(ctx: Context<DeleteMovieReview>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(title:String ,review : String)]
pub struct InitMovieReview<'info> {
 #[account(
    init , 
    seeds = [title.as_bytes() ,reviewer.key().as_ref()],
    bump ,
    payer = reviewer,
    space = 8 + 1 + 4 + title.len() + 4 + review.len(),
 )]
 pub movie_review_pda : Account<'info , Review> ,
 #[account(mut)]
 pub reviewer : Signer<'info> ,
 pub system_program: Program<'info, System>

}

#[derive(Accounts)]
#[instruction(title:String ,review : String)]
pub struct UpdateMovieReview<'info>{
    #[account(
    mut ,
    seeds = [title.as_bytes() ,reviewer.key().as_ref()],
    bump ,
    realloc = 8 + 1 + 4 + title.len() + 4 + review.len(),
    realloc::payer = reviewer,
    realloc::zero = false,
    )]
 pub movie_review_pda : Account<'info , Review> ,
 #[account(mut)]
 pub reviewer : Signer<'info> ,
 pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct DeleteMovieReview<'info> {
    #[account(mut, close = receiver)]
    pub movie_review_pda : Account<'info , Review> ,
    #[account(mut)]
    pub receiver : Signer<'info> ,
}

#[account]
pub struct Review {
    pub title: String,
    pub review: String,
    pub rating: u8,
}
