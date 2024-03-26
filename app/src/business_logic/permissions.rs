use ory_keto_client::models::{
    create_relationship_body::CreateRelationshipBody,
    post_check_permission_body::PostCheckPermissionBody,
};

#[tracing::instrument]
pub async fn add_user_as_editor(
    post_id: &String,
    new_editor_user_id: &String,
) -> Result<(), leptos::ServerFnError> {
    crate::auth::keto_utils::create_relationship(CreateRelationshipBody {
        namespace: Some("Posts".to_string()),
        object: Some(post_id.clone()),
        relation: Some("editors".to_string()),
        subject_id: Some(new_editor_user_id.clone()),
        subject_set: None,
    })
    .await
}
#[tracing::instrument]
pub async fn add_owner_post_permission(
    post_id: &String,
    owner_user_id: &String,
) -> Result<(), leptos::ServerFnError> {
    crate::auth::keto_utils::create_relationship(CreateRelationshipBody {
        namespace: Some("Posts".to_string()),
        object: Some(post_id.clone()),
        relation: Some("owners".to_string()),
        subject_id: Some(owner_user_id.clone()),
        subject_set: None,
    })
    .await
}

#[tracing::instrument(err)]
pub async fn user_can_edit_post(
    post_id: &String,
    user_id: &String,
) -> Result<(), leptos::ServerFnError> {
    if !crate::auth::keto_utils::check_permission(PostCheckPermissionBody {
        namespace: Some("Posts".to_string()),
        object: Some(post_id.clone()),
        relation: Some("edit".to_string()),
        subject_id: Some(user_id.clone()),
        subject_set: None,
    })
    .await?
    {
        Err(leptos::ServerFnError::new(ids::AUTH_ERROR_MSG))
    } else {
        Ok(())
    }
}

#[tracing::instrument(err)]
pub async fn user_can_view_post(
    post_id: &String,
    user_id: &String,
) -> Result<(), leptos::ServerFnError> {
    if !crate::auth::keto_utils::check_permission(PostCheckPermissionBody {
        namespace: Some("Posts".to_string()),
        object: Some(post_id.clone()),
        relation: Some("view".to_string()),
        subject_id: Some(user_id.clone()),
        subject_set: None,
    })
    .await?
    {
        Err(leptos::ServerFnError::new(ids::AUTH_ERROR_MSG))
    } else {
        Ok(())
    }
}

#[tracing::instrument(err)]
pub async fn user_can_delete_post(
    post_id: &String,
    user_id: &String,
) -> Result<(), leptos::ServerFnError> {
    if !crate::auth::keto_utils::check_permission(PostCheckPermissionBody {
        namespace: Some("Posts".to_string()),
        object: Some(post_id.clone()),
        relation: Some("delete".to_string()),
        subject_id: Some(user_id.clone()),
        subject_set: None,
    })
    .await?
    {
        Err(leptos::ServerFnError::new(ids::AUTH_ERROR_MSG))
    } else {
        Ok(())
    }
}
