# \TokenApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**token_create**](TokenApi.md#token_create) | **POST** /api/token/ | Create a new token
[**token_refresh**](TokenApi.md#token_refresh) | **POST** /api/token/refresh/ | Refresh a token
[**token_verify**](TokenApi.md#token_verify) | **POST** /api/token/verify/ | Verify a token



## token_create

> models::TokenObtainPair token_create(token_obtain_pair)
Create a new token

Create a new token for the user with the given email and password. On success, returns a JSON web token which can be used in the `Authorization` header to make API requests.  The response will be an JSON object with the access and refresh tokens.  **On the command line:**  ```bash blind token create --data '{\"email\": \"demo@example.com\", \"password\": \"pass1234\"}' ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token_obtain_pair** | [**TokenObtainPair**](TokenObtainPair.md) |  | [required] |

### Return type

[**models::TokenObtainPair**](TokenObtainPair.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## token_refresh

> models::TokenRefresh token_refresh(token_refresh)
Refresh a token

Refresh an existing token. On success, returns a JSON web token which can be used in the `Authorization` header to make API requests.  The response will be an JSON object with the new access token.  **On the command line:**  ```bash blind token refresh --data '{\"refresh\": \"<refresh-token>\"}' ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token_refresh** | [**TokenRefresh**](TokenRefresh.md) |  | [required] |

### Return type

[**models::TokenRefresh**](TokenRefresh.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## token_verify

> models::TokenVerify token_verify(token_verify)
Verify a token

Verify an access or refresh token is valid.  The response will be an empty object if it is valid, or a 401 unauthorized error if it is not.  **On the command line:**  ```bash blind token verify --data '{\"token\": \"<access-or-refresh-token>\"}' ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token_verify** | [**TokenVerify**](TokenVerify.md) |  | [required] |

### Return type

[**models::TokenVerify**](TokenVerify.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

