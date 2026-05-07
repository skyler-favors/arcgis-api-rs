use futures::{Future, Stream};
use serde::Serialize;
use snafu::ResultExt;
use std::collections::VecDeque;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

use crate::{
    error::{Result, UrlParseSnafu},
    models::{Item, SearchResponse},
    ArcGISSharingClient,
};

/// Builder for constructing ArcGIS search queries.
///
/// The `SearchBuilder` provides a fluent interface for configuring search parameters
/// before executing the search. The search is executed by calling [`send()`](Self::send),
/// which returns a [`SearchStream`] that yields individual search results.
///
/// # Search Reference
/// For detailed information, see the [ArcGIS Search API documentation](https://developers.arcgis.com/rest/users-groups-and-items/search-reference/).
///
/// # Core Search Logic
/// - Use [`query()`](Self::query): For full-text searches, fuzzy matching, and ranking by relevance.
/// - Use [`filter()`](Self::filter): For exact values and binary (yes/no) criteria to improve performance and precision.
///
/// # Query Syntax Rules
/// - **Boolean Operators**: Must be ALL CAPS (`AND`, `OR`, `NOT`). The default is `AND`.
/// - **Phrases**: Use double quotes for exact phrases (e.g., `title:"San Francisco"`).
/// - **Case Sensitivity**: Field names are case-insensitive, but Item Type values are case-sensitive and must be quoted (e.g., `type:"Web Map"`).
/// - **Wildcards**: Use `*` for multiple characters and `?` for a single character.
/// - **Ranges**: Use `[low TO high]` for inclusive ranges and `{low TO high}` for exclusive.
/// - **Dates**: Must be in UNIX time (milliseconds) (e.g., `created:[1259692864000 TO 1260384065000]`).
///
/// # Searchable Fields
///
/// ## For Items (Content Search)
/// - **Query Fields (`q`)**: id, owner, created, modified, title, type, typekeywords, description, tags, snippet, accessinformation, access (public|private|org|shared), group, numratings, numcomments, avgrating, culture, orgid, categories, contentStatus, classification.
/// - **Filter Fields (`filter`)**: title, tags, typekeywords, type, owner.
///
/// ## For Groups
/// - **Query Fields (`q`)**: id, title, owner, description, snippet, tags, phone, created, modified, access (private|org|public), isinvitationonly (true|false), orgid, typekeywords.
/// - **Filter Fields (`filter`)**: title, typekeywords, owner.
///
/// ## For Users
/// - **Filter Fields (`filter`)**: username, firstname, lastname, fullname, email.
///
/// # Example
/// ```no_run
/// # use arcgis_sharing_rs::ArcGISSharingClient;
/// # use futures::StreamExt;
/// # async fn example(client: &ArcGISSharingClient) {
/// let mut stream = client
///     .search()
///     .query("water AND type:\"Feature Service\"")
///     .set_num(20)
///     .set_max_pages(5)
///     .set_sort_field("modified")
///     .set_sort_order("desc")
///     .send();
///
/// while let Some(result) = stream.next().await {
///     println!("{}: {}", result.title, result.owner);
/// }
/// # }
/// ```
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchBuilder<'a> {
    #[serde(skip)]
    client: &'a ArcGISSharingClient,

    #[serde(skip)]
    max_pages: usize,

    #[serde(skip)]
    page_fetch_delay: Duration,

    // The query string used to search.
    // See Search reference for advanced options.
    // Example:
    // q=redlands+map
    #[serde(skip_serializing_if = "Option::is_none")]
    q: Option<String>,

    // TODO: should not be a string
    // The bounding box for a spatial search defined as minx, miny, maxx, or maxy. Spatial search is an overlaps/intersects function of the query bbox and the extent of the document. Documents that have no extent (for example, .mxd, .3ds, and .lyr) will not be found when doing a bbox search. The document extent is assumed to be in the WGS84 geographic coordinate system.
    // Query should be used instead of filter:
    // - for full-text search.
    // - for cases where the result depends on a relevance score.
    // Example:
    // bbox=-118,32,-116,34
    #[serde(skip_serializing_if = "Option::is_none")]
    bbox: Option<String>,

    // Structured filtering is accomplished by specifying a field name followed by a colon and the term you are searching for with double quotation marks. It allows the passing in of application-level filters based on the context. Use an exact keyword match of the expected value for the specified field. Partially matching the filter keyword will not return meaningful results.
    // As a general rule, filter should be used instead of query:
    // - for binary yes/no searches.
    // - for queries on exact values.
    // See Search reference for advanced options.
    // Example:
    // filter=type:"Web map"
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<String>,

    // A JSON array or comma-separated list of up to eight organization content categories to search items. The exact full path of each category is required, and an OR relationship between the categories must be specified. Each request allows a maximum of eight categories parameters with an AND relationship between the various categories parameters called.
    // Example (search for items with the water or forest category in the United States):
    // //JSON array
    // categories: ["/Categories/Water", "/Categories/Forest"]
    // categories: ["/Region/US"]
    //Note
    // / are reserved and can't be used in the category names. If commas are included in the categories, the user must use JSON format.
    #[serde(skip_serializing_if = "Option::is_none")]
    categories: Option<Vec<String>>,

    // TODO: should be a Vec<String>
    // A comma-separated list of up to three category terms to search for items that have matching categories. Up to two categoryFilters parameters are allowed per request. This parameter cannot be used with the categories parameter to search in a request.
    // Example:
    // //Search for items with categories that include basemap or ocean
    // categoryFilters=basemap, ocean
    #[serde(skip_serializing_if = "Option::is_none")]
    category_filters: Option<String>,

    // The result number of the first entry in the result set response. The index number is 1-based. The default value of start is 1 (in other words, the first search result). The start parameter, along with the num parameter, can be used to paginate the search results.
    // Example:
    // //Returns the 11th result as the first entry in the response
    // start=11
    #[serde(skip_serializing_if = "Option::is_none")]
    start: Option<i64>,

    // The maximum number of results to be included in the result set response. The default value is 10, and the maximum allowed value is 100. The num parameter, along with the start parameter, can be used to paginate the search results.
    // Note
    // The actual number of returned results may be fewer than the num value. This happens when the number of results remaining after start is fewer than the num value. num must be 0 if you are interested in the total item counts or aggregations matching a query with countFields and countSize values specified. Do not include results and aggregations in the same request; the results array will be empty when num=0.
    // Example:
    // //Returns a max of 50 results in the response
    // num=50
    #[serde(skip_serializing_if = "Option::is_none")]
    num: Option<i64>,

    // TODO: enum
    // The field to sort by. You can also sort by multiple fields (comma separated). Sort field names are not case sensitive.
    // Supported sort field names are title, created, type, owner, modified, avgrating, numratings, numcomments, numviews, and scorecompleteness.
    #[serde(skip_serializing_if = "Option::is_none")]
    sort_field: Option<String>,

    // TODO: enum
    // Describes whether the results are returned in ascending or descending order. The default is ascending.
    // Note
    // This applies when working with sortField.
    // Values: asc | desc
    #[serde(skip_serializing_if = "Option::is_none")]
    sort_order: Option<String>,

    // TODO: enum
    // A comma-separated list of fields to count. The maximum count fields allowed per request is three. Supported count fields are type, access, contentstatus, and categories.
    // Example:
    // countFields=categories, access
    #[serde(skip_serializing_if = "Option::is_none")]
    count_fields: Option<String>,

    // TODO: limit 0 - 200
    // The maximum number of field values to count for each countFields field. The default value is 10, and the maximum number allowed is 200.
    // Example:
    // countSize=200
    #[serde(skip_serializing_if = "Option::is_none")]
    count_size: Option<i64>,

    // TODO: validation 20 fields
    // TODO: should be a Vec<String>
    // Excludes fields from the search results. The maximum number of fields that can be excluded is 20.
    // Example:
    // exclude=title,culture,numViews
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude: Option<String>,

    // Returns feature layers for a hosted feature service. The default is false.
    // Values: true | false
    #[serde(skip_serializing_if = "Option::is_none")]
    display_sublayers: Option<bool>,

    // Introduced at ArcGIS Enterprise 11.1. Specifies whether the search results will include both literal and relevant matches or only literal matches. A literal match is defined as having the search criteria be present in an item's title or tag. A related match is defined as having a term related to the search criteria being present in an item's tag. If true, the search results will include both literal and relevant matches. If false, search results will include only those that are a literal match for the search criteria. Searches that do not include this parameter will only return literal matches.
    // Values: true | false
    #[serde(skip_serializing_if = "Option::is_none")]
    enriched: Option<bool>,

    // An integer representing mean size of image pixels, in meters. The formula used to calculate this: (PixelSizeX + PixelSizeY) / 2 * unitToMeterFactor.
    // Example to find single-band imagery based on resolution (meters):
    // type: "Image Service" AND MeanPixelSize:[0 TO 1]
    #[serde(rename = "MeanPixelSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    mean_pixel_size: Option<i64>,

    // An integer representing the number of bands in the imagery.
    // Example to find imagery based on band count:
    // type: "Image Service" AND bandcount:1   //Single band
    // type: "Image Service" AND bandcount:[3 TO 20]   //Multi-band
    #[serde(rename = "BandCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    band_count: Option<i64>,

    // Displays image service properties. The default is false.
    // Values: true | false
    #[serde(skip_serializing_if = "Option::is_none")]
    display_service_properties: Option<bool>,
}

/// A stream that yields individual `Item` results, automatically handling pagination.
///
/// The `SearchStream` implements `futures::Stream` and automatically fetches additional pages
/// from the ArcGIS API as needed. Results are buffered internally and yielded one at a time.
///
/// # Automatic Pagination
/// The stream transparently handles pagination by:
/// - Fetching pages on-demand as the stream is consumed
/// - Respecting the `max_pages` limit set via [`SearchBuilder::set_max_pages`]
/// - Adding a configurable delay between page fetches to avoid overwhelming the server
/// - Stopping when no more results are available (when `nextStart == -1`)
/// - Stopping silently on errors
///
/// # Usage with Stream Combinators
/// Since `SearchStream` implements `Stream`, you can use all standard stream combinators:
///
/// ```no_run
/// # use arcgis_sharing_rs::ArcGISSharingClient;
/// # use futures::StreamExt;
/// # async fn example(client: &ArcGISSharingClient) {
/// // Collect all results
/// let results: Vec<_> = client
///     .search()
///     .query("water")
///     .send()
///     .collect()
///     .await;
///
/// // Take only first N items
/// let first_10: Vec<_> = client
///     .search()
///     .query("water")
///     .send()
///     .take(10)
///     .collect()
///     .await;
///
/// // Filter results
/// let filtered: Vec<_> = client
///     .search()
///     .query("water")
///     .send()
///     .filter(|result| std::future::ready(result.owner == "esri"))
///     .collect()
///     .await;
/// # }
/// ```
pub struct SearchStream<'a> {
    client: &'a ArcGISSharingClient,
    params: SearchParams,
    buffer: VecDeque<Item>,
    current_start: i64,
    next_start: i64,
    pages_fetched: usize,
    max_pages: usize,
    finished: bool,
    fetch_future: Option<Pin<Box<dyn Future<Output = Result<SearchResponse>> + Send + 'a>>>,
    page_fetch_delay: Duration,
    delay_future: Option<Pin<Box<tokio::time::Sleep>>>,
}

/// Internal struct to hold search parameters for pagination
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct SearchParams {
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
    count_fields: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    count_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_sublayers: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enriched: Option<bool>,
    #[serde(rename = "MeanPixelSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    mean_pixel_size: Option<i64>,
    #[serde(rename = "BandCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    band_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_service_properties: Option<bool>,
}

impl<'a> SearchStream<'a> {
    fn new(
        client: &'a ArcGISSharingClient,
        params: SearchParams,
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
            let url = portal.join("sharing/rest/search").context(UrlParseSnafu)?;

            client.get(url, Some(&params)).await
        }));
    }

    fn process_response(&mut self, response: SearchResponse) {
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

impl<'a> Stream for SearchStream<'a> {
    type Item = Item;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();

        loop {
            // If we have buffered items, return one
            if let Some(item) = this.buffer.pop_front() {
                return Poll::Ready(Some(item));
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

impl<'a> SearchBuilder<'a> {
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
            count_fields: None,
            count_size: None,
            exclude: None,
            display_sublayers: None,
            enriched: None,
            mean_pixel_size: None,
            band_count: None,
            display_service_properties: None,
        }
    }

    /// Sets the query string for the search.
    ///
    /// See [Search Reference](https://developers.arcgis.com/rest/users-groups-and-items/search-reference/) for advanced options.
    ///
    /// # Example
    /// ```no_run
    /// # use arcgis_sharing_rs::ArcGISSharingClient;
    /// # async fn example(client: &ArcGISSharingClient) {
    /// let stream = client.search().query("redlands map").send();
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
    /// `max_pages * num` items.
    ///
    /// # Example
    /// ```no_run
    /// # use arcgis_sharing_rs::ArcGISSharingClient;
    /// # async fn example(client: &ArcGISSharingClient) {
    /// let stream = client
    ///     .search()
    ///     .query("water")
    ///     .set_num(10)
    ///     .set_max_pages(5) // Max 50 items (5 pages * 10 items/page)
    ///     .send();
    /// # }
    /// ```
    pub fn set_max_pages(mut self, max_pages: usize) -> Self {
        self.max_pages = max_pages;
        self
    }

    /// Sets the delay between page fetches.
    ///
    /// By default, a 500ms (0.5 second) delay is used between page fetches to avoid
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
    ///     .search()
    ///     .query("water")
    ///     .set_page_fetch_delay(Duration::from_secs(1))
    ///     .send();
    ///
    /// // Disable delay for faster fetching (be careful with server load)
    /// let stream = client
    ///     .search()
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
    /// Spatial search is an overlaps/intersects function of the query bbox and the extent
    /// of the document. Documents that have no extent (e.g., .mxd, .3ds, .lyr) will not be
    /// found when doing a bbox search. The document extent is assumed to be in the WGS84
    /// geographic coordinate system.
    ///
    /// # Example
    /// ```no_run
    /// # use arcgis_sharing_rs::ArcGISSharingClient;
    /// # async fn example(client: &ArcGISSharingClient) {
    /// let stream = client
    ///     .search()
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
    /// match of the expected value for the specified field. Partially matching the filter
    /// keyword will not return meaningful results.
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
    ///     .search()
    ///     .query("water")
    ///     .filter("type:\"Web Map\"")
    ///     .send();
    /// # }
    /// ```
    pub fn filter(mut self, filter: impl Into<String>) -> Self {
        self.filter = Some(filter.into());
        self
    }

    /// Sets organization content categories to search items.
    ///
    /// A JSON array or comma-separated list of up to eight organization content categories.
    /// The exact full path of each category is required, and an OR relationship between the
    /// categories is specified. Each request allows a maximum of eight categories parameters
    /// with an AND relationship between the various categories parameters called.
    ///
    /// Note: `/` are reserved and can't be used in the category names.
    ///
    /// # Example
    /// ```no_run
    /// # use arcgis_sharing_rs::ArcGISSharingClient;
    /// # async fn example(client: &ArcGISSharingClient) {
    /// let stream = client
    ///     .search()
    ///     .query("water")
    ///     .set_categories(vec![
    ///         "/Categories/Water".to_string(),
    ///         "/Categories/Forest".to_string()
    ///     ])
    ///     .send();
    /// # }
    /// ```
    pub fn set_categories(mut self, categories: Vec<String>) -> Self {
        self.categories = Some(categories);
        self
    }

    /// Sets category terms to search for items that have matching categories.
    ///
    /// A comma-separated list of up to three category terms. Up to two categoryFilters
    /// parameters are allowed per request. This parameter cannot be used with the categories
    /// parameter in the same request.
    ///
    /// # Example
    /// ```no_run
    /// # use arcgis_sharing_rs::ArcGISSharingClient;
    /// # async fn example(client: &ArcGISSharingClient) {
    /// let stream = client
    ///     .search()
    ///     .query("water")
    ///     .set_category_filters("basemap, ocean")
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
    /// The start parameter, along with the num parameter, can be used to paginate the
    /// search results.
    ///
    /// Note: When using the stream, pagination is handled automatically. This parameter
    /// is typically only needed if you want to skip initial results.
    ///
    /// # Example
    /// ```no_run
    /// # use arcgis_sharing_rs::ArcGISSharingClient;
    /// # async fn example(client: &ArcGISSharingClient) {
    /// let stream = client
    ///     .search()
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
    /// The default value is 10, and the maximum allowed value is 100. The num parameter,
    /// along with the start parameter, can be used to paginate the search results.
    ///
    /// Note: The actual number of returned results may be fewer than the num value. This
    /// happens when the number of results remaining after start is fewer than the num value.
    ///
    /// # Example
    /// ```no_run
    /// # use arcgis_sharing_rs::ArcGISSharingClient;
    /// # async fn example(client: &ArcGISSharingClient) {
    /// let stream = client
    ///     .search()
    ///     .query("water")
    ///     .set_num(50) // Fetch 50 items per page
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
    /// Supported sort field names: `title`, `created`, `type`, `owner`, `modified`,
    /// `avgrating`, `numratings`, `numcomments`, `numviews`, and `scorecompleteness`.
    ///
    /// # Example
    /// ```no_run
    /// # use arcgis_sharing_rs::ArcGISSharingClient;
    /// # async fn example(client: &ArcGISSharingClient) {
    /// let stream = client
    ///     .search()
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
    ///     .search()
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

    /// Specifies whether search results include both literal and relevant matches.
    ///
    /// Introduced at ArcGIS Enterprise 11.1. A literal match is defined as having the search
    /// criteria be present in an item's title or tag. A related match is defined as having a
    /// term related to the search criteria being present in an item's tag.
    ///
    /// - If `true`, the search results will include both literal and relevant matches.
    /// - If `false`, search results will include only literal matches for the search criteria.
    /// - If not specified, searches will only return literal matches.
    ///
    /// # Example
    /// ```no_run
    /// # use arcgis_sharing_rs::ArcGISSharingClient;
    /// # async fn example(client: &ArcGISSharingClient) {
    /// let stream = client
    ///     .search()
    ///     .query("water")
    ///     .set_enriched(true)
    ///     .send();
    /// # }
    /// ```
    pub fn set_enriched(mut self, enriched: bool) -> Self {
        self.enriched = Some(enriched);
        self
    }

    /// Sets fields to count in the search results.
    ///
    /// A comma-separated list of fields to count. The maximum count fields allowed per
    /// request is three.
    ///
    /// Supported count fields: `type`, `access`, `contentstatus`, and `categories`.
    ///
    /// # Example
    /// ```no_run
    /// # use arcgis_sharing_rs::ArcGISSharingClient;
    /// # async fn example(client: &ArcGISSharingClient) {
    /// let stream = client
    ///     .search()
    ///     .query("water")
    ///     .set_count_fields("categories, access")
    ///     .send();
    /// # }
    /// ```
    pub fn set_count_fields(mut self, fields: impl Into<String>) -> Self {
        self.count_fields = Some(fields.into());
        self
    }

    /// Sets the maximum number of field values to count for each countFields field.
    ///
    /// The default value is 10, and the maximum number allowed is 200.
    ///
    /// # Example
    /// ```no_run
    /// # use arcgis_sharing_rs::ArcGISSharingClient;
    /// # async fn example(client: &ArcGISSharingClient) {
    /// let stream = client
    ///     .search()
    ///     .query("water")
    ///     .set_count_fields("type")
    ///     .set_count_size(50)
    ///     .send();
    /// # }
    /// ```
    pub fn set_count_size(mut self, size: i64) -> Self {
        self.count_size = Some(size);
        self
    }

    /// Excludes fields from the search results.
    ///
    /// A comma-separated list of field names. The maximum number of fields that can be
    /// excluded is 20.
    ///
    /// # Example
    /// ```no_run
    /// # use arcgis_sharing_rs::ArcGISSharingClient;
    /// # async fn example(client: &ArcGISSharingClient) {
    /// let stream = client
    ///     .search()
    ///     .query("water")
    ///     .set_exclude("title,culture,numViews")
    ///     .send();
    /// # }
    /// ```
    pub fn set_exclude(mut self, fields: impl Into<String>) -> Self {
        self.exclude = Some(fields.into());
        self
    }

    /// Returns feature layers for a hosted feature service.
    ///
    /// The default is `false`.
    ///
    /// # Example
    /// ```no_run
    /// # use arcgis_sharing_rs::ArcGISSharingClient;
    /// # async fn example(client: &ArcGISSharingClient) {
    /// let stream = client
    ///     .search()
    ///     .query("type:\"Feature Service\"")
    ///     .set_display_sublayers(true)
    ///     .send();
    /// # }
    /// ```
    pub fn set_display_sublayers(mut self, display: bool) -> Self {
        self.display_sublayers = Some(display);
        self
    }

    /// Sets the mean pixel size filter for image services.
    ///
    /// An integer representing mean size of image pixels, in meters. The formula used to
    /// calculate this: `(PixelSizeX + PixelSizeY) / 2 * unitToMeterFactor`.
    ///
    /// # Example
    /// To find single-band imagery based on resolution (meters):
    /// ```no_run
    /// # use arcgis_sharing_rs::ArcGISSharingClient;
    /// # async fn example(client: &ArcGISSharingClient) {
    /// let stream = client
    ///     .search()
    ///     .query("type:\"Image Service\" AND MeanPixelSize:[0 TO 1]")
    ///     .send();
    /// # }
    /// ```
    pub fn set_mean_pixel_size(mut self, size: i64) -> Self {
        self.mean_pixel_size = Some(size);
        self
    }

    /// Sets the band count filter for image services.
    ///
    /// An integer representing the number of bands in the imagery.
    ///
    /// # Example
    /// ```no_run
    /// # use arcgis_sharing_rs::ArcGISSharingClient;
    /// # async fn example(client: &ArcGISSharingClient) {
    /// // Find single-band imagery
    /// let stream = client
    ///     .search()
    ///     .query("type:\"Image Service\" AND bandcount:1")
    ///     .send();
    ///
    /// // Find multi-band imagery
    /// let stream = client
    ///     .search()
    ///     .query("type:\"Image Service\" AND bandcount:[3 TO 20]")
    ///     .send();
    /// # }
    /// ```
    pub fn set_band_count(mut self, count: i64) -> Self {
        self.band_count = Some(count);
        self
    }

    /// Displays image service properties in the results.
    ///
    /// The default is `false`.
    ///
    /// # Example
    /// ```no_run
    /// # use arcgis_sharing_rs::ArcGISSharingClient;
    /// # async fn example(client: &ArcGISSharingClient) {
    /// let stream = client
    ///     .search()
    ///     .query("type:\"Image Service\"")
    ///     .set_display_service_properties(true)
    ///     .send();
    /// # }
    /// ```
    pub fn set_display_service_properties(mut self, display: bool) -> Self {
        self.display_service_properties = Some(display);
        self
    }

    fn to_params(&self) -> SearchParams {
        SearchParams {
            q: self.q.clone(),
            bbox: self.bbox.clone(),
            filter: self.filter.clone(),
            categories: self.categories.clone(),
            category_filters: self.category_filters.clone(),
            start: self.start,
            num: self.num,
            sort_field: self.sort_field.clone(),
            sort_order: self.sort_order.clone(),
            count_fields: self.count_fields.clone(),
            count_size: self.count_size,
            exclude: self.exclude.clone(),
            display_sublayers: self.display_sublayers,
            enriched: self.enriched,
            mean_pixel_size: self.mean_pixel_size,
            band_count: self.band_count,
            display_service_properties: self.display_service_properties,
        }
    }

    /// Executes the search and returns a stream of search results.
    ///
    /// The stream automatically handles pagination and yields individual `SearchResult` items.
    /// Results are fetched on-demand as the stream is consumed.
    ///
    /// # Returns
    /// A `SearchStream` that implements `futures::Stream<Item = SearchResult>`.
    ///
    /// # Example
    /// ```no_run
    /// # use arcgis_sharing_rs::ArcGISSharingClient;
    /// # use futures::StreamExt;
    /// # async fn example(client: &ArcGISSharingClient) {
    /// let mut stream = client
    ///     .search()
    ///     .query("water")
    ///     .set_num(10)
    ///     .set_max_pages(5)
    ///     .send();
    ///
    /// while let Some(result) = stream.next().await {
    ///     println!("{}: {}", result.title, result.owner);
    /// }
    ///
    /// // Or collect all results into a Vec
    /// let results: Vec<_> = client
    ///     .search()
    ///     .query("basemap")
    ///     .send()
    ///     .collect()
    ///     .await;
    /// # }
    /// ```
    pub fn send(self) -> SearchStream<'a> {
        SearchStream::new(
            self.client,
            self.to_params(),
            self.max_pages,
            self.page_fetch_delay,
        )
    }
}
