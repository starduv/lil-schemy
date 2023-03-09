import { expect } from "chai";
import { execSync } from "child_process"
import { unlinkSync } from "fs";
import path from "path";
import { TypeShiftOptions } from "../src/generator";

describe('cli', () => {
    const rootConfigPath = path.resolve(__dirname, 'typeshift.js');
    const customDirectory = path.resolve(__dirname, 'custom');
    const customConfigPath = path.resolve(customDirectory, 'typeshift.js');

    after(() => {
        unlinkSync(rootConfigPath);
        unlinkSync(customConfigPath);
    })

    it('generates configuration in current directory', async () => {
        execSync(`npx ts-node ../src/typeshift.ts init`, {
            cwd: __dirname
        });

        const config: TypeShiftOptions = (await import(rootConfigPath)).default;

        expect(config.openApi?.base.openapi).to.eq("3.0.3");
    })

    it('generates configuration in user defined directory', async () => {
        execSync(`npx ts-node ../src/typeshift.ts init --cwd ${customDirectory}`, {
            cwd: __dirname
        });

        const config: TypeShiftOptions = (await import(customConfigPath)).default;

        expect(config.openApi?.base.openapi).to.eq("3.0.3");
    })
})