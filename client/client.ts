import { PublicKey } from "@solana/web3.js";

const owner = pg.wallet.publicKey; // wallet del dueño

console.log("wallet:", owner.toString());

//////////////////////////// pda papeleria ////////////////////////////

function pdaPapeleria() {
  return PublicKey.findProgramAddressSync(
    [
      Buffer.from("papeleria"),
      owner.toBuffer()
    ],
    pg.PROGRAM_ID
  );
}

//////////////////////////// crear papeleria ////////////////////////////

async function crearPapeleria(nombre) {

  const [papeleria] = pdaPapeleria();

  const tx = await pg.program.methods
    .crearPapeleria(nombre)
    .accounts({
      owner,
      papeleria
    })
    .rpc();

  console.log("papeleria creada:", tx);
}

//////////////////////////// agregar producto ////////////////////////////

async function agregarProducto(nombre, precio) {

  const [papeleria] = pdaPapeleria();

  const tx = await pg.program.methods
    .agregarProducto(nombre, precio)
    .accounts({
      owner,
      papeleria
    })
    .rpc();

  console.log("producto agregado:", tx);
}

//////////////////////////// eliminar producto ////////////////////////////

async function eliminarProducto(nombre) {

  const [papeleria] = pdaPapeleria();

  const tx = await pg.program.methods
    .eliminarProducto(nombre)
    .accounts({
      owner,
      papeleria
    })
    .rpc();

  console.log("producto eliminado:", tx);
}

//////////////////////////// ver productos ////////////////////////////

async function verProductos() {

  const [papeleria] = pdaPapeleria();

  const papeleriaAccount =
    await pg.program.account.papeleria.fetch(papeleria);

  console.log("productos registrados:");

  for (let i = 0; i < papeleriaAccount.productos.length; i++) {

    const producto = papeleriaAccount.productos[i];

    console.log(
      `producto #${i + 1}
       nombre: ${producto.nombre}
       precio: ${producto.precio}
       disponible: ${producto.disponible}`
    );
  }
}

//////////////////////////// cambiar disponibilidad ////////////////////////////

async function cambiarEstado(nombre) {

  const [papeleria] = pdaPapeleria();

  const tx = await pg.program.methods
    .alternarEstado(nombre)
    .accounts({
      owner,
      papeleria
    })
    .rpc();

  console.log("estado actualizado:", tx);
}

//////////////////////////// script de pruebas ////////////////////////////

(async () => {

  // crear papeleria
  await crearPapeleria("Papeleria Central");

  // agregar productos
  await agregarProducto("Cuaderno Profesional", 35);
  await agregarProducto("Lapiz HB", 5);
  await agregarProducto("Pluma Azul", 10);

  // ver productos
  await verProductos();

  // cambiar disponibilidad
  await cambiarEstado("Lapiz HB");

  // eliminar producto
  await eliminarProducto("Pluma Azul");

  // ver nuevamente productos
  await verProductos();

})();