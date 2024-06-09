pub struct InputValidator {}

impl InputValidator {
    pub fn validate_service_type(service_type: u8) {
        match service_type {
            0 | 1 => {}
            _ => panic!("入力値が不正です。"),
        }
    }

    pub fn validate_register_type(register_type: u8) {
        match register_type {
            0 | 1 => {}
            _ => panic!("入力値が不正です。"),
        }
    }
}
