import { LilPath, LilResponse } from '../../../src';
import { AnimalsRequest, Request } from '../dtos/requests';
import { Router } from './router';

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

