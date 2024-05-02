use async_recursion::async_recursion;
use chrono::Utc;
use dotenvy::var;
use sqlx::{postgres::PgRow, PgPool, Row};

use crate::{
    comment::model::{CommentTreeNode, DbComment, DbPost, DbPostQuery},
    error::ErrorResponse,
    helpers::id::generate_id,
};

#[derive(Clone)]
pub struct CommentService {
    db: PgPool,
}

impl CommentService {
    pub async fn init() -> Self {
        let connection_string = var("DATABASE_URL")
            .unwrap_or("postgres://open_user:open_password@localhost:5432/openedu".into());

        let db = PgPool::connect(&connection_string)
            .await
            .expect("Cannot connect to the database");

        Self { db }
    }

    async fn top_level_comments_from_post_id(
        &self,
        post_id: String,
    ) -> Result<Vec<DbComment>, ErrorResponse> {
        let query = r#"
                SELECT
                    *
                FROM tbl_comments
                WHERE
                    post_id = $1 AND parent_comment_id IS NULL
                ORDER BY score DESC, created_at DESC
            ;"#;
        let comments: Vec<DbComment> = sqlx::query_as::<_, DbComment>(query)
            .bind(post_id)
            .fetch_all(&self.db)
            .await?;

        Ok(comments)
    }
    async fn get_level_comment(&self, comment_id: &str) -> Result<i32, ErrorResponse> {
        let query = r#"
                WITH RECURSIVE comment_tree(
                  pk_comment_id,
                  level
                ) AS (
                  SELECT
                    pk_comment_id,
                    0 AS level -- Set level 0 for root comments
                  FROM tbl_comments
                  WHERE parent_comment_id IS NULL

                  UNION ALL

                  SELECT
                    c.pk_comment_id,
                    t.level + 1 AS level -- Increment level for child comments
                  FROM tbl_comments AS c
                  INNER JOIN comment_tree AS t ON c.parent_comment_id = t.pk_comment_id
                )

                SELECT level FROM comment_tree 
                where pk_comment_id = $1
        "#;
        let res = sqlx::query(query)
            .bind(comment_id)
            .fetch_one(&self.db)
            .await?;

        Ok(res.get::<i32, _>("level"))
    }
    pub async fn create_comment(
        &mut self,
        parent_id: Option<String>,
        post_id: String,
        content: String,
        user_id: String,
    ) -> Result<DbComment, ErrorResponse> {
        let query = r#"
                INSERT INTO tbl_comments(
                    pk_comment_id,
                    parent_comment_id,
                    post_id,
                    user_id,
                    content
                ) VALUES 
                ($1, $2, $3, $4, $5) RETURNING *
            "#;

        let created_comment = sqlx::query_as::<_, DbComment>(query)
            .bind(generate_id())
            .bind(parent_id)
            .bind(post_id)
            .bind(user_id)
            .bind(content)
            .fetch_one(&self.db)
            .await?;

        Ok(created_comment)
    }

    #[async_recursion]
    async fn build_comment_tree(
        &self,
        comment: DbComment,
    ) -> Result<CommentTreeNode, ErrorResponse> {
        let mut children = Vec::new();
        let child_comments: Vec<DbComment> = sqlx::query_as::<_, DbComment>(
            r#" SELECT *
                FROM tbl_comments
                WHERE
                parent_comment_id = $1 ORDER BY score DESC, created_at DESC; "#,
        )
        .bind(&comment.id)
        .fetch_all(&self.db)
        .await?;

        for child_comment in child_comments {
            children.push(self.build_comment_tree(child_comment).await?);
        }

        let parent_comment_id = comment.clone().parent_comment_id.unwrap_or_default();
        let user_id = comment.user_id.to_string();

        let tree = CommentTreeNode {
            comment,
            children,
            user_id,
            parent_comment_id,
        };

        Ok(tree)
    }

    pub async fn get_comment_by_id(
        &self,
        id: String,
        post_id: String,
    ) -> Result<DbComment, ErrorResponse> {
        let query = r#"SELECT * FROM tbl_comments 
               WHERE pk_comment_id = $1 
               AND   post_id = $2"#;

        let comment: DbComment = sqlx::query_as::<_, DbComment>(query)
            .bind(id)
            .bind(post_id)
            .fetch_one(&self.db)
            .await?;

        Ok(comment)
    }

    async fn update_comment(&self, id: &str, content: &str) -> Result<DbComment, ErrorResponse> {
        let query: &str = r#"
        UPDATE tbl_comments 
        SET content = $1
        WHERE pk_comment_id = $2 RETURNING *;
    "#;

        let comment: DbComment = sqlx::query_as::<_, DbComment>(query)
            .bind(content)
            .bind(id)
            .fetch_one(&self.db)
            .await?;

        Ok(comment)
    }

    async fn delete_comment(&self, id: &str) -> Result<(), ErrorResponse> {
        let query: &str = r#"DELETE FROM tbl_comments WHERE pk_comment_id = $1"#;

        sqlx::query(query).bind(id).execute(&self.db).await?;
        Ok(())
    }

    pub async fn get_comment_post_call(
        &self,
        post_id: String,
    ) -> Result<Vec<CommentTreeNode>, ErrorResponse> {
        let comments: Vec<DbComment> = self
            .top_level_comments_from_post_id(post_id.clone())
            .await?;

        let mut comment_tree_nodes: Vec<CommentTreeNode> = Vec::new();

        for comment in comments {
            let comment_tree_node: CommentTreeNode = self.build_comment_tree(comment).await?;

            comment_tree_nodes.push(comment_tree_node);
        }

        Ok(comment_tree_nodes)
    }
}
// async fn get_post_by_id(&self, id: &str) -> Result<DbPost, ErrorResponse> {
//     let get_post_by_id_query: &str = r#"
//     SELECT * FROM core.tbl_posts WHERE pk_post_id = $1;
//     "#;
//     let post: DbPost = sqlx::query_as::<_, DbPost>(get_post_by_id_query)
//         .bind(id)
//         .fetch_one(&self.db)
//         .await?;
//
//     Ok(post)
// }
//
// async fn get_posts(
//     &self,
//     DbPostQuery {
//         offset,
//         limit,
//         start_at,
//         end_at,
//     }: DbPostQuery,
// ) -> Result<Vec<DbPost>, ErrorResponse> {
//     // Set offset to 0 if not provided
//     let offset = offset.unwrap_or(0);
//
//     /*
//         The limit number is defaulted to 10
//         Any number outside the 0 -> 50 range will be reset to 10
//     */
//     let limit = match limit {
//         Some(limit) => match limit {
//             0..=500 => limit,
//             _ => 500,
//         },
//         None => 10,
//     };
//
//     // SELECT clause
//     let select = "SELECT * FROM core.tbl_posts ";
//
//     // Counter for the final prepared statement
//     let mut counter = 0;
//
//     /*
//         I added WHERE true so that I don't have to deal with checking
//         if counter == 0 to append "AND" into the where clause from the beginning or not
//     */
//     let mut conditions = "WHERE true ".to_string();
//
//     // Add WHERE clause if there is any filter provided
//     if start_at.is_some() {
//         counter += 1;
//         conditions.push_str(&format!("AND start_at >= ${counter} "));
//     }
//     if end_at.is_some() {
//         counter += 1;
//         conditions.push_str(&format!("AND end_at <= ${counter} "));
//     }
//
//     let offset_limit = format!("AND pagination_id >= {offset} LIMIT {limit}");
//
//     let query = format!("{select}{conditions}{offset_limit}");
//
//     let mut query = sqlx::query_as::<_, DbPost>(&query);
//
//     if let Some(start_at) = start_at {
//         query = query.bind(start_at);
//     }
//     if let Some(end_at) = end_at {
//         query = query.bind(end_at);
//     }
//
//     let posts = query.fetch_all(&self.db).await?;
//     Ok(posts)
// }
//
// async fn update_post(
//     &self,
//     post_id: &str,
//     title: &str,
//     content: &str,
// ) -> Result<DbPost, ErrorResponse> {
//     let update = "UPDATE core.tbl_posts SET ".to_string();
//
//     // Counter to count how many fields to update
//     let mut counter = 0;
//
//     // Storing mutations
//     let mut mutations = "".to_string();
//
//     counter += 1;
//     mutations.push_str(&format!("title = ${counter}, "));
//
//     counter += 1;
//     mutations.push_str(&format!("content = ${counter}, "));
//
//     // If there's nothing to update
//
//     counter += 1;
//     mutations.push_str(&format!("updated_at = ${counter} "));
//
//     counter += 1;
//     let where_clause = format!("WHERE pk_post_id = ${counter} ");
//
//     let returning = "RETURNING *;";
//
//     let query = format!("{update}{mutations}{where_clause}{returning}");
//
//     let mut query = sqlx::query_as::<_, DbPost>(&query);
//
//     query = query.bind(title);
//
//     query = query.bind(content);
//
//     let post = query
//         .bind(Utc::now().timestamp_millis())
//         .bind(post_id)
//         .fetch_one(&self.db)
//         .await?;
//
//     Ok(post)
// }
//
// async fn delete_post(&self, post_id: &str) -> Result<(), ErrorResponse> {
//     let query: &str = r#"DELETE from core.tbl_posts WHERE pk_post_id = $1"#;
//     sqlx::query(query).bind(post_id).execute(&self.db).await?;
//     Ok(())
// }
//
// async fn create_post(&self, post: DbPost) -> Result<DbPost, ErrorResponse> {
//     let query: &str = r#"INSERT INTO core.tbl_posts(pk_post_id, user_id, title, content) VALUES ($1, $2, $3, $4) RETURNING *"#;
//
//     let post: DbPost = sqlx::query_as::<_, DbPost>(query)
//         .bind(&post.id)
//         .bind(&post.user_id)
//         .bind(&post.title)
//         .bind(&post.content)
//         .fetch_one(&self.db)
//         .await?;
//
//     Ok(post)
// }

// async fn add_post_course(&self, post_id: &str, course_id: &str) -> Result<()> {
//     let query: &str =
//         r#"INSERT INTO core.tbl_post_course_map(post_id, course_id) VALUES ($1, $2);"#;
//
//     sqlx::query(query)
//         .bind(post_id)
//         .bind(course_id)
//         .execute(&self.db)
//         .await?;
//
//     Ok(())
// }
// async fn add_post_assignment(
//     &self,
//     post_id: &str,
//     assignemnt_id: &str,
// ) -> Result<()> {
//     let query: &str =
//         r#"INSERT INTO core.tbl_post_assignment_map(post_id, assignment_id) VALUES ($1, $2);"#;
//
//     sqlx::query(query)
//         .bind(post_id)
//         .bind(assignemnt_id)
//         .execute(&self.db)
//         .await?;
//
//     Ok(())
// }
