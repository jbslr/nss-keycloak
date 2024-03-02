use libnss::interop::Response;
use libnss::group::{Group, GroupHooks};

struct KeycloakNssGroup;

impl GroupHooks for KeycloakNssGroup {
    fn get_all_entries() -> Response<Vec<Group>> {
        todo!()
    }

    fn get_entry_by_gid(gid: libc::gid_t) -> Response<Group> {
        todo!()
    }

    fn get_entry_by_name(name: String) -> Response<Group> {
        todo!()
    }
}
