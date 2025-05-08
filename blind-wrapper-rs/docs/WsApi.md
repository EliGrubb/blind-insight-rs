# \WsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**jobs_websocket**](WsApi.md#jobs_websocket) | **GET** /api/ws/jobs/{job_id}/ | Connect to a job status websocket (only available via proxy)



## jobs_websocket

> models::JobsStatus200Response jobs_websocket(job_id)
Connect to a job status websocket (only available via proxy)

Connect to a job status websocket by its ID.  **On the command line:**  ```bash blind jobs websocket --id \"<job-id>\" ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **uuid::Uuid** | The job ID to get the status of. | [required] |

### Return type

[**models::JobsStatus200Response**](jobs_status_200_response.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

