# Rust API wrapper for Blind Insight openapi

An end-to-end encrypted datastore, now in Rust! Built using [Blind Insight's OpenAPI Spec](https://docs.blindinsight.io/api-reference/).

## Overview

The initial code for this API client was generated using the [OpenAPI Generator](https://openapi-generator.tech) project.

- API version: 10.6.2
- Package version: 10.6.2
- Generator version: 7.13.0
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation


## Documentation for API Endpoints

All URIs are relative to *http://localhost*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*AccountsApi* | [**accounts_change_password**](docs/AccountsApi.md#accounts_change_password) | **POST** /api/accounts/change-password/ | Change user password
*AccountsApi* | [**accounts_login**](docs/AccountsApi.md#accounts_login) | **POST** /api/accounts/login/ | Login a user account
*AccountsApi* | [**accounts_logout**](docs/AccountsApi.md#accounts_logout) | **POST** /api/accounts/logout/ | Logout a user account
*AccountsApi* | [**accounts_profile_create**](docs/AccountsApi.md#accounts_profile_create) | **POST** /api/accounts/profile/ | Create user profile
*AccountsApi* | [**accounts_profile_partial_update**](docs/AccountsApi.md#accounts_profile_partial_update) | **PATCH** /api/accounts/profile/ | Partially update user profile
*AccountsApi* | [**accounts_profile_retrieve**](docs/AccountsApi.md#accounts_profile_retrieve) | **GET** /api/accounts/profile/ | Get user profile
*AccountsApi* | [**accounts_profile_update**](docs/AccountsApi.md#accounts_profile_update) | **PUT** /api/accounts/profile/ | Update user profile
*AccountsApi* | [**accounts_register**](docs/AccountsApi.md#accounts_register) | **POST** /api/accounts/register/ | Register a new user account
*AccountsApi* | [**accounts_register_email**](docs/AccountsApi.md#accounts_register_email) | **POST** /api/accounts/register-email/ | Register user email
*AccountsApi* | [**accounts_reset_password**](docs/AccountsApi.md#accounts_reset_password) | **POST** /api/accounts/reset-password/ | Reset user password
*AccountsApi* | [**accounts_send_reset_password_link**](docs/AccountsApi.md#accounts_send_reset_password_link) | **POST** /api/accounts/send-reset-password-link/ | Send reset password link
*AccountsApi* | [**accounts_verify_email**](docs/AccountsApi.md#accounts_verify_email) | **POST** /api/accounts/verify-email/ | Verify user email
*AccountsApi* | [**accounts_verify_registration**](docs/AccountsApi.md#accounts_verify_registration) | **POST** /api/accounts/verify-registration/ | Verify a new user account
*CsrfApi* | [**csrf_retrieve**](docs/CsrfApi.md#csrf_retrieve) | **GET** /api/csrf/ | Retrieve a CSRF token
*DatasetsApi* | [**datasets_create**](docs/DatasetsApi.md#datasets_create) | **POST** /api/datasets/ | Create a new Dataset
*DatasetsApi* | [**datasets_destroy**](docs/DatasetsApi.md#datasets_destroy) | **DELETE** /api/datasets/{id}/ | Delete a Dataset
*DatasetsApi* | [**datasets_list**](docs/DatasetsApi.md#datasets_list) | **GET** /api/datasets/ | Retrieve a list of Dataset objects, with filters
*DatasetsApi* | [**datasets_partial_update**](docs/DatasetsApi.md#datasets_partial_update) | **PATCH** /api/datasets/{id}/ | Partially update a Dataset
*DatasetsApi* | [**datasets_retrieve**](docs/DatasetsApi.md#datasets_retrieve) | **GET** /api/datasets/{id}/ | Retrieve one Dataset object
*DatasetsApi* | [**datasets_schema**](docs/DatasetsApi.md#datasets_schema) | **GET** /api/datasets/{id}/schema/{slug}/ | Retrieve a dataset schema by slug
*DatasetsApi* | [**datasets_update**](docs/DatasetsApi.md#datasets_update) | **PUT** /api/datasets/{id}/ | Update a Dataset
*FilesApi* | [**files_create**](docs/FilesApi.md#files_create) | **POST** /api/files/ | Create a new upload resource (only available via proxy)
*FilesApi* | [**files_delete**](docs/FilesApi.md#files_delete) | **DELETE** /api/files/{id}/ | Delete upload (only available via proxy)
*FilesApi* | [**files_patch**](docs/FilesApi.md#files_patch) | **PATCH** /api/files/{id}/ | Upload file chunk (only available via proxy)
*GrantsApi* | [**grants_create**](docs/GrantsApi.md#grants_create) | **POST** /api/grants/ | Create a new grant
*GrantsApi* | [**grants_destroy**](docs/GrantsApi.md#grants_destroy) | **DELETE** /api/grants/{id}/ | Delete a grant
*GrantsApi* | [**grants_list**](docs/GrantsApi.md#grants_list) | **GET** /api/grants/ | List all grants
*GrantsApi* | [**grants_partial_update**](docs/GrantsApi.md#grants_partial_update) | **PATCH** /api/grants/{id}/ | Partially update a grant
*GrantsApi* | [**grants_retrieve**](docs/GrantsApi.md#grants_retrieve) | **GET** /api/grants/{id}/ | Retrieve a single grant
*GrantsApi* | [**grants_update**](docs/GrantsApi.md#grants_update) | **PUT** /api/grants/{id}/ | Update a grant
*IdentitiesApi* | [**identities_create**](docs/IdentitiesApi.md#identities_create) | **POST** /api/identities/ | Create a new Identity
*IdentitiesApi* | [**identities_destroy**](docs/IdentitiesApi.md#identities_destroy) | **DELETE** /api/identities/{id}/ | Delete an Identity
*IdentitiesApi* | [**identities_list**](docs/IdentitiesApi.md#identities_list) | **GET** /api/identities/ | List Identities
*IdentitiesApi* | [**identities_partial_update**](docs/IdentitiesApi.md#identities_partial_update) | **PATCH** /api/identities/{id}/ | Partially update an Identity
*IdentitiesApi* | [**identities_retrieve**](docs/IdentitiesApi.md#identities_retrieve) | **GET** /api/identities/{id}/ | Retrieve a single Identity
*IdentitiesApi* | [**identities_update**](docs/IdentitiesApi.md#identities_update) | **PUT** /api/identities/{id}/ | Update an Identity
*JobsApi* | [**jobs_status**](docs/JobsApi.md#jobs_status) | **GET** /api/jobs/{job_id}/ | Get the status of a job (only available via proxy)
*JobsApi* | [**jobs_upload**](docs/JobsApi.md#jobs_upload) | **POST** /api/jobs/upload/ | Batch upload new records (only available via proxy)
*MaterialsApi* | [**materials_create**](docs/MaterialsApi.md#materials_create) | **POST** /api/materials/ | Create a new material
*MaterialsApi* | [**materials_destroy**](docs/MaterialsApi.md#materials_destroy) | **DELETE** /api/materials/{id}/ | Delete a material
*MaterialsApi* | [**materials_list**](docs/MaterialsApi.md#materials_list) | **GET** /api/materials/ | List all materials
*MaterialsApi* | [**materials_overrides**](docs/MaterialsApi.md#materials_overrides) | **GET** /api/materials/overrides/ | List all material overrides
*MaterialsApi* | [**materials_overrides_delete_detail**](docs/MaterialsApi.md#materials_overrides_delete_detail) | **DELETE** /api/materials/overrides/{key}/ | Delete a single material override
*MaterialsApi* | [**materials_overrides_delete_list**](docs/MaterialsApi.md#materials_overrides_delete_list) | **DELETE** /api/materials/overrides/ | Delete all material overrides
*MaterialsApi* | [**materials_proof_create**](docs/MaterialsApi.md#materials_proof_create) | **POST** /api/materials/{id}/proof/ | Validate a proof for a material
*MaterialsApi* | [**materials_receive_detail**](docs/MaterialsApi.md#materials_receive_detail) | **POST** /api/materials/{material_id}/receive/ | Receive a single material
*MaterialsApi* | [**materials_receive_list**](docs/MaterialsApi.md#materials_receive_list) | **POST** /api/materials/receive/ | Receive all materials
*MaterialsApi* | [**materials_retrieve**](docs/MaterialsApi.md#materials_retrieve) | **GET** /api/materials/{id}/ | Retrieve a single material
*OrganizationsApi* | [**organizations_audit_logs_list**](docs/OrganizationsApi.md#organizations_audit_logs_list) | **GET** /api/organizations/{organization_id}/audit-logs/ | List organization audit logs
*OrganizationsApi* | [**organizations_audit_logs_retrieve**](docs/OrganizationsApi.md#organizations_audit_logs_retrieve) | **GET** /api/organizations/{organization_id}/audit-logs/{id}/ | Retrieve organization audit log
*OrganizationsApi* | [**organizations_by_slug**](docs/OrganizationsApi.md#organizations_by_slug) | **GET** /api/organizations/by-slug/{slug}/ | Retrieve organization by slug
*OrganizationsApi* | [**organizations_create**](docs/OrganizationsApi.md#organizations_create) | **POST** /api/organizations/ | Create organization
*OrganizationsApi* | [**organizations_dataset**](docs/OrganizationsApi.md#organizations_dataset) | **GET** /api/organizations/{id}/dataset/{slug}/ | Retrieve organization dataset by slug
*OrganizationsApi* | [**organizations_destroy**](docs/OrganizationsApi.md#organizations_destroy) | **DELETE** /api/organizations/{id}/ | Delete organization
*OrganizationsApi* | [**organizations_invitations_create**](docs/OrganizationsApi.md#organizations_invitations_create) | **POST** /api/organizations/{organization_id}/invitations/ | Create organization invitation
*OrganizationsApi* | [**organizations_invitations_destroy**](docs/OrganizationsApi.md#organizations_invitations_destroy) | **DELETE** /api/organizations/{organization_id}/invitations/{id}/ | Delete organization invitation
*OrganizationsApi* | [**organizations_invitations_list**](docs/OrganizationsApi.md#organizations_invitations_list) | **GET** /api/organizations/{organization_id}/invitations/ | List organization invitations
*OrganizationsApi* | [**organizations_invitations_partial_update**](docs/OrganizationsApi.md#organizations_invitations_partial_update) | **PATCH** /api/organizations/{organization_id}/invitations/{id}/ | Partial update organization invitation
*OrganizationsApi* | [**organizations_invitations_retrieve**](docs/OrganizationsApi.md#organizations_invitations_retrieve) | **GET** /api/organizations/{organization_id}/invitations/{id}/ | Retrieve organization invitation
*OrganizationsApi* | [**organizations_invitations_update**](docs/OrganizationsApi.md#organizations_invitations_update) | **PUT** /api/organizations/{organization_id}/invitations/{id}/ | Update organization invitation
*OrganizationsApi* | [**organizations_list**](docs/OrganizationsApi.md#organizations_list) | **GET** /api/organizations/ | List organizations
*OrganizationsApi* | [**organizations_owner**](docs/OrganizationsApi.md#organizations_owner) | **GET** /api/organizations/{id}/owner/ | Retrieve organization owner
*OrganizationsApi* | [**organizations_partial_update**](docs/OrganizationsApi.md#organizations_partial_update) | **PATCH** /api/organizations/{id}/ | Partial update organization
*OrganizationsApi* | [**organizations_retrieve**](docs/OrganizationsApi.md#organizations_retrieve) | **GET** /api/organizations/{id}/ | Retrieve organization
*OrganizationsApi* | [**organizations_teams_add_member**](docs/OrganizationsApi.md#organizations_teams_add_member) | **POST** /api/organizations/{organization_id}/teams/{id}/add_member/ | Add a member to a team
*OrganizationsApi* | [**organizations_teams_create**](docs/OrganizationsApi.md#organizations_teams_create) | **POST** /api/organizations/{organization_id}/teams/ | Create a new team
*OrganizationsApi* | [**organizations_teams_destroy**](docs/OrganizationsApi.md#organizations_teams_destroy) | **DELETE** /api/organizations/{organization_id}/teams/{id}/ | Delete a team
*OrganizationsApi* | [**organizations_teams_list**](docs/OrganizationsApi.md#organizations_teams_list) | **GET** /api/organizations/{organization_id}/teams/ | List all teams
*OrganizationsApi* | [**organizations_teams_partial_update**](docs/OrganizationsApi.md#organizations_teams_partial_update) | **PATCH** /api/organizations/{organization_id}/teams/{id}/ | Partially update a team
*OrganizationsApi* | [**organizations_teams_remove_member**](docs/OrganizationsApi.md#organizations_teams_remove_member) | **POST** /api/organizations/{organization_id}/teams/{id}/remove_member/ | Remove a member from a team
*OrganizationsApi* | [**organizations_teams_retrieve**](docs/OrganizationsApi.md#organizations_teams_retrieve) | **GET** /api/organizations/{organization_id}/teams/{id}/ | Retrieve a single team
*OrganizationsApi* | [**organizations_teams_update**](docs/OrganizationsApi.md#organizations_teams_update) | **PUT** /api/organizations/{organization_id}/teams/{id}/ | Update a team
*OrganizationsApi* | [**organizations_update**](docs/OrganizationsApi.md#organizations_update) | **PUT** /api/organizations/{id}/ | Update organization
*OrganizationsApi* | [**organizations_users_create**](docs/OrganizationsApi.md#organizations_users_create) | **POST** /api/organizations/{organization_id}/users/ | Create organization user
*OrganizationsApi* | [**organizations_users_destroy**](docs/OrganizationsApi.md#organizations_users_destroy) | **DELETE** /api/organizations/{organization_id}/users/{id}/ | Delete organization user
*OrganizationsApi* | [**organizations_users_list**](docs/OrganizationsApi.md#organizations_users_list) | **GET** /api/organizations/{organization_id}/users/ | List organization users
*OrganizationsApi* | [**organizations_users_partial_update**](docs/OrganizationsApi.md#organizations_users_partial_update) | **PATCH** /api/organizations/{organization_id}/users/{id}/ | Partial update organization user
*OrganizationsApi* | [**organizations_users_retrieve**](docs/OrganizationsApi.md#organizations_users_retrieve) | **GET** /api/organizations/{organization_id}/users/{id}/ | Retrieve organization user
*OrganizationsApi* | [**organizations_users_update**](docs/OrganizationsApi.md#organizations_users_update) | **PUT** /api/organizations/{organization_id}/users/{id}/ | Update organization user
*RecordsApi* | [**records_bulk_destroy**](docs/RecordsApi.md#records_bulk_destroy) | **DELETE** /api/records/ | Bulk delete records
*RecordsApi* | [**records_create**](docs/RecordsApi.md#records_create) | **POST** /api/records/ | Create a new record
*RecordsApi* | [**records_decrypt**](docs/RecordsApi.md#records_decrypt) | **POST** /api/records/decrypt/ | Decrypt data and return records (only available via proxy)
*RecordsApi* | [**records_destroy**](docs/RecordsApi.md#records_destroy) | **DELETE** /api/records/{id}/ | Delete a record
*RecordsApi* | [**records_ingest**](docs/RecordsApi.md#records_ingest) | **POST** /api/records/ingest/ | Ingest data and encrypt it (only available via proxy)
*RecordsApi* | [**records_list**](docs/RecordsApi.md#records_list) | **GET** /api/records/ | List records
*RecordsApi* | [**records_retrieve**](docs/RecordsApi.md#records_retrieve) | **GET** /api/records/{id}/ | Retrieve a record
*RecordsApi* | [**records_search**](docs/RecordsApi.md#records_search) | **POST** /api/records/search/ | Search for records (only available via proxy)
*RequestsApi* | [**requests_create**](docs/RequestsApi.md#requests_create) | **POST** /api/requests/ | Create a new request
*RequestsApi* | [**requests_destroy**](docs/RequestsApi.md#requests_destroy) | **DELETE** /api/requests/{id}/ | Delete a request
*RequestsApi* | [**requests_fulfill_detail**](docs/RequestsApi.md#requests_fulfill_detail) | **POST** /api/requests/{request_id}/fulfill/ | Fulfill a single request
*RequestsApi* | [**requests_fulfill_list**](docs/RequestsApi.md#requests_fulfill_list) | **POST** /api/requests/fulfill/ | Fulfill all requests
*RequestsApi* | [**requests_list**](docs/RequestsApi.md#requests_list) | **GET** /api/requests/ | List all requests
*RequestsApi* | [**requests_partial_update**](docs/RequestsApi.md#requests_partial_update) | **PATCH** /api/requests/{id}/ | Partially update a request
*RequestsApi* | [**requests_retrieve**](docs/RequestsApi.md#requests_retrieve) | **GET** /api/requests/{id}/ | Retrieve a single request
*RequestsApi* | [**requests_update**](docs/RequestsApi.md#requests_update) | **PUT** /api/requests/{id}/ | Update a request
*SchemasApi* | [**schemas_create**](docs/SchemasApi.md#schemas_create) | **POST** /api/schemas/ | Create a new schema
*SchemasApi* | [**schemas_decrypt**](docs/SchemasApi.md#schemas_decrypt) | **POST** /api/schemas/decrypt/ | Decrypt a schema (only available via proxy)
*SchemasApi* | [**schemas_destroy**](docs/SchemasApi.md#schemas_destroy) | **DELETE** /api/schemas/{id}/ | Delete a schema
*SchemasApi* | [**schemas_list**](docs/SchemasApi.md#schemas_list) | **GET** /api/schemas/ | List all schemas
*SchemasApi* | [**schemas_partial_update**](docs/SchemasApi.md#schemas_partial_update) | **PATCH** /api/schemas/{id}/ | Partially update a schema
*SchemasApi* | [**schemas_retrieve**](docs/SchemasApi.md#schemas_retrieve) | **GET** /api/schemas/{id}/ | Retrieve a single schema
*SchemasApi* | [**schemas_teams**](docs/SchemasApi.md#schemas_teams) | **GET** /api/schemas/{id}/teams/ | List the teams for a schema
*SchemasApi* | [**schemas_update**](docs/SchemasApi.md#schemas_update) | **PUT** /api/schemas/{id}/ | Update a schema
*StatusApi* | [**status_retrieve**](docs/StatusApi.md#status_retrieve) | **GET** /api/status/ | Retrieve the status of the API
*TokenApi* | [**token_create**](docs/TokenApi.md#token_create) | **POST** /api/token/ | Create a new token
*TokenApi* | [**token_refresh**](docs/TokenApi.md#token_refresh) | **POST** /api/token/refresh/ | Refresh a token
*TokenApi* | [**token_verify**](docs/TokenApi.md#token_verify) | **POST** /api/token/verify/ | Verify a token
*UsersApi* | [**users_by_name**](docs/UsersApi.md#users_by_name) | **GET** /api/users/by_name/{email}/ | Retrieve one User object by name
*UsersApi* | [**users_create**](docs/UsersApi.md#users_create) | **POST** /api/users/ | Create a new User
*UsersApi* | [**users_destroy**](docs/UsersApi.md#users_destroy) | **DELETE** /api/users/{id}/ | Delete a User
*UsersApi* | [**users_list**](docs/UsersApi.md#users_list) | **GET** /api/users/ | Retrieve one or more User objects, with filters
*UsersApi* | [**users_partial_update**](docs/UsersApi.md#users_partial_update) | **PATCH** /api/users/{id}/ | Partially update a User
*UsersApi* | [**users_retrieve**](docs/UsersApi.md#users_retrieve) | **GET** /api/users/{id}/ | Retrieve one User object
*UsersApi* | [**users_self**](docs/UsersApi.md#users_self) | **GET** /api/users/self/ | Retrieve current user
*UsersApi* | [**users_update**](docs/UsersApi.md#users_update) | **PUT** /api/users/{id}/ | Update a User
*WsApi* | [**jobs_websocket**](docs/WsApi.md#jobs_websocket) | **GET** /api/ws/jobs/{job_id}/ | Connect to a job status websocket (only available via proxy)


## Documentation For Models

 - [AccountVerified](docs/AccountVerified.md)
 - [AccountVerifiedError](docs/AccountVerifiedError.md)
 - [ActionEnum](docs/ActionEnum.md)
 - [AuditLogEntry](docs/AuditLogEntry.md)
 - [ChangePassword](docs/ChangePassword.md)
 - [CsrfRetrieve200Response](docs/CsrfRetrieve200Response.md)
 - [Dataset](docs/Dataset.md)
 - [DefaultLogin](docs/DefaultLogin.md)
 - [DefaultRegisterEmail](docs/DefaultRegisterEmail.md)
 - [DefaultSendResetPasswordLink](docs/DefaultSendResetPasswordLink.md)
 - [DefaultUserProfile](docs/DefaultUserProfile.md)
 - [Grant](docs/Grant.md)
 - [Identity](docs/Identity.md)
 - [IndexedRecord](docs/IndexedRecord.md)
 - [IndexedRecordHashedLabelMapValue](docs/IndexedRecordHashedLabelMapValue.md)
 - [IndexedRecordHashedLabelMapValueFieldSchema](docs/IndexedRecordHashedLabelMapValueFieldSchema.md)
 - [JobsStatus200Response](docs/JobsStatus200Response.md)
 - [JobsStatus404Response](docs/JobsStatus404Response.md)
 - [JobsUpload200Response](docs/JobsUpload200Response.md)
 - [JobsUploadRequestInner](docs/JobsUploadRequestInner.md)
 - [JobsUploadRequestInnerDataValue](docs/JobsUploadRequestInnerDataValue.md)
 - [LoginSuccessful](docs/LoginSuccessful.md)
 - [LoginSuccessfulError](docs/LoginSuccessfulError.md)
 - [Logout](docs/Logout.md)
 - [LogoutSuccessful](docs/LogoutSuccessful.md)
 - [LogoutSuccessfulError](docs/LogoutSuccessfulError.md)
 - [Material](docs/Material.md)
 - [MaterialProof](docs/MaterialProof.md)
 - [MaterialsReceiveDetail200Response](docs/MaterialsReceiveDetail200Response.md)
 - [Organization](docs/Organization.md)
 - [OrganizationCreate](docs/OrganizationCreate.md)
 - [OrganizationInvitation](docs/OrganizationInvitation.md)
 - [OrganizationInvitationCreate](docs/OrganizationInvitationCreate.md)
 - [OrganizationRegisterUser](docs/OrganizationRegisterUser.md)
 - [OrganizationUpdate](docs/OrganizationUpdate.md)
 - [OrganizationUser](docs/OrganizationUser.md)
 - [OrganizationUserCreate](docs/OrganizationUserCreate.md)
 - [OrganizationUserProfile](docs/OrganizationUserProfile.md)
 - [OrganizationUserUpdate](docs/OrganizationUserUpdate.md)
 - [OrganizationsTeamsAddMemberRequest](docs/OrganizationsTeamsAddMemberRequest.md)
 - [PasswordChanged](docs/PasswordChanged.md)
 - [PasswordChangedError](docs/PasswordChangedError.md)
 - [PasswordReset](docs/PasswordReset.md)
 - [PasswordResetError](docs/PasswordResetError.md)
 - [PatchedDataset](docs/PatchedDataset.md)
 - [PatchedDefaultUserProfile](docs/PatchedDefaultUserProfile.md)
 - [PatchedGrant](docs/PatchedGrant.md)
 - [PatchedIdentity](docs/PatchedIdentity.md)
 - [PatchedOrganizationInvitation](docs/PatchedOrganizationInvitation.md)
 - [PatchedOrganizationUpdate](docs/PatchedOrganizationUpdate.md)
 - [PatchedOrganizationUserUpdate](docs/PatchedOrganizationUserUpdate.md)
 - [PatchedRequest](docs/PatchedRequest.md)
 - [PatchedSchema](docs/PatchedSchema.md)
 - [PatchedTeam](docs/PatchedTeam.md)
 - [PatchedUser](docs/PatchedUser.md)
 - [Record](docs/Record.md)
 - [RecordsSearchRequest](docs/RecordsSearchRequest.md)
 - [RecordsSearchRequestFiltersInner](docs/RecordsSearchRequestFiltersInner.md)
 - [RegisteredEmail](docs/RegisteredEmail.md)
 - [RegisteredEmailError](docs/RegisteredEmailError.md)
 - [Request](docs/Request.md)
 - [ResetPassword](docs/ResetPassword.md)
 - [ResetPasswordLinkSent](docs/ResetPasswordLinkSent.md)
 - [ResetPasswordLinkSentError](docs/ResetPasswordLinkSentError.md)
 - [Schema](docs/Schema.md)
 - [SchemasDecryptRequest](docs/SchemasDecryptRequest.md)
 - [StatusRetrieve200Response](docs/StatusRetrieve200Response.md)
 - [Team](docs/Team.md)
 - [TokenObtainPair](docs/TokenObtainPair.md)
 - [TokenRefresh](docs/TokenRefresh.md)
 - [TokenVerify](docs/TokenVerify.md)
 - [User](docs/User.md)
 - [VerifiedEmail](docs/VerifiedEmail.md)
 - [VerifiedEmailError](docs/VerifiedEmailError.md)
 - [VerifyEmail](docs/VerifyEmail.md)
 - [VerifyRegistration](docs/VerifyRegistration.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author

Elijah Grubb
Cryptography Engineer
ElijahLGrubb@gmail.com

