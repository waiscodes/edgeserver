use poem::{web::Data, Result};
use poem_openapi::{param::Path, payload::Json, Object, OpenApi};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::{
    middlewares::auth::UserAuth,
    models::{
        site::Site,
        team::{invite::UserTeamInvite, Team, TeamId},
        user::User,
    },
    routes::{error::HttpError, ApiTags},
    state::State,
};

#[derive(Debug, Serialize, Deserialize, Object)]
pub struct CreateTeamRequest {
    pub name: String,
}

pub struct TeamApi;

#[OpenApi]
impl TeamApi {
    /// Get all teams
    ///
    /// Gets a list of all the teams you have access to
    #[oai(path = "/team", method = "get", tag = "ApiTags::Team")]
    pub async fn get_teams(&self, user: UserAuth, state: Data<&State>) -> Result<Json<Vec<Team>>> {
        let user = user.required()?;
        info!("Getting teams for user: {:?}", user);

        Team::get_by_user_id(&state.0.database, &user.user_id)
            .await
            .map_err(HttpError::from)
            .map(Json)
            .map_err(poem::Error::from)
    }

    #[oai(path = "/team", method = "post", tag = "ApiTags::Team")]
    pub async fn create_team(
        &self,
        user: UserAuth,
        state: Data<&State>,
        body: Json<CreateTeamRequest>,
    ) -> Result<Json<Team>> {
        info!("Creating team for user: {:?}", user);
        let user = user.required()?;

        Team::new(&state.0.database, &body.name, &user.user_id)
            .await
            .map_err(HttpError::from)
            .map(Json)
            .map_err(poem::Error::from)
    }

    #[oai(path = "/team/:team_id", method = "get", tag = "ApiTags::Team")]
    pub async fn get_team(
        &self,
        user: UserAuth,
        state: Data<&State>,
        team_id: Path<String>,
    ) -> Result<Json<Team>> {
        info!("Getting team: {:?} for user: {:?}", team_id.0, user);
        user.verify_access_to(&TeamId(&team_id.0)).await?;

        Team::get_by_id(&state.0.database, &team_id.0)
            .await
            .map_err(HttpError::from)
            .map(Json)
            .map_err(poem::Error::from)
    }

    #[oai(path = "/team/:team_id/invites", method = "get", tag = "ApiTags::Team")]
    pub async fn get_team_invites(
        &self,
        user: UserAuth,
        state: Data<&State>,
        team_id: Path<String>,
    ) -> Result<Json<Vec<UserTeamInvite>>> {
        info!(
            "Getting team invites for team: {:?} for user: {:?}",
            team_id.0, user
        );

        user.verify_access_to(&TeamId(&team_id.0)).await?;

        UserTeamInvite::get_by_team_id(&state.0.database, &team_id.0)
            .await
            .map_err(HttpError::from)
            .map(Json)
            .map_err(poem::Error::from)
    }

    #[oai(path = "/team/:team_id/invites", method = "post", tag = "ApiTags::Team")]
    pub async fn invite_user_to_team(
        &self,
        user: UserAuth,
        state: Data<&State>,
        team_id: Path<String>,
    ) -> Result<Json<UserTeamInvite>> {
        info!(
            "Inviting user to team: {:?} for user: {:?}",
            team_id.0, user
        );

        user.verify_access_to(&TeamId(&team_id.0)).await?;

        let user = user.required()?;

        if !Team::is_owner(&state.0.database, &team_id.0, &user.user_id)
            .await
            .map_err(HttpError::from)?
        {
            Err(HttpError::Forbidden)?;
        }

        // Create "anonymous" invite (for now), replace None with user_id when we have a way to create invites for specific users
        UserTeamInvite::new(&state.0.database, &team_id.0, None::<String>, &user.user_id)
            .await
            .map_err(HttpError::from)
            .map(Json)
            .map_err(poem::Error::from)
    }

    #[oai(
        path = "/team/:team_id/invite/:invite_id",
        method = "delete",
        tag = "ApiTags::Team"
    )]
    pub async fn delete_team_invite(
        &self,
        user: UserAuth,
        state: Data<&State>,
        team_id: Path<String>,
        invite_id: Path<String>,
    ) -> Result<()> {
        info!(
            "Deleting team invite: {:?} for user: {:?}",
            invite_id.0, user
        );

        user.verify_access_to(&TeamId(&team_id.0)).await?;

        let user = user.required()?;

        if !Team::is_owner(&state.0.database, &team_id.0, &user.user_id)
            .await
            .map_err(HttpError::from)?
        {
            Err(HttpError::Forbidden)?;
        }

        UserTeamInvite::delete_by_id(&state.0.database, &invite_id.0)
            .await
            .map_err(HttpError::from)?;

        Ok(())
    }

    #[oai(path = "/team/:team_id/sites", method = "get", tag = "ApiTags::Team")]
    pub async fn get_team_sites(
        &self,
        user: UserAuth,
        state: Data<&State>,
        team_id: Path<String>,
    ) -> Result<Json<Vec<Site>>> {
        info!(
            "Getting sites for team: {:?} for user: {:?}",
            team_id.0, user
        );

        user.verify_access_to(&TeamId(&team_id.0)).await?;

        Site::get_by_team_id(&state.0.database, &team_id.0)
            .await
            .map_err(HttpError::from)
            .map(Json)
            .map_err(poem::Error::from)
    }

    #[oai(path = "/team/:team_id/members", method = "get", tag = "ApiTags::Team")]
    pub async fn get_team_members(
        &self,
        user: UserAuth,
        state: Data<&State>,
        team_id: Path<String>,
    ) -> Result<Json<Vec<User>>> {
        user.verify_access_to(&TeamId(&team_id.0)).await?;

        Team::get_members(&state.0.database, &team_id.0)
            .await
            .map_err(HttpError::from)
            .map(Json)
            .map_err(poem::Error::from)
    }

    #[oai(path = "/team/:team_id", method = "put", tag = "ApiTags::Team")]
    pub async fn update_team(
        &self,
        user: UserAuth,
        state: Data<&State>,
        team_id: Path<String>,
    ) -> Result<Json<Team>> {
        info!("Updating team: {:?} for user: {:?}", team_id.0, user);

        user.verify_access_to(&TeamId(&team_id.0)).await?;

        let user = user.required()?;

        if !Team::is_owner(&state.0.database, &team_id.0, &user.user_id)
            .await
            .map_err(HttpError::from)?
        {
            Err(HttpError::Forbidden)?;
        }

        todo!();
    }

    #[oai(path = "/team/:team_id", method = "delete", tag = "ApiTags::Team")]
    pub async fn delete_team(
        &self,
        user: UserAuth,
        state: Data<&State>,
        team_id: Path<String>,
    ) -> Result<()> {
        info!("Deleting team: {:?} for user: {:?}", team_id.0, user);

        let user = user.required()?;

        if !Team::is_owner(&state.0.database, &team_id.0, &user.user_id)
            .await
            .map_err(HttpError::from)?
        {
            Err(HttpError::Forbidden)?;
        }

        Team::delete_by_id(&state.0.database, &team_id.0)
            .await
            .map_err(HttpError::from)?;

        Ok(())
    }
}
