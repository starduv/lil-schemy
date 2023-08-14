import { LilPath, LilQueryParam, LilResponse, LilRouteParam } from '../../../src';
import { AnimalsRequest, Request } from '../dtos/requests';
import { Router } from './router';

class Animal {
    constructor(public name: string) { }
}

export default Router.get("", {}, LilPath(async (request: Request<AnimalsRequest>, reply: any): Promise<void> => {
    let response = LilResponse([] as Array<string>, {
        statusCode: 200,
        description: "List animals of a specific kind",
    });

    reply.send(response);
}, {
    method: 'GET',
    path: '/animals',
    tags: ['Animals'],
}));

Router.get("", {}, LilPath(async (request: Request<{ Querystring: { id: LilRouteParam<number> } }>, reply: any): Promise<void> => {
    let response = LilResponse(new Animal("Billy"), {
        statusCode: 200,
        description: "A specific animal",
    });

    reply.send(response);
}, {
    method: 'GET',
    path: '/animals/{id}',
    tags: ['Animals'],
}));

