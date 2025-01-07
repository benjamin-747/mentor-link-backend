use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Conference::Table)
                    .if_not_exists()
                    .col(pk_auto(Conference::Id))
                    .col(string(Conference::PlatformType))
                    .col(string(Conference::Subject))
                    .col(string(Conference::Description))
                    .col(string(Conference::StartTime))
                    .col(string(Conference::EndTime))
                    .col(string(Conference::ConferenceState))
                    .col(string(Conference::ConferenceLink))
                    .col(integer(Conference::HostId))
                    .col(date_time(Conference::CreateAt))
                    .col(date_time(Conference::UpdateAt))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Conference::Table).to_owned())
            .await
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(DeriveIden)]
enum Conference {
    Table,
    Id,
    PlatformType,
    Subject,
    Description,
    StartTime,
    EndTime,
    ConferenceState,
    ConferenceLink,
    HostId,
    CreateAt,
    UpdateAt,
}
