# Template for yew and tailwind based WASM code

WIP

## Setup

* Install Rust
* Install wasm-pack
* Install trunk
	> cargo install trunk wasm-bindgen-cli
* Copy project
* Search and replace "wasm-tailwind-rs" to your project name
* Open project folder in console
* Run
	> wasm-pack build
	>
	> npm init wasm-app www
* Modify package.json and add dependency to project:
	> cd www
    >
	>"dependencies": { "wasm-tailwind-rs": "file:../pkg", ...}
* Remove hello-wasm-pack devDependency
* Modify index.js and to import your projects wasm file.
* Install tailwindcss
	> cd www
	>
	> npm i -g tailwindcss
	>
	> ~/.npm-packages/bin/tailwindcss -o ./tailwind.css
* Minify tailwind:
	> cd www
	>
	> NODE_ENV=production ~/.npm-packages/bin/tailwindcss -c ../tailwind.config.js -o ./tailwind.css --minify
* Modify index.html, add tailwind stylesheet
    > <link href="./tailwind.css" rel="stylesheet" />
* Run
	> cd www
	>
	> npm install
	>
	> npm start
* Open http://localhost:8080

