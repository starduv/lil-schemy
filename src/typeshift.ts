import { Command } from 'commander';
import generate from './commands/generate';

const program = new Command();

program
    .name('code-first')
    .description('CLI to generate schemas from Typescript')
    .version('0.0.1')
    .option('--cwd <cwd>', "base dir, which all paths are relative to", process.cwd())
    .option('-c, --config <config>', 'configuraiton file', 'code-first.cjs');

program.addCommand(generate);

program.parse();