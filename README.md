<h1 align="center">Brick-rs</h1>

<p align="center">
  <a aria-label="NPM version" href="https://www.npmjs.com/package/brick-rs" style="margin-right: 20px;">Npm</a>
  <a aria-label="Creator" href="https://twitter.com/fveiras_">Creator</a>
</p>

## What is Brick?

> **​Brick is under active development.** The purpose of this binary is to run jobs on modified routes to avoid heavy loads and waiting times. Currently, it only works with Cypress e2e and Jest. watch the [GitHub repository](https://github.com/fran-veiras/brick-rs) to keep tabs on future releases.

![Untitled-2023-09-19-1214(1)(1)](https://github.com/fran-veiras/brick-rs/assets/74626997/3718278f-c240-4848-9f7b-119b865e553a)

## Add to a New Project

### Install

| Package Manager | Command                      |
|------------------|------------------------------|
| Yarn             | `yarn add -D brick-rs`      |
| Npm              | `npm i --save-dev brick-rs`  |
| Pnpm             | `pnpm add -D brick-rs`      |


### Config
#### For pnpm and npm Projects
Add the following script to your **package.json** file:
```json
{
		"scripts": {
			...
			"brick": "brickrs"
		}
}
```

#### For yarn Projects
If the binary doesn't execute properly with Yarn projects, configure it manually in your **package.json**

```json
{
		"scripts": {
			...
			"brick": "./node_modules/brick-rs/target/debug/brick"
		}
}
```

---

### Create a brick.config.json file with the following content:
```json
{
		"pm": "pnpm",
		"root": "components",
		"jobs": ["jest", "cypress"]
}
```

`pm` Project package manager

`root` The entry point where the program looks for changes.

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

`jobs` The jobs to execute on the specified directories with changes.

In this case, the jobs will run on the home folder (components/home/*). If you have Cypress, it will look for the folder with the same name: `cypress/e2e/home`. It's crucial that they have matching names.

**Note**: Ensure Cypress is properly configured, and the server is running, especially for e2e tests.

## Finally

To run the `brick` script after configuring your project, use the following command if you are using **pnpm** as your package manager:

```bash
pnpm run brick
```

![image](https://github.com/fran-veiras/brick-rs/assets/74626997/51f35e9b-e6ff-4dc5-b098-997f1a5e3c18)


## Contributing

After completing the development, you can test it locally on our front end by following these steps:

1. Build the project and copy the binary to the test environment:

`cargo build && ./copy_target.sh`

This will copy the binary to our front-end testing environment /test-brick.

2. Inside our testing environment /test-brick, run the following command to execute the binary. This allows us to avoid working with the entire package and only focus on running the binary, which is crucial.
`pnpm run brick-build`

This script enables us to execute the binary efficiently, simplifying the testing process.


