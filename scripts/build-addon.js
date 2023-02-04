const util = require('util')
const process = require('node:process');
const {rename, copyFile} = require('fs/promises')
const execFile = util.promisify(require('child_process').execFile);

const run = async () => {
    const {stdout, stderr} = await execFile('cargo', [
        'build',
        '--manifest-path',
        './native/Cargo.toml',
        '--release',
        '--verbose'
    ]);

    console.log(stdout);
    console.log(stderr)

    const src = `native/target/release/typeshift.${process.platform == 'darwin' ? 'dylib' : 'dll'}`;
    const dest = `src/generator/${process.arch}-${process.platform}.node`;

    await copyFile(src, dest);
}    

module.exports = run().catch(err => console.error(err))

/**
 * Platforms
 * https://nodejs.org/docs/latest-v18.x/api/process.html#processplatform
 *  'darwin'
 *  'freebsd'
 *  'linux'
 *  'openbsd'
 *  'win32'
 */

/**
 * Arch
 * https://nodejs.org/docs/latest-v18.x/api/process.html#processarch
 * 'arm'
 * 'arm64'
 * 'ia32'
 * 'x64'
 */