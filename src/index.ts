export { generateOpenApi } from './commands/generate';
export { getRootFiles } from './utils';

type NumberFormat = "int32" | "int64" | "float" | "double";
type OptionalString = string | undefined;
type F = StringFormat | NumberFormat | undefined;
type StringFormat = "date-time" |
    "time" |
    "date" |
    "duration" |
    "email" |
    "idn-email" |
    "hostname" |
    "idn-hostname" |
    "ipv4" |
    "ipv6" |
    "uuid" |
    "uri" |
    "uri-reference" |
    "uri-template" |
    "json-pointer" |
    "relative-json-pointer" |
    "regex" |
    "iri" |
    "iri-reference" |
    undefined;

export type OperationMethod = 'GET' | 'PUT' | 'POST' | 'DELETE' | 'OPTIONS' | 'HEAD' | 'PATCH' | 'TRACE';

export interface PathItemOptions {
    method: OperationMethod;
    path: string;
    tags?: string[];
}
export function Path<Func>(fn: Func, options: PathItemOptions | null = null) {
    return fn;
}

export interface ResponseOptions {
    description?: string;
    example?: string;
    namespace?: string;
    statusCode: number;
}
export function Response<ResponseType>(response: ResponseType, options: ResponseOptions) {
    return response;
}

export type BodyParam<Param, Required extends boolean = true, Namespace extends OptionalString = undefined> = Param;
export type Header<Param, Required extends boolean = true, Namespace extends OptionalString = undefined, Format extends F = undefined> = Param;
export type QueryParam<Param, Required extends boolean = false, Namespace extends OptionalString = undefined, Format extends F = undefined> = Param;
export type RouteParam<Param, Required extends true = true, Namespace extends OptionalString = undefined, Format extends F = undefined> = Param;