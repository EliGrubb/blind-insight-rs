# \FilesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**files_create**](FilesApi.md#files_create) | **POST** /api/files/ | Create a new upload resource (only available via proxy)
[**files_delete**](FilesApi.md#files_delete) | **DELETE** /api/files/{id}/ | Delete upload (only available via proxy)
[**files_patch**](FilesApi.md#files_patch) | **PATCH** /api/files/{id}/ | Upload file chunk (only available via proxy)



## files_create

> files_create(tus_resumable, upload_length, upload_metadata, content_length, body)
Create a new upload resource (only available via proxy)

Create a new upload resource. The Upload-Length header indicates the size of the entire upload in bytes.  **On the command line:**  ```bash blind files create --Tus-Resumable 1.0.0  --Upload-Length 1000 --data records.json ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tus_resumable** | **String** | [required] The TUS version to use (default: 1.0.0). | [required] |[default to 1.0.0]
**upload_length** | **i32** | [required] The size of the entire upload in bytes. | [required] |
**upload_metadata** | **String** | [required] The upload metadata. Must consist of `filename` and `filetype` keys, separated by commas. The key and value MUST be separated by a space. The value MUST be Base64 encoded. Please see the examples for more information. | [required] |
**content_length** | Option<**i32**> | Set to 0 if the file size is unknown. |  |
**body** | Option<**std::path::PathBuf**> |  |  |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, text/csv
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files_delete

> serde_json::Value files_delete(tus_resumable, id)
Delete upload (only available via proxy)

Delete an upload resource.  **On the command line:**  ```bash blind files delete --id \"<upload-id>\" ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tus_resumable** | **String** | [required] The TUS version to use (default: 1.0.0). | [required] |
**id** | **String** | [required] The ID of the upload to delete. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files_patch

> files_patch(content_type, id, upload_checksum, body)
Upload file chunk (only available via proxy)

Upload a chunk of a file.  **On the command line:**  ```bash blind files patch --id \"<upload-id>\" --Upload-Offset 1000 --Upload-Checksum \"b64:checksum\" --data records.json ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**content_type** | **String** | [required] Must be application/offset+octet-stream | [required] |[default to application/offset+octet-stream]
**id** | **String** | [required] The ID of the upload to upload a chunk of. | [required] |
**upload_checksum** | Option<**String**> | The base64 encoded checksum of the current chunk. |  |
**body** | Option<**std::path::PathBuf**> |  |  |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

