Dessert Filesize
============

[![NPM version](https://img.shields.io/npm/v/dessert-filesize-core.svg)](https://www.npmjs.org/package/dessert-filesize-core)


This library is the base API for the [filesize] module, written in Rust for WebAssembly.

[filesize]: https://github.com/dessert-wasm/dessert-filesize


## Summary
* [Installation](#installation)
* [API](#api)
* [Building](#building)
* [Testing](#testing)


## Installation

> Note:  
Although this module is not supposed to be used by itself, it can still be used as a standalone module.  
dessert-filesize-core is depended on by [dessert-filesize](https://github.com/dessert-wasm/dessert-filesize)
```sh
npm install dessert-filesize-core
```

## API
Here is a quick lookup of how you can use filesize  
To know more about the api and what options are available, refer to the [filesize.js] module

[filesize.js]: https://github.com/avoidwork/filesize.js

```javascript
let filesize = require('dessert-filesize-core').filesize;

let val = 500;
let options = { exponent: -2, output: "array" };

console.log(filesize(val, options));
```

## Building
The project is built using [wasm-pack]  
To build the project, run

[wasm-pack]: https://github.com/rustwasm/wasm-pack
```sh
wasm-pack build
```

## Testing

```sh
wasm-pack test --headless --firefox # or --chrome
```

## License
MIT

## Contributing
See [CONTRIBUTING.md](CONTRIBUTING.md)
