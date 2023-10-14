# brick-rs

La finalidad de este binary es correr jobs sobre rutas que fueron modificadas, para evitar grandes cargas y tiempo de espera. Por ahora solo funciona con cypress e2e y jest

![Untitled-2023-09-19-1214(1)(1)](https://github.com/fran-veiras/brick-rs/assets/74626997/3718278f-c240-4848-9f7b-119b865e553a)


### Añadir a un nuevo proyecto

**Install**

`yarn add -D brick-rs` 

`npm i --save-dev brick-rs`

`pnpm add -D brick-rs`

**Config** 

`pnpm` y `npm`

package.json
```json
{
		"scripts": {
			...
			"brick": "brickrs"
		}
}
```

`yarn`

Por motivos que desconozco el binario no se ejecuta con proyectos de yarn, entonces lo configuramos manual

package.json
```json
{
		"scripts": {
			...
			"brick": "./node_modules/brick-rs/target/debug/brick"
		}
}
```

`brick.config.json`

brick.config.json
```json
{
		"pm": "pnpm",
		"root": "components",
		"jobs": ["jest", "cypress"]
}
```

`pm` project package manager

`root` Es el punto de entrada donde queremos que el programa busque los cambios.

```css
📁 cypress
	📁 e2e
		📁 home
    📁 about
📁 src
  📁 components **(root)**
    📁 home
      📄 index.ts **x** **(modified)**
      📁 __tests__
					📄 index.test.ts
	  📁 about
```

`jobs` Los trabajos que queremos ejecutar sobre esos directorios con cambios

Los jobs se van a correr en este caso sobre la carpeta home (`components/home/*`) y a su vez si tenemos cypress va a buscar la carpeta con el mismo nombre `cypress/e2e/home` 

Es importante que cuenten con el mismo nombre.

Cypress

Para ver el output de los test es necesario tenerlo bien configurado y corriendo el servidor si son e2e.
