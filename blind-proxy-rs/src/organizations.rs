use blind_wrapper_rs::apis::organizations_api::organizations_list;
use crate::{
    error::ProxyError,
    keyring::get_blind_insight_configuration,
};

pub async fn list_all_organizations() -> Result<Vec<String>, ProxyError> {
    let configuration = get_blind_insight_configuration();
    let mut organizations_vec: Vec<String> = Vec::new();
    match organizations_list(&configuration, None, None, None, None, None, None, None, None, None).await {
        Ok(orgs) => {
            for org in orgs {
                organizations_vec.push(serde_json::to_string(&org)?);
            }
        },
        Err(e) => {
            match e {
                blind_wrapper_rs::apis::Error::ResponseError(response) => {
                    if let Some(entity) = response.entity {
                        match entity {
                            blind_wrapper_rs::apis::organizations_api::OrganizationsListError::Status403(details) => {
                                if let Some(d) = details.detail {
                                    println!("{}", d);
                                }
                            },
                            _ => {
                                println!("Unexpected error: {:?}", entity);
                            }
                        }
                    }
                },
                _ => return Err(ProxyError::Unknown(e.to_string())),
            }
        }
    }

    Ok(organizations_vec)
}