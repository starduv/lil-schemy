const util = require('util')
const process = require('node:process');
const { rename, copyFile } = require('fs/promises')
const execFile = util.promisify(require('child_process').execFile);

const run = async (args) => {
    const debug = args.includes('--debug');
    const verbose = args.includes('--verbose');

    const cmd_args = [
        'build',
        '--manifest-path=./native/Cargo.toml',
        `--message-format=${verbose ? 'json-render-diagnostics' : 'short'}`,
    ]

    if (!debug) cmd_args.push('--release');

    const { stdout, stderr } = await execFile('cargo', cmd_args);

    console.log(stdout);

    console.log(stderr)

    const folder = debug ? 'debug' : 'release';
    const prefix = process.platform == 'darwin' ? "lib" : process.platform == 'linux' ? "lib" : "";
    const ext = process.platform == 'darwin' ? 'dylib' : process.platform == 'linux' ? 'so' : 'dll';
    const src = `native/target/${folder}/${prefix}lil_schemy.${ext}`;
    const dest = `src/generator/lil-schemy-${process.arch}-${process.platform}.node`;

    await copyFile(src, dest);
}

module.exports = run(process.argv.slice(2)).catch(err => console.error(err))