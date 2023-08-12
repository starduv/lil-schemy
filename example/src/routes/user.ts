import { LilBodyParam, LilHeader, LilPath, LilQueryParam, LilResponse, LilRouteParam } from '../../../src';
import AdminUser, { CreateUserRequest, User, UserPatch } from '../dtos';
import { Router } from './router'

Router.get("", {}, LilPath(async (request: { lat: LilQueryParam<number, false>, long: LilQueryParam<number, false>; headers: { user: LilHeader<User, true, "v1">; }; }, reply: any): Promise<void> => {
    let success = {} as User;

    let response = LilResponse(success, {
        statusCode: 200,
        namespace: "v1",
        description: "Who am I",
        example: "User"
    });

    reply.send(response);
}, {
    method: 'GET',
    path: '/user',
    tags: ['space'],
}));

Router.patch("", {}, LilPath(async (request: { id: LilRouteParam<string, true>; date: LilQueryParam<string, false, undefined, "date">; }, reply: any): Promise<void> => {
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
        namespace: "v1",
        example: "NoContent"
    });

    reply.send(response);
}, {
    method: 'DELETE',
    path: '/user/{id}',
    tags: ["Admin", "Users"],
}));

Router.post("", {}, LilPath(async (request: { user: BodyParam<CreateUserRequest, true, "v1">; }, reply: any): Promise<void> => {
    let response = LilResponse(<User>{}, {
        statusCode: 201,
        description: "Create a new user",
        namespace: "v1"
    });

    reply.send(response);
}, {
    method: 'POST',
    path: '/user',
    tags: ["Admin"]
}));

Router.put("", {}, LilPath(async (request: { tomato: BodyParam<UserPatch, false, "v1">; }, reply: any): Promise<void> => {
    let response = LilResponse({} as AdminUser, {
        statusCode: 202,
        description: "Updated User",
        namespace: "v1"
    });

    reply.send(response);
}, {
    method: 'PUT',
    path: '/user',
    tags: ["User"]
}));