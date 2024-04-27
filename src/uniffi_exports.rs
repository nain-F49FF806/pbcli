//! Simpler interfaces exported to uniffi
//! "inner" correspond to the native library structs

use std::collections::HashMap;

use crate::{
    api, opts, uniffi_custom::Url, DecryptedComment, DecryptedPaste, PasteError, PbResult,
    PostPasteResponse,
};

#[uniffi::export]
fn read_paste(paste_url: &Url, password: &Option<String>) -> PbResult<ReadPasteOutput> {
    let paste_id = paste_url.query().unwrap();
    let fragment = paste_url
        .fragment()
        .ok_or(PasteError::MissingDecryptionKey)?;
    // '-' character may be found at start of fragment. This should be stripped.
    // It is used to activate "warn before read" feature for burn on read pastes.
    let bs58_key = fragment.strip_prefix('-').unwrap_or(fragment);
    let password = password.as_deref().unwrap_or("");

    let api = api::API::new(paste_url.clone(), opts::Opts::default());
    let paste = api.get_paste(paste_id)?;

    let decrypted_paste = paste.decrypt_with_password(bs58_key, password)?;
    let decrypted_comments = paste.decrypt_comments_with_password(bs58_key, password)?;
    let mut comment_tree: HashMap<String, Vec<String>> = HashMap::new();
    if let Some(comments) = &paste.comments {
        for c in comments {
            let id = c.id.clone();
            let parentid = if c.parentid == c.pasteid {"".to_owned()} else {c.parentid.clone()};
            comment_tree.entry(parentid).or_default().push(id);
        }
    }
    Ok(ReadPasteOutput {
        decrypted_paste,
        decrypted_comments,
        comment_tree,
    })
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


#[derive(Debug, uniffi::Record)]
struct ReadPasteOutput {
    decrypted_paste: DecryptedPaste,
    /// id -> decrypted_comment
    decrypted_comments: HashMap<String, DecryptedComment>,
    /// id -> [children ids]
    comment_tree: HashMap<String, Vec<String>>,
}

impl ReadPasteOutput {
    fn format_paste(&self) -> String {
        serde_json::to_string_pretty(&self.decrypted_paste).unwrap()
    }

    fn format_comments(&self) -> String {
        use serde_json::{Value,json};

        fn format_comments_below_id(
            id: &str,
            decrypted_comments: &HashMap<String, DecryptedComment>,
            comment_tree: &HashMap<String, Vec<String>>,
        ) -> Value {
            let children_array: Vec<Value> = comment_tree
                .get(id)
                .unwrap_or(&Vec::new())
                .iter()
                .map(|child_id| {
                    format_comments_below_id(child_id, decrypted_comments, comment_tree)
                })
                .collect();

            let comment_content =
                serde_json::to_value(decrypted_comments.get(id).unwrap_or(&DecryptedComment {
                    comment: "".to_owned(),
                    nickname: None,
                }))
                .unwrap();

            json!({
                id: {
                    "content": comment_content,
                    "children": children_array
                }
            })
        }
        let comments_json =
            format_comments_below_id("", &self.decrypted_comments, &self.comment_tree);
        serde_json::to_string_pretty(&comments_json).unwrap()
    }
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
