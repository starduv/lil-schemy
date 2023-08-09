import { BodyParam, Header, Path, QueryParam, Response, RouteParam } from '../../../src';
import AdminUser, { Account, CreateUserRequest, User, UserPatch } from '../dtos';
import { GetAccountRequest } from './requests'
import { Router } from './router';

export default Router.get("", {}, Path(async (request: { lat: QueryParam<number, false>, long: QueryParam<number, false>; headers: { user: Header<User, true>; }; }, reply: any): Promise<void> => {
    let success = {} as User;

    let response = Response(success, {
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

Router.patch("", {}, Path(async (request: { id: RouteParam<string, true>; date: QueryParam<string, false, undefined, "date">; }, reply: any): Promise<void> => {
    let admin = new AdminUser();

    let response = Response(admin, {
        statusCode: 202,
        description: "a modified admin user"
    });

    reply.send(response);
}, {
    method: 'PATCH',
    path: '/user/{id}',
}));

Router.get("", {}, Path(async (request: { id: RouteParam<string, true>; }, reply: any): Promise<void> => {
    let response = Response(new AdminUser(), {
        statusCode: 200,
        description: "a specific admin user"
    });

    reply.send(response);
}, {
    method: 'GET',
    path: '/user/{id}',
}));

Router.delete("", {}, Path(async (request: { id: RouteParam<string, true>; }, reply: any): Promise<void> => {
    let response = Response(null, {
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

Router.post("", {}, Path(async (request: { user: BodyParam<CreateUserRequest, true>; }, reply: any): Promise<void> => {
    let response = Response(<User>{}, {
        statusCode: 201,
        description: "Create a new user",
    });

    reply.send(response);
}, {
    method: 'POST',
    path: '/user',
    tags: ["Admin"]
}));

Router.put("", {}, Path(async (request: { tomato: BodyParam<UserPatch, false>; }, reply: any): Promise<void> => {
    let response = Response({} as AdminUser, {
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

Router.get("", {}, Path(async (request: GetAccountRequest, reply: any): Promise<void> => {
    let success: Account = ledger.getAccount();

    let response = Response(success, {
        statusCode: 200,
        description: "Get user account",
    });

    reply.send(response);
}, {
    method: 'GET',
    path: '/account',
    tags: ['Account'],
}));