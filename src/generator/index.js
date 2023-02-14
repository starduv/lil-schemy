const process = require('process');

const nativeModule = `./typeshift-${process.arch}-${process.platform}.node`;

module.exports = {
    ...require(nativeModule)
};