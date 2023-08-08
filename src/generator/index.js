const process = require('process');

const nativeModule = `./lil-schemy-${process.arch}-${process.platform}.node`;

module.exports = {
    ...require(nativeModule)
};