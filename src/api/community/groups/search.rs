use futures::{Future, Stream};
use serde::Serialize;
use snafu::ResultExt;
use std::collections::VecDeque;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

use crate::{
    error::{Result, UrlParseSnafu},
    models::{Group, GroupSearchResponse},
    ArcGISSharingClient,
};

/// Builder for constructing ArcGIS group search queries.
///
/// The `GroupSearchBuilder` provides a fluent interface for configuring group search parameters
/// before executing the search. The search is executed by calling [`send()`](Self::send),
/// which returns a [`GroupSearchStream`] that yields individual group results.
///
/// # Group Search Reference
/// For detailed information, see the [ArcGIS Group Search API documentation](https://developers.arcgis.com/rest/users-groups-and-items/group-search/).
///
/// # Core Search Logic
/// - Use [`query()`](Self::query): For full-text searches, fuzzy matching, and ranking by relevance.
/// - Use [`filter()`](Self::filter): For exact values and binary (yes/no) criteria to improve performance and precision.
///
/// # Query Syntax Rules
/// - **Boolean Operators**: Must be ALL CAPS (`AND`, `OR`, `NOT`). The default is `AND`.
/// - **Phrases**: Use double quotes for exact phrases (e.g., `title:"Water Management"`).
/// - **Case Sensitivity**: Field names are case-insensitive.
/// - **Wildcards**: Use `*` for multiple characters and `?` for a single character.
/// - **Ranges**: Use `[low TO high]` for inclusive ranges and `{low TO high}` for exclusive.
/// - **Dates**: Must be in UNIX time (milliseconds) (e.g., `created:[1259692864000 TO 1260384065000]`).
///
/// # Searchable Fields for Groups
/// - **Query Fields (`q`)**: id, title, owner, description, snippet, tags, phone, created, modified, access (private|org|public), isinvitationonly (true|false), orgid, typekeywords.
/// - **Filter Fields (`filter`)**: title, typekeywords, owner.
///
/// # Example
/// ```no_run
/// # use arcgis_sharing_rs::ArcGISSharingClient;
/// # use futures::StreamExt;
/// # async fn example(client: &ArcGISSharingClient) {
/// let mut stream = client
///     .search_groups()
///     .query("water AND access:public")
///     .set_num(20)
///     .set_max_pages(5)
///     .set_sort_field("title")
///     .set_sort_order("asc")
///     .send();
///
/// while let Some(group) = stream.next().await {
///     println!("{}: {}", group.title, group.owner);
/// }
/// # }
/// ```
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupSearchBuilder<'a> {
    #[serde(skip)]
    client: &'a ArcGISSharingClient,

    #[serde(skip)]
    max_pages: usize,

    #[serde(skip)]
    page_fetch_delay: Duration,

    // The query string used to search groups.
    // See Group Search reference for advanced options.
    // Example:
    // q=water+management
    #[serde(skip_serializing_if = "Option::is_none")]
    q: Option<String>,

    // The bounding box for a spatial search defined as minx, miny, maxx, or maxy.
    // The document extent is assumed to be in the WGS84 geographic coordinate system.
    // Example:
    // bbox=-118,32,-116,34
    #[serde(skip_serializing_if = "Option::is_none")]
    bbox: Option<String>,

    // Structured filtering is accomplished by specifying a field name followed by a colon
    // and the term you are searching for with double quotation marks.
    // As a general rule, filter should be used instead of query:
    // - for binary yes/no searches.
    // - for queries on exact values.
    // See Group Search reference for advanced options.
    // Example:
    // filter=title:"Water Resources"
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<String>,

    // A JSON array or comma-separated list of up to eight organization content categories.
    #[serde(skip_serializing_if = "Option::is_none")]
    categories: Option<Vec<String>>,

    // A comma-separated list of up to three category terms to search for groups
    // that have matching categories.
    #[serde(skip_serializing_if = "Option::is_none")]
    category_filters: Option<String>,

    // The result number of the first entry in the result set response. The index number is 1-based.
    // The default value of start is 1 (in other words, the first search result).
    // The start parameter, along with the num parameter, can be used to paginate the search results.
    // Example:
    // //Returns the 11th result as the first entry in the response
    // start=11
    #[serde(skip_serializing_if = "Option::is_none")]
    start: Option<i64>,

    // The maximum number of results to be included in the result set response.
    // The default value is 10, and the maximum allowed value is 100.
    // The num parameter, along with the start parameter, can be used to paginate the search results.
    // Example:
    // //Returns a max of 50 results in the response
    // num=50
    #[serde(skip_serializing_if = "Option::is_none")]
    num: Option<i64>,

    // The field to sort by. You can also sort by multiple fields (comma separated).
    // Sort field names are not case sensitive.
    // Supported sort field names are title, owner, created, modified, numviews.
    #[serde(skip_serializing_if = "Option::is_none")]
    sort_field: Option<String>,

    // Describes whether the results are returned in ascending or descending order.
    // The default is ascending.
    // Values: asc | desc
    #[serde(skip_serializing_if = "Option::is_none")]
    sort_order: Option<String>,

    // Restricts results to only those groups that the user belongs to.
    // Values: member | admin | user
    #[serde(skip_serializing_if = "Option::is_none")]
    search_user_access: Option<String>,
}

/// A stream that yields individual `Group` results, automatically handling pagination.
///
/// The `GroupSearchStream` implements `futures::Stream` and automatically fetches additional pages
/// from the ArcGIS API as needed. Results are buffered internally and yielded one at a time.
///
/// # Automatic Pagination
/// The stream transparently handles pagination by:
/// - Fetching pages on-demand as the stream is consumed
/// - Respecting the `max_pages` limit set via [`GroupSearchBuilder::set_max_pages`]
/// - Adding a configurable delay between page fetches to avoid overwhelming the server
/// - Stopping when no more results are available (when `nextStart == -1`)
/// - Stopping silently on errors
///
/// # Usage with Stream Combinators
/// Since `GroupSearchStream` implements `Stream`, you can use all standard stream combinators:
///
/// ```no_run
/// # use arcgis_sharing_rs::ArcGISSharingClient;
/// # use futures::StreamExt;
/// # async fn example(client: &ArcGISSharingClient) {
/// // Collect all results
/// let results: Vec<_> = client
///     .search_groups()
///     .query("water")
///     .send()
///     .collect()
///     .await;
///
/// // Take only first N groups
/// let first_10: Vec<_> = client
///     .search_groups()
///     .query("water")
///     .send()
///     .take(10)
///     .collect()
///     .await;
///
/// // Filter results
/// let filtered: Vec<_> = client
///     .search_groups()
///     .query("water")
///     .send()
///     .filter(|group| std::future::ready(group.owner == "esri"))
///     .collect()
///     .await;
/// # }
/// ```
pub struct GroupSearchStream<'a> {
    client: &'a ArcGISSharingClient,
    params: GroupSearchParams,
    buffer: VecDeque<Group>,
    current_start: i64,
    next_start: i64,
    pages_fetched: usize,
    max_pages: usize,
    finished: bool,
    fetch_future: Option<Pin<Box<dyn Future<Output = Result<GroupSearchResponse>> + Send + 'a>>>,
    page_fetch_delay: Duration,
    delay_future: Option<Pin<Box<tokio::time::Sleep>>>,
}

/// Internal struct to hold group search parameters for pagination
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct GroupSearchParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    q: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bbox: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    categories: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    category_filters: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    num: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort_field: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort_order: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    search_user_access: Option<String>,
}

impl<'a> GroupSearchStream<'a> {
    fn new(
        client: &'a ArcGISSharingClient,
        params: GroupSearchParams,
        max_pages: usize,
        page_fetch_delay: Duration,
    ) -> Self {
        Self {
            client,
            params,
            buffer: VecDeque::new(),
            current_start: 1,
            next_start: 1,
            pages_fetched: 0,
            max_pages,
            finished: false,
            fetch_future: None,
            page_fetch_delay,
            delay_future: None,
        }
    }

    fn start_fetch(&mut self) {
        if self.finished || self.pages_fetched >= self.max_pages {
            return;
        }

        // Update params with current start position
        let mut params = self.params.clone();
        params.start = Some(self.current_start);

        let client = self.client;
        let portal = client.portal.clone();

        self.fetch_future = Some(Box::pin(async move {
            let url = portal
                .join("sharing/rest/community/groups")
                .context(UrlParseSnafu)?;

            client.get(url, Some(&params)).await
        }));
    }

    fn process_response(&mut self, response: GroupSearchResponse) {
        self.pages_fetched += 1;
        self.next_start = response.next_start;

        // If next_start is -1 or the same as current start, we're done
        if self.next_start == -1 || self.next_start <= self.current_start {
            self.finished = true;
        } else {
            self.current_start = self.next_start;
        }

        // Add results to buffer
        if response.results.is_empty() {
            self.finished = true;
        } else {
            self.buffer.extend(response.results);
        }
    }
}

impl<'a> Stream for GroupSearchStream<'a> {
    type Item = Group;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();

        loop {
            // If we have buffered items, return one
            if let Some(group) = this.buffer.pop_front() {
                return Poll::Ready(Some(group));
            }

            // If finished, return None
            if this.finished {
                return Poll::Ready(None);
            }

            // If we have a pending delay, poll it
            if let Some(mut delay_future) = this.delay_future.take() {
                match delay_future.as_mut().poll(cx) {
                    Poll::Ready(()) => {
                        // Delay completed, continue to fetch
                        continue;
                    }
                    Poll::Pending => {
                        // Put the delay future back and return Pending
                        this.delay_future = Some(delay_future);
                        return Poll::Pending;
                    }
                }
            }

            // If we have a pending fetch, poll it
            if let Some(mut fetch_future) = this.fetch_future.take() {
                match fetch_future.as_mut().poll(cx) {
                    Poll::Ready(Ok(response)) => {
                        // Process the response
                        this.process_response(response);

                        // Start delay before next fetch if we're not finished and have more pages
                        if !this.finished
                            && this.pages_fetched > 0
                            && !this.page_fetch_delay.is_zero()
                        {
                            this.delay_future =
                                Some(Box::pin(tokio::time::sleep(this.page_fetch_delay)));
                        }

                        // Continue loop to check if buffer has items or if we're done
                        continue;
                    }
                    Poll::Ready(Err(_)) => {
                        // Error occurred, stop the stream
                        this.finished = true;
                        return Poll::Ready(None);
                    }
                    Poll::Pending => {
                        // Put the future back and return Pending
                        this.fetch_future = Some(fetch_future);
                        return Poll::Pending;
                    }
                }
            }

            // No fetch in progress, start a new one
            this.start_fetch();

            // If we couldn't start a fetch (finished or max_pages reached), return None
            if this.fetch_future.is_none() {
                return Poll::Ready(None);
            }
        }
    }
}

impl<'a> GroupSearchBuilder<'a> {
    pub(crate) fn new(client: &'a ArcGISSharingClient) -> Self {
        Self {
            client,
            max_pages: usize::MAX, // Default to unlimited
            page_fetch_delay: Duration::from_millis(100), // Default 0.1 second delay
            q: None,
            bbox: None,
            filter: None,
            categories: None,
            category_filters: None,
            start: None,
            num: None,
            sort_field: None,
            sort_order: None,
            search_user_access: None,
        }
    }

    /// Sets the query string for the group search.
    ///
    /// See [Group Search Reference](https://developers.arcgis.com/rest/users-groups-and-items/group-search/) for advanced options.
    ///
    /// # Example
    /// ```no_run
    /// # use arcgis_sharing_rs::ArcGISSharingClient;
    /// # async fn example(client: &ArcGISSharingClient) {
    /// let stream = client.search_groups().query("water management").send();
    /// # }
    /// ```
    pub fn query(mut self, query: impl Into<String>) -> Self {
        self.q = Some(query.into());
        self
    }

    /// Sets the maximum number of pages to fetch.
    ///
    /// By default, the stream will fetch all available pages. Use this to limit
    /// the total number of pages fetched, which also limits the total results to
    /// `max_pages * num` groups.
    ///
    /// # Example
    /// ```no_run
    /// # use arcgis_sharing_rs::ArcGISSharingClient;
    /// # async fn example(client: &ArcGISSharingClient) {
    /// let stream = client
    ///     .search_groups()
    ///     .query("water")
    ///     .set_num(10)
    ///     .set_max_pages(5) // Max 50 groups (5 pages * 10 groups/page)
    ///     .send();
    /// # }
    /// ```
    pub fn set_max_pages(mut self, max_pages: usize) -> Self {
        self.max_pages = max_pages;
        self
    }

    /// Sets the delay between page fetches.
    ///
    /// By default, a 100ms (0.1 second) delay is used between page fetches to avoid
    /// overwhelming the server with rapid requests. Set to `Duration::ZERO` to disable
    /// the delay.
    ///
    /// # Example
    /// ```no_run
    /// # use arcgis_sharing_rs::ArcGISSharingClient;
    /// # use std::time::Duration;
    /// # async fn example(client: &ArcGISSharingClient) {
    /// // Use a 1 second delay between pages
    /// let stream = client
    ///     .search_groups()
    ///     .query("water")
    ///     .set_page_fetch_delay(Duration::from_secs(1))
    ///     .send();
    ///
    /// // Disable delay for faster fetching (be careful with server load)
    /// let stream = client
    ///     .search_groups()
    ///     .query("water")
    ///     .set_page_fetch_delay(Duration::ZERO)
    ///     .send();
    /// # }
    /// ```
    pub fn set_page_fetch_delay(mut self, delay: Duration) -> Self {
        self.page_fetch_delay = delay;
        self
    }

    /// Sets the bounding box for a spatial search (minx, miny, maxx, maxy).
    ///
    /// The document extent is assumed to be in the WGS84 geographic coordinate system.
    ///
    /// # Example
    /// ```no_run
    /// # use arcgis_sharing_rs::ArcGISSharingClient;
    /// # async fn example(client: &ArcGISSharingClient) {
    /// let stream = client
    ///     .search_groups()
    ///     .query("parks")
    ///     .set_bbox("-118,32,-116,34")
    ///     .send();
    /// # }
    /// ```
    pub fn set_bbox(mut self, bbox: impl Into<String>) -> Self {
        self.bbox = Some(bbox.into());
        self
    }

    /// Sets structured filters for the search.
    ///
    /// Structured filtering is accomplished by specifying a field name followed by a colon
    /// and the term you are searching for with double quotation marks. Use an exact keyword
    /// match of the expected value for the specified field.
    ///
    /// As a general rule, filter should be used instead of query:
    /// - for binary yes/no searches
    /// - for queries on exact values
    ///
    /// # Example
    /// ```no_run
    /// # use arcgis_sharing_rs::ArcGISSharingClient;
    /// # async fn example(client: &ArcGISSharingClient) {
    /// let stream = client
    ///     .search_groups()
    ///     .query("water")
    ///     .filter("title:\"Water Resources\"")
    ///     .send();
    /// # }
    /// ```
    pub fn filter(mut self, filter: impl Into<String>) -> Self {
        self.filter = Some(filter.into());
        self
    }

    /// Sets organization content categories to search groups.
    ///
    /// A JSON array or comma-separated list of up to eight organization content categories.
    /// The exact full path of each category is required.
    ///
    /// # Example
    /// ```no_run
    /// # use arcgis_sharing_rs::ArcGISSharingClient;
    /// # async fn example(client: &ArcGISSharingClient) {
    /// let stream = client
    ///     .search_groups()
    ///     .query("water")
    ///     .set_categories(vec![
    ///         "/Categories/Water".to_string(),
    ///         "/Categories/Environment".to_string()
    ///     ])
    ///     .send();
    /// # }
    /// ```
    pub fn set_categories(mut self, categories: Vec<String>) -> Self {
        self.categories = Some(categories);
        self
    }

    /// Sets category terms to search for groups that have matching categories.
    ///
    /// A comma-separated list of up to three category terms.
    ///
    /// # Example
    /// ```no_run
    /// # use arcgis_sharing_rs::ArcGISSharingClient;
    /// # async fn example(client: &ArcGISSharingClient) {
    /// let stream = client
    ///     .search_groups()
    ///     .query("water")
    ///     .set_category_filters("water, environment")
    ///     .send();
    /// # }
    /// ```
    pub fn set_category_filters(mut self, filters: impl Into<String>) -> Self {
        self.category_filters = Some(filters.into());
        self
    }

    /// Sets the result number of the first entry in the result set.
    ///
    /// The index number is 1-based. The default value is 1 (the first search result).
    ///
    /// Note: When using the stream, pagination is handled automatically. This parameter
    /// is typically only needed if you want to skip initial results.
    ///
    /// # Example
    /// ```no_run
    /// # use arcgis_sharing_rs::ArcGISSharingClient;
    /// # async fn example(client: &ArcGISSharingClient) {
    /// let stream = client
    ///     .search_groups()
    ///     .query("water")
    ///     .set_start(11) // Start from the 11th result
    ///     .send();
    /// # }
    /// ```
    pub fn set_start(mut self, start: i64) -> Self {
        self.start = Some(start);
        self
    }

    /// Sets the maximum number of results per page.
    ///
    /// The default value is 10, and the maximum allowed value is 100.
    ///
    /// # Example
    /// ```no_run
    /// # use arcgis_sharing_rs::ArcGISSharingClient;
    /// # async fn example(client: &ArcGISSharingClient) {
    /// let stream = client
    ///     .search_groups()
    ///     .query("water")
    ///     .set_num(50) // Fetch 50 groups per page
    ///     .send();
    /// # }
    /// ```
    pub fn set_num(mut self, num: i64) -> Self {
        self.num = Some(num);
        self
    }

    /// Sets the field to sort results by.
    ///
    /// You can also sort by multiple fields (comma separated). Sort field names are not
    /// case sensitive.
    ///
    /// Supported sort field names: `title`, `owner`, `created`, `modified`, `numviews`.
    ///
    /// # Example
    /// ```no_run
    /// # use arcgis_sharing_rs::ArcGISSharingClient;
    /// # async fn example(client: &ArcGISSharingClient) {
    /// let stream = client
    ///     .search_groups()
    ///     .query("water")
    ///     .set_sort_field("modified")
    ///     .set_sort_order("desc")
    ///     .send();
    /// # }
    /// ```
    pub fn set_sort_field(mut self, field: impl Into<String>) -> Self {
        self.sort_field = Some(field.into());
        self
    }

    /// Sets whether results are returned in ascending or descending order.
    ///
    /// The default is ascending. This applies when working with `set_sort_field`.
    ///
    /// # Values
    /// - `"asc"` - Ascending order
    /// - `"desc"` - Descending order
    ///
    /// # Example
    /// ```no_run
    /// # use arcgis_sharing_rs::ArcGISSharingClient;
    /// # async fn example(client: &ArcGISSharingClient) {
    /// let stream = client
    ///     .search_groups()
    ///     .query("water")
    ///     .set_sort_field("created")
    ///     .set_sort_order("desc")
    ///     .send();
    /// # }
    /// ```
    pub fn set_sort_order(mut self, order: impl Into<String>) -> Self {
        self.sort_order = Some(order.into());
        self
    }

    /// Restricts results to only those groups that the user belongs to.
    ///
    /// # Values
    /// - `"member"` - Groups the user is a member of
    /// - `"admin"` - Groups the user is an admin of
    /// - `"user"` - Groups owned by the user
    ///
    /// # Example
    /// ```no_run
    /// # use arcgis_sharing_rs::ArcGISSharingClient;
    /// # async fn example(client: &ArcGISSharingClient) {
    /// let stream = client
    ///     .search_groups()
    ///     .query("water")
    ///     .set_search_user_access("member")
    ///     .send();
    /// # }
    /// ```
    pub fn set_search_user_access(mut self, access: impl Into<String>) -> Self {
        self.search_user_access = Some(access.into());
        self
    }

    fn to_params(&self) -> GroupSearchParams {
        GroupSearchParams {
            q: self.q.clone(),
            bbox: self.bbox.clone(),
            filter: self.filter.clone(),
            categories: self.categories.clone(),
            category_filters: self.category_filters.clone(),
            start: self.start,
            num: self.num,
            sort_field: self.sort_field.clone(),
            sort_order: self.sort_order.clone(),
            search_user_access: self.search_user_access.clone(),
        }
    }

    /// Executes the group search and returns a stream of group results.
    ///
    /// The stream automatically handles pagination and yields individual `Group` items.
    /// Results are fetched on-demand as the stream is consumed.
    ///
    /// # Returns
    /// A `GroupSearchStream` that implements `futures::Stream<Item = Group>`.
    ///
    /// # Example
    /// ```no_run
    /// # use arcgis_sharing_rs::ArcGISSharingClient;
    /// # use futures::StreamExt;
    /// # async fn example(client: &ArcGISSharingClient) {
    /// let mut stream = client
    ///     .search_groups()
    ///     .query("water")
    ///     .set_num(10)
    ///     .set_max_pages(5)
    ///     .send();
    ///
    /// while let Some(group) = stream.next().await {
    ///     println!("{}: {}", group.title, group.owner);
    /// }
    ///
    /// // Or collect all results into a Vec
    /// let results: Vec<_> = client
    ///     .search_groups()
    ///     .query("basemap")
    ///     .send()
    ///     .collect()
    ///     .await;
    /// # }
    /// ```
    pub fn send(self) -> GroupSearchStream<'a> {
        GroupSearchStream::new(
            self.client,
            self.to_params(),
            self.max_pages,
            self.page_fetch_delay,
        )
    }
}
