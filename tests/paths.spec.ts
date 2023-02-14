import { expect } from 'chai';
import { OpenAPIV3 } from 'openapi-types'
import { generateSchemas } from '../src/generator'
import { getContext } from '../src/utils'

describe('paths', () => {
    let schema: OpenAPIV3.Document;
    let context = getContext(__dirname, ["test-api/routes/*.ts"], true, {
        project: './tsconfig.json'
    })

    before(() => {
        const result = generateSchemas({
            getAst: context.getAst,
            openApi: {
                base: JSON.stringify({}),
                paths: context.rootFiles
            }
        })

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
                                    "$ref": "User"
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
                    }
                ],
                "tags": [
                    "space"
                ]
            }
        });
    })
})