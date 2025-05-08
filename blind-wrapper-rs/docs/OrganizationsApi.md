# \OrganizationsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**organizations_audit_logs_list**](OrganizationsApi.md#organizations_audit_logs_list) | **GET** /api/organizations/{organization_id}/audit-logs/ | List organization audit logs
[**organizations_audit_logs_retrieve**](OrganizationsApi.md#organizations_audit_logs_retrieve) | **GET** /api/organizations/{organization_id}/audit-logs/{id}/ | Retrieve organization audit log
[**organizations_by_slug**](OrganizationsApi.md#organizations_by_slug) | **GET** /api/organizations/by-slug/{slug}/ | Retrieve organization by slug
[**organizations_create**](OrganizationsApi.md#organizations_create) | **POST** /api/organizations/ | Create organization
[**organizations_dataset**](OrganizationsApi.md#organizations_dataset) | **GET** /api/organizations/{id}/dataset/{slug}/ | Retrieve organization dataset by slug
[**organizations_destroy**](OrganizationsApi.md#organizations_destroy) | **DELETE** /api/organizations/{id}/ | Delete organization
[**organizations_invitations_create**](OrganizationsApi.md#organizations_invitations_create) | **POST** /api/organizations/{organization_id}/invitations/ | Create organization invitation
[**organizations_invitations_destroy**](OrganizationsApi.md#organizations_invitations_destroy) | **DELETE** /api/organizations/{organization_id}/invitations/{id}/ | Delete organization invitation
[**organizations_invitations_list**](OrganizationsApi.md#organizations_invitations_list) | **GET** /api/organizations/{organization_id}/invitations/ | List organization invitations
[**organizations_invitations_partial_update**](OrganizationsApi.md#organizations_invitations_partial_update) | **PATCH** /api/organizations/{organization_id}/invitations/{id}/ | Partial update organization invitation
[**organizations_invitations_retrieve**](OrganizationsApi.md#organizations_invitations_retrieve) | **GET** /api/organizations/{organization_id}/invitations/{id}/ | Retrieve organization invitation
[**organizations_invitations_update**](OrganizationsApi.md#organizations_invitations_update) | **PUT** /api/organizations/{organization_id}/invitations/{id}/ | Update organization invitation
[**organizations_list**](OrganizationsApi.md#organizations_list) | **GET** /api/organizations/ | List organizations
[**organizations_owner**](OrganizationsApi.md#organizations_owner) | **GET** /api/organizations/{id}/owner/ | Retrieve organization owner
[**organizations_partial_update**](OrganizationsApi.md#organizations_partial_update) | **PATCH** /api/organizations/{id}/ | Partial update organization
[**organizations_retrieve**](OrganizationsApi.md#organizations_retrieve) | **GET** /api/organizations/{id}/ | Retrieve organization
[**organizations_teams_add_member**](OrganizationsApi.md#organizations_teams_add_member) | **POST** /api/organizations/{organization_id}/teams/{id}/add_member/ | Add a member to a team
[**organizations_teams_create**](OrganizationsApi.md#organizations_teams_create) | **POST** /api/organizations/{organization_id}/teams/ | Create a new team
[**organizations_teams_destroy**](OrganizationsApi.md#organizations_teams_destroy) | **DELETE** /api/organizations/{organization_id}/teams/{id}/ | Delete a team
[**organizations_teams_list**](OrganizationsApi.md#organizations_teams_list) | **GET** /api/organizations/{organization_id}/teams/ | List all teams
[**organizations_teams_partial_update**](OrganizationsApi.md#organizations_teams_partial_update) | **PATCH** /api/organizations/{organization_id}/teams/{id}/ | Partially update a team
[**organizations_teams_remove_member**](OrganizationsApi.md#organizations_teams_remove_member) | **POST** /api/organizations/{organization_id}/teams/{id}/remove_member/ | Remove a member from a team
[**organizations_teams_retrieve**](OrganizationsApi.md#organizations_teams_retrieve) | **GET** /api/organizations/{organization_id}/teams/{id}/ | Retrieve a single team
[**organizations_teams_update**](OrganizationsApi.md#organizations_teams_update) | **PUT** /api/organizations/{organization_id}/teams/{id}/ | Update a team
[**organizations_update**](OrganizationsApi.md#organizations_update) | **PUT** /api/organizations/{id}/ | Update organization
[**organizations_users_create**](OrganizationsApi.md#organizations_users_create) | **POST** /api/organizations/{organization_id}/users/ | Create organization user
[**organizations_users_destroy**](OrganizationsApi.md#organizations_users_destroy) | **DELETE** /api/organizations/{organization_id}/users/{id}/ | Delete organization user
[**organizations_users_list**](OrganizationsApi.md#organizations_users_list) | **GET** /api/organizations/{organization_id}/users/ | List organization users
[**organizations_users_partial_update**](OrganizationsApi.md#organizations_users_partial_update) | **PATCH** /api/organizations/{organization_id}/users/{id}/ | Partial update organization user
[**organizations_users_retrieve**](OrganizationsApi.md#organizations_users_retrieve) | **GET** /api/organizations/{organization_id}/users/{id}/ | Retrieve organization user
[**organizations_users_update**](OrganizationsApi.md#organizations_users_update) | **PUT** /api/organizations/{organization_id}/users/{id}/ | Update organization user



## organizations_audit_logs_list

> Vec<models::AuditLogEntry> organizations_audit_logs_list(organization_id, action, actor, actor_email, changes_text, cid, content_type, limit, object_id, object_pk, object_repr, offset, remote_addr, remote_port, timestamp)
List organization audit logs

Return a list of all organization audit logs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **String** |  | [required] |
**action** | Option<**i32**> | * `0` - create * `1` - update * `2` - delete * `3` - access |  |
**actor** | Option<**String**> |  |  |
**actor_email** | Option<**String**> |  |  |
**changes_text** | Option<**String**> |  |  |
**cid** | Option<**String**> |  |  |
**content_type** | Option<**i32**> |  |  |
**limit** | Option<**i32**> | Number of results to return per page. |  |
**object_id** | Option<**i32**> |  |  |
**object_pk** | Option<**String**> |  |  |
**object_repr** | Option<**String**> |  |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**remote_addr** | Option<**String**> |  |  |
**remote_port** | Option<**i32**> |  |  |
**timestamp** | Option<**String**> |  |  |

### Return type

[**Vec<models::AuditLogEntry>**](AuditLogEntry.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## organizations_audit_logs_retrieve

> models::AuditLogEntry organizations_audit_logs_retrieve(id, organization_id)
Retrieve organization audit log

Retrieve a single organization audit log by its ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this log entry. | [required] |
**organization_id** | **String** |  | [required] |

### Return type

[**models::AuditLogEntry**](AuditLogEntry.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## organizations_by_slug

> models::Organization organizations_by_slug(slug)
Retrieve organization by slug

Retrieve a single organization by its slug.  The response will be the organization object.  **On the command line:**  ```bash blind organizations by-slug --slug \"<organization-slug>\" ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

[**models::Organization**](Organization.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## organizations_create

> models::OrganizationCreate organizations_create(organization_create)
Create organization

Create a new organization. The only required field is `name`. The currently authenticated user will be set as the owner of the organization.  The response will be the newly created organization object.  **On the command line:**  ```bash blind organizations create --data '{\"name\": \"Example Organization\"}' ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_create** | [**OrganizationCreate**](OrganizationCreate.md) |  | [required] |

### Return type

[**models::OrganizationCreate**](OrganizationCreate.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## organizations_dataset

> models::Organization organizations_dataset(id, slug)
Retrieve organization dataset by slug

Retrieve a single organization dataset by its slug.  The response is the dataset object.  **On the command line:**  ```bash blind organizations dataset --id \"<organization-id>\" --slug \"<dataset-slug>\" ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | A unique value identifying this organization. | [required] |
**slug** | **String** |  | [required] |

### Return type

[**models::Organization**](Organization.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## organizations_destroy

> organizations_destroy(id)
Delete organization

Delete an organization. Note that this does not delete the organization's members or any associated data.  The response will be a 204 No Content if successful.  **On the command line:**  ```bash blind organizations destroy --id \"<organization-id>\" ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | A unique value identifying this organization. | [required] |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## organizations_invitations_create

> models::OrganizationInvitationCreate organizations_invitations_create(organization_id, organization_invitation_create)
Create organization invitation

Invite a user to join an organization.  The response is the newly created invitation object.  **On the command line:**  ```bash blind organizations invitations create --organization-id \"<organization-id>\" --data organization-invitation.json ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **String** |  | [required] |
**organization_invitation_create** | [**OrganizationInvitationCreate**](OrganizationInvitationCreate.md) |  | [required] |

### Return type

[**models::OrganizationInvitationCreate**](OrganizationInvitationCreate.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## organizations_invitations_destroy

> organizations_invitations_destroy(id, organization_id)
Delete organization invitation

Delete an organization invitation.  The response is a 204 No Content if successful.  **On the command line:**  ```bash blind organizations invitations destroy --organization-id \"<organization-id>\" --id \"<invitation-id>\" ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | A unique value identifying this organization invitation. | [required] |
**organization_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## organizations_invitations_list

> Vec<models::OrganizationInvitation> organizations_invitations_list(organization_id, limit, offset)
List organization invitations

Return a list of all organization invitations, including those which have already been accepted.  The response is a list of invitation objects.  **On the command line:**  ```bash blind organizations invitations list --organization-id \"<organization-id>\" ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **String** |  | [required] |
**limit** | Option<**i32**> | Number of results to return per page. |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |

### Return type

[**Vec<models::OrganizationInvitation>**](OrganizationInvitation.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## organizations_invitations_partial_update

> models::OrganizationInvitation organizations_invitations_partial_update(id, organization_id, patched_organization_invitation)
Partial update organization invitation

Update one or more fields of an organization invitation. This does not replace the entire invitation, it only updates the fields provided in the request.  The response is the updated invitation object.  **On the command line:**  ```bash blind organizations invitations partial-update --organization-id \"<organization-id>\" --id \"<invitation-id>\" --data organization-invitation-update.json ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | A unique value identifying this organization invitation. | [required] |
**organization_id** | **String** |  | [required] |
**patched_organization_invitation** | Option<[**PatchedOrganizationInvitation**](PatchedOrganizationInvitation.md)> |  |  |

### Return type

[**models::OrganizationInvitation**](OrganizationInvitation.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## organizations_invitations_retrieve

> models::OrganizationInvitation organizations_invitations_retrieve(id, organization_id)
Retrieve organization invitation

Retrieve a single organization invitation by its ID.  The response is the invitation object.  **On the command line:**  ```bash blind organizations invitations retrieve --organization-id \"<organization-id>\" --id \"<invitation-id>\" ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | A unique value identifying this organization invitation. | [required] |
**organization_id** | **String** |  | [required] |

### Return type

[**models::OrganizationInvitation**](OrganizationInvitation.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## organizations_invitations_update

> models::OrganizationInvitation organizations_invitations_update(id, organization_id, organization_invitation)
Update organization invitation

Update an organization invitation.  The response is the updated invitation object.  **On the command line:**  ```bash blind organizations invitations update --organization-id \"<organization-id>\" --id \"<invitation-id>\" --data organization-invitation.json ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | A unique value identifying this organization invitation. | [required] |
**organization_id** | **String** |  | [required] |
**organization_invitation** | [**OrganizationInvitation**](OrganizationInvitation.md) |  | [required] |

### Return type

[**models::OrganizationInvitation**](OrganizationInvitation.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## organizations_list

> Vec<models::Organization> organizations_list(created, id, is_active, limit, modified, name, offset, slug, users)
List organizations

Query for a list of all organizations that the currently authenticated user has access to.  The response will be a list of organization objects.  **On the command line:**  ```bash blind organizations list ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**created** | Option<**String**> |  |  |
**id** | Option<**String**> |  |  |
**is_active** | Option<**bool**> |  |  |
**limit** | Option<**i32**> | Number of results to return per page. |  |
**modified** | Option<**String**> |  |  |
**name** | Option<**String**> | The name of the organization |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**slug** | Option<**String**> | The name in all lowercase, suitable for URL identification |  |
**users** | Option<[**Vec<String>**](String.md)> |  |  |

### Return type

[**Vec<models::Organization>**](Organization.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## organizations_owner

> models::Organization organizations_owner(id)
Retrieve organization owner

Retrieve a single owner of an organization by its ID.  The response will be the user object of the owner.  **On the command line:**  ```bash blind organizations owner --id \"<organization-id>\" ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | A unique value identifying this organization. | [required] |

### Return type

[**models::Organization**](Organization.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## organizations_partial_update

> models::OrganizationUpdate organizations_partial_update(id, patched_organization_update)
Partial update organization

Update one or more fields of an organization. This does not replace the entire organization, it only updates the fields provided in the request.  The response will be the updated organization object.  **On the command line:**  ```bash blind organizations partial-update --id \"<organization-id>\" --data organization-update.json ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | A unique value identifying this organization. | [required] |
**patched_organization_update** | Option<[**PatchedOrganizationUpdate**](PatchedOrganizationUpdate.md)> |  |  |

### Return type

[**models::OrganizationUpdate**](OrganizationUpdate.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## organizations_retrieve

> models::Organization organizations_retrieve(id)
Retrieve organization

Retrieve a single organization by its ID.  The response will be the organization object.  **On the command line:**  ```bash blind organizations retrieve --id \"<organization-id>\" ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | A unique value identifying this organization. | [required] |

### Return type

[**models::Organization**](Organization.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## organizations_teams_add_member

> serde_json::Value organizations_teams_add_member(id, organization_id, organizations_teams_add_member_request)
Add a member to a team

Adds a member to a team if you have permissions to manage that team.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | A unique value identifying this team. | [required] |
**organization_id** | **String** |  | [required] |
**organizations_teams_add_member_request** | Option<[**OrganizationsTeamsAddMemberRequest**](OrganizationsTeamsAddMemberRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## organizations_teams_create

> models::Team organizations_teams_create(organization_id, team)
Create a new team

Create a new Team object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **String** |  | [required] |
**team** | [**Team**](Team.md) |  | [required] |

### Return type

[**models::Team**](Team.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## organizations_teams_destroy

> organizations_teams_destroy(id, organization_id)
Delete a team

Delete a Team object by its ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | A unique value identifying this team. | [required] |
**organization_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## organizations_teams_list

> Vec<models::Team> organizations_teams_list(organization_id, dataset, dataset_slug, id, limit, members, name, offset, organization, schema, schema_slug, slug)
List all teams

List all Team objects that you have access to.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **String** |  | [required] |
**dataset** | Option<**String**> | Filter by dataset ID. |  |
**dataset_slug** | Option<**String**> | Filter by dataset slug. |  |
**id** | Option<**String**> |  |  |
**limit** | Option<**i32**> | Number of results to return per page. |  |
**members** | Option<[**Vec<String>**](String.md)> | The users that are members of this team. |  |
**name** | Option<**String**> | A human-readable name for this team. |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**organization** | Option<**String**> | The organization that this team belongs to. |  |
**schema** | Option<**String**> | Filter by schema ID. |  |
**schema_slug** | Option<**String**> | Filter by schema slug. |  |
**slug** | Option<**String**> | A unique slug for this team. |  |

### Return type

[**Vec<models::Team>**](Team.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## organizations_teams_partial_update

> models::Team organizations_teams_partial_update(id, organization_id, patched_team)
Partially update a team

Partially update a Team object by its ID. This only replaces the specified fields.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | A unique value identifying this team. | [required] |
**organization_id** | **String** |  | [required] |
**patched_team** | Option<[**PatchedTeam**](PatchedTeam.md)> |  |  |

### Return type

[**models::Team**](Team.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## organizations_teams_remove_member

> serde_json::Value organizations_teams_remove_member(id, organization_id, organizations_teams_add_member_request)
Remove a member from a team

Removes a member from a team if you have permissions to manage that team.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | A unique value identifying this team. | [required] |
**organization_id** | **String** |  | [required] |
**organizations_teams_add_member_request** | Option<[**OrganizationsTeamsAddMemberRequest**](OrganizationsTeamsAddMemberRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## organizations_teams_retrieve

> models::Team organizations_teams_retrieve(id, organization_id)
Retrieve a single team

Retrieve a single Team object by its ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | A unique value identifying this team. | [required] |
**organization_id** | **String** |  | [required] |

### Return type

[**models::Team**](Team.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## organizations_teams_update

> models::Team organizations_teams_update(id, organization_id, team)
Update a team

Update a Team object by its ID. This completely replaces the object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | A unique value identifying this team. | [required] |
**organization_id** | **String** |  | [required] |
**team** | [**Team**](Team.md) |  | [required] |

### Return type

[**models::Team**](Team.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## organizations_update

> models::OrganizationUpdate organizations_update(id, organization_update)
Update organization

Replace an organization with the data provided in the request.  The response will be the updated organization object.  **On the command line:**  ```bash blind organizations update --id \"<organization-id>\" --data organization.json ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | A unique value identifying this organization. | [required] |
**organization_update** | [**OrganizationUpdate**](OrganizationUpdate.md) |  | [required] |

### Return type

[**models::OrganizationUpdate**](OrganizationUpdate.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## organizations_users_create

> models::OrganizationUserCreate organizations_users_create(organization_id, organization_user_create)
Create organization user

Add a new user to an organization.  The response is the newly added user object.  **On the command line:**  ```bash blind organizations users create --organization-id \"<organization-id>\" --data organization-user.json ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **String** |  | [required] |
**organization_user_create** | [**OrganizationUserCreate**](OrganizationUserCreate.md) |  | [required] |

### Return type

[**models::OrganizationUserCreate**](OrganizationUserCreate.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## organizations_users_destroy

> organizations_users_destroy(id, organization_id)
Delete organization user

Delete a user from an organization.  The response is a 204 No Content if successful.  **On the command line:**  ```bash blind organizations users destroy --organization-id \"<organization-id>\" --id \"<user-id>\" ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | A unique value identifying this organization user. | [required] |
**organization_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## organizations_users_list

> Vec<models::OrganizationUser> organizations_users_list(organization_id, created, has_identity, id, is_admin, limit, modified, offset, organization, user)
List organization users

Return a list of all users of an organization.  The response is a list of user objects belonging to the organization.  **On the command line:**  ```bash blind organizations users list --organization-id \"<organization-id>\" ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **String** |  | [required] |
**created** | Option<**String**> |  |  |
**has_identity** | Option<**bool**> | Filter by whether the organization user has an identity. |  |
**id** | Option<**String**> |  |  |
**is_admin** | Option<**bool**> |  |  |
**limit** | Option<**i32**> | Number of results to return per page. |  |
**modified** | Option<**String**> |  |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**organization** | Option<**String**> |  |  |
**user** | Option<**String**> |  |  |

### Return type

[**Vec<models::OrganizationUser>**](OrganizationUser.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## organizations_users_partial_update

> models::OrganizationUserUpdate organizations_users_partial_update(id, organization_id, patched_organization_user_update)
Partial update organization user

Update one or more fields of a user of an organization. This does not replace the entire user, it only updates the fields provided in the request.  The response is the updated user object.  **On the command line:**  ```bash blind organizations users partial-update --organization-id \"<organization-id>\" --id \"<user-id>\" --data organization-user-update.json ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | A unique value identifying this organization user. | [required] |
**organization_id** | **String** |  | [required] |
**patched_organization_user_update** | Option<[**PatchedOrganizationUserUpdate**](PatchedOrganizationUserUpdate.md)> |  |  |

### Return type

[**models::OrganizationUserUpdate**](OrganizationUserUpdate.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## organizations_users_retrieve

> models::OrganizationUser organizations_users_retrieve(id, organization_id)
Retrieve organization user

Retrieve a single user of an organization by their ID.  The response is the user object.  **On the command line:**  ```bash blind organizations users retrieve --organization-id \"<organization-id>\" --id \"<user-id>\" ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | A unique value identifying this organization user. | [required] |
**organization_id** | **String** |  | [required] |

### Return type

[**models::OrganizationUser**](OrganizationUser.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## organizations_users_update

> models::OrganizationUserUpdate organizations_users_update(id, organization_id, organization_user_update)
Update organization user

Replace a user of an organization with the data provided in the request.  The response is the updated user object.  **On the command line:**  ```bash blind organizations users update --organization-id \"<organization-id>\" --id \"<user-id>\" --data organization-user.json ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | A unique value identifying this organization user. | [required] |
**organization_id** | **String** |  | [required] |
**organization_user_update** | Option<[**OrganizationUserUpdate**](OrganizationUserUpdate.md)> |  |  |

### Return type

[**models::OrganizationUserUpdate**](OrganizationUserUpdate.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

