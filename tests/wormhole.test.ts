import { AnchorProvider, type Idl, Program, setProvider } from '@coral-xyz/anchor'
import EchoIdl from '../target/idl/echo.json'
import type { Echo } from '../target/types/echo'

interface EchoProgram extends Echo {
  address: string
  metadata: Idl['metadata']
}

describe('temp', () => {
  // Configure the client to use the local cluster.
  setProvider(AnchorProvider.env())
  const program = new Program<Echo>(EchoIdl as unknown as Idl)

  it('Is initialized!', async () => {
    // Add your test here.
    // const tx = await program.methods.initialize().rpc();
    // console.log("Your transaction signature", tx);
  })
})
