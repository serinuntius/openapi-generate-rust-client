# \PetsApi

All URIs are relative to *http://petstore.swagger.io/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_pets**](PetsApi.md#create_pets) | **POST** /pets | Create a pet
[**list_pets**](PetsApi.md#list_pets) | **GET** /pets | List all pets
[**show_pet_by_id**](PetsApi.md#show_pet_by_id) | **GET** /pets/{petId} | Info for a specific pet



## create_pets

> create_pets()
Create a pet

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_pets

> Vec<crate::models::Pet> list_pets(limit)
List all pets

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | How many items to return at one time (max 100) |  |

### Return type

[**Vec<crate::models::Pet>**](Pet.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## show_pet_by_id

> crate::models::Pet show_pet_by_id(pet_id)
Info for a specific pet

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pet_id** | **String** | The id of the pet to retrieve | [required] |

### Return type

[**crate::models::Pet**](Pet.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

