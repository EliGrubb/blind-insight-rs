# \GrantsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**grants_create**](GrantsApi.md#grants_create) | **POST** /api/grants/ | Create a new grant
[**grants_destroy**](GrantsApi.md#grants_destroy) | **DELETE** /api/grants/{id}/ | Delete a grant
[**grants_list**](GrantsApi.md#grants_list) | **GET** /api/grants/ | List all grants
[**grants_partial_update**](GrantsApi.md#grants_partial_update) | **PATCH** /api/grants/{id}/ | Partially update a grant
[**grants_retrieve**](GrantsApi.md#grants_retrieve) | **GET** /api/grants/{id}/ | Retrieve a single grant
[**grants_update**](GrantsApi.md#grants_update) | **PUT** /api/grants/{id}/ | Update a grant



## grants_create

> models::Grant grants_create(grant)
Create a new grant

Create a new grant.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**grant** | [**Grant**](Grant.md) |  | [required] |

### Return type

[**models::Grant**](Grant.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## grants_destroy

> grants_destroy(id)
Delete a grant

Delete a grant by its ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | A unique value identifying this grant. | [required] |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## grants_list

> Vec<models::Grant> grants_list(can_create_records, can_share_keys, codename, dataset, id, limit, name, offset, organization, schema, teams)
List all grants

List all grants that you have access to.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**can_create_records** | Option<**bool**> | Whether this permission allows the user to create records. |  |
**can_share_keys** | Option<**bool**> | Whether this permission allows the user to share the key(s) assigned to this permission. |  |
**codename** | Option<**String**> | The codename for this grant. |  |
**dataset** | Option<**String**> | The dataset that this grant belongs to. |  |
**id** | Option<**String**> |  |  |
**limit** | Option<**i32**> | Number of results to return per page. |  |
**name** | Option<**String**> | A human-readable name for this grant. |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**organization** | Option<**String**> | The organization that this grant belongs to. |  |
**schema** | Option<**String**> | The schema that this grant belongs to. |  |
**teams** | Option<[**Vec<String>**](String.md)> | The team(s) that this grant is assigned to. |  |

### Return type

[**Vec<models::Grant>**](Grant.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## grants_partial_update

> models::Grant grants_partial_update(id, patched_grant)
Partially update a grant

Partially update a grant by its ID. This only replaces the specified fields.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | A unique value identifying this grant. | [required] |
**patched_grant** | Option<[**PatchedGrant**](PatchedGrant.md)> |  |  |

### Return type

[**models::Grant**](Grant.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## grants_retrieve

> models::Grant grants_retrieve(id)
Retrieve a single grant

Retrieve a single grant by its ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | A unique value identifying this grant. | [required] |

### Return type

[**models::Grant**](Grant.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## grants_update

> models::Grant grants_update(id, grant)
Update a grant

Update a grant by its ID. This completely replaces the object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | A unique value identifying this grant. | [required] |
**grant** | [**Grant**](Grant.md) |  | [required] |

### Return type

[**models::Grant**](Grant.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

