use crate::errors::AppError;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct GetAllCategoriesParams {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
    pub name: Option<String>,
}

impl GetAllCategoriesParams {
    pub fn validate(self) -> Result<ValidatedGetAllCategoriesParams, AppError> {
        let (Some(page), Some(page_size)) = (self.page, self.page_size) else {
            if (self.page.is_some() && self.page_size.is_none())
                || (self.page.is_none() && self.page_size.is_some())
            {
                return Err(AppError::ParseQueryError(String::from(
                    "Both of page and page_size must exists or not exists",
                )));
            }

            return Ok(ValidatedGetAllCategoriesParams {
                paginated: None,
                name: self.name,
            });
        };

        if page == 0 {
            return Err(AppError::ParseQueryError(String::from("page cannot be 0")));
        }
        if page_size == 0 {
            return Err(AppError::ParseQueryError(String::from(
                "page_size cannot be 0",
            )));
        }

        Ok(ValidatedGetAllCategoriesParams {
            paginated: Some(Paginated { page, page_size }),
            name: self.name,
        })
    }
}

#[derive(Clone)]
pub struct ValidatedGetAllCategoriesParams {
    pub paginated: Option<Paginated>,
    pub name: Option<String>,
}

#[derive(Clone)]
pub struct Paginated {
    pub page: u64,
    pub page_size: u64,
}
