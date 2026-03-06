use anchor_lang::prelude::*;

declare_id!("DtrXksPN2k3whjeWLfhNa1wTSijD78YgJEcTZdjBdG6x");

// Constantes de tamaño para el cálculo de espacio
const MAX_NOMBRE: usize = 30 * 4; 
const MAX_RAZA: usize = 30 * 4;
const MAX_FECHA: usize = 12 * 4; 
const MAX_DIAGNOSTICO: usize = 50 * 4;
const MAX_CONSULTAS: usize = 10; 

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct EntradaMedica {
    pub fecha: String,
    pub diagnostico: String,
    pub costo: u64,
}

impl EntradaMedica {
    pub const SIZE: usize = 4 + MAX_FECHA + 4 + MAX_DIAGNOSTICO + 8;
}

#[account]
pub struct Mascota {
    pub owner: Pubkey,
    pub nombre: String,
    pub raza: String,
    pub historial: Vec<EntradaMedica>,
}

impl Mascota {
    pub const SIZE: usize = 32 
        + 4 + MAX_NOMBRE 
        + 4 + MAX_RAZA 
        + 4 + (MAX_CONSULTAS * EntradaMedica::SIZE);
}

#[program]
pub mod pet_health_vault {
    use super::*;

    pub fn registrar_mascota(
        ctx: Context<RegistrarMascota>,
        nombre: String,
        raza: String,
    ) -> Result<()> {
        let mascota = &mut ctx.accounts.mascota_account;

        require!(nombre.chars().count() <= 30, ErrorMascota::NombreMuyLargo);
        require!(raza.chars().count() <= 30, ErrorMascota::RazaMuyLarga);

        mascota.owner = ctx.accounts.owner.key();
        mascota.nombre = nombre;
        mascota.raza = raza;
        mascota.historial = Vec::new();

        msg!("Expediente creado para: {}", mascota.nombre);
        Ok(())
    }

    pub fn agregar_consulta(
        ctx: Context<GestionarExpediente>,
        fecha: String,
        diagnostico: String,
        costo: u64,
    ) -> Result<()> {
        let mascota = &mut ctx.accounts.mascota_account;

        require!(mascota.owner == ctx.accounts.owner.key(), ErrorMascota::NoAutorizado);
        require!(fecha.chars().count() <= 12, ErrorMascota::FechaMuyLarga);
        require!(diagnostico.chars().count() <= 50, ErrorMascota::DiagnosticoMuyLargo);
        require!(mascota.historial.len() < MAX_CONSULTAS, ErrorMascota::HistorialLleno);

        let nueva_entrada = EntradaMedica { fecha, diagnostico, costo };
        mascota.historial.push(nueva_entrada);

        msg!("Consulta registrada con costo: {}", costo);
        Ok(())
    }

   //Funcion para ver el expediente
    pub fn ver_expediente(ctx: Context<VerExpediente>) -> Result<()> {
        let mascota = &ctx.accounts.mascota_account;
        msg!("Expediente de: {}", mascota.nombre);
        msg!("Raza: {}", mascota.raza);
        msg!("Consultas registradas: {}", mascota.historial.len());
        Ok(())
    }

    pub fn cerrar_expediente(_ctx: Context<GestionarExpediente>) -> Result<()> {
        msg!("Expediente cerrado y cuenta eliminada.");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct RegistrarMascota<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + Mascota::SIZE,
        seeds = [b"expediente", owner.key().as_ref()], 
        bump
    )]
    pub mascota_account: Account<'info, Mascota>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GestionarExpediente<'info> {
    #[account(
        mut,
        seeds = [b"expediente", owner.key().as_ref()],
        bump
    )]
    pub mascota_account: Account<'info, Mascota>,
    pub owner: Signer<'info>,
}

//Contexto para la vizuali
#[derive(Accounts)]
pub struct VerExpediente<'info> {
    #[account(
        seeds = [b"expediente", owner.key().as_ref()],
        bump
    )]
    pub mascota_account: Account<'info, Mascota>,
    pub owner: Signer<'info>,
}

#[error_code]
pub enum ErrorMascota {
    #[msg("No autorizado.")]
    NoAutorizado,
    #[msg("Nombre muy largo.")]
    NombreMuyLargo,
    #[msg("Raza muy larga.")]
    RazaMuyLarga,
    #[msg("Fecha muy larga.")]
    FechaMuyLarga,
    #[msg("Diagnóstico muy largo.")]
    DiagnosticoMuyLargo,
    #[msg("Historial lleno.")]
    HistorialLleno,
}
