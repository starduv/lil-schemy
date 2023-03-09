export interface Reply<T> {
    send: (responseType: T) => void;
}

export const Router = {
    get: (path: string, stuff: unknown, handler: (request: any, reply: any) => Promise<void>) => {
        return null;
    },
    delete: (path: string, stuff: unknown, handler: (request: any, reply: any) => Promise<void>) => {
        return null;
    },
    patch: (path: string, stuff: unknown, handler: (request: any, reply: any) => Promise<void>) => {
        return null;
    },
    post: (path: string, stuff: unknown, handler: (request: any, reply: any) => Promise<void>) => {
        return null;
    },
    put: (path: string, stuff: unknown, handler: (request: any, reply: any) => Promise<void>) => {
        return null;
    }
};