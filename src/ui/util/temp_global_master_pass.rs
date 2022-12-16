


pub(crate) static mut MASTER_PASSWORD: String = String::new();

pub fn set_master_password(password: String) {
    unsafe {
        MASTER_PASSWORD = password;
    }
}

pub fn get_master_password() -> String {
    unsafe {
        MASTER_PASSWORD.clone()
    }
}
