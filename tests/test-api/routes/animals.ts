import { LilBodyParam, LilPath, LilRequiredProp, LilResponse, LilRouteParam, LilSub } from '../../../src';
import { AnimalsRequest, Request, AnimalUpdate, AnimalLicense } from '../dtos/requests';
import { Router } from './router';

class Animal {
    constructor(public name: LilRequiredProp<string>, public shots: string[]) { }
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
    const lilTike: LilSub<unknown, Animal> = {
        name: "Billy",
        shots: ["tetnis", "rabies"]
    };

    await reply.send(LilResponse(lilTike, {
        statusCode: 200,
        description: "An updated animal",
    }));
}, {
    method: 'PUT',
    path: '/animals/{id}',
    tags: ['Animals'],
}));

Router.put("", {}, LilPath(async (request: Request<{ Body: LilBodyParam<AnimalUpdate> }>, reply: any): Promise<void> => {
    await reply.send(LilResponse(new Animal("Billy", ["tetnis", "rabies"]), {
        statusCode: 200,
        description: "An unregistered animal",
    }));
}, {
    method: 'PUT',
    path: '/animals/{id}/unregister',
    tags: ['Animals'],
}));

Router.get("", {}, LilPath(async (request: Request<{ Body: LilBodyParam<AnimalUpdate> }>, reply: any): Promise<void> => {
    const license: AnimalLicense = { adjacents: [], exp: new Date("2020-01-01"), state: "NY" }

    await reply.send(LilResponse(license, {
        statusCode: 200,
        description: "A specific animal license",
    }));
}, {
    method: 'GET',
    path: '/animals/{id}/license',
    tags: ['Animals'],
}));

Router.post("", {}, LilPath(async (request: Request<{ Body: LilBodyParam<{ name: string, freindliness: number, photo: Uint8Array }, true, "multipart/form-data"> }>, reply: any): Promise<void> => {
    const status: { status: string } = { status: "processing" }

    if (status.status === "processing") {
        await reply.send(LilResponse(status, {
            statusCode: 200,
            description: "Status of animal registration",
            mediaType: "application/xml"
        }));
    } else {
        reply.code(LilResponse(404, { statusCode: 404, description: "animal not found" })).send();
    }
}, {
    method: 'POST',
    path: '/animals/{id}/register',
    tags: ['Animals'],
}));

