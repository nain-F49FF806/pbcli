//! Simpler interfaces exported to uniffi
//! "inner" correspond to the native library structs

use crate::{
    api, opts,
    privatebin::{Paste, PostPasteResponse},
    uniffi_custom::Url,
    DecryptedPaste, PasteError, PbResult,
};

///Simplified API instance for given url
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

    fn post_paste(
        &self,
        content: &DecryptedPaste,
        password: &str,
        opts: &Opts,
    ) -> PbResult<PostPasteResponse> {
        self.inner.post_paste(content, password, &opts.get_inner())
    }
}

#[uniffi::export]
fn read_paste(paste_url: Url, password: Option<String>) -> PbResult<DecryptedPaste> {
    let paste_id = paste_url.query().unwrap();
    let fragment = paste_url
        .fragment()
        .ok_or(PasteError::MissingDecryptionKey)?;
    // '-' character may be found at start of fragment. This should be stripped.
    // It is used to activate "warn before read" feature for burn on read pastes.
    let bs58_key = fragment.strip_prefix('-').unwrap_or(fragment);

    let api = api::API::new(paste_url.clone(), opts::Opts::default());
    let paste = api.get_paste(paste_id)?;
    if let Some(password) = password {
        paste.decrypt_with_password(bs58_key, &password)
    } else {
        paste.decrypt(bs58_key)
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
