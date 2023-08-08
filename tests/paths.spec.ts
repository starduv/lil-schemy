import { expect, use } from 'chai';
import deepEqual from 'deep-equal-in-any-order';
import { OpenAPIV3 } from 'openapi-types';
import { generateSchemas } from '../src/generator';
import { getRootFiles } from '../src/utils';

use(deepEqual);

describe('open api generator', () => {
    let schema: OpenAPIV3.Document;

    before(() => {
        const result = generateSchemas({
            openApi: {
                base: JSON.stringify({}),
                entry: getRootFiles(__dirname, ["test-api/routes/*.ts", "!test-api/routes/router.ts"]),
            }
        });

        schema = JSON.parse(result.openApi.schema || "");
    });

    it('generates schemas', () => {
        expect(schema.components?.schemas).to.deep.equalInAnyOrder({
            v1: {
                type: "object",
                properties: {
                    User: {
                        type: "object",
                        properties: {
                            name: {
                                type: "string"
                            }
                        },
                        required: ["name"]
                    },
                    CreateUserRequest: {
                        type: 'object',
                        properties: {
                            name: {
                                type: 'string'
                            }
                        },
                        required: ['name']
                    }
                },
                required: [
                    "CreateUserRequest",
                    "User",
                ]
            },
            Account: {
                type: 'object',
                properties: {
                    number: {
                        type: 'string'
                    }
                },
                required: ['number']
            },
            AdminUser: {
                type: 'object',
                properties: {
                    permissions: {
                        type: 'array',
                        items: {
                            type: 'string'
                        }
                    },
                    name: {
                        type: 'string'
                    }
                },
                required: ['permissions', 'name']
            },
        });
    });

    it('generates api paths', () => {
        expect(schema.paths).to.deep.equal({
            "/account": {
                get: {
                    parameters: [
                        {
                            content: {
                                "application/json": {
                                    "schema": {
                                        "type": "string"
                                    }
                                }
                            },
                            in: "path",
                            name: "id",
                            required: true
                        }
                    ],
                    responses: {
                        200: {
                            description: "Get user account",
                            content: {
                                "application/json": {
                                    schema: {
                                        $ref: "#/components/schemas/Account"
                                    }
                                }
                            }
                        }
                    },
                    tags: ["Account"]
                }
            },
            "/user": {
                get: {
                    responses: {
                        200: {
                            description: "Who am I",
                            content: {
                                "application/json": {
                                    schema: {
                                        $ref: "#/components/schemas/v1/properties/User",
                                    },
                                    example: {
                                        $ref: "#/components/examples/v1.User"
                                    }
                                }
                            }
                        }
                    },
                    parameters: [
                        {
                            name: "lat",
                            in: "query",
                            content: {
                                "application/json": {
                                    schema: {
                                        type: "number"
                                    }
                                }
                            },
                            required: false
                        },
                        {
                            name: "long",
                            in: "query",
                            content: {
                                "application/json": {
                                    schema: {
                                        type: "number"
                                    }
                                }
                            },
                            required: false
                        },
                        {
                            name: "user",
                            in: "header",
                            content: {
                                "application/json": {
                                    schema: {
                                        "$ref": "#/components/schemas/v1/properties/User"
                                    }
                                }
                            },
                            required: true
                        }
                    ],
                    tags: [
                        "Users"
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
                },
                put: {
                    requestBody: {
                        content: {
                            "application/json": {
                                schema: {
                                    $ref: "#/components/schemas/v1/properties/UserPatch"
                                }
                            }
                        },
                        required: false
                    },
                    responses: {
                        202: {
                            description: "Updated User",
                            content: {
                                "application/json": {
                                    schema: {
                                        $ref: "#/components/schemas/AdminUser"
                                    }
                                }
                            }
                        }
                    },
                    tags: ["User"]
                }
            },
            "/user/{id}": {
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
            }
        });
    });
});