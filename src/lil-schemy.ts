import { Command } from 'commander';
import generate from './commands/generate';
import init from './commands/init';

const program = new Command();

program
    .name('lil-schemy')
    .description('CLI to generate schemas from TypeScript')
    .version('0.0.1')
    .option('--cwd <cwd>', "base dir, for relative paths", process.cwd())

program.addCommand(init);
program.addCommand(generate);

program.parse();