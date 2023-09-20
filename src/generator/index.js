const process = require('process');
const { generateSchemas } = require(`./lil-schemy-${process.arch}-${process.platform}.node`);

module.exports = {
    generateSchemas: (options) => JSON.parse(generateSchemas(JSON.stringify(options)) ?? "{}")
};