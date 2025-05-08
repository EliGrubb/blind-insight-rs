# \AccountsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**accounts_change_password**](AccountsApi.md#accounts_change_password) | **POST** /api/accounts/change-password/ | Change user password
[**accounts_login**](AccountsApi.md#accounts_login) | **POST** /api/accounts/login/ | Login a user account
[**accounts_logout**](AccountsApi.md#accounts_logout) | **POST** /api/accounts/logout/ | Logout a user account
[**accounts_profile_create**](AccountsApi.md#accounts_profile_create) | **POST** /api/accounts/profile/ | Create user profile
[**accounts_profile_partial_update**](AccountsApi.md#accounts_profile_partial_update) | **PATCH** /api/accounts/profile/ | Partially update user profile
[**accounts_profile_retrieve**](AccountsApi.md#accounts_profile_retrieve) | **GET** /api/accounts/profile/ | Get user profile
[**accounts_profile_update**](AccountsApi.md#accounts_profile_update) | **PUT** /api/accounts/profile/ | Update user profile
[**accounts_register**](AccountsApi.md#accounts_register) | **POST** /api/accounts/register/ | Register a new user account
[**accounts_register_email**](AccountsApi.md#accounts_register_email) | **POST** /api/accounts/register-email/ | Register user email
[**accounts_reset_password**](AccountsApi.md#accounts_reset_password) | **POST** /api/accounts/reset-password/ | Reset user password
[**accounts_send_reset_password_link**](AccountsApi.md#accounts_send_reset_password_link) | **POST** /api/accounts/send-reset-password-link/ | Send reset password link
[**accounts_verify_email**](AccountsApi.md#accounts_verify_email) | **POST** /api/accounts/verify-email/ | Verify user email
[**accounts_verify_registration**](AccountsApi.md#accounts_verify_registration) | **POST** /api/accounts/verify-registration/ | Verify a new user account



## accounts_change_password

> models::PasswordChanged accounts_change_password(change_password)
Change user password

Change the password of the currently authenticated user. The `old_password` field must match the currently set password. The `password` and `password_confirm` fields must match.  The response will be a detail message indicating if the password change was successful.  **On the command line:**  ```bash blind accounts change-password --data password-change.json ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**change_password** | [**ChangePassword**](ChangePassword.md) |  | [required] |

### Return type

[**models::PasswordChanged**](PasswordChanged.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## accounts_login

> models::LoginSuccessful accounts_login(default_login)
Login a user account

Log in a user using their email and password. On success, returns a JSON web token which should be included in the `Authorization` header for all subsequent API requests in client implementations.  The response will be a detail message indicating if the login was successful.  **On the command line:**  ```bash blind accounts login --data login.json ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**default_login** | [**DefaultLogin**](DefaultLogin.md) |  | [required] |

### Return type

[**models::LoginSuccessful**](LoginSuccessful.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## accounts_logout

> models::LogoutSuccessful accounts_logout(logout)
Logout a user account

Log out the currently authenticated user. This simply invalidates the JSON web token used.  This is separate from the porcelain `blind logout` command which removes stored credentials on the local machine.  The response will be a detail message indicating if the logout was successful.  **On the command line:**  ```bash blind accounts logout ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**logout** | Option<[**Logout**](Logout.md)> |  |  |

### Return type

[**models::LogoutSuccessful**](LogoutSuccessful.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## accounts_profile_create

> models::DefaultUserProfile accounts_profile_create(default_user_profile)
Create user profile

Create a profile for the currently authenticated user. This is useful if you don't have a profile yet, or if you want to edit an existing profile.  The response will be the newly created profile object.  **On the command line:**  ```bash blind accounts profile create --data profile.json ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**default_user_profile** | Option<[**DefaultUserProfile**](DefaultUserProfile.md)> |  |  |

### Return type

[**models::DefaultUserProfile**](DefaultUserProfile.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## accounts_profile_partial_update

> models::DefaultUserProfile accounts_profile_partial_update(patched_default_user_profile)
Partially update user profile

Update one or more fields of the currently authenticated user's profile. This does not replace the entire profile, it only updates the fields provided in the request.  The response will be the updated profile object.  **On the command line:**  ```bash blind accounts profile partial-update --data profile-update.json ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**patched_default_user_profile** | Option<[**PatchedDefaultUserProfile**](PatchedDefaultUserProfile.md)> |  |  |

### Return type

[**models::DefaultUserProfile**](DefaultUserProfile.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## accounts_profile_retrieve

> models::DefaultUserProfile accounts_profile_retrieve()
Get user profile

Retrieve the profile of the currently authenticated user.  The response will be the profile object.  **On the command line:**  ```bash blind accounts profile retrieve ```

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::DefaultUserProfile**](DefaultUserProfile.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## accounts_profile_update

> models::DefaultUserProfile accounts_profile_update(default_user_profile)
Update user profile

Replace the currently authenticated user's profile with the data provided in the request. Fields not specified in the request will be dropped.  The response will be the updated profile object.  **On the command line:**  ```bash blind accounts profile update --data profile.json ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**default_user_profile** | Option<[**DefaultUserProfile**](DefaultUserProfile.md)> |  |  |

### Return type

[**models::DefaultUserProfile**](DefaultUserProfile.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## accounts_register

> models::OrganizationUserProfile accounts_register(organization_register_user)
Register a new user account

This endpoint registers a new user. The user will receive an email with a link to activate their account.  This will also create a new Organization for the user. The user will be the owner of the organization.  See the [user registration](https://docs.blindinsight.io/) documentation for more information.  The response will be the newly created user object.  **On the command line:**  You can supply record data on standard input with the `--data -` or write it to a file and give the `--data <filename>` flag.  ```bash blind accounts register --data '{   \"email\": \"demo@example.com\",   \"first_name\": \"Demo\",   \"last_name\": \"User\",   \"organization_name\": \"Demo\",   \"password\": \"pass1234\",   \"password_confirm\": \"pass1234\" }' ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_register_user** | [**OrganizationRegisterUser**](OrganizationRegisterUser.md) |  | [required] |

### Return type

[**models::OrganizationUserProfile**](OrganizationUserProfile.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## accounts_register_email

> models::RegisteredEmail accounts_register_email(default_register_email)
Register user email

Register a new email address for the currently authenticated user. This is useful if the user wants to change email addresses associated with their account.  The new email must be verified before it is associated with the account.  The response will be a detail message indicating if the email was registered.  **On the command line:**  ```bash blind accounts register-email --data '{\"email\": \"demo-example@example.com\"}' ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**default_register_email** | [**DefaultRegisterEmail**](DefaultRegisterEmail.md) |  | [required] |

### Return type

[**models::RegisteredEmail**](RegisteredEmail.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## accounts_reset_password

> models::PasswordReset accounts_reset_password(reset_password)
Reset user password

Reset the password of the given user using the provided signature and timestamp.  This endpoint is generally only used by an administrator to reset a user's password.  The response will be a detail message indicating if the password reset was successful.  **On the command line:**  ```bash blind accounts reset-password --data reset-password.json ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**reset_password** | [**ResetPassword**](ResetPassword.md) |  | [required] |

### Return type

[**models::PasswordReset**](PasswordReset.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## accounts_send_reset_password_link

> models::ResetPasswordLinkSent accounts_send_reset_password_link(default_send_reset_password_link)
Send reset password link

Send a password reset link email to the user with the given email address. This endpoint does not require the user to have activated their account yet.  The response will be a detail message indicating if the reset link was sent.  **On the command line:**  ```bash blind accounts send-reset-password-link --data '{\"email\": \"demo@example.com\"}' ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**default_send_reset_password_link** | [**DefaultSendResetPasswordLink**](DefaultSendResetPasswordLink.md) |  | [required] |

### Return type

[**models::ResetPasswordLinkSent**](ResetPasswordLinkSent.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## accounts_verify_email

> models::VerifiedEmail accounts_verify_email(verify_email)
Verify user email

Verify the authenticated user's email address. This is useful if you want to automate onboarding of users within your organization.  The response will be a detail message indicating if the verification was successful.  **On the command line:**  ```bash blind accounts verify-email --data verification.json ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**verify_email** | [**VerifyEmail**](VerifyEmail.md) |  | [required] |

### Return type

[**models::VerifiedEmail**](VerifiedEmail.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## accounts_verify_registration

> models::AccountVerified accounts_verify_registration(verify_registration)
Verify a new user account

This endpoint verifies a new user with their user ID, a timestamp, and a signature. These values must come from the received verification email.  The response will be a detail message indicating if the verification was successful.  **On the command line:**  ```bash blind accounts verify-registration --data registration.json ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**verify_registration** | [**VerifyRegistration**](VerifyRegistration.md) |  | [required] |

### Return type

[**models::AccountVerified**](AccountVerified.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

