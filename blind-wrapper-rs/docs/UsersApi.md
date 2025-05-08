# \UsersApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**users_by_name**](UsersApi.md#users_by_name) | **GET** /api/users/by_name/{email}/ | Retrieve one User object by name
[**users_create**](UsersApi.md#users_create) | **POST** /api/users/ | Create a new User
[**users_destroy**](UsersApi.md#users_destroy) | **DELETE** /api/users/{id}/ | Delete a User
[**users_list**](UsersApi.md#users_list) | **GET** /api/users/ | Retrieve one or more User objects, with filters
[**users_partial_update**](UsersApi.md#users_partial_update) | **PATCH** /api/users/{id}/ | Partially update a User
[**users_retrieve**](UsersApi.md#users_retrieve) | **GET** /api/users/{id}/ | Retrieve one User object
[**users_self**](UsersApi.md#users_self) | **GET** /api/users/self/ | Retrieve current user
[**users_update**](UsersApi.md#users_update) | **PUT** /api/users/{id}/ | Update a User



## users_by_name

> models::User users_by_name(email)
Retrieve one User object by name

Retrieve a single user by their name (email).  The response will be the user object.  **On the command line:**  ```bash blind users by-name --email \"<user-name>\" ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | **String** |  | [required] |

### Return type

[**models::User**](User.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_create

> models::User users_create(user)
Create a new User

Create a new user.  The response will be the newly created user object.  **On the command line:**  ```bash blind users create --data user.json ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user** | [**User**](User.md) |  | [required] |

### Return type

[**models::User**](User.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_destroy

> users_destroy(id)
Delete a User

Delete an existing user by their ID.  The response will be a 204 No Content if successful.  **On the command line:**  ```bash blind users destroy --id \"<user-id>\" ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | A unique value identifying this user. | [required] |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_list

> Vec<models::User> users_list(date_joined, email, first_name, has_identity, id, is_active, is_staff, is_superuser, last_login, last_name, limit, offset)
Retrieve one or more User objects, with filters

Retrieve a list of users. You can filter the list by providing query parameters. Only users that you have access to will be returned, which in practice means they are in the same organization.  The response will be a list of user objects.  **On the command line:**  ```bash blind users list ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**date_joined** | Option<**String**> |  |  |
**email** | Option<[**Vec<String>**](String.md)> |  |  |
**first_name** | Option<**String**> | First name (optional) |  |
**has_identity** | Option<**bool**> | Filter by whether the user has an identity. |  |
**id** | Option<**String**> |  |  |
**is_active** | Option<**bool**> | Designates whether this user should be treated as active. Unselect this instead of deleting accounts. |  |
**is_staff** | Option<**bool**> | Designates whether the user can log into this admin site. |  |
**is_superuser** | Option<**bool**> | Designates that this user has all permissions without explicitly assigning them. |  |
**last_login** | Option<**String**> |  |  |
**last_name** | Option<**String**> | Last name (optional) |  |
**limit** | Option<**i32**> | Number of results to return per page. |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |

### Return type

[**Vec<models::User>**](User.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_partial_update

> models::User users_partial_update(id, patched_user)
Partially update a User

Update an existing user with only the specified fields.  The response will be the updated user object.  **On the command line:**  ```bash blind users partial-update --id \"<user-id>\" --data user-update.json ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | A unique value identifying this user. | [required] |
**patched_user** | Option<[**PatchedUser**](PatchedUser.md)> |  |  |

### Return type

[**models::User**](User.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_retrieve

> models::User users_retrieve(id)
Retrieve one User object

Retrieve a single user by their ID.  The response will be the user object.  **On the command line:**  ```bash blind users retrieve --id \"<user-id>\" ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | A unique value identifying this user. | [required] |

### Return type

[**models::User**](User.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_self

> models::User users_self()
Retrieve current user

Retrieve the currently authenticated user.  The response will be the user object.  **On the command line:**  ```bash blind users self ```

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::User**](User.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_update

> models::User users_update(id, user)
Update a User

Update an entire existing user, replacing all fields.  The response will be the updated user object.  **On the command line:**  ```bash blind users update --id \"<user-id>\" --data user.json ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | A unique value identifying this user. | [required] |
**user** | [**User**](User.md) |  | [required] |

### Return type

[**models::User**](User.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

