#[cfg(test)]
mod tests {

    use crate::backend::db::{Totp2Fa, User};

    // Note this useful idiom: importing names from outer (for mod tests) scope.

    #[test]
    fn test_gen_totp() {
        let totp = Totp2Fa::new(User::new(
            "Jasinco".to_string(),
            "nimaisle".to_string(),
            false,
            "123123".to_string(),
        ));
        dbg!(totp.generate_totp_code().expect("xc"));
        dbg!(totp.generate_totp_url().expect("url"));
    }
}
