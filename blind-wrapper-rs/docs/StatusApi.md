# \StatusApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**status_retrieve**](StatusApi.md#status_retrieve) | **GET** /api/status/ | Retrieve the status of the API



## status_retrieve

> models::StatusRetrieve200Response status_retrieve()
Retrieve the status of the API

This endpoint is used to check the status of the API. When behind the Blind Proxy, the `behind_proxy` field will be set to true.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::StatusRetrieve200Response**](status_retrieve_200_response.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

