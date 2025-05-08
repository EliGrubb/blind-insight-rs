# \RecordsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**records_bulk_destroy**](RecordsApi.md#records_bulk_destroy) | **DELETE** /api/records/ | Bulk delete records
[**records_create**](RecordsApi.md#records_create) | **POST** /api/records/ | Create a new record
[**records_decrypt**](RecordsApi.md#records_decrypt) | **POST** /api/records/decrypt/ | Decrypt data and return records (only available via proxy)
[**records_destroy**](RecordsApi.md#records_destroy) | **DELETE** /api/records/{id}/ | Delete a record
[**records_ingest**](RecordsApi.md#records_ingest) | **POST** /api/records/ingest/ | Ingest data and encrypt it (only available via proxy)
[**records_list**](RecordsApi.md#records_list) | **GET** /api/records/ | List records
[**records_retrieve**](RecordsApi.md#records_retrieve) | **GET** /api/records/{id}/ | Retrieve a record
[**records_search**](RecordsApi.md#records_search) | **POST** /api/records/search/ | Search for records (only available via proxy)



## records_bulk_destroy

> records_bulk_destroy(id)
Bulk delete records

Delete multiple records by their IDs.  The response will be a 204 No Content if successful.  **On the command line:**  ```bash blind records bulk-destroy --id \"<record-id>\" --id \"<record-id>\" ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**Vec<String>**](String.md) | An array of record IDs to be deleted. | [required] |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## records_create

> models::IndexedRecord records_create(indexed_record)
Create a new record

Create a new record with an already encrypted payload. This is the non-porcelain endpoint. If you want to create a new record with plaintext data, use the `record create` porcelain command or the `records ingest` command with a proxy.  The response will be the newly created record object.  **On the command line:**  ```bash blind records create --data encrypted-record.json ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**indexed_record** | [**Vec<models::IndexedRecord>**](IndexedRecord.md) |  | [required] |

### Return type

[**models::IndexedRecord**](IndexedRecord.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## records_decrypt

> Vec<models::Record> records_decrypt(jobs_upload_request_inner)
Decrypt data and return records (only available via proxy)

[Blind Insight Proxy] Decrypt data and return records

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**jobs_upload_request_inner** | Option<[**Vec<models::JobsUploadRequestInner>**](jobs_upload_request_inner.md)> |  |  |

### Return type

[**Vec<models::Record>**](Record.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## records_destroy

> records_destroy(id)
Delete a record

Delete a record by its ID.  The response will be a 204 No Content if successful.  **On the command line:**  ```bash blind records destroy --id \"<record-id>\" ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | A unique value identifying this record. | [required] |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## records_ingest

> Vec<models::Record> records_ingest(jobs_upload_request_inner)
Ingest data and encrypt it (only available via proxy)

[Blind Insight Proxy] Ingest data and encrypt it returning the encrypted records

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**jobs_upload_request_inner** | Option<[**Vec<models::JobsUploadRequestInner>**](jobs_upload_request_inner.md)> |  |  |

### Return type

[**Vec<models::Record>**](Record.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## records_list

> Vec<models::Record> records_list(limit, offset, q, schema)
List records

List records for the given schema ID. Takes optional pagination parameters `limit` and `offset`, as well as an optional query filter `q`.  The response will be a list of record objects.  **On the command line:**  ```bash blind records list --schema \"<schema-id>\" ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | Number of results to return per page. |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**q** | Option<[**Vec<String>**](String.md)> | Query filter for records. |  |
**schema** | Option<**String**> | The schema ID that this record belongs to. |  |

### Return type

[**Vec<models::Record>**](Record.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## records_retrieve

> models::Record records_retrieve(id)
Retrieve a record

Retrieve a record by its ID.  The response will be the record object.  **On the command line:**  ```bash blind records retrieve --id \"<record-id>\" ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | A unique value identifying this record. | [required] |

### Return type

[**models::Record**](Record.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## records_search

> Vec<models::Record> records_search(records_search_request)
Search for records (only available via proxy)

[Blind Insight Proxy] Search for records by label and value. If label or value is specified, both must be present. If neither are present, this endpoint does not filter records.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**records_search_request** | Option<[**RecordsSearchRequest**](RecordsSearchRequest.md)> |  |  |

### Return type

[**Vec<models::Record>**](Record.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

