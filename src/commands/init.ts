import { Command } from 'commander';
import { copyFileSync, constants } from 'fs';
import path from 'path';

const createConfiguration = (cwd: string) => {
    const src = path.resolve(__dirname, 'default-config.txt');
    const dest = path.resolve(cwd, 'typeshift.js');

    copyFileSync(src, dest);

    console.info('Configuration written to %s', dest);
};

export default new Command('init')
    .description('Create default typeshift configuration')
    .action(async (_, command: Command) => {
        let parentOptions = command.parent?.opts();
        createConfiguration(parentOptions?.cwd);
    });