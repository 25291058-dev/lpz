import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PetHealthVault } from "../target/types/pet_health_vault";

describe("pet_health_vault", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.PetHealthVault as Program<PetHealthVault>;

  it("Registra una mascota correctamente", async () => {
    const [mascotaPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("expediente"), anchor.getProvider().publicKey.toBuffer()],
      program.programId
    );

    await program.methods
      .registrarMascota("Calvillo", "Ags")
      .accounts({
        mascotaAccount: mascotaPDA,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();
  });
});
