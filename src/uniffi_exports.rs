//! Simpler interfaces exported to uniffi
//! "inner" correspond to the native library structs

use crate::{
    api, opts, uniffi_custom::Url, DecryptedPaste, PasteError, PbResult, PostPasteResponse,
};

#[uniffi::export]
fn read_paste(paste_url: &Url, password: &Option<String>) -> PbResult<DecryptedPaste> {
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
        paste.decrypt_with_password(bs58_key, password)
    } else {
        paste.decrypt(bs58_key)
    }
}

#[uniffi::export]
fn write_paste(
    host: &Url,
    content: &DecryptedPaste,
    opts: &PasteOpts,
) -> PbResult<PostPasteResponse> {
    let password = opts.password.as_deref().unwrap_or("");
    let api = api::API::new(host.clone(), opts.into());
    api.post_paste(content, password, &opts.into())
}

#[derive(uniffi::Record)]
pub struct PasteOpts {
    // todo: set default enum variant once supported by uniffi
    format: crate::PasteFormat,
    #[uniffi(default = "1week")]
    expire: String,
    #[uniffi(default = false)]
    burn: bool,
    #[uniffi(default = false)]
    discussion: bool,
    password: Option<String>,
}

/// get native library version of Opts
impl From<&PasteOpts> for opts::Opts {
    fn from(paste_opts: &PasteOpts) -> Self {
        opts::Opts {
            format: paste_opts.format.clone(),
            expire: paste_opts.expire.clone(),
            burn: paste_opts.burn,
            discussion: paste_opts.discussion,
            password: paste_opts.password.clone(),
            ..Default::default()
        }
    }
}
