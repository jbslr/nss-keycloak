use libnss::passwd::{PasswdHooks, Passwd};
use libnss::interop::Response;

use crate::keycloak::{auth::TokenProvider, users::{list_users, KeycloakUser}};

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
        match list_users(
            &crate::CONFIG.keycloak, 
            &crate::CONFIG.mapping, 
            crate::AUTH.lock().unwrap().get_access_token().unwrap(),
        ) {
            Ok(users) => Response::Success(users.into_iter().map(|user| Passwd::from(user)).collect()),
            _ => Response::Unavail,
        }
    }

    fn get_entry_by_uid(uid: libc::uid_t) -> libnss::interop::Response<Passwd> {
        todo!()
    }

    fn get_entry_by_name(name: String) -> libnss::interop::Response<Passwd> {
        todo!()
    }
} 