use serde::Serialize;
use sqlx::PgConnection;
use utoipa::ToSchema;

use crate::model::error::ModelError;

#[derive(Debug, Clone)]
pub struct Page<T> {
    pub items: Vec<T>,
    pub total_items: i64,
    pub total_pages: i32,
    pub current_page: i32,
    pub page_size: i32,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PageDto<T> {
    #[schema(example = "[item21, item22, item23, ..., item30]")]
    pub items: Vec<T>,

    #[schema(example = 108)]
    pub total_items: i64,

    #[schema(example = 11)]
    pub total_pages: i32,

    #[schema(example = 3)]
    pub current_page: i32,

    #[schema(example = 10)]
    pub page_size: i32,
}

impl<T> From<Page<T>> for PageDto<T> {
    fn from(value: Page<T>) -> Self {
        PageDto {
            items: value.items,
            total_items: value.total_items,
            total_pages: value.total_pages,
            current_page: value.current_page,
            page_size: value.page_size,
        }
    }
}

impl<T> Page<T> {
    pub fn build_from(items: Vec<T>, total_items: i64, current_page: i32, page_size: i32) -> Self {
        let total_pages = if page_size > 0 {
            (total_items as f64 / page_size as f64).ceil() as i32
        } else {
            0
        };

        Self {
            items,
            total_items,
            total_pages,
            current_page,
            page_size,
        }
    }

    pub fn map_into<U: From<T>>(self) -> PageDto<U> {
        PageDto::<U> {
            items: self.items.into_iter().map(|e| U::from(e)).collect(),
            total_items: self.total_items,
            total_pages: self.total_pages,
            current_page: self.current_page,
            page_size: self.page_size,
        }
    }

    pub fn try_map_into<U: TryFrom<T, Error = ModelError>>(self) -> Result<PageDto<U>, ModelError> {
        let items = self
            .items
            .into_iter()
            .map(U::try_from)
            .collect::<Result<Vec<U>, ModelError>>()?;

        Ok(PageDto {
            items,
            total_items: self.total_items,
            total_pages: self.total_pages,
            current_page: self.current_page,
            page_size: self.page_size,
        })
    }
}

#[allow(async_fn_in_trait)]
pub trait Paginate<Q>: Sized {
    async fn page(params: &Q, connection: &mut PgConnection) -> Result<Page<Self>, ModelError>;
}
