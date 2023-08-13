import { LilBodyParam, LilHeader, LilPath, LilQueryParam, LilResponse, LilRouteParam } from '../../../src';
import AdminUser, { Account, CreateUserRequest, User, UserPatch } from '../dtos';
import { GetAccountRequest } from '../dtos/requests';
import { Router } from './router';

export default Router.get("", {}, LilPath(async (request: { lat: LilQueryParam<number, false>, long: LilQueryParam<number, false>; headers: { user: LilHeader<User, true>; }; }, reply: any): Promise<void> => {
    let success = {} as User;

    let response = LilResponse(success, {
        statusCode: 200,
        description: "Who am I",
        example: "User"
    });

    reply.send(response);
}, {
    method: 'GET',
    path: '/user',
    tags: ['Users'],
}));

Router.patch("", {}, LilPath(async (request: { id: LilRouteParam<string, true>; date: LilQueryParam<string, false, "date">; }, reply: any): Promise<void> => {
    let admin = new AdminUser();

    let response = LilResponse(admin, {
        statusCode: 202,
        description: "a modified admin user"
    });

    reply.send(response);
}, {
    method: 'PATCH',
    path: '/user/{id}',
}));

Router.get("", {}, LilPath(async (request: { id: LilRouteParam<string, true>; }, reply: any): Promise<void> => {
    let response = LilResponse(new AdminUser(), {
        statusCode: 200,
        description: "a specific admin user"
    });

    reply.send(response);
}, {
    method: 'GET',
    path: '/user/{id}',
}));

Router.delete("", {}, LilPath(async (request: { id: LilRouteParam<string, true>; }, reply: any): Promise<void> => {
    let response = LilResponse(null, {
        statusCode: 204,
        description: "no content",
        example: "NoContent"
    });

    reply.send(response);
}, {
    method: 'DELETE',
    path: '/user/{id}',
    tags: ["Admin", "Users"],
}));

Router.post("", {}, LilPath(async (request: { user: LilBodyParam<CreateUserRequest, true>; }, reply: any): Promise<void> => {
    let response = LilResponse(<User>{}, {
        statusCode: 201,
        description: "Create a new user",
    });

    reply.send(response);
}, {
    method: 'POST',
    path: '/user',
    tags: ["Admin"]
}));

Router.put("", {}, LilPath(async (request: { tomato: LilBodyParam<UserPatch, false>; }, reply: any): Promise<void> => {
    let response = LilResponse({} as AdminUser, {
        statusCode: 202,
        description: "Updated User",
    });

    reply.send(response);
}, {
    method: 'PUT',
    path: '/user',
    tags: ["User"]
}));

const ledger = {
    getAccount: () => ({} as Account)
};

Router.get("", {}, LilPath(async (request: GetAccountRequest, reply: any): Promise<void> => {
    let success: Account = ledger.getAccount();

    let response = LilResponse(success, {
        statusCode: 200,
        description: "Get user account",
    });

    reply.send(response);
}, {
    method: 'GET',
    path: '/account',
    tags: ['Account'],
}));