use anchor_lang::prelude::*;

// id del programa
declare_id!("GfTew82mKJGS3wpDHTQ74Pqa22R7njULD8Vp1cipARAt");

#[program]
pub mod papeleria {
    use super::*;

    //////////////////////////// crear papeleria /////////////////////////////////////
    /*
    crea una cuenta donde se guardan los productos de la papeleria
    */
    pub fn crear_papeleria(context: Context<NuevaPapeleria>, nombre: String) -> Result<()> {
        // obtenemos la wallet del dueño
        let owner_id = context.accounts.owner.key();
        msg!("owner id: {}", owner_id);

        // vector vacio de productos
        let productos: Vec<Producto> = Vec::new();

        // guardamos la papeleria
        context.accounts.papeleria.set_inner(Papeleria {
            owner: owner_id,
            nombre,
            productos,
        });

        Ok(())
    }

    //////////////////////////// agregar producto /////////////////////////////////////
    /*
    agrega un producto nuevo a la papeleria
    */
    pub fn agregar_producto(
        context: Context<NuevoProducto>,
        nombre: String,
        precio: u16,
    ) -> Result<()> {

        // verificamos que quien modifica sea el dueño
        require!(
            context.accounts.papeleria.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        // creamos el producto
        let producto = Producto {
            nombre,
            precio,
            disponible: true,
        };

        // lo agregamos al vector
        context.accounts.papeleria.productos.push(producto);

        Ok(())
    }

    //////////////////////////// eliminar producto /////////////////////////////////////
    /*
    elimina un producto por su nombre
    */
    pub fn eliminar_producto(context: Context<NuevoProducto>, nombre: String) -> Result<()> {

        require!(
            context.accounts.papeleria.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        // referencia al vector de productos
        let productos = &mut context.accounts.papeleria.productos;

        // buscamos el producto
        for i in 0..productos.len() {
            if productos[i].nombre == nombre {
                productos.remove(i);
                msg!("producto {} eliminado", nombre);
                return Ok(());
            }
        }

        Err(Errores::ProductoNoExiste.into())
    }

    //////////////////////////// ver productos /////////////////////////////////////
    /*
    muestra todos los productos guardados
    */
    pub fn ver_productos(context: Context<NuevoProducto>) -> Result<()> {

        require!(
            context.accounts.papeleria.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        msg!(
            "lista de productos: {:#?}",
            context.accounts.papeleria.productos
        );

        Ok(())
    }

    //////////////////////////// cambiar disponibilidad /////////////////////////////////////
    /*
    cambia si el producto esta disponible o no
    */
    pub fn alternar_estado(context: Context<NuevoProducto>, nombre: String) -> Result<()> {

        require!(
            context.accounts.papeleria.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let productos = &mut context.accounts.papeleria.productos;

        for i in 0..productos.len() {
            let estado = productos[i].disponible;

            if productos[i].nombre == nombre {
                let nuevo_estado = !estado;
                productos[i].disponible = nuevo_estado;

                msg!(
                    "el producto {} ahora tiene disponibilidad {}",
                    nombre,
                    nuevo_estado
                );

                return Ok(());
            }
        }

        Err(Errores::ProductoNoExiste.into())
    }
}

//////////////////////////// errores /////////////////////////////////////

#[error_code]
pub enum Errores {
    #[msg("error, no eres el dueño de esta papeleria")]
    NoEresElOwner,

    #[msg("error, el producto no existe")]
    ProductoNoExiste,
}

//////////////////////////// cuenta papeleria /////////////////////////////////////

#[account]
#[derive(InitSpace)]
pub struct Papeleria {
    owner: Pubkey,

    #[max_len(60)]
    nombre: String,

    #[max_len(10)]
    productos: Vec<Producto>,
}

//////////////////////////// struct producto /////////////////////////////////////

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Producto {
    #[max_len(60)]
    nombre: String,

    precio: u16,

    disponible: bool,
}

//////////////////////////// contextos /////////////////////////////////////

#[derive(Accounts)]
pub struct NuevaPapeleria<'info> {

    // dueño que paga la transaccion
    #[account(mut)]
    pub owner: Signer<'info>,

    // cuenta donde se guarda la papeleria
    #[account(
        init,
        payer = owner,
        space = Papeleria::INIT_SPACE + 8,
        seeds = [b"papeleria", owner.key().as_ref()],
        bump
    )]
    pub papeleria: Account<'info, Papeleria>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct NuevoProducto<'info> {

    // dueño que hace los cambios
    pub owner: Signer<'info>,

    // papeleria que se va a modificar
    #[account(mut)]
    pub papeleria: Account<'info, Papeleria>,
}