use libnss::interop::Response;
use libnss::passwd::{Passwd, PasswdHooks};

use crate::keycloak::auth::TokenProvider;
use crate::keycloak::users::{get_user_by_name, get_user_by_uid, list_users, KeycloakUser};

pub struct KeycloakNssPasswd;

impl From<KeycloakUser> for Passwd {
    fn from(user: KeycloakUser) -> Self {
        Passwd {
            name: user.username.clone(),
            passwd: "x".to_string(),
            uid: user.uid,
            gid: user.gid,
            gecos: user.gecos.clone(),
            dir: user.homedir.clone(),
            shell: user.loginshell.clone(),
        }
    }
}

impl PasswdHooks for KeycloakNssPasswd {
    fn get_all_entries() -> libnss::interop::Response<Vec<Passwd>> {
        let access_token = match crate::AUTH.lock().unwrap().get_access_token() {
            Ok(token) => token.clone(),
            Err(err) => {
                log::error!("Failed to get access token: {:?}", err);
                return Response::TryAgain;
            }
        };
        match list_users(
            &crate::CONFIG.keycloak,
            &crate::CONFIG.mapping,
            &access_token,
        ) {
            Ok(users) => {
                Response::Success(users.into_iter().map(|user| Passwd::from(user)).collect())
            }
            Err(err) => {
                log::error!("Failed to get all users: {:?}", err);
                Response::TryAgain
            }
        }
    }

    fn get_entry_by_uid(uid: libc::uid_t) -> libnss::interop::Response<Passwd> {
        let access_token = match crate::AUTH.lock().unwrap().get_access_token() {
            Ok(token) => token.clone(),
            Err(err) => {
                log::error!("Failed to get access token: {:?}", err);
                return Response::TryAgain;
            }
        };
        match get_user_by_uid(
            &crate::CONFIG.keycloak,
            &crate::CONFIG.mapping,
            &access_token,
            uid,
        ) {
            Ok(Some(user)) => Response::Success(Passwd::from(user)),
            Ok(None) => Response::NotFound,
            Err(err) => {
                log::error!("Failed to get user by uid: {:?}", err);
                Response::TryAgain
            }
        }
    }

    fn get_entry_by_name(name: String) -> libnss::interop::Response<Passwd> {
        let access_token = match crate::AUTH.lock().unwrap().get_access_token() {
            Ok(token) => token.clone(),
            Err(err) => {
                log::error!("Failed to get access token: {:?}", err);
                return Response::TryAgain;
            }
        };
        match get_user_by_name(
            &crate::CONFIG.keycloak,
            &crate::CONFIG.mapping,
            &access_token,
            &name,
        ) {
            Ok(Some(user)) => Response::Success(Passwd::from(user)),
            Ok(None) => Response::NotFound,
            Err(err) => {
                log::error!("Failed to get user by name: {:?}", err);
                Response::TryAgain
            }
        }
    }
}
