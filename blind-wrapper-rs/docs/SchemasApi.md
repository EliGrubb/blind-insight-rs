# \SchemasApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**schemas_create**](SchemasApi.md#schemas_create) | **POST** /api/schemas/ | Create a new schema
[**schemas_decrypt**](SchemasApi.md#schemas_decrypt) | **POST** /api/schemas/decrypt/ | Decrypt a schema (only available via proxy)
[**schemas_destroy**](SchemasApi.md#schemas_destroy) | **DELETE** /api/schemas/{id}/ | Delete a schema
[**schemas_list**](SchemasApi.md#schemas_list) | **GET** /api/schemas/ | List all schemas
[**schemas_partial_update**](SchemasApi.md#schemas_partial_update) | **PATCH** /api/schemas/{id}/ | Partially update a schema
[**schemas_retrieve**](SchemasApi.md#schemas_retrieve) | **GET** /api/schemas/{id}/ | Retrieve a single schema
[**schemas_teams**](SchemasApi.md#schemas_teams) | **GET** /api/schemas/{id}/teams/ | List the teams for a schema
[**schemas_update**](SchemasApi.md#schemas_update) | **PUT** /api/schemas/{id}/ | Update a schema



## schemas_create

> models::Schema schemas_create(schema)
Create a new schema

Create a new schema.  The response will be the newly created schema object.  **On the command line:**  ```bash blind schemas create --data schema.json ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**schema** | [**Schema**](Schema.md) |  | [required] |

### Return type

[**models::Schema**](Schema.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## schemas_decrypt

> std::collections::HashMap<String, serde_json::Value> schemas_decrypt(schemas_decrypt_request)
Decrypt a schema (only available via proxy)

[Blind Insight Proxy] Decrypt a schema and return a hashed schema

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**schemas_decrypt_request** | Option<[**SchemasDecryptRequest**](SchemasDecryptRequest.md)> |  |  |

### Return type

[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## schemas_destroy

> schemas_destroy(id)
Delete a schema

Delete an existing schema by its ID.  The response will be a 204 No Content if successful.  **On the command line:**  ```bash blind schemas destroy --id \"<schema-id>\" ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | A unique value identifying this schema. | [required] |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## schemas_list

> Vec<models::Schema> schemas_list(dataset, description, id, limit, name, offset, slug)
List all schemas

List all schemas, filtered by the given parameters. Takes optional pagination parameters `limit` and `offset`.  The response will be a list of schema objects.  **On the command line:**  ```bash blind schemas list ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dataset** | Option<**String**> | The dataset that this schema belongs to. |  |
**description** | Option<**String**> | A longer description of the schema. |  |
**id** | Option<**String**> |  |  |
**limit** | Option<**i32**> | Number of results to return per page. |  |
**name** | Option<**String**> | A human-readable name for this schema. |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**slug** | Option<**String**> | A unique slug for this schema. |  |

### Return type

[**Vec<models::Schema>**](Schema.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## schemas_partial_update

> models::Schema schemas_partial_update(id, patched_schema)
Partially update a schema

Update an existing schema with only the specified fields.  The response will be the updated schema object.  **On the command line:**  ```bash blind schemas partial-update --id \"<schema-id>\" --data schema-update.json ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | A unique value identifying this schema. | [required] |
**patched_schema** | Option<[**PatchedSchema**](PatchedSchema.md)> |  |  |

### Return type

[**models::Schema**](Schema.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## schemas_retrieve

> models::Schema schemas_retrieve(id)
Retrieve a single schema

Retrieve a single schema by its ID.  The response will be the schema object.  **On the command line:**  ```bash blind schemas retrieve --id \"<schema-id>\" ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | A unique value identifying this schema. | [required] |

### Return type

[**models::Schema**](Schema.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## schemas_teams

> models::Schema schemas_teams(id)
List the teams for a schema

List the teams for a schema.  **On the command line:**  ```bash blind schemas teams --id \"<schema-id>\" ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | A unique value identifying this schema. | [required] |

### Return type

[**models::Schema**](Schema.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## schemas_update

> models::Schema schemas_update(id, schema)
Update a schema

Update an existing schema by its ID, replacing the entire schema.  The response will be the updated schema object.  **On the command line:**  ```bash blind schemas update --id \"<schema-id>\" --data schema.json ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | A unique value identifying this schema. | [required] |
**schema** | [**Schema**](Schema.md) |  | [required] |

### Return type

[**models::Schema**](Schema.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

