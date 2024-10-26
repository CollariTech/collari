use crate::json::{no_content, CollariResponse};

pub async fn logout() -> CollariResponse<()> {
    no_content()
}