use serde::{Serialize, Deserialize};

/// A DTO to represent pagination information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationDto {
    /// Current page (starts at 0)
    pub page: usize,
    /// Page size
    pub page_size: usize,
    /// Total number of items
    pub total_items: usize,
    /// Total number of pages
    pub total_pages: usize,
    /// Indicates if there is a next page
    pub has_next: bool,
    /// Indicates if there is a previous page
    pub has_prev: bool,
}

/// A DTO to represent a pagination request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationRequestDto {
    /// Requested page (starts at 0)
    #[serde(default)]
    pub page: usize,
    /// Requested page size
    #[serde(default = "default_page_size")]
    pub page_size: usize,
}

/// A DTO to represent a paginated response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponseDto<T> {
    /// Data on the current page
    pub items: Vec<T>,
    /// Pagination information
    pub pagination: PaginationDto,
}

impl Default for PaginationRequestDto {
    fn default() -> Self {
        Self {
            page: 0,
            page_size: default_page_size(),
        }
    }
}

/// Function to set the default page size
fn default_page_size() -> usize {
    100 // By default, 100 items per page
}

impl PaginationRequestDto {
    /// Calculates the offset for paginated queries
    pub fn offset(&self) -> usize {
        self.page * self.page_size
    }
    
    /// Calculates the limit for paginated queries
    pub fn limit(&self) -> usize {
        self.page_size
    }
    
    /// Validates and adjusts the pagination parameters
    pub fn validate_and_adjust(&self) -> Self {
        let mut page = self.page;
        let mut page_size = self.page_size;
        
        // Ensure the page is at least 0
        if page < 1 {
            page = 0;
        }
        
        // Ensure the page size is between 10 and 500
        if page_size < 10 {
            page_size = 10;
        } else if page_size > 500 {
            page_size = 500;
        }
        
        Self {
            page,
            page_size,
        }
    }
}

impl<T> PaginatedResponseDto<T> {
    /// Creates a new paginated response from the data and pagination information
    pub fn new(
        items: Vec<T>,
        page: usize,
        page_size: usize,
        total_items: usize,
    ) -> Self {
        let total_pages = if total_items == 0 {
            0
        } else {
            total_items.div_ceil(page_size)
        };
        
        let pagination = PaginationDto {
            page,
            page_size,
            total_items,
            total_pages,
            has_next: page < total_pages - 1,
            has_prev: page > 0,
        };
        
        Self {
            items,
            pagination,
        }
    }
}