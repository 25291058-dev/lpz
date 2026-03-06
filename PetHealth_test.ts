import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PetHealthVault } from "../target/types/pet_health_vault";

describe("pet_health_vault", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.PetHealthVault as Program<PetHealthVault>;

  it("Registra la mascota y agrega una consulta", async () => {
    const [mascotaPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("expediente"), anchor.getProvider().publicKey.toBuffer()],
      program.programId
    );

    // 1. Registrar (Sin nombres de relleno)
    await program.methods
      .registrarMascota("NombreMascota", "RazaMascota")
      .accounts({
        mascotaAccount: mascotaPDA,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    // 2. Agregar Consulta
    await program.methods
      .agregarConsulta("2026-03-05", "Chequeo General", new anchor.BN(100))
      .accounts({
        mascotaAccount: mascotaPDA,
      })
      .rpc();
  });
});
