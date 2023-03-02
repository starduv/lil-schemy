import { expect } from 'chai';
import { OpenAPIV3 } from 'openapi-types'
import { generateSchemas } from '../src/generator'
import { getContext } from '../src/utils'

describe('paths', () => {
    let schema: OpenAPIV3.Document;
    let context = getContext(__dirname, ["test-api/routes/*.ts"], {
        project: './tsconfig.json'
    })

    before(() => {
        const result = generateSchemas({
            asts: context.asts,
            modules: JSON.stringify(context.moduleNames),
            openApi: {
                base: JSON.stringify({}),
                paths: context.rootFiles,
            }
        })

        console.log(result.openApi.schema)

        schema = JSON.parse(result.openApi.schema || "");
    })

    it('generates api paths', () => {
        expect(schema.paths["/user"]).to.deep.equal({
            "get": {
                "responses": {
                    "200": {
                        "description": "Who am I",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "$ref": "#/components/schemas/v1/properties/User",
                                },
                                "example": {
                                    "$ref": "#/components/examples/v1.User"
                                }
                            }
                        }
                    }
                },
                "parameters": [
                    {
                        "name": "lat",
                        "in": "query",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "number"
                                }
                            }
                        },
                        "required": false
                    },
                    {
                        "name": "long",
                        "in": "query",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "number"
                                }
                            }
                        },
                        "required": false
                    },
                    {
                        "name": "user",
                        "in": "header",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "$ref": "#/components/schemas/v1/properties/User"
                                }
                            }
                        },
                        "required": true
                    }
                ],
                "tags": [
                    "space"
                ]
            },
            post: {
                requestBody: {
                    content: {
                        "application/json": {
                            schema: {
                                $ref: "#/components/schemas/v1/properties/CreateUserRequest"
                            }
                        }
                    },
                    required: true
                },
                responses: {
                    201: {
                        description: "Create a new user",
                        content: {
                            "application/json": {
                                schema: {
                                    $ref: "#/components/schemas/v1/properties/User"
                                }
                            }
                        }
                    }
                },
                tags: ["Admin"]
            }
        });
        expect(schema.paths["/user/{id}"]).to.deep.eq({
            get: {
                parameters: [
                    {
                        in: "path",
                        name: "id",
                        content: {
                            "application/json": {
                                schema: {
                                    type: "string",
                                }
                            }
                        },
                        required: true
                    }
                ],
                responses: {
                    200: {
                        description: "a specific admin user",
                        content: {
                            "application/json": {
                                schema: {
                                    $ref: "#/components/schemas/AdminUser"
                                }
                            }
                        }
                    }
                }
            },
            patch: {
                parameters: [
                    {
                        in: "path",
                        name: "id",
                        content: {
                            "application/json": {
                                schema: {
                                    type: "string",
                                }
                            }
                        },
                        required: true
                    },
                    {
                        in: "query",
                        name: "date",
                        content: {
                            "application/json": {
                                schema: {
                                    type: "string",
                                    format: "date"
                                }
                            }
                        },
                        required: false
                    }
                ],
                responses: {
                    202: {
                        description: "a modified admin user",
                        content: {
                            "application/json": {
                                schema: {
                                    $ref: "#/components/schemas/AdminUser",
                                },
                            }
                        }
                    }
                }
            },
            delete: {
                parameters: [
                    {
                        in: "path",
                        name: "id",
                        content: {
                            "application/json": {
                                schema: {
                                    type: "string",
                                }
                            }
                        },
                        required: true
                    }
                ],
                responses: {
                    204: {
                        description: "no content",
                        content: {
                            "application/json": {
                                example: {
                                    $ref: "#/components/examples/v1.NoContent"
                                }
                            }
                        }
                    }
                },
                tags: ["Admin", "Users"],
            }
        })
    })
})