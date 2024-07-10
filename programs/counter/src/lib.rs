use anchor_lang::prelude::*;

declare_id!("4Qo6zT3LNsVM4msHUa3vKAhyMK4PtjT74RLdhYDnVeDu");
// Declara el ID del programa en Solana.
// Este ID debe coincidir con el que se usó para desplegar el programa en la red.

#[program]
pub mod counter {
    use super::*;

    pub fn create_counter(ctx: Context<CreateCounter>, initial_count: u64) -> Result<()> {
        msg!("creando un contador con numero inicial {} ", initial_count); // Emite un mensaje en los logs.
        let counter = &mut ctx.accounts.counter;
        counter.count = initial_count; // Asigna el número inicial al contador.
        counter.authority = *ctx.accounts.authority.key;
        Ok(())
    }

    pub fn update_counter(ctx: Context<UpdateCounter>, count: u64) -> Result<()> {
        msg!("actualizando el contador a: {}!", count);
        ctx.accounts.counter.count = count;
        Ok(())
    }

    pub fn increment_counter(ctx: Context<IncrementCounter>) -> Result<()> {
        let count = ctx.accounts.counter.count + 1;
        ctx.accounts.counter.count = count;
        msg!("incrementando el contador a: {}!", count);
        Ok(())
    }

    pub fn decrement_counter(ctx: Context<DecrementCounter>) -> Result<()> {
        let count = ctx.accounts.counter.count - 1;
        ctx.accounts.counter.count = count;
        msg!("decrementando el contador a: {}!", count);
        Ok(())
    }

    pub fn delete_counter(_ctx: Context<DeleteCounter>) -> Result<()> {
        msg!("borrando contador");
        Ok(())
    }
}

// 8 (discriminador) + 8 (campo count) + 32 (authority): 48 bytes (total espacio reservado).
const COUNTER_SIZE: usize = 8 + 8 + 32;

#[derive(Accounts)]
#[instruction(initial_count: u64)] // Definir un argumento adicional para la instrucción
pub struct CreateCounter<'info> {
    // `init` inicializa una nueva cuenta.
    // `payer = authority` especifica que `autoridad` paga por la creación de la cuenta.
    // `space = COUNTER_SIZE` especifica el tamaño de la cuenta.
    #[account(init, payer = authority, space = COUNTER_SIZE)]
    pub counter: Account<'info, Counter>, // Cuenta de tipo `Contador`.
    #[account(mut)]
    pub authority: Signer<'info>, // Firmante mutable de la transacción, paga por la creación de la cuenta.
    pub system_program: Program<'info, System>, // Programa del sistema necesario para la creación de cuentas.
}

#[derive(Accounts)]
#[instruction(count: u64)]
pub struct UpdateCounter<'info> {
    #[account(
        mut,
        constraint = counter.authority == authority.key() @ ErrorCode::NotAuthorized,
    )]
    pub counter: Account<'info, Counter>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct IncrementCounter<'info> {
    #[account(
        mut,
        constraint = counter.authority == authority.key() @ ErrorCode::NotAuthorized,
        constraint = counter.count < u64::MAX @ ErrorCode::CantIncrement,
    )]
    pub counter: Account<'info, Counter>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct DecrementCounter<'info> {
    #[account(
        mut,
        constraint = counter.authority == authority.key() @ ErrorCode::NotAuthorized,
        constraint = counter.count > 0 @ ErrorCode::CantDecrement,
    )]
    pub counter: Account<'info, Counter>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct DeleteCounter<'info> {
    #[account(
        mut,
        constraint = counter.authority == authority.key() @ ErrorCode::NotAuthorized,
        close = authority // Permitir que la cuenta sea cerrada por `authority` y los fondos transferidos a ella.
    )]
    pub counter: Account<'info, Counter>,
    #[account(mut)] // Permitir permisos de escritura en la cuenta para transferir los fondos.
    pub authority: Signer<'info>,
}

#[account]
pub struct Counter {
    count: u64,        // Campo `count` de 8 bytes.
    authority: Pubkey, // Campo `authority` de 32 bytes.
}

#[error_code]
pub enum ErrorCode {
    #[msg("No autorizado: solo el creador del contador puede modificarlo.")]
    NotAuthorized,
    #[msg("Decremento no permitido: el contador ya está en 0.")]
    CantDecrement,
    #[msg("Incremento no permitido: el contador ha alcanzado el valor máximo.")]
    CantIncrement,
}
