# \MaterialsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**materials_create**](MaterialsApi.md#materials_create) | **POST** /api/materials/ | Create a new material
[**materials_destroy**](MaterialsApi.md#materials_destroy) | **DELETE** /api/materials/{id}/ | Delete a material
[**materials_list**](MaterialsApi.md#materials_list) | **GET** /api/materials/ | List all materials
[**materials_overrides**](MaterialsApi.md#materials_overrides) | **GET** /api/materials/overrides/ | List all material overrides
[**materials_overrides_delete_detail**](MaterialsApi.md#materials_overrides_delete_detail) | **DELETE** /api/materials/overrides/{key}/ | Delete a single material override
[**materials_overrides_delete_list**](MaterialsApi.md#materials_overrides_delete_list) | **DELETE** /api/materials/overrides/ | Delete all material overrides
[**materials_proof_create**](MaterialsApi.md#materials_proof_create) | **POST** /api/materials/{id}/proof/ | Validate a proof for a material
[**materials_receive_detail**](MaterialsApi.md#materials_receive_detail) | **POST** /api/materials/{material_id}/receive/ | Receive a single material
[**materials_receive_list**](MaterialsApi.md#materials_receive_list) | **POST** /api/materials/receive/ | Receive all materials
[**materials_retrieve**](MaterialsApi.md#materials_retrieve) | **GET** /api/materials/{id}/ | Retrieve a single material



## materials_create

> models::Material materials_create(material)
Create a new material

Create a new material.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**material** | [**Material**](Material.md) |  | [required] |

### Return type

[**models::Material**](Material.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## materials_destroy

> materials_destroy(id)
Delete a material

Delete a material by its ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | A unique value identifying this material. | [required] |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## materials_list

> Vec<models::Material> materials_list(created, id, limit, modified, offset, payload, proof, proofed, request, signature, uploaded_by)
List all materials

List all materials that you have access to.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**created** | Option<**String**> |  |  |
**id** | Option<**String**> |  |  |
**limit** | Option<**i32**> | Number of results to return per page. |  |
**modified** | Option<**String**> |  |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**payload** | Option<**String**> | The encrypted material. |  |
**proof** | Option<**String**> |  |  |
**proofed** | Option<**bool**> |  |  |
**request** | Option<**String**> |  |  |
**signature** | Option<**String**> | The signature of the material. |  |
**uploaded_by** | Option<**String**> | The identity that uploaded this material. |  |

### Return type

[**Vec<models::Material>**](Material.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## materials_overrides

> Vec<String> materials_overrides()
List all material overrides

[Blind Insight Proxy] List all material overrides

### Parameters

This endpoint does not need any parameter.

### Return type

**Vec<String>**

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## materials_overrides_delete_detail

> serde_json::Value materials_overrides_delete_detail(key)
Delete a single material override

[Blind Insight Proxy] Delete a single material override

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | The key of the material override to delete. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## materials_overrides_delete_list

> materials_overrides_delete_list()
Delete all material overrides

[Blind Insight Proxy] Delete all material overrides

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## materials_proof_create

> serde_json::Value materials_proof_create(id, material_proof)
Validate a proof for a material

Validate a proof for a material.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | A unique value identifying this material. | [required] |
**material_proof** | [**MaterialProof**](MaterialProof.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## materials_receive_detail

> models::MaterialsReceiveDetail200Response materials_receive_detail(material_id)
Receive a single material

[Blind Insight Proxy] Receive a single material

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**material_id** | **uuid::Uuid** | The material ID to receive. | [required] |

### Return type

[**models::MaterialsReceiveDetail200Response**](materials_receive_detail_200_response.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## materials_receive_list

> Vec<models::MaterialsReceiveDetail200Response> materials_receive_list()
Receive all materials

[Blind Insight Proxy] Receive all materials

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::MaterialsReceiveDetail200Response>**](materials_receive_detail_200_response.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## materials_retrieve

> models::Material materials_retrieve(id)
Retrieve a single material

Retrieve a single material by its ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | A unique value identifying this material. | [required] |

### Return type

[**models::Material**](Material.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

