# Grant

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | [readonly]
**url** | **String** |  | [readonly]
**organization** | **String** |  | 
**teams** | **Vec<String>** |  | 
**name** | **String** | A human-readable name for this grant. | 
**codename** | **String** | The codename for this grant. | 
**field_names** | Option<[**serde_json::Value**](.md)> | Field names of keys to request. None means allows all fields & query key; or a mapping of specific field names: `{'field_name': True, ...}` | [optional]
**can_share_keys** | Option<**bool**> | Whether this permission allows the user to share the key(s) assigned to this permission. | [optional]
**can_create_records** | Option<**bool**> | Whether this permission allows the user to create records. | [optional]
**dataset** | **String** | The dataset that this grant belongs to. | 
**schema** | **String** | The schema that this grant belongs to. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


