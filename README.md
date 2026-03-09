# PapeleriaJuana

# papeleria — solana program

programa de ejemplo en la blockchain de solana desarrollado con anchor.

este smart contract permite gestionar una papelería y sus productos directamente on-chain.

## funcionalidades

* crear una papeleria
* agregar productos
* eliminar productos
* ver lista de productos
* alternar disponibilidad de un producto

## tecnologias

* rust
* anchor framework
* solana
* typescript (cliente)

## estructura del proyecto

```
programs/
 └ papeleria/
     └ lib.rs

client/
 └ client.ts
```

## como funciona

1. se crea una cuenta **papeleria** asociada al dueño.
2. los productos se guardan dentro de un vector en esa cuenta.
3. solo el **owner** puede modificar los productos.

cada producto contiene:

* nombre
* precio
* disponibilidad

## ejemplo de producto

```
Producto {
  nombre: "cuaderno",
  precio: 50,
  disponible: true
}
```

## instrucciones del programa

* `crear_papeleria`
* `agregar_producto`
* `eliminar_producto`
* `ver_productos`
* `alternar_estado`

## objetivo

este proyecto fue hecho como práctica para aprender:

* desarrollo de smart contracts en solana
* uso del framework anchor
* interacción cliente → programa on-chain


