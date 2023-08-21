import { LilBodyParam, LilPath, LilRequiredField, LilResponse, LilRouteParam } from '../../../src';
import { AnimalsRequest, Request, AnimalUpdate, AnimalLicense } from '../dtos/requests';
import { Router } from './router';

class Animal {
    constructor(public name: LilRequiredField<string>, public shots: string[]) { }
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
    await reply.send(LilResponse(new Animal("Billy", ["tetnis", "rabies"]), {
        statusCode: 200,
        description: "A specific animal",
    }));
}, {
    method: 'GET',
    path: '/animals/{id}',
    tags: ['Animals'],
}));

Router.put("", {}, LilPath(async (request: Request<{ Body: LilBodyParam<AnimalUpdate> }>, reply: any): Promise<void> => {
    await reply.send(LilResponse(new Animal("Billy", ["tetnis", "rabies"]), {
        statusCode: 200,
        description: "A specific animal",
    }));
}, {
    method: 'POST',
    path: '/animals/{id}',
    tags: ['Animals'],
}));

Router.put("", {}, LilPath(async (request: Request<{ Body: LilBodyParam<AnimalUpdate> }>, reply: any): Promise<void> => {
    await reply.send(LilResponse(new Animal("Billy", ["tetnis", "rabies"]), {
        statusCode: 200,
        description: "A specific animal",
    }));
}, {
    method: 'POST',
    path: '/animals/{id}/unregister',
    tags: ['Animals'],
}));

Router.get("", {}, LilPath(async (request: Request<{ Body: LilBodyParam<AnimalUpdate> }>, reply: any): Promise<void> => {
    const license: AnimalLicense = { adjacents: [], exp: "2020-01-01", state: "NY" }

    await reply.send(LilResponse(license, {
        statusCode: 200,
        description: "A specific animal license",
    }));
}, {
    method: 'GET',
    path: '/animals/{id}/license',
    tags: ['Animals'],
}));
