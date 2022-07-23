import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { Betana } from '../target/types/betana';
import assert from "assert";

const { SystemProgram } = anchor.web3;

describe('betana', () => {

  // Configure the client to use the local cluster.
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Betana as Program<Betana>;

  let _baseAccount: anchor.web3.Keypair;

  it('Is initialized!', async () => {
    const baseAccount = anchor.web3.Keypair.generate();

    const tx = await program.rpc.initialize({
      accounts: {
        baseAccount: baseAccount.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [baseAccount],
    });
    // fetch the base account
    const account = await program.account.baseAccount.fetch(
      baseAccount.publicKey
    );

    _baseAccount = baseAccount;
    console.log("Your transaction signature", tx);
  });

  it("Place bet", async () => {
    const accountBefore = await program.account.baseAccount.fetch(
      _baseAccount.publicKey
    );
    console.log(accountBefore.currentBet.amount.toString())

    const poolWallet = anchor.web3.Keypair.generate();
    const tx = await program.rpc.placeBet('0', '1', 250, {
      accounts: {
        baseAccount: _baseAccount.publicKey,
        from: provider.wallet.publicKey,
        to: poolWallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
    });

    /*const accountAfter = await program.account.baseAccount.fetch(
      _baseAccount.publicKey
    );
    console.log(accountAfter.currentBet.idMatch)
    console.log(accountAfter.currentBet.idTeam)
    console.log(accountAfter.currentBet.amount)
    console.log(accountAfter.currentBet.userAddress)
    console.log('Success!')*/
  });

});
