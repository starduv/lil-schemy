import { OpenAPIV3 } from 'openapi-types';

export interface TypeShiftOptions {
    cwd: string;
    openApi?: OpenApiOptions;
    project: string;
}

interface OpenApiOptions {
    base: Omit<OpenAPIV3.Document, "paths"> & {
        openapi: "3.0.3";
        paths?: OpenAPIV3.PathsObject;
    },
    output?: string;
    paths: string[];
}

export interface SchemasResult {
    openApi: {
        schema?: string;
        filepath?: string;
    };
}

interface GenerateSchemaArgs extends Omit<TypeShiftOptions, "cwd", "project"> {
    // getAst: (reference: string, moduleFileName: string) => string | undefined;
    openApi?: Omit<OpenApiOptions, "base"> & {
        base: string;
    };
}

export function generateSchemas(options: GenerateSchemaArgs): SchemasResult;
