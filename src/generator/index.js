const process = require('process');

const nativeModule = `./${process.arch}-${process.platform}.node`;

module.exports = {
    ...require(nativeModule)
};