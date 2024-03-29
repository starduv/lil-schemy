export { generate } from './commands/generate';
export { getRootFiles } from './utils';

type NumberFormat = "int32" | "int64" | "float" | "double";
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

export type format = StringFormat | NumberFormat | undefined;

export type OperationMethod = 'GET' | 'PUT' | 'POST' | 'DELETE' | 'OPTIONS' | 'HEAD' | 'PATCH' | 'TRACE';

export interface PathItemOptions {
    method: OperationMethod;
    path: string;
    tags?: string[];
}
export function LilPath<Func>(fn: Func, options: PathItemOptions) {
    return fn;
}

export interface ResponseOptions {
    description: string;
    example?: string;
    statusCode: number;
    mediaType?: string;
}
export function LilResponse<ResponseType>(response: ResponseType, options: ResponseOptions) {
    return response;
}

export type LilBodyParam<Param, Required extends boolean = true, MediaType extends string = "application/json"> = Param;
export type LilHeader<Param, Required extends boolean = true, Format extends format = undefined> = Param;
export type LilQueryParam<Param, Required extends boolean = false, Format extends format = undefined> = Param;
export type LilRouteParam<Param, Required extends true = true, Format extends format = undefined> = Param;
export type LilRequiredProp<Param> = Param;
export type LilSub<From, To> = From;