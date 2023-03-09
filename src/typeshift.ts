import { Command } from 'commander';
import generate from './commands/generate';
import init from './commands/init';

const program = new Command();

program
    .name('typeshift')
    .description('CLI to generate schemas from TypeScript')
    .version('0.0.1')
    .option('--cwd <cwd>', "base dir, which all paths are relative to", process.cwd())
    .option('-c, --config <config>', 'configuration module', 'typeshift');

program.addCommand(init);
program.addCommand(generate);

program.parse();