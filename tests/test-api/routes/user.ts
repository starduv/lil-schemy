import { BodyParam, Path, Response, RouteParam } from '../../../src';
import AdminUser, { CreateUserRequest, GetUserRequest, User, UserPatch, UserPatchRequest } from '../dtos';
import { Router } from './router';

Router.get("", {}, Path(async (request: GetUserRequest, reply: any): Promise<void> => {
    let success = {} as User;

    let response = Response(success, {
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

Router.patch("", {}, Path(async (request: UserPatchRequest, reply: any): Promise<void> => {
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
        namespace: "v1",
        example: "NoContent"
    });

    reply.send(response);
}, {
    method: 'DELETE',
    path: '/user/{id}',
    tags: ["Admin", "Users"],
}));

Router.post("", {}, Path(async (request: { user: BodyParam<CreateUserRequest, true, "v1">; }, reply: any): Promise<void> => {
    let response = Response(<User>{}, {
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

Router.put("", {}, Path(async (request: { tomato: BodyParam<UserPatch, false, "v1">; }, reply: any): Promise<void> => {
    let response = Response({} as AdminUser, {
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