[![](https://user-images.githubusercontent.com/25987204/78205790-10b0c680-74d8-11ea-9767-5bb93e920044.png)](https://dessert.dev/)

Dessert Filesize
============

[![npm-badge]][npm-url]
[![license-badge]][license]

[npm-badge]: https://img.shields.io/npm/v/dessert-filesize-core.svg
[npm-url]: https://www.npmjs.org/package/dessert-filesize-core
[license-badge]: https://img.shields.io/github/license/dessert-wasm/dessert-filesize-core
[license]: LICENSE_MIT


> Exposes the base API for the [filesize] module, written in Rust for WebAssembly.

[filesize]: https://github.com/dessert-wasm/dessert-filesize


## Summary
* [Usage](#usage)
* [API](#api)
* [Installation](#installation)
* [Building](#building)
* [Testing](#testing)
* [License](#license)
* [Contributing](#contributing)


## Usage

> This module is **not** supposed to be used directly as a dependence by an application, it is used as a backend for js connector

```js
const filesize = require('dessert-filesize-core');

filesize(500, {/*options*/});
```

## API

### filesize(bytes, options={})

Returns a human readable string from `bytes` (number)

`filesize()` accepts an optional descriptor Object as a second argument, so you can customize the output.

### base
_*(number)*_ Number base, default is `2`

### bits
_*(boolean)*_ Enables `bit` sizes, default is `false`

### exponent
_*(number)*_ Specifies the symbol via exponent, e.g. `2` is `MB` for base 2, default is `-1`

### fullform
_*(boolean)*_ Enables full form of unit of measure, default is `false`

### fullforms
_*(array)*_ Array of full form overrides, default is `[]`

### locale (overrides 'separator')

### output
_*(string)*_ Output of function (`array`, `exponent`, `object`, or `string`), default is `string`

### round
_*(number)*_ Decimal place, default is `2`

### separator
_*(string)*_ Decimal separator character, default is `.`

### spacer
_*(string)*_ Character between the `result` and `symbol`, default is `" "`

### standard
_*(string)*_ Standard unit of measure, can be `iec` or `jedec`, default is `jedec`; can be overruled by `base`

### symbols
_*(object)*_ Dictionary of SI/JEDEC/IEC symbols to replace for localization, defaults to english if no match is found

### unix
_*(boolean)*_ Enables unix style human readable output, e.g `ls -lh`, default is `false`


## Installation

> dessert-filesize-core is depended on by [dessert-filesize](https://github.com/dessert-wasm/dessert-filesize)
```sh
npm install dessert-filesize-core
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
