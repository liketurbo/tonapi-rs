# Message

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**msg_type** | **String** |  | 
**created_lt** | **i64** |  | 
**ihr_disabled** | **bool** |  | 
**bounce** | **bool** |  | 
**bounced** | **bool** |  | 
**value** | **i64** |  | 
**fwd_fee** | **i64** |  | 
**ihr_fee** | **i64** |  | 
**destination** | Option<[**crate::models::AccountAddress**](AccountAddress.md)> |  | [optional]
**source** | Option<[**crate::models::AccountAddress**](AccountAddress.md)> |  | [optional]
**import_fee** | **i64** |  | 
**created_at** | **i64** |  | 
**op_code** | Option<**String**> |  | [optional]
**init** | Option<[**crate::models::StateInit**](StateInit.md)> |  | [optional]
**raw_body** | Option<**String**> | hex-encoded BoC with raw message body | [optional]
**decoded_op_name** | Option<**String**> |  | [optional]
**decoded_body** | Option<[**serde_json::Value**](.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


