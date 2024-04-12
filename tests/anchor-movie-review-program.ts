import * as anchor from "@coral-xyz/anchor";
import { utils, BN } from "@project-serum/anchor";
import { expect } from "chai"

import { Program } from "@coral-xyz/anchor";
import { AnchorMovieReviewProgram } from "../target/types/anchor_movie_review_program";

describe("anchor-movie-review-program", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.AnchorMovieReviewProgram as Program<AnchorMovieReviewProgram>;
  const movie = {
    title: "Just a test movie",
    review: "Wow what a good movie it was real great",
    rating: 5,
  }

  const [moviePda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from(movie.title), provider.wallet.publicKey.toBuffer()],
    program.programId
  )

  it("Is initialized!", async () => {
    // Add your test here.
    const rating = new BN(10);
    const tx = await program.methods
    .createMovieReview(movie.title,movie.review,10)
    .accounts({movieReviewPda :moviePda})
    .rpc();

    const account = await program.account.review.fetch(moviePda)
    expect(movie.title === account.title)
    expect(movie.rating === account.rating)
    expect(movie.review === account.review)
    // expect(account.reviewer === provider.wallet.publicKey)
    
    console.log("Your transaction signature", tx);
  });

  it("update the review", async () => {
    const newDescription = "Wow this is new"
    const newRating = 4
    const tx = await program.methods
    .updateMovieReview(movie.title,newDescription,newRating)
    .accounts({movieReviewPda :moviePda})
    .rpc();

    const account = await program.account.review.fetch(moviePda)
    expect(movie.title === account.title)
    expect(newRating === account.rating)
    expect(newDescription === account.review)
    // expect(account.reviewer === provider.wallet.publicKey)
    
    console.log("Your transaction signature", tx);
  })


  it("delete the review", async () => {
  
    const tx = await program.methods
    .deleteMovieReview()
    .accounts({movieReviewPda :moviePda})
    .rpc();

    // const account = await program.account.review.fetch(moviePda)
  
    
    console.log("Your transaction signature", tx);
  })
});
