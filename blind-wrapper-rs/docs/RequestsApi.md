# \RequestsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**requests_create**](RequestsApi.md#requests_create) | **POST** /api/requests/ | Create a new request
[**requests_destroy**](RequestsApi.md#requests_destroy) | **DELETE** /api/requests/{id}/ | Delete a request
[**requests_fulfill_detail**](RequestsApi.md#requests_fulfill_detail) | **POST** /api/requests/{request_id}/fulfill/ | Fulfill a single request
[**requests_fulfill_list**](RequestsApi.md#requests_fulfill_list) | **POST** /api/requests/fulfill/ | Fulfill all requests
[**requests_list**](RequestsApi.md#requests_list) | **GET** /api/requests/ | List all requests
[**requests_partial_update**](RequestsApi.md#requests_partial_update) | **PATCH** /api/requests/{id}/ | Partially update a request
[**requests_retrieve**](RequestsApi.md#requests_retrieve) | **GET** /api/requests/{id}/ | Retrieve a single request
[**requests_update**](RequestsApi.md#requests_update) | **PUT** /api/requests/{id}/ | Update a request



## requests_create

> models::Request requests_create(request)
Create a new request

Create a new request for keys.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request** | [**Request**](Request.md) |  | [required] |

### Return type

[**models::Request**](Request.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## requests_destroy

> requests_destroy(id)
Delete a request

Delete a request by its ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | A unique value identifying this request. | [required] |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## requests_fulfill_detail

> models::Material requests_fulfill_detail(request_id)
Fulfill a single request

[Blind Insight Proxy] Fulfill a single request

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_id** | **uuid::Uuid** | The request ID to fulfill. | [required] |

### Return type

[**models::Material**](Material.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## requests_fulfill_list

> Vec<models::Material> requests_fulfill_list()
Fulfill all requests

[Blind Insight Proxy] Fulfill all requests

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::Material>**](Material.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## requests_list

> Vec<models::Request> requests_list(created, expires, field_name, fulfilled, id, identity, limit, modified, offset, received, schema, shared)
List all requests

List all requests that you have access to.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**created** | Option<**String**> |  |  |
**expires** | Option<**String**> |  |  |
**field_name** | Option<**String**> |  |  |
**fulfilled** | Option<**bool**> |  |  |
**id** | Option<**String**> |  |  |
**identity** | Option<**String**> |  |  |
**limit** | Option<**i32**> | Number of results to return per page. |  |
**modified** | Option<**String**> |  |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**received** | Option<**bool**> |  |  |
**schema** | Option<**String**> |  |  |
**shared** | Option<**bool**> | Whether a request has had keys shared or not. |  |

### Return type

[**Vec<models::Request>**](Request.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## requests_partial_update

> models::Request requests_partial_update(id, patched_request)
Partially update a request

Partially update a request by its ID. This only replaces the specified fields.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | A unique value identifying this request. | [required] |
**patched_request** | Option<[**PatchedRequest**](PatchedRequest.md)> |  |  |

### Return type

[**models::Request**](Request.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## requests_retrieve

> models::Request requests_retrieve(id)
Retrieve a single request

Retrieve a single request by its ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | A unique value identifying this request. | [required] |

### Return type

[**models::Request**](Request.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## requests_update

> models::Request requests_update(id, request)
Update a request

Update a request by its ID. This completely replaces the object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | A unique value identifying this request. | [required] |
**request** | [**Request**](Request.md) |  | [required] |

### Return type

[**models::Request**](Request.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

