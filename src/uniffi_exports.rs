use crate::{
    api, opts,
    privatebin::{Paste, PostPasteResponse},
    uniffi_custom::Url,
    DecryptedPaste, PbResult,
};

/// Simpler interfaces exported to uniffi
/// "inner" correspond to the native library structs

#[derive(uniffi::Object)]
pub struct API {
    inner: api::API,
}

#[uniffi::export]
impl API {
    /// Construct new API instance for given url
    #[uniffi::constructor]
    fn new(url: Url) -> API {
        Self {
            inner: api::API::new(url, opts::Opts::default()),
        }
    }

    fn get_paste(&self, paste_id: &str) -> PbResult<Paste> {
        self.inner.get_paste(paste_id)
    }

    fn post_paste(
        &self,
        content: &DecryptedPaste,
        password: &str,
        opts: &Opts,
    ) -> PbResult<PostPasteResponse> {
        self.inner.post_paste(content, password, &opts.get_inner())
    }
}

#[derive(uniffi::Record)]
pub struct Opts {
    // todo: set default enum variant once supported by uniffi
    format: crate::PasteFormat,
    #[uniffi(default = "1week")]
    expire: String,
    #[uniffi(default = false)]
    burn: bool,
    #[uniffi(default = false)]
    discussion: bool,
    #[uniffi(default = None)]
    password: Option<String>,
}

impl Opts {
    /// get native library version of Opts
    fn get_inner(&self) -> opts::Opts {
        opts::Opts {
            format: self.format.clone(),
            expire: self.expire.clone(),
            burn: self.burn,
            discussion: self.discussion,
            password: self.password.clone(),
            ..Default::default()
        }
    }
}
