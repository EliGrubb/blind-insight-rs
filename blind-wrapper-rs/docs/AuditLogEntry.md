# AuditLogEntry

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**model** | **String** |  | 
**object_pk** | **String** |  | 
**object_repr** | **String** |  | 
**action** | [**models::ActionEnum**](ActionEnum.md) |  | 
**changes_text** | Option<**String**> |  | [optional]
**changes** | Option<[**serde_json::Value**](.md)> |  | [optional]
**cid** | Option<**String**> |  | [optional]
**remote_addr** | Option<**String**> |  | [optional]
**remote_port** | Option<**i32**> |  | [optional]
**timestamp** | Option<**String**> |  | [optional]
**actor_email** | Option<**String**> |  | [optional]
**actor** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


