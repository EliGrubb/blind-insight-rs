# \IdentitiesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**identities_create**](IdentitiesApi.md#identities_create) | **POST** /api/identities/ | Create a new Identity
[**identities_destroy**](IdentitiesApi.md#identities_destroy) | **DELETE** /api/identities/{id}/ | Delete an Identity
[**identities_list**](IdentitiesApi.md#identities_list) | **GET** /api/identities/ | List Identities
[**identities_partial_update**](IdentitiesApi.md#identities_partial_update) | **PATCH** /api/identities/{id}/ | Partially update an Identity
[**identities_retrieve**](IdentitiesApi.md#identities_retrieve) | **GET** /api/identities/{id}/ | Retrieve a single Identity
[**identities_update**](IdentitiesApi.md#identities_update) | **PUT** /api/identities/{id}/ | Update an Identity



## identities_create

> models::Identity identities_create(identity)
Create a new Identity

Create a new identity.  The response will be the newly created identity object.  **On the command line:**  ```bash blind identities create --user-id \"<user_id>\" --data identity.json ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identity** | [**Identity**](Identity.md) |  | [required] |

### Return type

[**models::Identity**](Identity.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## identities_destroy

> identities_destroy(id)
Delete an Identity

Delete an identity.  The response will be an empty object with a 204 status.  **On the command line:**  ```bash blind identities destroy --user-id \"<user_id>\" --id \"<identity_id>\" ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | A unique value identifying this identity. | [required] |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## identities_list

> Vec<models::Identity> identities_list(device_id, id, limit, offset, primary_public_key, sub_public_key, user)
List Identities

List identities, filtered by the given parameters. Takes optional pagination parameters.  The response will be a list of identity objects.  **On the command line:**  ```bash blind identities list --user-id \"<user_id>\" ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**device_id** | Option<**String**> | The device ID associated with this identity. |  |
**id** | Option<**String**> |  |  |
**limit** | Option<**i32**> | Number of results to return per page. |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**primary_public_key** | Option<**String**> | The primary PGP key for the identity. |  |
**sub_public_key** | Option<**String**> | The PGP sub-key for the identity. |  |
**user** | Option<**String**> |  |  |

### Return type

[**Vec<models::Identity>**](Identity.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## identities_partial_update

> models::Identity identities_partial_update(id, patched_identity)
Partially update an Identity

Update an identity with only the specified fields.  The response will be the updated identity object.  **On the command line:**  ```bash blind identities partial-update --user-id \"<user_id>\" --id \"<identity_id>\" --data identity-update.json ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | A unique value identifying this identity. | [required] |
**patched_identity** | Option<[**PatchedIdentity**](PatchedIdentity.md)> |  |  |

### Return type

[**models::Identity**](Identity.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## identities_retrieve

> models::Identity identities_retrieve(id)
Retrieve a single Identity

Retrieve an identity by its ID.  The response will be the identity object.  **On the command line:**  ```bash blind identities retrieve --user-id \"<user_id>\" --id \"<identity_id>\" ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | A unique value identifying this identity. | [required] |

### Return type

[**models::Identity**](Identity.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## identities_update

> models::Identity identities_update(id, identity)
Update an Identity

Update an identity replacing it entirely.  The response will be the updated identity object.  **On the command line:**  ```bash blind identities update --user-id \"<user_id>\" --id \"<identity_id>\" --data identity.json ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | A unique value identifying this identity. | [required] |
**identity** | [**Identity**](Identity.md) |  | [required] |

### Return type

[**models::Identity**](Identity.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

