import { Path, Response } from '../../../src';
import { AnimalsRequest, Request } from '../dtos/requests';
import { Router } from './router';

export default Router.get("", {}, Path(async (request: Request<AnimalsRequest>, reply: any): Promise<void> => {
    let response = Response([] as Array<string>, {
        statusCode: 200,
        description: "List animals of a specific kind",
    });

    reply.send(response);
}, {
    method: 'GET',
    path: '/animals',
    tags: ['Animals'],
}));

