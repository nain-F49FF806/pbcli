//! Simpler interfaces exported to uniffi
//! "inner" correspond to the native library structs

use crate::{
    api, opts, privatebin::Paste, uniffi_custom::Url, DecryptedPaste, PbResult, PostPasteResponse,
};

#[uniffi::export]
fn post_paste(paste_url: &Url) -> PbResult<Paste> {
    let paste_id = paste_url.query().unwrap();
    let api = api::API::new(paste_url.clone(), opts::Opts::default());
    api.get_paste(paste_id)
}

#[uniffi::export]
fn get_paste(
    host: &Url,
    content: &DecryptedPaste,
    paste_opts: &PasteOpts,
) -> PbResult<PostPasteResponse> {
    let password = paste_opts.password.as_deref().unwrap_or("");
    let api = api::API::new(host.clone(), paste_opts.into());
    api.post_paste(content, password, &paste_opts.into())
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
            format: paste_opts.format,
            expire: paste_opts.expire.clone(),
            burn: paste_opts.burn,
            discussion: paste_opts.discussion,
            password: paste_opts.password.clone(),
            ..Default::default()
        }
    }
}
