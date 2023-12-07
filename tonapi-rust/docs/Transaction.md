# Transaction

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**hash** | **String** |  | 
**lt** | **i64** |  | 
**account** | [**crate::models::AccountAddress**](AccountAddress.md) |  | 
**success** | **bool** |  | 
**utime** | **i64** |  | 
**orig_status** | [**crate::models::AccountStatus**](AccountStatus.md) |  | 
**end_status** | [**crate::models::AccountStatus**](AccountStatus.md) |  | 
**total_fees** | **i64** |  | 
**transaction_type** | [**crate::models::TransactionType**](TransactionType.md) |  | 
**state_update_old** | **String** |  | 
**state_update_new** | **String** |  | 
**in_msg** | Option<[**crate::models::Message**](Message.md)> |  | [optional]
**out_msgs** | [**Vec<crate::models::Message>**](Message.md) |  | 
**block** | **String** |  | 
**prev_trans_hash** | Option<**String**> |  | [optional]
**prev_trans_lt** | Option<**i64**> |  | [optional]
**compute_phase** | Option<[**crate::models::ComputePhase**](ComputePhase.md)> |  | [optional]
**storage_phase** | Option<[**crate::models::StoragePhase**](StoragePhase.md)> |  | [optional]
**credit_phase** | Option<[**crate::models::CreditPhase**](CreditPhase.md)> |  | [optional]
**action_phase** | Option<[**crate::models::ActionPhase**](ActionPhase.md)> |  | [optional]
**bounce_phase** | Option<[**crate::models::BouncePhaseType**](BouncePhaseType.md)> |  | [optional]
**aborted** | **bool** |  | 
**destroyed** | **bool** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


