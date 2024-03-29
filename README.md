# Rust WASM Project
Repository for CMSC388Z Rust Web Assembly Project

This project uses Rust on the back-end and Web Assembly/Javascript on the front-end to create a simple webpage displaying a grid of blocks with gravity-like features that can be interacted with by the user.

## To install...
'''
npm install
'''

## To run...
'''
npm run
'''

## File Overview
* `Cargo.toml` contains the standard Rust metadata. You put your Rust dependencies in here. You must change this file with your details (name, description, version, authors, categories)

* `package.json` contains the standard npm metadata. You put your JavaScript dependencies in here. You must change this file with your details (author, name, version)

* `webpack.config.js` contains the Webpack configuration. You shouldn't need to change this, unless you have very special needs.

* The `js` folder contains your JavaScript code (`index.js` is used to hook everything into Webpack, you don't need to change it).

* The `src` folder contains your Rust code.

* The `static` folder contains any files that you want copied as-is into the final build. It contains an `index.html` file which loads the `index.js` file.

* The `tests` folder contains your Rust unit tests.
