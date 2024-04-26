/// Simpler interfaces exported to uniffi
/// "inner" correspond to the native library structs

#[derive(uniffi::Record)]
pub struct Opts {
    format: crate::privatebin::PasteFormat,
    expire: String,
    burn: bool,
    discussion: bool,
    password: Option<String>,
}

impl Opts {
    fn get_inner(&self) -> crate::opts::Opts {
        crate::opts::Opts {
            format: self.format,
            expire: self.expire.clone(),
            burn: self.burn,
            discussion: self.discussion,
            password: self.password.clone(),
            ..Default::default()
        }
    }
}
