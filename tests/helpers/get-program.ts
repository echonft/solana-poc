import { Program, type Provider } from '@coral-xyz/anchor'
import { type Connection, PublicKey, type PublicKeyInitData } from '@solana/web3.js'
import EchoIdl from 'target/idl/echo.json'
import type { Echo } from 'target/types/echo'

export function getProgram(
  connection: Connection,
  programId: PublicKeyInitData,
  payer?: PublicKeyInitData
) {
  const provider: Provider = {
    connection,
    publicKey: payer == undefined ? undefined : new PublicKey(payer)
  }
  return new Program<Echo>(
    // eslint-disable-next-line @typescript-eslint/no-unsafe-argument
    EchoIdl as never,
    new PublicKey(programId),
    provider
  )
}
