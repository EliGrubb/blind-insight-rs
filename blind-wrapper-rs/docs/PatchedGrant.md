# PatchedGrant

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> |  | [optional][readonly]
**url** | Option<**String**> |  | [optional][readonly]
**organization** | Option<**String**> |  | [optional]
**teams** | Option<**Vec<String>**> |  | [optional]
**name** | Option<**String**> | A human-readable name for this grant. | [optional]
**codename** | Option<**String**> | The codename for this grant. | [optional]
**field_names** | Option<[**serde_json::Value**](.md)> | Field names of keys to request. None means allows all fields & query key; or a mapping of specific field names: `{'field_name': True, ...}` | [optional]
**can_share_keys** | Option<**bool**> | Whether this permission allows the user to share the key(s) assigned to this permission. | [optional]
**can_create_records** | Option<**bool**> | Whether this permission allows the user to create records. | [optional]
**dataset** | Option<**String**> | The dataset that this grant belongs to. | [optional]
**schema** | Option<**String**> | The schema that this grant belongs to. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


