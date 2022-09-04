# GetContractsPublicRegionId200Ok

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**buyout** | Option<**f64**> | Buyout price (for Auctions only) | [optional]
**collateral** | Option<**f64**> | Collateral price (for Couriers only) | [optional]
**contract_id** | **i32** | contract_id integer | 
**date_expired** | **String** | Expiration date of the contract | 
**date_issued** | **String** | Сreation date of the contract | 
**days_to_complete** | Option<**i32**> | Number of days to perform the contract | [optional]
**end_location_id** | Option<**i64**> | End location ID (for Couriers contract) | [optional]
**for_corporation** | Option<**bool**> | true if the contract was issued on behalf of the issuer's corporation | [optional]
**issuer_corporation_id** | **i32** | Character's corporation ID for the issuer | 
**issuer_id** | **i32** | Character ID for the issuer | 
**price** | Option<**f64**> | Price of contract (for ItemsExchange and Auctions) | [optional]
**reward** | Option<**f64**> | Remuneration for contract (for Couriers only) | [optional]
**start_location_id** | Option<**i64**> | Start location ID (for Couriers contract) | [optional]
**title** | Option<**String**> | Title of the contract | [optional]
**_type** | **String** | Type of the contract | 
**volume** | Option<**f64**> | Volume of items in the contract | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


