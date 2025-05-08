# \DatasetsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**datasets_create**](DatasetsApi.md#datasets_create) | **POST** /api/datasets/ | Create a new Dataset
[**datasets_destroy**](DatasetsApi.md#datasets_destroy) | **DELETE** /api/datasets/{id}/ | Delete a Dataset
[**datasets_list**](DatasetsApi.md#datasets_list) | **GET** /api/datasets/ | Retrieve a list of Dataset objects, with filters
[**datasets_partial_update**](DatasetsApi.md#datasets_partial_update) | **PATCH** /api/datasets/{id}/ | Partially update a Dataset
[**datasets_retrieve**](DatasetsApi.md#datasets_retrieve) | **GET** /api/datasets/{id}/ | Retrieve one Dataset object
[**datasets_schema**](DatasetsApi.md#datasets_schema) | **GET** /api/datasets/{id}/schema/{slug}/ | Retrieve a dataset schema by slug
[**datasets_update**](DatasetsApi.md#datasets_update) | **PUT** /api/datasets/{id}/ | Update a Dataset



## datasets_create

> models::Dataset datasets_create(dataset)
Create a new Dataset

Create a new dataset.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dataset** | [**Dataset**](Dataset.md) |  | [required] |

### Return type

[**models::Dataset**](Dataset.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## datasets_destroy

> datasets_destroy(id)
Delete a Dataset

Delete an existing dataset.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | A unique value identifying this dataset. | [required] |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## datasets_list

> Vec<models::Dataset> datasets_list(description, id, limit, name, offset, organization, slug)
Retrieve a list of Dataset objects, with filters

Retrieve a list of datasets. You can filter the list by providing query parameters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**description** | Option<**String**> | A longer description of the dataset. |  |
**id** | Option<**String**> |  |  |
**limit** | Option<**i32**> | Number of results to return per page. |  |
**name** | Option<**String**> | A human-readable name for this dataset. |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**organization** | Option<**String**> | The organization that this dataset belongs to. |  |
**slug** | Option<**String**> | A unique slug for this dataset. |  |

### Return type

[**Vec<models::Dataset>**](Dataset.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## datasets_partial_update

> models::Dataset datasets_partial_update(id, patched_dataset)
Partially update a Dataset

Update an existing dataset with only the specified fields.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | A unique value identifying this dataset. | [required] |
**patched_dataset** | Option<[**PatchedDataset**](PatchedDataset.md)> |  |  |

### Return type

[**models::Dataset**](Dataset.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## datasets_retrieve

> models::Dataset datasets_retrieve(id)
Retrieve one Dataset object

Retrieve one dataset.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | A unique value identifying this dataset. | [required] |

### Return type

[**models::Dataset**](Dataset.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## datasets_schema

> models::Dataset datasets_schema(id, slug)
Retrieve a dataset schema by slug

Retrieve a dataset schema by its slug.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | A unique value identifying this dataset. | [required] |
**slug** | **String** |  | [required] |

### Return type

[**models::Dataset**](Dataset.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## datasets_update

> models::Dataset datasets_update(id, dataset)
Update a Dataset

Update an existing dataset.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | A unique value identifying this dataset. | [required] |
**dataset** | [**Dataset**](Dataset.md) |  | [required] |

### Return type

[**models::Dataset**](Dataset.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

