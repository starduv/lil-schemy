import { expect } from "chai";
import { execSync } from "child_process"
import { unlinkSync } from "fs";
import path from "path";
import { LilSchemyOptions } from "../src/generator";

describe('cli', function () {
    this.timeout(20000);

    const rootConfigPath = path.resolve(__dirname, 'schemy-config.js');
    const customDirectory = path.resolve(__dirname, 'custom');
    const customConfigPath = path.resolve(customDirectory, 'schemy-config.js');

    after(() => {
        unlinkSync(rootConfigPath);
        unlinkSync(customConfigPath);
    })

    it('generates configuration in current directory', async () => {
        execSync(`npx ts-node ../src/lil-schemy.ts init`, {
            cwd: __dirname
        });

        const config: LilSchemyOptions = (await import(rootConfigPath)).default;

        expect(config.openApi?.base.openapi).to.eq("3.0.3");
    })

    it('generates configuration in user defined directory', async () => {
        execSync(`npx ts-node ../src/lil-schemy.ts init --cwd ${customDirectory}`, {
            cwd: __dirname
        });

        const config: LilSchemyOptions = (await import(customConfigPath)).default;

        expect(config.openApi?.base.openapi).to.eq("3.0.3");
    })
})