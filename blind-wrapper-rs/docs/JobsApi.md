# \JobsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**jobs_status**](JobsApi.md#jobs_status) | **GET** /api/jobs/{job_id}/ | Get the status of a job (only available via proxy)
[**jobs_upload**](JobsApi.md#jobs_upload) | **POST** /api/jobs/upload/ | Batch upload new records (only available via proxy)



## jobs_status

> models::JobsStatus200Response jobs_status(job_id)
Get the status of a job (only available via proxy)

Get the status of a job by its ID.  **On the command line:**  ```bash blind jobs status --id \"<job-id>\" ```

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


## jobs_upload

> models::JobsUpload200Response jobs_upload(x_batch_size, jobs_upload_request_inner)
Batch upload new records (only available via proxy)

Upload new records to the database in a single request.  **On the command line:**  ```bash blind jobs upload --data records.json ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_batch_size** | Option<**i32**> | The number of records to upload in each batch. |  |[default to 10]
**jobs_upload_request_inner** | Option<[**Vec<models::JobsUploadRequestInner>**](jobs_upload_request_inner.md)> |  |  |

### Return type

[**models::JobsUpload200Response**](jobs_upload_200_response.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, text/csv
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

