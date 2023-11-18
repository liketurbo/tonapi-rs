# PoolInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**address** | **String** |  | 
**name** | **String** |  | 
**total_amount** | **i64** |  | 
**implementation** | [**crate::models::PoolImplementationType**](PoolImplementationType.md) |  | 
**apy** | **f32** | APY in percent | 
**min_stake** | **i64** |  | 
**cycle_start** | **i64** | current nomination cycle beginning timestamp | 
**cycle_end** | **i64** | current nomination cycle ending timestamp | 
**verified** | **bool** | this pool has verified source code or managed by trusted company | 
**current_nominators** | **i32** | current number of nominators | 
**max_nominators** | **i32** | maximum number of nominators | 
**liquid_jetton_master** | Option<**String**> | for liquid staking master account of jetton | [optional]
**nominators_stake** | **i64** | total stake of all nominators | 
**validator_stake** | **i64** | stake of validator | 
**cycle_length** | Option<**i64**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


