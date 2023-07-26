use sqlx::SqlitePool;

pub type PlaylistId = i64;

pub struct Playlist {
    name: String,
    songs: Vec<String>,
}

impl Playlist {
    pub fn new(name: String, song: String) -> Self {
        Self {
            name,
            songs: vec![song],
        }
    }

    pub async fn save_playlist(&self, pool: &SqlitePool) -> anyhow::Result<()> {
        let mut conn = pool.acquire().await?;

        sqlx::query!(
            r#"
            INSERT INTO playlists (name) VALUES (?1)
        "#,
            self.name
        )
        .execute(&mut *conn)
        .await?;

        Ok(())
    }
}
