use libnss::interop::Response;
use libnss::passwd::{Passwd, PasswdHooks};
use libnss::libnss_passwd_hooks;

struct NobodyPasswd;
libnss_passwd_hooks!(nobody, NobodyPasswd);

// Creates an account with username "test", and password "pass"
// Ensure the home directory "/home/test" exists, and is owned by 1007:1007
impl PasswdHooks for NobodyPasswd {
    fn get_all_entries() -> Response<Vec<Passwd>> {
        Response::NotFound
    }

    fn get_entry_by_uid(_uid: libc::uid_t) -> Response<Passwd> {
        Response::NotFound
    }

    fn get_entry_by_name(name: String) -> Response<Passwd> {
        Response::Success(Passwd {
            name: name.clone(),
            passwd: "x".to_string(),
            uid: 65534,
            gid: 65534,
            gecos: name.clone(),
            dir: "/home/".to_string() + name.clone().as_str(),
            shell: "/bin/bash".to_string(),
        })
    }
}
