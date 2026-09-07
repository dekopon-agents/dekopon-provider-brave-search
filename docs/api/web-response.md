# Web Search response field catalog

All fetched **200** reference paths are listed below; arrays use `[]`. A `?` records vendor optional notation, not proof of when a field is sent. The schema conservatively accepts null for these fields; it does not assert every optional field is actually nullable. Required-looking fields are not required by the permissive response schema. Unknown fields survive bounded raw projection.

See [response semantics](../response-semantics.md), [endpoint](./web.md), and [upstream schema](../../schemas/upstream/web.response.schema.json). Unspecified rich-field semantics are explicitly unknown, not inferred.

| Field path | Vendor type | Meaning / interpretation |
|---|---|---|
| `type` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `query` | `object?` | Query metadata object, or corrected/completed phrase on suggestion/spellcheck items. |
| `query.original` | `string` | Original user phrase (or original asset URL within a thumbnail). |
| `query.show_strict_warning` | `bool?` | Indicates results were reduced by strict safety filtering. |
| `query.altered` | `string?` | Spelling-corrected phrase actually used upstream. |
| `query.cleaned` | `string?` | Normalized query text. |
| `query.safesearch` | `bool?` | Safesearch metadata/value within this model. |
| `query.is_navigational` | `bool?` | Is navigational metadata/value within this model. |
| `query.is_geolocal` | `bool?` | Is geolocal metadata/value within this model. |
| `query.local_decision` | `string?` | Local decision metadata/value within this model. |
| `query.local_locations_idx` | `int?` | Local locations idx metadata/value within this model. |
| `query.is_trending` | `bool?` | Is trending metadata/value within this model. |
| `query.is_news_breaking` | `bool?` | Is news breaking metadata/value within this model. |
| `query.ask_for_location` | `bool?` | Ask for location metadata/value within this model. |
| `query.language` | `object?` | Language metadata/value within this model. |
| `query.language.main` | `string` | Main metadata/value within this model. |
| `query.spellcheck_off` | `bool?` | Spellcheck off metadata/value within this model. |
| `query.country` | `string?` | Country metadata/value within this model. |
| `query.bad_results` | `bool?` | Bad results metadata/value within this model. |
| `query.should_fallback` | `bool?` | Vendor ranking hint; provider does not initiate fallback. |
| `query.lat` | `string?` | Lat metadata/value within this model. |
| `query.long` | `string?` | Long metadata/value within this model. |
| `query.postal_code` | `string?` | Postal code metadata/value within this model. |
| `query.city` | `string?` | City metadata/value within this model. |
| `query.header_country` | `string?` | Header country metadata/value within this model. |
| `query.more_results_available` | `bool?` | Vendor continuation hint, only actionable where the endpoint has pagination. |
| `query.state` | `string?` | State metadata/value within this model. |
| `query.custom_location_label` | `string?` | Custom location label metadata/value within this model. |
| `query.reddit_cluster` | `string?` | Reddit cluster metadata/value within this model. |
| `query.summary_key` | `string?` | Opaque legacy generated-answer key; no automatic consumption. |
| `query.search_operators` | `object?` | Search operators metadata/value within this model. |
| `query.search_operators.applied` | `bool?` | Applied metadata/value within this model. |
| `query.search_operators.cleaned_query` | `string?` | Cleaned query metadata/value within this model. |
| `query.search_operators.sites` | `string[]?` | Sites collection. |
| `discussions` | `object?` | Discussions metadata/value within this model. |
| `discussions.type` | `string?` | Documented discriminator `search` identifies this enclosing model; unknown future tags remain accepted. |
| `discussions.results` | `object[]` | Results collection. |
| `discussions.results[].title` | `string` | Title metadata/value within this model. |
| `discussions.results[].url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `discussions.results[].is_source_local` | `bool?` | Is source local metadata/value within this model. |
| `discussions.results[].is_source_both` | `bool?` | Is source both metadata/value within this model. |
| `discussions.results[].description` | `string?` | Display description/excerpt; POI Descriptions text is generated and Context source description is query-independent. |
| `discussions.results[].page_age` | `string?` | Content publication/modification date, not necessarily crawl time. |
| `discussions.results[].page_fetched` | `string?` | Last-fetch date/time as provided by the vendor. |
| `discussions.results[].fetched_content_timestamp` | `int?` | Fetch timestamp; do not infer units beyond the source contract. |
| `discussions.results[].profile` | `object?` | Profile metadata/value within this model. |
| `discussions.results[].profile.name` | `string` | Name metadata/value within this model. |
| `discussions.results[].profile.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `discussions.results[].profile.long_name` | `string?` | Long name metadata/value within this model. |
| `discussions.results[].profile.img` | `string?` | Img metadata/value within this model. |
| `discussions.results[].language` | `string?` | Language metadata/value within this model. |
| `discussions.results[].family_friendly` | `bool?` | Vendor safety classification, not a content-safety guarantee. |
| `discussions.results[].type` | `string?` | Documented discriminator `discussion` identifies this enclosing model; unknown future tags remain accepted. |
| `discussions.results[].subtype` | `string?` | Subtype metadata/value within this model. |
| `discussions.results[].is_live` | `bool?` | Live/current-result indicator; vendor documents false as the default. |
| `discussions.results[].deep_results` | `object?` | Deep results metadata/value within this model. |
| `discussions.results[].deep_results.news` | `object[]?` | News collection. |
| `discussions.results[].deep_results.news[].title` | `string` | Title metadata/value within this model. |
| `discussions.results[].deep_results.news[].url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `discussions.results[].deep_results.news[].is_source_local` | `bool?` | Is source local metadata/value within this model. |
| `discussions.results[].deep_results.news[].is_source_both` | `bool?` | Is source both metadata/value within this model. |
| `discussions.results[].deep_results.news[].description` | `string?` | Display description/excerpt; POI Descriptions text is generated and Context source description is query-independent. |
| `discussions.results[].deep_results.news[].page_age` | `string?` | Content publication/modification date, not necessarily crawl time. |
| `discussions.results[].deep_results.news[].page_fetched` | `string?` | Last-fetch date/time as provided by the vendor. |
| `discussions.results[].deep_results.news[].fetched_content_timestamp` | `int?` | Fetch timestamp; do not infer units beyond the source contract. |
| `discussions.results[].deep_results.news[].profile` | `object?` | Profile metadata/value within this model. |
| `discussions.results[].deep_results.news[].profile.name` | `string` | Name metadata/value within this model. |
| `discussions.results[].deep_results.news[].profile.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `discussions.results[].deep_results.news[].profile.long_name` | `string?` | Long name metadata/value within this model. |
| `discussions.results[].deep_results.news[].profile.img` | `string?` | Img metadata/value within this model. |
| `discussions.results[].deep_results.news[].language` | `string?` | Language metadata/value within this model. |
| `discussions.results[].deep_results.news[].family_friendly` | `bool?` | Vendor safety classification, not a content-safety guarantee. |
| `discussions.results[].deep_results.news[].meta_url` | `object?` | Meta url metadata/value within this model. |
| `discussions.results[].deep_results.news[].source` | `string?` | Source metadata/value within this model. |
| `discussions.results[].deep_results.news[].breaking` | `bool?` | Breaking metadata/value within this model. |
| `discussions.results[].deep_results.news[].is_live` | `bool?` | Is live metadata/value within this model. |
| `discussions.results[].deep_results.news[].thumbnail` | `object?` | Thumbnail metadata/value within this model. |
| `discussions.results[].deep_results.news[].thumbnail.src` | `string` | Src metadata/value within this model. |
| `discussions.results[].deep_results.news[].thumbnail.alt` | `string?` | Alt metadata/value within this model. |
| `discussions.results[].deep_results.news[].thumbnail.height` | `int?` | Height metadata/value within this model. |
| `discussions.results[].deep_results.news[].thumbnail.width` | `int?` | Width metadata/value within this model. |
| `discussions.results[].deep_results.news[].thumbnail.bg_color` | `string?` | Bg color metadata/value within this model. |
| `discussions.results[].deep_results.news[].thumbnail.original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `discussions.results[].deep_results.news[].thumbnail.logo` | `bool?` | Logo metadata/value within this model. |
| `discussions.results[].deep_results.news[].thumbnail.duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `discussions.results[].deep_results.news[].thumbnail.theme` | `string?` | Theme metadata/value within this model. |
| `discussions.results[].deep_results.news[].age` | `string?` | Human-readable age; Context sources use a separate four-position date array. |
| `discussions.results[].deep_results.news[].extra_snippets` | `string[]?` | Alternative page excerpts, not continuous full text. |
| `discussions.results[].deep_results.news[].icons` | `object[]?` | Icons collection. |
| `discussions.results[].deep_results.news[].icons[].href` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `discussions.results[].deep_results.news[].icons[].sizes` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `discussions.results[].deep_results.news[].icons[].rel` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `discussions.results[].deep_results.news[].icons[].type` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `discussions.results[].deep_results.news[].icons[].ext` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `discussions.results[].deep_results.buttons` | `object[]?` | Buttons collection. |
| `discussions.results[].deep_results.buttons[].type` | `string?` | Documented discriminator `button_result` identifies this enclosing model; unknown future tags remain accepted. |
| `discussions.results[].deep_results.buttons[].title` | `string` | Title metadata/value within this model. |
| `discussions.results[].deep_results.buttons[].url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `discussions.results[].deep_results.videos` | `object[]?` | Videos collection. |
| `discussions.results[].deep_results.videos[].title` | `string` | Title metadata/value within this model. |
| `discussions.results[].deep_results.videos[].url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `discussions.results[].deep_results.videos[].is_source_local` | `bool?` | Is source local metadata/value within this model. |
| `discussions.results[].deep_results.videos[].is_source_both` | `bool?` | Is source both metadata/value within this model. |
| `discussions.results[].deep_results.videos[].description` | `string?` | Display description/excerpt; POI Descriptions text is generated and Context source description is query-independent. |
| `discussions.results[].deep_results.videos[].page_age` | `string?` | Content publication/modification date, not necessarily crawl time. |
| `discussions.results[].deep_results.videos[].page_fetched` | `string?` | Last-fetch date/time as provided by the vendor. |
| `discussions.results[].deep_results.videos[].fetched_content_timestamp` | `int?` | Fetch timestamp; do not infer units beyond the source contract. |
| `discussions.results[].deep_results.videos[].profile` | `object?` | Profile metadata/value within this model. |
| `discussions.results[].deep_results.videos[].profile.name` | `string` | Name metadata/value within this model. |
| `discussions.results[].deep_results.videos[].profile.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `discussions.results[].deep_results.videos[].profile.long_name` | `string?` | Long name metadata/value within this model. |
| `discussions.results[].deep_results.videos[].profile.img` | `string?` | Img metadata/value within this model. |
| `discussions.results[].deep_results.videos[].language` | `string?` | Language metadata/value within this model. |
| `discussions.results[].deep_results.videos[].family_friendly` | `bool?` | Vendor safety classification, not a content-safety guarantee. |
| `discussions.results[].deep_results.videos[].type` | `string?` | Documented discriminator `video_result` identifies this enclosing model; unknown future tags remain accepted. |
| `discussions.results[].deep_results.videos[].video` | `object` | Video metadata/value within this model. |
| `discussions.results[].deep_results.videos[].video.duration` | `string?` | Duration metadata/value within this model. |
| `discussions.results[].deep_results.videos[].video.views` | `string?` | Views metadata/value within this model. |
| `discussions.results[].deep_results.videos[].video.creator` | `string?` | Creator metadata/value within this model. |
| `discussions.results[].deep_results.videos[].video.publisher` | `string?` | Publisher metadata/value within this model. |
| `discussions.results[].deep_results.videos[].video.thumbnail` | `object?` | Thumbnail metadata/value within this model. |
| `discussions.results[].deep_results.videos[].video.thumbnail.src` | `string` | Src metadata/value within this model. |
| `discussions.results[].deep_results.videos[].video.thumbnail.alt` | `string?` | Alt metadata/value within this model. |
| `discussions.results[].deep_results.videos[].video.thumbnail.height` | `int?` | Height metadata/value within this model. |
| `discussions.results[].deep_results.videos[].video.thumbnail.width` | `int?` | Width metadata/value within this model. |
| `discussions.results[].deep_results.videos[].video.thumbnail.bg_color` | `string?` | Bg color metadata/value within this model. |
| `discussions.results[].deep_results.videos[].video.thumbnail.original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `discussions.results[].deep_results.videos[].video.thumbnail.logo` | `bool?` | Logo metadata/value within this model. |
| `discussions.results[].deep_results.videos[].video.thumbnail.duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `discussions.results[].deep_results.videos[].video.thumbnail.theme` | `string?` | Theme metadata/value within this model. |
| `discussions.results[].deep_results.videos[].video.tags` | `string[]?` | Tags collection. |
| `discussions.results[].deep_results.videos[].video.author` | `object?` | Author metadata/value within this model. |
| `discussions.results[].deep_results.videos[].video.author.name` | `string` | Name metadata/value within this model. |
| `discussions.results[].deep_results.videos[].video.author.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `discussions.results[].deep_results.videos[].video.author.long_name` | `string?` | Long name metadata/value within this model. |
| `discussions.results[].deep_results.videos[].video.author.img` | `string?` | Img metadata/value within this model. |
| `discussions.results[].deep_results.videos[].video.requires_subscription` | `bool?` | Source-content access restriction, independent of Brave API billing. |
| `discussions.results[].deep_results.videos[].meta_url` | `object?` | Meta url metadata/value within this model. |
| `discussions.results[].deep_results.videos[].meta_url.scheme` | `string` | Scheme metadata/value within this model. |
| `discussions.results[].deep_results.videos[].meta_url.netloc` | `string` | Netloc metadata/value within this model. |
| `discussions.results[].deep_results.videos[].meta_url.hostname` | `string?` | Hostname metadata/value within this model. |
| `discussions.results[].deep_results.videos[].meta_url.favicon` | `string` | Favicon metadata/value within this model. |
| `discussions.results[].deep_results.videos[].meta_url.path` | `string` | Path metadata/value within this model. |
| `discussions.results[].deep_results.videos[].thumbnail` | `object?` | Thumbnail metadata/value within this model. |
| `discussions.results[].deep_results.videos[].thumbnail.src` | `string` | Src metadata/value within this model. |
| `discussions.results[].deep_results.videos[].thumbnail.alt` | `string?` | Alt metadata/value within this model. |
| `discussions.results[].deep_results.videos[].thumbnail.height` | `int?` | Height metadata/value within this model. |
| `discussions.results[].deep_results.videos[].thumbnail.width` | `int?` | Width metadata/value within this model. |
| `discussions.results[].deep_results.videos[].thumbnail.bg_color` | `string?` | Bg color metadata/value within this model. |
| `discussions.results[].deep_results.videos[].thumbnail.original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `discussions.results[].deep_results.videos[].thumbnail.logo` | `bool?` | Logo metadata/value within this model. |
| `discussions.results[].deep_results.videos[].thumbnail.duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `discussions.results[].deep_results.videos[].thumbnail.theme` | `string?` | Theme metadata/value within this model. |
| `discussions.results[].deep_results.videos[].age` | `string?` | Human-readable age; Context sources use a separate four-position date array. |
| `discussions.results[].deep_results.videos[].publisher` | `string?` | Publisher metadata/value within this model. |
| `discussions.results[].deep_results.images` | `object[]?` | Images collection. |
| `discussions.results[].deep_results.images[].thumbnail` | `object` | Thumbnail metadata/value within this model. |
| `discussions.results[].deep_results.images[].thumbnail.src` | `string` | Src metadata/value within this model. |
| `discussions.results[].deep_results.images[].thumbnail.alt` | `string?` | Alt metadata/value within this model. |
| `discussions.results[].deep_results.images[].thumbnail.height` | `int?` | Height metadata/value within this model. |
| `discussions.results[].deep_results.images[].thumbnail.width` | `int?` | Width metadata/value within this model. |
| `discussions.results[].deep_results.images[].thumbnail.bg_color` | `string?` | Bg color metadata/value within this model. |
| `discussions.results[].deep_results.images[].thumbnail.original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `discussions.results[].deep_results.images[].thumbnail.logo` | `bool?` | Logo metadata/value within this model. |
| `discussions.results[].deep_results.images[].thumbnail.duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `discussions.results[].deep_results.images[].thumbnail.theme` | `string?` | Theme metadata/value within this model. |
| `discussions.results[].deep_results.images[].url` | `string?` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `discussions.results[].deep_results.images[].properties` | `object?` | Properties metadata/value within this model. |
| `discussions.results[].deep_results.images[].properties.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `discussions.results[].deep_results.images[].properties.resized` | `string` | Resized metadata/value within this model. |
| `discussions.results[].deep_results.images[].properties.placeholder` | `string` | Placeholder metadata/value within this model. |
| `discussions.results[].deep_results.images[].properties.height` | `int?` | Height metadata/value within this model. |
| `discussions.results[].deep_results.images[].properties.width` | `int?` | Width metadata/value within this model. |
| `discussions.results[].deep_results.images[].properties.format` | `string?` | Format metadata/value within this model. |
| `discussions.results[].deep_results.images[].properties.content_size` | `string?` | Content size metadata/value within this model. |
| `discussions.results[].schemas` | `any[]?` | Extracted structured data resembling schema.org; heterogeneous and not guaranteed to conform. |
| `discussions.results[].meta_url` | `object?` | Meta url metadata/value within this model. |
| `discussions.results[].thumbnail` | `object?` | Thumbnail metadata/value within this model. |
| `discussions.results[].thumbnail.src` | `string` | Src metadata/value within this model. |
| `discussions.results[].thumbnail.alt` | `string?` | Alt metadata/value within this model. |
| `discussions.results[].thumbnail.height` | `int?` | Height metadata/value within this model. |
| `discussions.results[].thumbnail.width` | `int?` | Width metadata/value within this model. |
| `discussions.results[].thumbnail.bg_color` | `string?` | Bg color metadata/value within this model. |
| `discussions.results[].thumbnail.original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `discussions.results[].thumbnail.logo` | `bool?` | Logo metadata/value within this model. |
| `discussions.results[].thumbnail.duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `discussions.results[].thumbnail.theme` | `string?` | Theme metadata/value within this model. |
| `discussions.results[].age` | `string?` | Human-readable age; Context sources use a separate four-position date array. |
| `discussions.results[].location` | `object?` | Location metadata/value within this model. |
| `discussions.results[].location.title` | `string` | Title metadata/value within this model. |
| `discussions.results[].location.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `discussions.results[].location.is_source_local` | `bool?` | Is source local metadata/value within this model. |
| `discussions.results[].location.is_source_both` | `bool?` | Is source both metadata/value within this model. |
| `discussions.results[].location.description` | `string?` | Display description/excerpt; POI Descriptions text is generated and Context source description is query-independent. |
| `discussions.results[].location.page_age` | `string?` | Content publication/modification date, not necessarily crawl time. |
| `discussions.results[].location.page_fetched` | `string?` | Last-fetch date/time as provided by the vendor. |
| `discussions.results[].location.fetched_content_timestamp` | `int?` | Fetch timestamp; do not infer units beyond the source contract. |
| `discussions.results[].location.profile` | `object?` | Profile metadata/value within this model. |
| `discussions.results[].location.profile.name` | `string` | Name metadata/value within this model. |
| `discussions.results[].location.profile.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `discussions.results[].location.profile.long_name` | `string?` | Long name metadata/value within this model. |
| `discussions.results[].location.profile.img` | `string?` | Img metadata/value within this model. |
| `discussions.results[].location.language` | `string?` | Language metadata/value within this model. |
| `discussions.results[].location.family_friendly` | `bool?` | Vendor safety classification, not a content-safety guarantee. |
| `discussions.results[].location.type` | `string?` | Documented discriminator `location_result` identifies this enclosing model; unknown future tags remain accepted. |
| `discussions.results[].location.provider_url` | `string` | Attribution/provider destination URL. |
| `discussions.results[].location.coordinates` | `array?` | Latitude/longitude pair as described by the location model; no geometry guarantee for unrelated arrays. |
| `discussions.results[].location.zoom_level` | `int?` | Zoom level metadata/value within this model. |
| `discussions.results[].location.thumbnail` | `object?` | Thumbnail metadata/value within this model. |
| `discussions.results[].location.thumbnail.src` | `string` | Src metadata/value within this model. |
| `discussions.results[].location.thumbnail.alt` | `string?` | Alt metadata/value within this model. |
| `discussions.results[].location.thumbnail.height` | `int?` | Height metadata/value within this model. |
| `discussions.results[].location.thumbnail.width` | `int?` | Width metadata/value within this model. |
| `discussions.results[].location.thumbnail.bg_color` | `string?` | Bg color metadata/value within this model. |
| `discussions.results[].location.thumbnail.original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `discussions.results[].location.thumbnail.logo` | `bool?` | Logo metadata/value within this model. |
| `discussions.results[].location.thumbnail.duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `discussions.results[].location.thumbnail.theme` | `string?` | Theme metadata/value within this model. |
| `discussions.results[].location.postal_address` | `object?` | Postal address metadata/value within this model. |
| `discussions.results[].location.postal_address.type` | `string?` | Documented discriminator `PostalAddress` identifies this enclosing model; unknown future tags remain accepted. |
| `discussions.results[].location.postal_address.country` | `string?` | Country metadata/value within this model. |
| `discussions.results[].location.postal_address.postalCode` | `string?` | Postal code metadata/value within this model. |
| `discussions.results[].location.postal_address.streetAddress` | `string?` | Street address metadata/value within this model. |
| `discussions.results[].location.postal_address.addressRegion` | `string?` | Address region metadata/value within this model. |
| `discussions.results[].location.postal_address.addressLocality` | `string?` | Address locality metadata/value within this model. |
| `discussions.results[].location.postal_address.displayAddress` | `string` | Display address metadata/value within this model. |
| `discussions.results[].location.opening_hours` | `object?` | Business hours; days is a nested array and a day may contain multiple intervals. |
| `discussions.results[].location.opening_hours.current_day` | `object[]?` | Current day collection. |
| `discussions.results[].location.opening_hours.current_day[].abbr_name` | `string` | Abbr name metadata/value within this model. |
| `discussions.results[].location.opening_hours.current_day[].full_name` | `string` | Full name metadata/value within this model. |
| `discussions.results[].location.opening_hours.current_day[].opens` | `string` | Local 24-hour opening-time string. |
| `discussions.results[].location.opening_hours.current_day[].closes` | `string` | Local 24-hour closing-time string. |
| `discussions.results[].location.opening_hours.days` | `object[][]?` | Days collection. |
| `discussions.results[].location.contact` | `object?` | Contact metadata/value within this model. |
| `discussions.results[].location.contact.email` | `string?` | Email metadata/value within this model. |
| `discussions.results[].location.contact.telephone` | `string?` | Telephone metadata/value within this model. |
| `discussions.results[].location.price_range` | `string?` | Price range metadata/value within this model. |
| `discussions.results[].location.rating` | `object?` | Rating metadata/value within this model. |
| `discussions.results[].location.rating.ratingValue` | `number` | Source-specific rating value; compare only with that source and scale. |
| `discussions.results[].location.rating.bestRating` | `number` | Source rating scale metadata; not global relevance. |
| `discussions.results[].location.rating.reviewCount` | `int?` | Number of reviews reported by source. |
| `discussions.results[].location.rating.profile` | `object?` | Profile metadata/value within this model. |
| `discussions.results[].location.rating.profile.name` | `string` | Name metadata/value within this model. |
| `discussions.results[].location.rating.profile.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `discussions.results[].location.rating.profile.long_name` | `string?` | Long name metadata/value within this model. |
| `discussions.results[].location.rating.profile.img` | `string?` | Img metadata/value within this model. |
| `discussions.results[].location.rating.is_tripadvisor` | `bool?` | Attribution flag for the rating source. |
| `discussions.results[].location.distance` | `object?` | Distance metadata/value within this model. |
| `discussions.results[].location.distance.value` | `number` | Value metadata/value within this model. |
| `discussions.results[].location.distance.units` | `string` | Units metadata/value within this model. |
| `discussions.results[].location.profiles` | `object[]?` | Profiles collection. |
| `discussions.results[].location.profiles[].type` | `string?` | Source-origin type; external normally identifies an external data source, but is not a closed enum. |
| `discussions.results[].location.profiles[].name` | `string` | Name metadata/value within this model. |
| `discussions.results[].location.profiles[].url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `discussions.results[].location.profiles[].long_name` | `string?` | Long name metadata/value within this model. |
| `discussions.results[].location.profiles[].img` | `string?` | Img metadata/value within this model. |
| `discussions.results[].location.reviews` | `object?` | Reviews metadata/value within this model. |
| `discussions.results[].location.reviews.results` | `object[]` | Results collection. |
| `discussions.results[].location.reviews.results[].title` | `string` | Title metadata/value within this model. |
| `discussions.results[].location.reviews.results[].description` | `string` | Display description/excerpt; POI Descriptions text is generated and Context source description is query-independent. |
| `discussions.results[].location.reviews.results[].date` | `string` | Date metadata/value within this model. |
| `discussions.results[].location.reviews.results[].rating` | `object` | Rating metadata/value within this model. |
| `discussions.results[].location.reviews.results[].rating.ratingValue` | `number` | Source-specific rating value; compare only with that source and scale. |
| `discussions.results[].location.reviews.results[].rating.bestRating` | `number` | Source rating scale metadata; not global relevance. |
| `discussions.results[].location.reviews.results[].rating.reviewCount` | `int?` | Number of reviews reported by source. |
| `discussions.results[].location.reviews.results[].rating.profile` | `object?` | Profile metadata/value within this model. |
| `discussions.results[].location.reviews.results[].rating.profile.name` | `string` | Name metadata/value within this model. |
| `discussions.results[].location.reviews.results[].rating.profile.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `discussions.results[].location.reviews.results[].rating.profile.long_name` | `string?` | Long name metadata/value within this model. |
| `discussions.results[].location.reviews.results[].rating.profile.img` | `string?` | Img metadata/value within this model. |
| `discussions.results[].location.reviews.results[].rating.is_tripadvisor` | `bool?` | Attribution flag for the rating source. |
| `discussions.results[].location.reviews.results[].author` | `object` | Author metadata/value within this model. |
| `discussions.results[].location.reviews.results[].author.type` | `string?` | Documented discriminator `person` identifies this enclosing model; unknown future tags remain accepted. |
| `discussions.results[].location.reviews.results[].author.name` | `string` | Name metadata/value within this model. |
| `discussions.results[].location.reviews.results[].author.url` | `string?` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `discussions.results[].location.reviews.results[].author.thumbnail` | `object?` | Thumbnail metadata/value within this model. |
| `discussions.results[].location.reviews.results[].author.thumbnail.src` | `string` | Src metadata/value within this model. |
| `discussions.results[].location.reviews.results[].author.thumbnail.alt` | `string?` | Alt metadata/value within this model. |
| `discussions.results[].location.reviews.results[].author.thumbnail.height` | `int?` | Height metadata/value within this model. |
| `discussions.results[].location.reviews.results[].author.thumbnail.width` | `int?` | Width metadata/value within this model. |
| `discussions.results[].location.reviews.results[].author.thumbnail.bg_color` | `string?` | Bg color metadata/value within this model. |
| `discussions.results[].location.reviews.results[].author.thumbnail.original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `discussions.results[].location.reviews.results[].author.thumbnail.logo` | `bool?` | Logo metadata/value within this model. |
| `discussions.results[].location.reviews.results[].author.thumbnail.duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `discussions.results[].location.reviews.results[].author.thumbnail.theme` | `string?` | Theme metadata/value within this model. |
| `discussions.results[].location.reviews.results[].author.email` | `string?` | Email metadata/value within this model. |
| `discussions.results[].location.reviews.results[].review_url` | `string` | Review url metadata/value within this model. |
| `discussions.results[].location.reviews.results[].language` | `string` | Language metadata/value within this model. |
| `discussions.results[].location.reviews.viewMoreUrl` | `string` | View more url metadata/value within this model. |
| `discussions.results[].location.reviews.reviews_in_foreign_language` | `bool` | Reviews in foreign language metadata/value within this model. |
| `discussions.results[].location.pictures` | `object?` | Pictures metadata/value within this model. |
| `discussions.results[].location.pictures.viewMoreUrl` | `string?` | View more url metadata/value within this model. |
| `discussions.results[].location.pictures.results` | `object[]` | Results collection. |
| `discussions.results[].location.pictures.results[].src` | `string` | Src metadata/value within this model. |
| `discussions.results[].location.pictures.results[].alt` | `string?` | Alt metadata/value within this model. |
| `discussions.results[].location.pictures.results[].height` | `int?` | Height metadata/value within this model. |
| `discussions.results[].location.pictures.results[].width` | `int?` | Width metadata/value within this model. |
| `discussions.results[].location.pictures.results[].bg_color` | `string?` | Bg color metadata/value within this model. |
| `discussions.results[].location.pictures.results[].original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `discussions.results[].location.pictures.results[].logo` | `bool?` | Logo metadata/value within this model. |
| `discussions.results[].location.pictures.results[].duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `discussions.results[].location.pictures.results[].theme` | `string?` | Theme metadata/value within this model. |
| `discussions.results[].location.action` | `object?` | Action metadata/value within this model. |
| `discussions.results[].location.action.type` | `string` | Type metadata/value within this model. |
| `discussions.results[].location.action.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `discussions.results[].location.serves_cuisine` | `string[]?` | Serves cuisine collection. |
| `discussions.results[].location.categories` | `string[]?` | Categories collection. |
| `discussions.results[].location.icon_category` | `string?` | Icon category metadata/value within this model. |
| `discussions.results[].location.timezone` | `string?` | Timezone metadata/value within this model. |
| `discussions.results[].location.timezone_offset` | `int?` | Timezone offset metadata/value within this model. |
| `discussions.results[].location.id` | `string?` | Identifier scoped to the enclosing result; local-place IDs expire in about eight hours. |
| `discussions.results[].location.results` | `object[]?` | Results collection. |
| `discussions.results[].location.results[].title` | `string` | Title metadata/value within this model. |
| `discussions.results[].location.results[].url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `discussions.results[].location.results[].is_source_local` | `bool?` | Is source local metadata/value within this model. |
| `discussions.results[].location.results[].is_source_both` | `bool?` | Is source both metadata/value within this model. |
| `discussions.results[].location.results[].description` | `string?` | Display description/excerpt; POI Descriptions text is generated and Context source description is query-independent. |
| `discussions.results[].location.results[].page_age` | `string?` | Content publication/modification date, not necessarily crawl time. |
| `discussions.results[].location.results[].page_fetched` | `string?` | Last-fetch date/time as provided by the vendor. |
| `discussions.results[].location.results[].fetched_content_timestamp` | `int?` | Fetch timestamp; do not infer units beyond the source contract. |
| `discussions.results[].location.results[].profile` | `object?` | Profile metadata/value within this model. |
| `discussions.results[].location.results[].profile.name` | `string` | Name metadata/value within this model. |
| `discussions.results[].location.results[].profile.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `discussions.results[].location.results[].profile.long_name` | `string?` | Long name metadata/value within this model. |
| `discussions.results[].location.results[].profile.img` | `string?` | Img metadata/value within this model. |
| `discussions.results[].location.results[].language` | `string?` | Language metadata/value within this model. |
| `discussions.results[].location.results[].family_friendly` | `bool?` | Vendor safety classification, not a content-safety guarantee. |
| `discussions.results[].location.results[].meta_url` | `object` | Meta url metadata/value within this model. |
| `discussions.results[].location.results[].meta_url.scheme` | `string` | Scheme metadata/value within this model. |
| `discussions.results[].location.results[].meta_url.netloc` | `string` | Netloc metadata/value within this model. |
| `discussions.results[].location.results[].meta_url.hostname` | `string?` | Hostname metadata/value within this model. |
| `discussions.results[].location.results[].meta_url.favicon` | `string` | Favicon metadata/value within this model. |
| `discussions.results[].location.results[].meta_url.path` | `string` | Path metadata/value within this model. |
| `discussions.results[].restaurant` | `object?` | Deprecated result property; prefer location. |
| `discussions.results[].restaurant.title` | `string` | Title metadata/value within this model. |
| `discussions.results[].restaurant.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `discussions.results[].restaurant.is_source_local` | `bool?` | Is source local metadata/value within this model. |
| `discussions.results[].restaurant.is_source_both` | `bool?` | Is source both metadata/value within this model. |
| `discussions.results[].restaurant.description` | `string?` | Display description/excerpt; POI Descriptions text is generated and Context source description is query-independent. |
| `discussions.results[].restaurant.page_age` | `string?` | Content publication/modification date, not necessarily crawl time. |
| `discussions.results[].restaurant.page_fetched` | `string?` | Last-fetch date/time as provided by the vendor. |
| `discussions.results[].restaurant.fetched_content_timestamp` | `int?` | Fetch timestamp; do not infer units beyond the source contract. |
| `discussions.results[].restaurant.profile` | `object?` | Profile metadata/value within this model. |
| `discussions.results[].restaurant.profile.name` | `string` | Name metadata/value within this model. |
| `discussions.results[].restaurant.profile.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `discussions.results[].restaurant.profile.long_name` | `string?` | Long name metadata/value within this model. |
| `discussions.results[].restaurant.profile.img` | `string?` | Img metadata/value within this model. |
| `discussions.results[].restaurant.language` | `string?` | Language metadata/value within this model. |
| `discussions.results[].restaurant.family_friendly` | `bool?` | Vendor safety classification, not a content-safety guarantee. |
| `discussions.results[].restaurant.type` | `string?` | Documented discriminator `location_result` identifies this enclosing model; unknown future tags remain accepted. |
| `discussions.results[].restaurant.provider_url` | `string` | Attribution/provider destination URL. |
| `discussions.results[].restaurant.coordinates` | `array?` | Latitude/longitude pair as described by the location model; no geometry guarantee for unrelated arrays. |
| `discussions.results[].restaurant.zoom_level` | `int?` | Zoom level metadata/value within this model. |
| `discussions.results[].restaurant.thumbnail` | `object?` | Thumbnail metadata/value within this model. |
| `discussions.results[].restaurant.thumbnail.src` | `string` | Src metadata/value within this model. |
| `discussions.results[].restaurant.thumbnail.alt` | `string?` | Alt metadata/value within this model. |
| `discussions.results[].restaurant.thumbnail.height` | `int?` | Height metadata/value within this model. |
| `discussions.results[].restaurant.thumbnail.width` | `int?` | Width metadata/value within this model. |
| `discussions.results[].restaurant.thumbnail.bg_color` | `string?` | Bg color metadata/value within this model. |
| `discussions.results[].restaurant.thumbnail.original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `discussions.results[].restaurant.thumbnail.logo` | `bool?` | Logo metadata/value within this model. |
| `discussions.results[].restaurant.thumbnail.duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `discussions.results[].restaurant.thumbnail.theme` | `string?` | Theme metadata/value within this model. |
| `discussions.results[].restaurant.postal_address` | `object?` | Postal address metadata/value within this model. |
| `discussions.results[].restaurant.postal_address.type` | `string?` | Documented discriminator `PostalAddress` identifies this enclosing model; unknown future tags remain accepted. |
| `discussions.results[].restaurant.postal_address.country` | `string?` | Country metadata/value within this model. |
| `discussions.results[].restaurant.postal_address.postalCode` | `string?` | Postal code metadata/value within this model. |
| `discussions.results[].restaurant.postal_address.streetAddress` | `string?` | Street address metadata/value within this model. |
| `discussions.results[].restaurant.postal_address.addressRegion` | `string?` | Address region metadata/value within this model. |
| `discussions.results[].restaurant.postal_address.addressLocality` | `string?` | Address locality metadata/value within this model. |
| `discussions.results[].restaurant.postal_address.displayAddress` | `string` | Display address metadata/value within this model. |
| `discussions.results[].restaurant.opening_hours` | `object?` | Business hours; days is a nested array and a day may contain multiple intervals. |
| `discussions.results[].restaurant.opening_hours.current_day` | `object[]?` | Current day collection. |
| `discussions.results[].restaurant.opening_hours.current_day[].abbr_name` | `string` | Abbr name metadata/value within this model. |
| `discussions.results[].restaurant.opening_hours.current_day[].full_name` | `string` | Full name metadata/value within this model. |
| `discussions.results[].restaurant.opening_hours.current_day[].opens` | `string` | Local 24-hour opening-time string. |
| `discussions.results[].restaurant.opening_hours.current_day[].closes` | `string` | Local 24-hour closing-time string. |
| `discussions.results[].restaurant.opening_hours.days` | `object[][]?` | Days collection. |
| `discussions.results[].restaurant.contact` | `object?` | Contact metadata/value within this model. |
| `discussions.results[].restaurant.contact.email` | `string?` | Email metadata/value within this model. |
| `discussions.results[].restaurant.contact.telephone` | `string?` | Telephone metadata/value within this model. |
| `discussions.results[].restaurant.price_range` | `string?` | Price range metadata/value within this model. |
| `discussions.results[].restaurant.rating` | `object?` | Rating metadata/value within this model. |
| `discussions.results[].restaurant.rating.ratingValue` | `number` | Source-specific rating value; compare only with that source and scale. |
| `discussions.results[].restaurant.rating.bestRating` | `number` | Source rating scale metadata; not global relevance. |
| `discussions.results[].restaurant.rating.reviewCount` | `int?` | Number of reviews reported by source. |
| `discussions.results[].restaurant.rating.profile` | `object?` | Profile metadata/value within this model. |
| `discussions.results[].restaurant.rating.profile.name` | `string` | Name metadata/value within this model. |
| `discussions.results[].restaurant.rating.profile.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `discussions.results[].restaurant.rating.profile.long_name` | `string?` | Long name metadata/value within this model. |
| `discussions.results[].restaurant.rating.profile.img` | `string?` | Img metadata/value within this model. |
| `discussions.results[].restaurant.rating.is_tripadvisor` | `bool?` | Attribution flag for the rating source. |
| `discussions.results[].restaurant.distance` | `object?` | Distance metadata/value within this model. |
| `discussions.results[].restaurant.distance.value` | `number` | Value metadata/value within this model. |
| `discussions.results[].restaurant.distance.units` | `string` | Units metadata/value within this model. |
| `discussions.results[].restaurant.profiles` | `object[]?` | Profiles collection. |
| `discussions.results[].restaurant.profiles[].type` | `string?` | Source-origin type; external normally identifies an external data source, but is not a closed enum. |
| `discussions.results[].restaurant.profiles[].name` | `string` | Name metadata/value within this model. |
| `discussions.results[].restaurant.profiles[].url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `discussions.results[].restaurant.profiles[].long_name` | `string?` | Long name metadata/value within this model. |
| `discussions.results[].restaurant.profiles[].img` | `string?` | Img metadata/value within this model. |
| `discussions.results[].restaurant.reviews` | `object?` | Reviews metadata/value within this model. |
| `discussions.results[].restaurant.reviews.results` | `object[]` | Results collection. |
| `discussions.results[].restaurant.reviews.results[].title` | `string` | Title metadata/value within this model. |
| `discussions.results[].restaurant.reviews.results[].description` | `string` | Display description/excerpt; POI Descriptions text is generated and Context source description is query-independent. |
| `discussions.results[].restaurant.reviews.results[].date` | `string` | Date metadata/value within this model. |
| `discussions.results[].restaurant.reviews.results[].rating` | `object` | Rating metadata/value within this model. |
| `discussions.results[].restaurant.reviews.results[].rating.ratingValue` | `number` | Source-specific rating value; compare only with that source and scale. |
| `discussions.results[].restaurant.reviews.results[].rating.bestRating` | `number` | Source rating scale metadata; not global relevance. |
| `discussions.results[].restaurant.reviews.results[].rating.reviewCount` | `int?` | Number of reviews reported by source. |
| `discussions.results[].restaurant.reviews.results[].rating.profile` | `object?` | Profile metadata/value within this model. |
| `discussions.results[].restaurant.reviews.results[].rating.profile.name` | `string` | Name metadata/value within this model. |
| `discussions.results[].restaurant.reviews.results[].rating.profile.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `discussions.results[].restaurant.reviews.results[].rating.profile.long_name` | `string?` | Long name metadata/value within this model. |
| `discussions.results[].restaurant.reviews.results[].rating.profile.img` | `string?` | Img metadata/value within this model. |
| `discussions.results[].restaurant.reviews.results[].rating.is_tripadvisor` | `bool?` | Attribution flag for the rating source. |
| `discussions.results[].restaurant.reviews.results[].author` | `object` | Author metadata/value within this model. |
| `discussions.results[].restaurant.reviews.results[].author.type` | `string?` | Documented discriminator `person` identifies this enclosing model; unknown future tags remain accepted. |
| `discussions.results[].restaurant.reviews.results[].author.name` | `string` | Name metadata/value within this model. |
| `discussions.results[].restaurant.reviews.results[].author.url` | `string?` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `discussions.results[].restaurant.reviews.results[].author.thumbnail` | `object?` | Thumbnail metadata/value within this model. |
| `discussions.results[].restaurant.reviews.results[].author.thumbnail.src` | `string` | Src metadata/value within this model. |
| `discussions.results[].restaurant.reviews.results[].author.thumbnail.alt` | `string?` | Alt metadata/value within this model. |
| `discussions.results[].restaurant.reviews.results[].author.thumbnail.height` | `int?` | Height metadata/value within this model. |
| `discussions.results[].restaurant.reviews.results[].author.thumbnail.width` | `int?` | Width metadata/value within this model. |
| `discussions.results[].restaurant.reviews.results[].author.thumbnail.bg_color` | `string?` | Bg color metadata/value within this model. |
| `discussions.results[].restaurant.reviews.results[].author.thumbnail.original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `discussions.results[].restaurant.reviews.results[].author.thumbnail.logo` | `bool?` | Logo metadata/value within this model. |
| `discussions.results[].restaurant.reviews.results[].author.thumbnail.duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `discussions.results[].restaurant.reviews.results[].author.thumbnail.theme` | `string?` | Theme metadata/value within this model. |
| `discussions.results[].restaurant.reviews.results[].author.email` | `string?` | Email metadata/value within this model. |
| `discussions.results[].restaurant.reviews.results[].review_url` | `string` | Review url metadata/value within this model. |
| `discussions.results[].restaurant.reviews.results[].language` | `string` | Language metadata/value within this model. |
| `discussions.results[].restaurant.reviews.viewMoreUrl` | `string` | View more url metadata/value within this model. |
| `discussions.results[].restaurant.reviews.reviews_in_foreign_language` | `bool` | Reviews in foreign language metadata/value within this model. |
| `discussions.results[].restaurant.pictures` | `object?` | Pictures metadata/value within this model. |
| `discussions.results[].restaurant.pictures.viewMoreUrl` | `string?` | View more url metadata/value within this model. |
| `discussions.results[].restaurant.pictures.results` | `object[]` | Results collection. |
| `discussions.results[].restaurant.pictures.results[].src` | `string` | Src metadata/value within this model. |
| `discussions.results[].restaurant.pictures.results[].alt` | `string?` | Alt metadata/value within this model. |
| `discussions.results[].restaurant.pictures.results[].height` | `int?` | Height metadata/value within this model. |
| `discussions.results[].restaurant.pictures.results[].width` | `int?` | Width metadata/value within this model. |
| `discussions.results[].restaurant.pictures.results[].bg_color` | `string?` | Bg color metadata/value within this model. |
| `discussions.results[].restaurant.pictures.results[].original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `discussions.results[].restaurant.pictures.results[].logo` | `bool?` | Logo metadata/value within this model. |
| `discussions.results[].restaurant.pictures.results[].duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `discussions.results[].restaurant.pictures.results[].theme` | `string?` | Theme metadata/value within this model. |
| `discussions.results[].restaurant.action` | `object?` | Action metadata/value within this model. |
| `discussions.results[].restaurant.action.type` | `string` | Type metadata/value within this model. |
| `discussions.results[].restaurant.action.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `discussions.results[].restaurant.serves_cuisine` | `string[]?` | Serves cuisine collection. |
| `discussions.results[].restaurant.categories` | `string[]?` | Categories collection. |
| `discussions.results[].restaurant.icon_category` | `string?` | Icon category metadata/value within this model. |
| `discussions.results[].restaurant.timezone` | `string?` | Timezone metadata/value within this model. |
| `discussions.results[].restaurant.timezone_offset` | `int?` | Timezone offset metadata/value within this model. |
| `discussions.results[].restaurant.id` | `string?` | Identifier scoped to the enclosing result; local-place IDs expire in about eight hours. |
| `discussions.results[].restaurant.results` | `object[]?` | Results collection. |
| `discussions.results[].restaurant.results[].title` | `string` | Title metadata/value within this model. |
| `discussions.results[].restaurant.results[].url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `discussions.results[].restaurant.results[].is_source_local` | `bool?` | Is source local metadata/value within this model. |
| `discussions.results[].restaurant.results[].is_source_both` | `bool?` | Is source both metadata/value within this model. |
| `discussions.results[].restaurant.results[].description` | `string?` | Display description/excerpt; POI Descriptions text is generated and Context source description is query-independent. |
| `discussions.results[].restaurant.results[].page_age` | `string?` | Content publication/modification date, not necessarily crawl time. |
| `discussions.results[].restaurant.results[].page_fetched` | `string?` | Last-fetch date/time as provided by the vendor. |
| `discussions.results[].restaurant.results[].fetched_content_timestamp` | `int?` | Fetch timestamp; do not infer units beyond the source contract. |
| `discussions.results[].restaurant.results[].profile` | `object?` | Profile metadata/value within this model. |
| `discussions.results[].restaurant.results[].profile.name` | `string` | Name metadata/value within this model. |
| `discussions.results[].restaurant.results[].profile.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `discussions.results[].restaurant.results[].profile.long_name` | `string?` | Long name metadata/value within this model. |
| `discussions.results[].restaurant.results[].profile.img` | `string?` | Img metadata/value within this model. |
| `discussions.results[].restaurant.results[].language` | `string?` | Language metadata/value within this model. |
| `discussions.results[].restaurant.results[].family_friendly` | `bool?` | Vendor safety classification, not a content-safety guarantee. |
| `discussions.results[].restaurant.results[].meta_url` | `object` | Meta url metadata/value within this model. |
| `discussions.results[].restaurant.results[].meta_url.scheme` | `string` | Scheme metadata/value within this model. |
| `discussions.results[].restaurant.results[].meta_url.netloc` | `string` | Netloc metadata/value within this model. |
| `discussions.results[].restaurant.results[].meta_url.hostname` | `string?` | Hostname metadata/value within this model. |
| `discussions.results[].restaurant.results[].meta_url.favicon` | `string` | Favicon metadata/value within this model. |
| `discussions.results[].restaurant.results[].meta_url.path` | `string` | Path metadata/value within this model. |
| `discussions.results[].video` | `object?` | Video metadata/value within this model. |
| `discussions.results[].video.duration` | `string?` | Duration metadata/value within this model. |
| `discussions.results[].video.views` | `string?` | Views metadata/value within this model. |
| `discussions.results[].video.creator` | `string?` | Creator metadata/value within this model. |
| `discussions.results[].video.publisher` | `string?` | Publisher metadata/value within this model. |
| `discussions.results[].video.thumbnail` | `object?` | Thumbnail metadata/value within this model. |
| `discussions.results[].video.thumbnail.src` | `string` | Src metadata/value within this model. |
| `discussions.results[].video.thumbnail.alt` | `string?` | Alt metadata/value within this model. |
| `discussions.results[].video.thumbnail.height` | `int?` | Height metadata/value within this model. |
| `discussions.results[].video.thumbnail.width` | `int?` | Width metadata/value within this model. |
| `discussions.results[].video.thumbnail.bg_color` | `string?` | Bg color metadata/value within this model. |
| `discussions.results[].video.thumbnail.original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `discussions.results[].video.thumbnail.logo` | `bool?` | Logo metadata/value within this model. |
| `discussions.results[].video.thumbnail.duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `discussions.results[].video.thumbnail.theme` | `string?` | Theme metadata/value within this model. |
| `discussions.results[].video.tags` | `string[]?` | Tags collection. |
| `discussions.results[].video.author` | `object?` | Author metadata/value within this model. |
| `discussions.results[].video.author.name` | `string` | Name metadata/value within this model. |
| `discussions.results[].video.author.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `discussions.results[].video.author.long_name` | `string?` | Long name metadata/value within this model. |
| `discussions.results[].video.author.img` | `string?` | Img metadata/value within this model. |
| `discussions.results[].video.requires_subscription` | `bool?` | Source-content access restriction, independent of Brave API billing. |
| `discussions.results[].movie` | `object?` | Movie metadata/value within this model. |
| `discussions.results[].movie.name` | `string?` | Name metadata/value within this model. |
| `discussions.results[].movie.description` | `string?` | Display description/excerpt; POI Descriptions text is generated and Context source description is query-independent. |
| `discussions.results[].movie.url` | `string?` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `discussions.results[].movie.thumbnail` | `object?` | Thumbnail metadata/value within this model. |
| `discussions.results[].movie.thumbnail.src` | `string` | Src metadata/value within this model. |
| `discussions.results[].movie.thumbnail.alt` | `string?` | Alt metadata/value within this model. |
| `discussions.results[].movie.thumbnail.height` | `int?` | Height metadata/value within this model. |
| `discussions.results[].movie.thumbnail.width` | `int?` | Width metadata/value within this model. |
| `discussions.results[].movie.thumbnail.bg_color` | `string?` | Bg color metadata/value within this model. |
| `discussions.results[].movie.thumbnail.original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `discussions.results[].movie.thumbnail.logo` | `bool?` | Logo metadata/value within this model. |
| `discussions.results[].movie.thumbnail.duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `discussions.results[].movie.thumbnail.theme` | `string?` | Theme metadata/value within this model. |
| `discussions.results[].movie.release` | `string?` | Release metadata/value within this model. |
| `discussions.results[].movie.directors` | `object[]?` | Directors collection. |
| `discussions.results[].movie.directors[].type` | `string?` | Documented discriminator `person` identifies this enclosing model; unknown future tags remain accepted. |
| `discussions.results[].movie.directors[].name` | `string` | Name metadata/value within this model. |
| `discussions.results[].movie.directors[].url` | `string?` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `discussions.results[].movie.directors[].thumbnail` | `object?` | Thumbnail metadata/value within this model. |
| `discussions.results[].movie.directors[].thumbnail.src` | `string` | Src metadata/value within this model. |
| `discussions.results[].movie.directors[].thumbnail.alt` | `string?` | Alt metadata/value within this model. |
| `discussions.results[].movie.directors[].thumbnail.height` | `int?` | Height metadata/value within this model. |
| `discussions.results[].movie.directors[].thumbnail.width` | `int?` | Width metadata/value within this model. |
| `discussions.results[].movie.directors[].thumbnail.bg_color` | `string?` | Bg color metadata/value within this model. |
| `discussions.results[].movie.directors[].thumbnail.original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `discussions.results[].movie.directors[].thumbnail.logo` | `bool?` | Logo metadata/value within this model. |
| `discussions.results[].movie.directors[].thumbnail.duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `discussions.results[].movie.directors[].thumbnail.theme` | `string?` | Theme metadata/value within this model. |
| `discussions.results[].movie.directors[].email` | `string?` | Email metadata/value within this model. |
| `discussions.results[].movie.actors` | `object[]?` | Actors collection. |
| `discussions.results[].movie.actors[].type` | `string?` | Documented discriminator `person` identifies this enclosing model; unknown future tags remain accepted. |
| `discussions.results[].movie.actors[].name` | `string` | Name metadata/value within this model. |
| `discussions.results[].movie.actors[].url` | `string?` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `discussions.results[].movie.actors[].thumbnail` | `object?` | Thumbnail metadata/value within this model. |
| `discussions.results[].movie.actors[].thumbnail.src` | `string` | Src metadata/value within this model. |
| `discussions.results[].movie.actors[].thumbnail.alt` | `string?` | Alt metadata/value within this model. |
| `discussions.results[].movie.actors[].thumbnail.height` | `int?` | Height metadata/value within this model. |
| `discussions.results[].movie.actors[].thumbnail.width` | `int?` | Width metadata/value within this model. |
| `discussions.results[].movie.actors[].thumbnail.bg_color` | `string?` | Bg color metadata/value within this model. |
| `discussions.results[].movie.actors[].thumbnail.original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `discussions.results[].movie.actors[].thumbnail.logo` | `bool?` | Logo metadata/value within this model. |
| `discussions.results[].movie.actors[].thumbnail.duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `discussions.results[].movie.actors[].thumbnail.theme` | `string?` | Theme metadata/value within this model. |
| `discussions.results[].movie.actors[].email` | `string?` | Email metadata/value within this model. |
| `discussions.results[].movie.rating` | `object?` | Rating metadata/value within this model. |
| `discussions.results[].movie.rating.ratingValue` | `number` | Source-specific rating value; compare only with that source and scale. |
| `discussions.results[].movie.rating.bestRating` | `number` | Source rating scale metadata; not global relevance. |
| `discussions.results[].movie.rating.reviewCount` | `int?` | Number of reviews reported by source. |
| `discussions.results[].movie.rating.profile` | `object?` | Profile metadata/value within this model. |
| `discussions.results[].movie.rating.profile.name` | `string` | Name metadata/value within this model. |
| `discussions.results[].movie.rating.profile.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `discussions.results[].movie.rating.profile.long_name` | `string?` | Long name metadata/value within this model. |
| `discussions.results[].movie.rating.profile.img` | `string?` | Img metadata/value within this model. |
| `discussions.results[].movie.rating.is_tripadvisor` | `bool?` | Attribution flag for the rating source. |
| `discussions.results[].movie.duration` | `string?` | Duration metadata/value within this model. |
| `discussions.results[].movie.genre` | `string[]?` | Genre collection. |
| `discussions.results[].movie.query` | `string?` | Query metadata object, or corrected/completed phrase on suggestion/spellcheck items. |
| `discussions.results[].faq` | `object?` | Faq metadata/value within this model. |
| `discussions.results[].faq.items` | `object[]` | Items collection. |
| `discussions.results[].faq.items[].question` | `string` | Question metadata/value within this model. |
| `discussions.results[].faq.items[].answer` | `string` | Calculator or question/answer result value according to enclosing type; not a general full-page field. |
| `discussions.results[].faq.items[].title` | `string` | Title metadata/value within this model. |
| `discussions.results[].faq.items[].url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `discussions.results[].faq.items[].meta_url` | `object?` | Meta url metadata/value within this model. |
| `discussions.results[].faq.items[].meta_url.scheme` | `string` | Scheme metadata/value within this model. |
| `discussions.results[].faq.items[].meta_url.netloc` | `string` | Netloc metadata/value within this model. |
| `discussions.results[].faq.items[].meta_url.hostname` | `string?` | Hostname metadata/value within this model. |
| `discussions.results[].faq.items[].meta_url.favicon` | `string` | Favicon metadata/value within this model. |
| `discussions.results[].faq.items[].meta_url.path` | `string` | Path metadata/value within this model. |
| `discussions.results[].qa` | `object?` | Qa metadata/value within this model. |
| `discussions.results[].qa.question` | `string` | Question metadata/value within this model. |
| `discussions.results[].qa.answer` | `object` | Calculator or question/answer result value according to enclosing type; not a general full-page field. |
| `discussions.results[].qa.answer.text` | `string` | Text metadata/value within this model. |
| `discussions.results[].qa.answer.author` | `string?` | Author metadata/value within this model. |
| `discussions.results[].qa.answer.upvoteCount` | `int?` | Upvote count metadata/value within this model. |
| `discussions.results[].qa.answer.downvoteCount` | `int?` | Downvote count metadata/value within this model. |
| `discussions.results[].book` | `object?` | Book metadata/value within this model. |
| `discussions.results[].book.title` | `string` | Title metadata/value within this model. |
| `discussions.results[].book.author` | `object[]` | Author collection. |
| `discussions.results[].book.author[].type` | `string?` | Documented discriminator `person` identifies this enclosing model; unknown future tags remain accepted. |
| `discussions.results[].book.author[].name` | `string` | Name metadata/value within this model. |
| `discussions.results[].book.author[].url` | `string?` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `discussions.results[].book.author[].thumbnail` | `object?` | Thumbnail metadata/value within this model. |
| `discussions.results[].book.author[].thumbnail.src` | `string` | Src metadata/value within this model. |
| `discussions.results[].book.author[].thumbnail.alt` | `string?` | Alt metadata/value within this model. |
| `discussions.results[].book.author[].thumbnail.height` | `int?` | Height metadata/value within this model. |
| `discussions.results[].book.author[].thumbnail.width` | `int?` | Width metadata/value within this model. |
| `discussions.results[].book.author[].thumbnail.bg_color` | `string?` | Bg color metadata/value within this model. |
| `discussions.results[].book.author[].thumbnail.original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `discussions.results[].book.author[].thumbnail.logo` | `bool?` | Logo metadata/value within this model. |
| `discussions.results[].book.author[].thumbnail.duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `discussions.results[].book.author[].thumbnail.theme` | `string?` | Theme metadata/value within this model. |
| `discussions.results[].book.author[].email` | `string?` | Email metadata/value within this model. |
| `discussions.results[].book.date` | `string?` | Date metadata/value within this model. |
| `discussions.results[].book.price` | `object?` | Price metadata/value within this model. |
| `discussions.results[].book.price.price` | `string` | Price metadata/value within this model. |
| `discussions.results[].book.price.priceCurrency` | `string` | Price currency metadata/value within this model. |
| `discussions.results[].book.pages` | `int?` | Pages metadata/value within this model. |
| `discussions.results[].book.publisher` | `object?` | Publisher metadata/value within this model. |
| `discussions.results[].book.publisher.type` | `string?` | Documented discriminator `person` identifies this enclosing model; unknown future tags remain accepted. |
| `discussions.results[].book.publisher.name` | `string` | Name metadata/value within this model. |
| `discussions.results[].book.publisher.url` | `string?` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `discussions.results[].book.publisher.thumbnail` | `object?` | Thumbnail metadata/value within this model. |
| `discussions.results[].book.publisher.thumbnail.src` | `string` | Src metadata/value within this model. |
| `discussions.results[].book.publisher.thumbnail.alt` | `string?` | Alt metadata/value within this model. |
| `discussions.results[].book.publisher.thumbnail.height` | `int?` | Height metadata/value within this model. |
| `discussions.results[].book.publisher.thumbnail.width` | `int?` | Width metadata/value within this model. |
| `discussions.results[].book.publisher.thumbnail.bg_color` | `string?` | Bg color metadata/value within this model. |
| `discussions.results[].book.publisher.thumbnail.original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `discussions.results[].book.publisher.thumbnail.logo` | `bool?` | Logo metadata/value within this model. |
| `discussions.results[].book.publisher.thumbnail.duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `discussions.results[].book.publisher.thumbnail.theme` | `string?` | Theme metadata/value within this model. |
| `discussions.results[].book.publisher.email` | `string?` | Email metadata/value within this model. |
| `discussions.results[].book.rating` | `object?` | Rating metadata/value within this model. |
| `discussions.results[].book.rating.ratingValue` | `number` | Source-specific rating value; compare only with that source and scale. |
| `discussions.results[].book.rating.bestRating` | `number` | Source rating scale metadata; not global relevance. |
| `discussions.results[].book.rating.reviewCount` | `int?` | Number of reviews reported by source. |
| `discussions.results[].book.rating.profile` | `object?` | Profile metadata/value within this model. |
| `discussions.results[].book.rating.profile.name` | `string` | Name metadata/value within this model. |
| `discussions.results[].book.rating.profile.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `discussions.results[].book.rating.profile.long_name` | `string?` | Long name metadata/value within this model. |
| `discussions.results[].book.rating.profile.img` | `string?` | Img metadata/value within this model. |
| `discussions.results[].book.rating.is_tripadvisor` | `bool?` | Attribution flag for the rating source. |
| `discussions.results[].rating` | `object?` | Rating metadata/value within this model. |
| `discussions.results[].rating.ratingValue` | `number` | Source-specific rating value; compare only with that source and scale. |
| `discussions.results[].rating.bestRating` | `number` | Source rating scale metadata; not global relevance. |
| `discussions.results[].rating.reviewCount` | `int?` | Number of reviews reported by source. |
| `discussions.results[].rating.profile` | `object?` | Profile metadata/value within this model. |
| `discussions.results[].rating.profile.name` | `string` | Name metadata/value within this model. |
| `discussions.results[].rating.profile.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `discussions.results[].rating.profile.long_name` | `string?` | Long name metadata/value within this model. |
| `discussions.results[].rating.profile.img` | `string?` | Img metadata/value within this model. |
| `discussions.results[].rating.is_tripadvisor` | `bool?` | Attribution flag for the rating source. |
| `discussions.results[].article` | `object?` | Article metadata/value within this model. |
| `discussions.results[].article.author` | `object[]?` | Author collection. |
| `discussions.results[].article.author[].type` | `string?` | Documented discriminator `person` identifies this enclosing model; unknown future tags remain accepted. |
| `discussions.results[].article.author[].name` | `string` | Name metadata/value within this model. |
| `discussions.results[].article.author[].url` | `string?` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `discussions.results[].article.author[].thumbnail` | `object?` | Thumbnail metadata/value within this model. |
| `discussions.results[].article.author[].thumbnail.src` | `string` | Src metadata/value within this model. |
| `discussions.results[].article.author[].thumbnail.alt` | `string?` | Alt metadata/value within this model. |
| `discussions.results[].article.author[].thumbnail.height` | `int?` | Height metadata/value within this model. |
| `discussions.results[].article.author[].thumbnail.width` | `int?` | Width metadata/value within this model. |
| `discussions.results[].article.author[].thumbnail.bg_color` | `string?` | Bg color metadata/value within this model. |
| `discussions.results[].article.author[].thumbnail.original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `discussions.results[].article.author[].thumbnail.logo` | `bool?` | Logo metadata/value within this model. |
| `discussions.results[].article.author[].thumbnail.duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `discussions.results[].article.author[].thumbnail.theme` | `string?` | Theme metadata/value within this model. |
| `discussions.results[].article.author[].email` | `string?` | Email metadata/value within this model. |
| `discussions.results[].article.date` | `string?` | Date metadata/value within this model. |
| `discussions.results[].article.publisher` | `object?` | Publisher metadata/value within this model. |
| `discussions.results[].article.publisher.type` | `string?` | Documented discriminator `organization` identifies this enclosing model; unknown future tags remain accepted. |
| `discussions.results[].article.publisher.name` | `string` | Name metadata/value within this model. |
| `discussions.results[].article.publisher.url` | `string?` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `discussions.results[].article.publisher.thumbnail` | `object?` | Thumbnail metadata/value within this model. |
| `discussions.results[].article.publisher.thumbnail.src` | `string` | Src metadata/value within this model. |
| `discussions.results[].article.publisher.thumbnail.alt` | `string?` | Alt metadata/value within this model. |
| `discussions.results[].article.publisher.thumbnail.height` | `int?` | Height metadata/value within this model. |
| `discussions.results[].article.publisher.thumbnail.width` | `int?` | Width metadata/value within this model. |
| `discussions.results[].article.publisher.thumbnail.bg_color` | `string?` | Bg color metadata/value within this model. |
| `discussions.results[].article.publisher.thumbnail.original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `discussions.results[].article.publisher.thumbnail.logo` | `bool?` | Logo metadata/value within this model. |
| `discussions.results[].article.publisher.thumbnail.duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `discussions.results[].article.publisher.thumbnail.theme` | `string?` | Theme metadata/value within this model. |
| `discussions.results[].article.publisher.contact_points` | `object[]?` | Contact points collection. |
| `discussions.results[].article.publisher.contact_points[].type` | `string?` | Documented discriminator `contact_point` identifies this enclosing model; unknown future tags remain accepted. |
| `discussions.results[].article.publisher.contact_points[].name` | `string` | Name metadata/value within this model. |
| `discussions.results[].article.publisher.contact_points[].url` | `string?` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `discussions.results[].article.publisher.contact_points[].thumbnail` | `object?` | Thumbnail metadata/value within this model. |
| `discussions.results[].article.publisher.contact_points[].thumbnail.src` | `string` | Src metadata/value within this model. |
| `discussions.results[].article.publisher.contact_points[].thumbnail.alt` | `string?` | Alt metadata/value within this model. |
| `discussions.results[].article.publisher.contact_points[].thumbnail.height` | `int?` | Height metadata/value within this model. |
| `discussions.results[].article.publisher.contact_points[].thumbnail.width` | `int?` | Width metadata/value within this model. |
| `discussions.results[].article.publisher.contact_points[].thumbnail.bg_color` | `string?` | Bg color metadata/value within this model. |
| `discussions.results[].article.publisher.contact_points[].thumbnail.original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `discussions.results[].article.publisher.contact_points[].thumbnail.logo` | `bool?` | Logo metadata/value within this model. |
| `discussions.results[].article.publisher.contact_points[].thumbnail.duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `discussions.results[].article.publisher.contact_points[].thumbnail.theme` | `string?` | Theme metadata/value within this model. |
| `discussions.results[].article.publisher.contact_points[].telephone` | `string?` | Telephone metadata/value within this model. |
| `discussions.results[].article.publisher.contact_points[].email` | `string?` | Email metadata/value within this model. |
| `discussions.results[].article.thumbnail` | `object?` | Thumbnail metadata/value within this model. |
| `discussions.results[].article.thumbnail.src` | `string` | Src metadata/value within this model. |
| `discussions.results[].article.thumbnail.alt` | `string?` | Alt metadata/value within this model. |
| `discussions.results[].article.thumbnail.height` | `int?` | Height metadata/value within this model. |
| `discussions.results[].article.thumbnail.width` | `int?` | Width metadata/value within this model. |
| `discussions.results[].article.thumbnail.bg_color` | `string?` | Bg color metadata/value within this model. |
| `discussions.results[].article.thumbnail.original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `discussions.results[].article.thumbnail.logo` | `bool?` | Logo metadata/value within this model. |
| `discussions.results[].article.thumbnail.duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `discussions.results[].article.thumbnail.theme` | `string?` | Theme metadata/value within this model. |
| `discussions.results[].article.isAccessibleForFree` | `bool?` | Is accessible for free metadata/value within this model. |
| `discussions.results[].product` | `any?` | Product metadata/value within this model. |
| `discussions.results[].product.type` | `string?` | Documented discriminator `Product` identifies this enclosing model; unknown future tags remain accepted. |
| `discussions.results[].product.name` | `string` | Name metadata/value within this model. |
| `discussions.results[].product.url` | `string?` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `discussions.results[].product.category` | `string?` | Category metadata/value within this model. |
| `discussions.results[].product.price` | `string` | Price metadata/value within this model. |
| `discussions.results[].product.thumbnail` | `object` | Thumbnail metadata/value within this model. |
| `discussions.results[].product.thumbnail.src` | `string` | Src metadata/value within this model. |
| `discussions.results[].product.thumbnail.alt` | `string?` | Alt metadata/value within this model. |
| `discussions.results[].product.thumbnail.height` | `int?` | Height metadata/value within this model. |
| `discussions.results[].product.thumbnail.width` | `int?` | Width metadata/value within this model. |
| `discussions.results[].product.thumbnail.bg_color` | `string?` | Bg color metadata/value within this model. |
| `discussions.results[].product.thumbnail.original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `discussions.results[].product.thumbnail.logo` | `bool?` | Logo metadata/value within this model. |
| `discussions.results[].product.thumbnail.duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `discussions.results[].product.thumbnail.theme` | `string?` | Theme metadata/value within this model. |
| `discussions.results[].product.description` | `string?` | Display description/excerpt; POI Descriptions text is generated and Context source description is query-independent. |
| `discussions.results[].product.offers` | `object[]?` | Offers collection. |
| `discussions.results[].product.offers[].url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `discussions.results[].product.offers[].priceCurrency` | `string` | Price currency metadata/value within this model. |
| `discussions.results[].product.offers[].price` | `string` | Price metadata/value within this model. |
| `discussions.results[].product.rating` | `object?` | Rating metadata/value within this model. |
| `discussions.results[].product.rating.ratingValue` | `number` | Source-specific rating value; compare only with that source and scale. |
| `discussions.results[].product.rating.bestRating` | `number` | Source rating scale metadata; not global relevance. |
| `discussions.results[].product.rating.reviewCount` | `int?` | Number of reviews reported by source. |
| `discussions.results[].product.rating.profile` | `object?` | Profile metadata/value within this model. |
| `discussions.results[].product.rating.profile.name` | `string` | Name metadata/value within this model. |
| `discussions.results[].product.rating.profile.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `discussions.results[].product.rating.profile.long_name` | `string?` | Long name metadata/value within this model. |
| `discussions.results[].product.rating.profile.img` | `string?` | Img metadata/value within this model. |
| `discussions.results[].product.rating.is_tripadvisor` | `bool?` | Attribution flag for the rating source. |
| `discussions.results[].product.gtin` | `string?` | Gtin metadata/value within this model. |
| `discussions.results[].product.gtin8` | `string?` | Gtin8 metadata/value within this model. |
| `discussions.results[].product.gtin12` | `string?` | Gtin12 metadata/value within this model. |
| `discussions.results[].product.gtin13` | `string?` | Gtin13 metadata/value within this model. |
| `discussions.results[].product.gtin14` | `string?` | Gtin14 metadata/value within this model. |
| `discussions.results[].product_cluster` | `any[]?` | Product cluster collection. |
| `discussions.results[].product_cluster[].type` | `string?` | Documented discriminator `Product` identifies this enclosing model; unknown future tags remain accepted. |
| `discussions.results[].product_cluster[].name` | `string` | Name metadata/value within this model. |
| `discussions.results[].product_cluster[].url` | `string?` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `discussions.results[].product_cluster[].category` | `string?` | Category metadata/value within this model. |
| `discussions.results[].product_cluster[].price` | `string` | Price metadata/value within this model. |
| `discussions.results[].product_cluster[].thumbnail` | `object` | Thumbnail metadata/value within this model. |
| `discussions.results[].product_cluster[].thumbnail.src` | `string` | Src metadata/value within this model. |
| `discussions.results[].product_cluster[].thumbnail.alt` | `string?` | Alt metadata/value within this model. |
| `discussions.results[].product_cluster[].thumbnail.height` | `int?` | Height metadata/value within this model. |
| `discussions.results[].product_cluster[].thumbnail.width` | `int?` | Width metadata/value within this model. |
| `discussions.results[].product_cluster[].thumbnail.bg_color` | `string?` | Bg color metadata/value within this model. |
| `discussions.results[].product_cluster[].thumbnail.original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `discussions.results[].product_cluster[].thumbnail.logo` | `bool?` | Logo metadata/value within this model. |
| `discussions.results[].product_cluster[].thumbnail.duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `discussions.results[].product_cluster[].thumbnail.theme` | `string?` | Theme metadata/value within this model. |
| `discussions.results[].product_cluster[].description` | `string?` | Display description/excerpt; POI Descriptions text is generated and Context source description is query-independent. |
| `discussions.results[].product_cluster[].offers` | `object[]?` | Offers collection. |
| `discussions.results[].product_cluster[].offers[].url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `discussions.results[].product_cluster[].offers[].priceCurrency` | `string` | Price currency metadata/value within this model. |
| `discussions.results[].product_cluster[].offers[].price` | `string` | Price metadata/value within this model. |
| `discussions.results[].product_cluster[].rating` | `object?` | Rating metadata/value within this model. |
| `discussions.results[].product_cluster[].rating.ratingValue` | `number` | Source-specific rating value; compare only with that source and scale. |
| `discussions.results[].product_cluster[].rating.bestRating` | `number` | Source rating scale metadata; not global relevance. |
| `discussions.results[].product_cluster[].rating.reviewCount` | `int?` | Number of reviews reported by source. |
| `discussions.results[].product_cluster[].rating.profile` | `object?` | Profile metadata/value within this model. |
| `discussions.results[].product_cluster[].rating.profile.name` | `string` | Name metadata/value within this model. |
| `discussions.results[].product_cluster[].rating.profile.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `discussions.results[].product_cluster[].rating.profile.long_name` | `string?` | Long name metadata/value within this model. |
| `discussions.results[].product_cluster[].rating.profile.img` | `string?` | Img metadata/value within this model. |
| `discussions.results[].product_cluster[].rating.is_tripadvisor` | `bool?` | Attribution flag for the rating source. |
| `discussions.results[].product_cluster[].gtin` | `string?` | Gtin metadata/value within this model. |
| `discussions.results[].product_cluster[].gtin8` | `string?` | Gtin8 metadata/value within this model. |
| `discussions.results[].product_cluster[].gtin12` | `string?` | Gtin12 metadata/value within this model. |
| `discussions.results[].product_cluster[].gtin13` | `string?` | Gtin13 metadata/value within this model. |
| `discussions.results[].product_cluster[].gtin14` | `string?` | Gtin14 metadata/value within this model. |
| `discussions.results[].cluster_type` | `string?` | Cluster type metadata/value within this model. |
| `discussions.results[].cluster` | `object[]?` | Cluster collection. |
| `discussions.results[].cluster[].title` | `string` | Title metadata/value within this model. |
| `discussions.results[].cluster[].url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `discussions.results[].cluster[].is_source_local` | `bool?` | Is source local metadata/value within this model. |
| `discussions.results[].cluster[].is_source_both` | `bool?` | Is source both metadata/value within this model. |
| `discussions.results[].cluster[].description` | `string?` | Display description/excerpt; POI Descriptions text is generated and Context source description is query-independent. |
| `discussions.results[].cluster[].page_age` | `string?` | Content publication/modification date, not necessarily crawl time. |
| `discussions.results[].cluster[].page_fetched` | `string?` | Last-fetch date/time as provided by the vendor. |
| `discussions.results[].cluster[].fetched_content_timestamp` | `int?` | Fetch timestamp; do not infer units beyond the source contract. |
| `discussions.results[].cluster[].profile` | `object?` | Profile metadata/value within this model. |
| `discussions.results[].cluster[].profile.name` | `string` | Name metadata/value within this model. |
| `discussions.results[].cluster[].profile.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `discussions.results[].cluster[].profile.long_name` | `string?` | Long name metadata/value within this model. |
| `discussions.results[].cluster[].profile.img` | `string?` | Img metadata/value within this model. |
| `discussions.results[].cluster[].language` | `string?` | Language metadata/value within this model. |
| `discussions.results[].cluster[].family_friendly` | `bool?` | Vendor safety classification, not a content-safety guarantee. |
| `discussions.results[].creative_work` | `object?` | Creative work metadata/value within this model. |
| `discussions.results[].creative_work.name` | `string` | Name metadata/value within this model. |
| `discussions.results[].creative_work.rating` | `object?` | Rating metadata/value within this model. |
| `discussions.results[].creative_work.rating.ratingValue` | `number` | Source-specific rating value; compare only with that source and scale. |
| `discussions.results[].creative_work.rating.bestRating` | `number` | Source rating scale metadata; not global relevance. |
| `discussions.results[].creative_work.rating.reviewCount` | `int?` | Number of reviews reported by source. |
| `discussions.results[].creative_work.rating.profile` | `object?` | Profile metadata/value within this model. |
| `discussions.results[].creative_work.rating.profile.name` | `string` | Name metadata/value within this model. |
| `discussions.results[].creative_work.rating.profile.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `discussions.results[].creative_work.rating.profile.long_name` | `string?` | Long name metadata/value within this model. |
| `discussions.results[].creative_work.rating.profile.img` | `string?` | Img metadata/value within this model. |
| `discussions.results[].creative_work.rating.is_tripadvisor` | `bool?` | Attribution flag for the rating source. |
| `discussions.results[].creative_work.thumbnail` | `object` | Thumbnail metadata/value within this model. |
| `discussions.results[].creative_work.thumbnail.src` | `string` | Src metadata/value within this model. |
| `discussions.results[].creative_work.thumbnail.alt` | `string?` | Alt metadata/value within this model. |
| `discussions.results[].creative_work.thumbnail.height` | `int?` | Height metadata/value within this model. |
| `discussions.results[].creative_work.thumbnail.width` | `int?` | Width metadata/value within this model. |
| `discussions.results[].creative_work.thumbnail.bg_color` | `string?` | Bg color metadata/value within this model. |
| `discussions.results[].creative_work.thumbnail.original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `discussions.results[].creative_work.thumbnail.logo` | `bool?` | Logo metadata/value within this model. |
| `discussions.results[].creative_work.thumbnail.duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `discussions.results[].creative_work.thumbnail.theme` | `string?` | Theme metadata/value within this model. |
| `discussions.results[].music_recording` | `object?` | Music recording metadata/value within this model. |
| `discussions.results[].music_recording.name` | `string` | Name metadata/value within this model. |
| `discussions.results[].music_recording.rating` | `object?` | Rating metadata/value within this model. |
| `discussions.results[].music_recording.rating.ratingValue` | `number` | Source-specific rating value; compare only with that source and scale. |
| `discussions.results[].music_recording.rating.bestRating` | `number` | Source rating scale metadata; not global relevance. |
| `discussions.results[].music_recording.rating.reviewCount` | `int?` | Number of reviews reported by source. |
| `discussions.results[].music_recording.rating.profile` | `object?` | Profile metadata/value within this model. |
| `discussions.results[].music_recording.rating.profile.name` | `string` | Name metadata/value within this model. |
| `discussions.results[].music_recording.rating.profile.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `discussions.results[].music_recording.rating.profile.long_name` | `string?` | Long name metadata/value within this model. |
| `discussions.results[].music_recording.rating.profile.img` | `string?` | Img metadata/value within this model. |
| `discussions.results[].music_recording.rating.is_tripadvisor` | `bool?` | Attribution flag for the rating source. |
| `discussions.results[].music_recording.thumbnail` | `object?` | Thumbnail metadata/value within this model. |
| `discussions.results[].music_recording.thumbnail.src` | `string` | Src metadata/value within this model. |
| `discussions.results[].music_recording.thumbnail.alt` | `string?` | Alt metadata/value within this model. |
| `discussions.results[].music_recording.thumbnail.height` | `int?` | Height metadata/value within this model. |
| `discussions.results[].music_recording.thumbnail.width` | `int?` | Width metadata/value within this model. |
| `discussions.results[].music_recording.thumbnail.bg_color` | `string?` | Bg color metadata/value within this model. |
| `discussions.results[].music_recording.thumbnail.original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `discussions.results[].music_recording.thumbnail.logo` | `bool?` | Logo metadata/value within this model. |
| `discussions.results[].music_recording.thumbnail.duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `discussions.results[].music_recording.thumbnail.theme` | `string?` | Theme metadata/value within this model. |
| `discussions.results[].review` | `object?` | Review metadata/value within this model. |
| `discussions.results[].review.type` | `string?` | Documented discriminator `Review` identifies this enclosing model; unknown future tags remain accepted. |
| `discussions.results[].review.name` | `string` | Name metadata/value within this model. |
| `discussions.results[].review.thumbnail` | `object` | Thumbnail metadata/value within this model. |
| `discussions.results[].review.thumbnail.src` | `string` | Src metadata/value within this model. |
| `discussions.results[].review.thumbnail.alt` | `string?` | Alt metadata/value within this model. |
| `discussions.results[].review.thumbnail.height` | `int?` | Height metadata/value within this model. |
| `discussions.results[].review.thumbnail.width` | `int?` | Width metadata/value within this model. |
| `discussions.results[].review.thumbnail.bg_color` | `string?` | Bg color metadata/value within this model. |
| `discussions.results[].review.thumbnail.original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `discussions.results[].review.thumbnail.logo` | `bool?` | Logo metadata/value within this model. |
| `discussions.results[].review.thumbnail.duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `discussions.results[].review.thumbnail.theme` | `string?` | Theme metadata/value within this model. |
| `discussions.results[].review.description` | `string` | Display description/excerpt; POI Descriptions text is generated and Context source description is query-independent. |
| `discussions.results[].review.rating` | `object` | Rating metadata/value within this model. |
| `discussions.results[].review.rating.ratingValue` | `number` | Source-specific rating value; compare only with that source and scale. |
| `discussions.results[].review.rating.bestRating` | `number` | Source rating scale metadata; not global relevance. |
| `discussions.results[].review.rating.reviewCount` | `int?` | Number of reviews reported by source. |
| `discussions.results[].review.rating.profile` | `object?` | Profile metadata/value within this model. |
| `discussions.results[].review.rating.profile.name` | `string` | Name metadata/value within this model. |
| `discussions.results[].review.rating.profile.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `discussions.results[].review.rating.profile.long_name` | `string?` | Long name metadata/value within this model. |
| `discussions.results[].review.rating.profile.img` | `string?` | Img metadata/value within this model. |
| `discussions.results[].review.rating.is_tripadvisor` | `bool?` | Attribution flag for the rating source. |
| `discussions.results[].recipe` | `object?` | Recipe metadata/value within this model. |
| `discussions.results[].recipe.title` | `string` | Title metadata/value within this model. |
| `discussions.results[].recipe.description` | `string` | Display description/excerpt; POI Descriptions text is generated and Context source description is query-independent. |
| `discussions.results[].recipe.thumbnail` | `object` | Thumbnail metadata/value within this model. |
| `discussions.results[].recipe.thumbnail.src` | `string` | Src metadata/value within this model. |
| `discussions.results[].recipe.thumbnail.alt` | `string?` | Alt metadata/value within this model. |
| `discussions.results[].recipe.thumbnail.height` | `int?` | Height metadata/value within this model. |
| `discussions.results[].recipe.thumbnail.width` | `int?` | Width metadata/value within this model. |
| `discussions.results[].recipe.thumbnail.bg_color` | `string?` | Bg color metadata/value within this model. |
| `discussions.results[].recipe.thumbnail.original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `discussions.results[].recipe.thumbnail.logo` | `bool?` | Logo metadata/value within this model. |
| `discussions.results[].recipe.thumbnail.duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `discussions.results[].recipe.thumbnail.theme` | `string?` | Theme metadata/value within this model. |
| `discussions.results[].recipe.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `discussions.results[].recipe.domain` | `string` | Domain metadata/value within this model. |
| `discussions.results[].recipe.favicon` | `string` | Favicon metadata/value within this model. |
| `discussions.results[].recipe.time` | `string?` | Time metadata/value within this model. |
| `discussions.results[].recipe.prep_time` | `string?` | Prep time metadata/value within this model. |
| `discussions.results[].recipe.cook_time` | `string?` | Cook time metadata/value within this model. |
| `discussions.results[].recipe.ingredients` | `string?` | Ingredients metadata/value within this model. |
| `discussions.results[].recipe.instructions` | `object[]?` | Instructions collection. |
| `discussions.results[].recipe.instructions[].text` | `string` | Text metadata/value within this model. |
| `discussions.results[].recipe.instructions[].name` | `string?` | Name metadata/value within this model. |
| `discussions.results[].recipe.instructions[].url` | `string?` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `discussions.results[].recipe.instructions[].image` | `string[]?` | Image collection. |
| `discussions.results[].recipe.servings` | `int?` | Servings metadata/value within this model. |
| `discussions.results[].recipe.calories` | `int?` | Calories metadata/value within this model. |
| `discussions.results[].recipe.publisher` | `string?` | Publisher metadata/value within this model. |
| `discussions.results[].recipe.rating` | `object?` | Rating metadata/value within this model. |
| `discussions.results[].recipe.rating.ratingValue` | `number` | Source-specific rating value; compare only with that source and scale. |
| `discussions.results[].recipe.rating.bestRating` | `number` | Source rating scale metadata; not global relevance. |
| `discussions.results[].recipe.rating.reviewCount` | `int?` | Number of reviews reported by source. |
| `discussions.results[].recipe.rating.profile` | `object?` | Profile metadata/value within this model. |
| `discussions.results[].recipe.rating.profile.name` | `string` | Name metadata/value within this model. |
| `discussions.results[].recipe.rating.profile.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `discussions.results[].recipe.rating.profile.long_name` | `string?` | Long name metadata/value within this model. |
| `discussions.results[].recipe.rating.profile.img` | `string?` | Img metadata/value within this model. |
| `discussions.results[].recipe.rating.is_tripadvisor` | `bool?` | Attribution flag for the rating source. |
| `discussions.results[].recipe.recipeCategory` | `string?` | Recipe category metadata/value within this model. |
| `discussions.results[].recipe.recipeCuisine` | `string?` | Recipe cuisine metadata/value within this model. |
| `discussions.results[].recipe.video` | `object?` | Video metadata/value within this model. |
| `discussions.results[].recipe.video.duration` | `string?` | Duration metadata/value within this model. |
| `discussions.results[].recipe.video.views` | `string?` | Views metadata/value within this model. |
| `discussions.results[].recipe.video.creator` | `string?` | Creator metadata/value within this model. |
| `discussions.results[].recipe.video.publisher` | `string?` | Publisher metadata/value within this model. |
| `discussions.results[].recipe.video.thumbnail` | `object?` | Thumbnail metadata/value within this model. |
| `discussions.results[].recipe.video.thumbnail.src` | `string` | Src metadata/value within this model. |
| `discussions.results[].recipe.video.thumbnail.alt` | `string?` | Alt metadata/value within this model. |
| `discussions.results[].recipe.video.thumbnail.height` | `int?` | Height metadata/value within this model. |
| `discussions.results[].recipe.video.thumbnail.width` | `int?` | Width metadata/value within this model. |
| `discussions.results[].recipe.video.thumbnail.bg_color` | `string?` | Bg color metadata/value within this model. |
| `discussions.results[].recipe.video.thumbnail.original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `discussions.results[].recipe.video.thumbnail.logo` | `bool?` | Logo metadata/value within this model. |
| `discussions.results[].recipe.video.thumbnail.duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `discussions.results[].recipe.video.thumbnail.theme` | `string?` | Theme metadata/value within this model. |
| `discussions.results[].recipe.video.tags` | `string[]?` | Tags collection. |
| `discussions.results[].recipe.video.author` | `object?` | Author metadata/value within this model. |
| `discussions.results[].recipe.video.author.name` | `string` | Name metadata/value within this model. |
| `discussions.results[].recipe.video.author.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `discussions.results[].recipe.video.author.long_name` | `string?` | Long name metadata/value within this model. |
| `discussions.results[].recipe.video.author.img` | `string?` | Img metadata/value within this model. |
| `discussions.results[].recipe.video.requires_subscription` | `bool?` | Source-content access restriction, independent of Brave API billing. |
| `discussions.results[].software` | `object?` | Software metadata/value within this model. |
| `discussions.results[].software.name` | `string?` | Name metadata/value within this model. |
| `discussions.results[].software.author` | `string?` | Author metadata/value within this model. |
| `discussions.results[].software.version` | `string?` | Version metadata/value within this model. |
| `discussions.results[].software.codeRepository` | `string?` | Code repository metadata/value within this model. |
| `discussions.results[].software.homepage` | `string?` | Homepage metadata/value within this model. |
| `discussions.results[].software.datePublished` | `string?` | Date published metadata/value within this model. |
| `discussions.results[].software.is_npm` | `bool?` | Is npm metadata/value within this model. |
| `discussions.results[].software.is_pypi` | `bool?` | Is pypi metadata/value within this model. |
| `discussions.results[].software.stars` | `int?` | Stars metadata/value within this model. |
| `discussions.results[].software.forks` | `int?` | Forks metadata/value within this model. |
| `discussions.results[].software.programmingLanguage` | `string?` | Programming language metadata/value within this model. |
| `discussions.results[].organization` | `object?` | Organization metadata/value within this model. |
| `discussions.results[].organization.type` | `string?` | Documented discriminator `organization` identifies this enclosing model; unknown future tags remain accepted. |
| `discussions.results[].organization.name` | `string` | Name metadata/value within this model. |
| `discussions.results[].organization.url` | `string?` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `discussions.results[].organization.thumbnail` | `object?` | Thumbnail metadata/value within this model. |
| `discussions.results[].organization.thumbnail.src` | `string` | Src metadata/value within this model. |
| `discussions.results[].organization.thumbnail.alt` | `string?` | Alt metadata/value within this model. |
| `discussions.results[].organization.thumbnail.height` | `int?` | Height metadata/value within this model. |
| `discussions.results[].organization.thumbnail.width` | `int?` | Width metadata/value within this model. |
| `discussions.results[].organization.thumbnail.bg_color` | `string?` | Bg color metadata/value within this model. |
| `discussions.results[].organization.thumbnail.original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `discussions.results[].organization.thumbnail.logo` | `bool?` | Logo metadata/value within this model. |
| `discussions.results[].organization.thumbnail.duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `discussions.results[].organization.thumbnail.theme` | `string?` | Theme metadata/value within this model. |
| `discussions.results[].organization.contact_points` | `object[]?` | Contact points collection. |
| `discussions.results[].organization.contact_points[].type` | `string?` | Documented discriminator `contact_point` identifies this enclosing model; unknown future tags remain accepted. |
| `discussions.results[].organization.contact_points[].name` | `string` | Name metadata/value within this model. |
| `discussions.results[].organization.contact_points[].url` | `string?` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `discussions.results[].organization.contact_points[].thumbnail` | `object?` | Thumbnail metadata/value within this model. |
| `discussions.results[].organization.contact_points[].thumbnail.src` | `string` | Src metadata/value within this model. |
| `discussions.results[].organization.contact_points[].thumbnail.alt` | `string?` | Alt metadata/value within this model. |
| `discussions.results[].organization.contact_points[].thumbnail.height` | `int?` | Height metadata/value within this model. |
| `discussions.results[].organization.contact_points[].thumbnail.width` | `int?` | Width metadata/value within this model. |
| `discussions.results[].organization.contact_points[].thumbnail.bg_color` | `string?` | Bg color metadata/value within this model. |
| `discussions.results[].organization.contact_points[].thumbnail.original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `discussions.results[].organization.contact_points[].thumbnail.logo` | `bool?` | Logo metadata/value within this model. |
| `discussions.results[].organization.contact_points[].thumbnail.duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `discussions.results[].organization.contact_points[].thumbnail.theme` | `string?` | Theme metadata/value within this model. |
| `discussions.results[].organization.contact_points[].telephone` | `string?` | Telephone metadata/value within this model. |
| `discussions.results[].organization.contact_points[].email` | `string?` | Email metadata/value within this model. |
| `discussions.results[].content_type` | `string?` | Content type metadata/value within this model. |
| `discussions.results[].extra_snippets` | `string[]?` | Alternative page excerpts, not continuous full text. |
| `discussions.results[].icons` | `object[]?` | Icons collection. |
| `discussions.results[].icons[].href` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `discussions.results[].icons[].sizes` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `discussions.results[].icons[].rel` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `discussions.results[].icons[].type` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `discussions.results[].icons[].ext` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `discussions.results[].data` | `object?` | Data metadata/value within this model. |
| `discussions.results[].data.forum_name` | `string` | Forum name metadata/value within this model. |
| `discussions.results[].data.num_answers` | `int?` | Num answers metadata/value within this model. |
| `discussions.results[].data.score` | `string?` | Forum-post score where documented; not a normalized search-relevance score. |
| `discussions.results[].data.title` | `string?` | Title metadata/value within this model. |
| `discussions.results[].data.question` | `string?` | Question metadata/value within this model. |
| `discussions.results[].data.top_comment` | `string?` | Top comment metadata/value within this model. |
| `discussions.mutated_by_goggles` | `bool?` | Whether Goggles modified these discussion results; documented default false. |
| `faq` | `object?` | Faq metadata/value within this model. |
| `faq.type` | `string?` | Documented discriminator `faq` identifies this enclosing model; unknown future tags remain accepted. |
| `faq.results` | `object[]` | Results collection. |
| `faq.results[].question` | `string` | Question metadata/value within this model. |
| `faq.results[].answer` | `string` | Calculator or question/answer result value according to enclosing type; not a general full-page field. |
| `faq.results[].title` | `string` | Title metadata/value within this model. |
| `faq.results[].url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `faq.results[].meta_url` | `object?` | Meta url metadata/value within this model. |
| `faq.results[].meta_url.scheme` | `string` | Scheme metadata/value within this model. |
| `faq.results[].meta_url.netloc` | `string` | Netloc metadata/value within this model. |
| `faq.results[].meta_url.hostname` | `string?` | Hostname metadata/value within this model. |
| `faq.results[].meta_url.favicon` | `string` | Favicon metadata/value within this model. |
| `faq.results[].meta_url.path` | `string` | Path metadata/value within this model. |
| `infobox` | `object?` | Infobox metadata/value within this model. |
| `infobox.type` | `string?` | Documented discriminator `graph` identifies this enclosing model; unknown future tags remain accepted. |
| `infobox.results` | `any[]` | Results collection. |
| `infobox.results[].title` | `string` | Title metadata/value within this model. |
| `infobox.results[].url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `infobox.results[].is_source_local` | `bool?` | Is source local metadata/value within this model. |
| `infobox.results[].is_source_both` | `bool?` | Is source both metadata/value within this model. |
| `infobox.results[].description` | `string?` | Display description/excerpt; POI Descriptions text is generated and Context source description is query-independent. |
| `infobox.results[].page_age` | `string?` | Content publication/modification date, not necessarily crawl time. |
| `infobox.results[].page_fetched` | `string?` | Last-fetch date/time as provided by the vendor. |
| `infobox.results[].fetched_content_timestamp` | `int?` | Fetch timestamp; do not infer units beyond the source contract. |
| `infobox.results[].profile` | `object?` | Profile metadata/value within this model. |
| `infobox.results[].profile.name` | `string` | Name metadata/value within this model. |
| `infobox.results[].profile.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `infobox.results[].profile.long_name` | `string?` | Long name metadata/value within this model. |
| `infobox.results[].profile.img` | `string?` | Img metadata/value within this model. |
| `infobox.results[].language` | `string?` | Language metadata/value within this model. |
| `infobox.results[].family_friendly` | `bool?` | Vendor safety classification, not a content-safety guarantee. |
| `infobox.results[].type` | `string?` | Documented discriminator `infobox` identifies this enclosing model; unknown future tags remain accepted. |
| `infobox.results[].position` | `int` | Position metadata/value within this model. |
| `infobox.results[].label` | `string?` | Label metadata/value within this model. |
| `infobox.results[].category` | `string?` | Category metadata/value within this model. |
| `infobox.results[].long_desc` | `string?` | Long desc metadata/value within this model. |
| `infobox.results[].thumbnail` | `object?` | Thumbnail metadata/value within this model. |
| `infobox.results[].thumbnail.src` | `string` | Src metadata/value within this model. |
| `infobox.results[].thumbnail.alt` | `string?` | Alt metadata/value within this model. |
| `infobox.results[].thumbnail.height` | `int?` | Height metadata/value within this model. |
| `infobox.results[].thumbnail.width` | `int?` | Width metadata/value within this model. |
| `infobox.results[].thumbnail.bg_color` | `string?` | Bg color metadata/value within this model. |
| `infobox.results[].thumbnail.original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `infobox.results[].thumbnail.logo` | `bool?` | Logo metadata/value within this model. |
| `infobox.results[].thumbnail.duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `infobox.results[].thumbnail.theme` | `string?` | Theme metadata/value within this model. |
| `infobox.results[].attributes` | `string[][]?` | Attributes collection. |
| `infobox.results[].profiles` | `any?` | Profiles metadata/value within this model. |
| `infobox.results[].profiles[].name` | `string` | Name metadata/value within this model. |
| `infobox.results[].profiles[].url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `infobox.results[].profiles[].long_name` | `string?` | Long name metadata/value within this model. |
| `infobox.results[].profiles[].img` | `string?` | Img metadata/value within this model. |
| `infobox.results[].website_url` | `string?` | Website url metadata/value within this model. |
| `infobox.results[].ratings` | `object[]?` | Ratings collection. |
| `infobox.results[].ratings[].ratingValue` | `number` | Source-specific rating value; compare only with that source and scale. |
| `infobox.results[].ratings[].bestRating` | `number` | Source rating scale metadata; not global relevance. |
| `infobox.results[].ratings[].reviewCount` | `int?` | Number of reviews reported by source. |
| `infobox.results[].ratings[].profile` | `object?` | Profile metadata/value within this model. |
| `infobox.results[].ratings[].profile.name` | `string` | Name metadata/value within this model. |
| `infobox.results[].ratings[].profile.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `infobox.results[].ratings[].profile.long_name` | `string?` | Long name metadata/value within this model. |
| `infobox.results[].ratings[].profile.img` | `string?` | Img metadata/value within this model. |
| `infobox.results[].ratings[].is_tripadvisor` | `bool?` | Attribution flag for the rating source. |
| `infobox.results[].providers` | `object[]?` | Providers collection. |
| `infobox.results[].providers[].type` | `string?` | Source-origin type; external normally identifies an external data source, but is not a closed enum. |
| `infobox.results[].providers[].name` | `string` | Name metadata/value within this model. |
| `infobox.results[].providers[].url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `infobox.results[].providers[].long_name` | `string?` | Long name metadata/value within this model. |
| `infobox.results[].providers[].img` | `string?` | Img metadata/value within this model. |
| `infobox.results[].distance` | `object?` | Distance metadata/value within this model. |
| `infobox.results[].distance.value` | `number` | Value metadata/value within this model. |
| `infobox.results[].distance.units` | `string` | Units metadata/value within this model. |
| `infobox.results[].images` | `object[]?` | Images collection. |
| `infobox.results[].images[].src` | `string` | Src metadata/value within this model. |
| `infobox.results[].images[].alt` | `string?` | Alt metadata/value within this model. |
| `infobox.results[].images[].height` | `int?` | Height metadata/value within this model. |
| `infobox.results[].images[].width` | `int?` | Width metadata/value within this model. |
| `infobox.results[].images[].bg_color` | `string?` | Bg color metadata/value within this model. |
| `infobox.results[].images[].original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `infobox.results[].images[].logo` | `bool?` | Logo metadata/value within this model. |
| `infobox.results[].images[].duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `infobox.results[].images[].theme` | `string?` | Theme metadata/value within this model. |
| `infobox.results[].movie` | `object?` | Movie metadata/value within this model. |
| `infobox.results[].movie.name` | `string?` | Name metadata/value within this model. |
| `infobox.results[].movie.description` | `string?` | Display description/excerpt; POI Descriptions text is generated and Context source description is query-independent. |
| `infobox.results[].movie.url` | `string?` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `infobox.results[].movie.thumbnail` | `object?` | Thumbnail metadata/value within this model. |
| `infobox.results[].movie.thumbnail.src` | `string` | Src metadata/value within this model. |
| `infobox.results[].movie.thumbnail.alt` | `string?` | Alt metadata/value within this model. |
| `infobox.results[].movie.thumbnail.height` | `int?` | Height metadata/value within this model. |
| `infobox.results[].movie.thumbnail.width` | `int?` | Width metadata/value within this model. |
| `infobox.results[].movie.thumbnail.bg_color` | `string?` | Bg color metadata/value within this model. |
| `infobox.results[].movie.thumbnail.original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `infobox.results[].movie.thumbnail.logo` | `bool?` | Logo metadata/value within this model. |
| `infobox.results[].movie.thumbnail.duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `infobox.results[].movie.thumbnail.theme` | `string?` | Theme metadata/value within this model. |
| `infobox.results[].movie.release` | `string?` | Release metadata/value within this model. |
| `infobox.results[].movie.directors` | `object[]?` | Directors collection. |
| `infobox.results[].movie.directors[].type` | `string?` | Documented discriminator `person` identifies this enclosing model; unknown future tags remain accepted. |
| `infobox.results[].movie.directors[].name` | `string` | Name metadata/value within this model. |
| `infobox.results[].movie.directors[].url` | `string?` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `infobox.results[].movie.directors[].thumbnail` | `object?` | Thumbnail metadata/value within this model. |
| `infobox.results[].movie.directors[].thumbnail.src` | `string` | Src metadata/value within this model. |
| `infobox.results[].movie.directors[].thumbnail.alt` | `string?` | Alt metadata/value within this model. |
| `infobox.results[].movie.directors[].thumbnail.height` | `int?` | Height metadata/value within this model. |
| `infobox.results[].movie.directors[].thumbnail.width` | `int?` | Width metadata/value within this model. |
| `infobox.results[].movie.directors[].thumbnail.bg_color` | `string?` | Bg color metadata/value within this model. |
| `infobox.results[].movie.directors[].thumbnail.original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `infobox.results[].movie.directors[].thumbnail.logo` | `bool?` | Logo metadata/value within this model. |
| `infobox.results[].movie.directors[].thumbnail.duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `infobox.results[].movie.directors[].thumbnail.theme` | `string?` | Theme metadata/value within this model. |
| `infobox.results[].movie.directors[].email` | `string?` | Email metadata/value within this model. |
| `infobox.results[].movie.actors` | `object[]?` | Actors collection. |
| `infobox.results[].movie.actors[].type` | `string?` | Documented discriminator `person` identifies this enclosing model; unknown future tags remain accepted. |
| `infobox.results[].movie.actors[].name` | `string` | Name metadata/value within this model. |
| `infobox.results[].movie.actors[].url` | `string?` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `infobox.results[].movie.actors[].thumbnail` | `object?` | Thumbnail metadata/value within this model. |
| `infobox.results[].movie.actors[].thumbnail.src` | `string` | Src metadata/value within this model. |
| `infobox.results[].movie.actors[].thumbnail.alt` | `string?` | Alt metadata/value within this model. |
| `infobox.results[].movie.actors[].thumbnail.height` | `int?` | Height metadata/value within this model. |
| `infobox.results[].movie.actors[].thumbnail.width` | `int?` | Width metadata/value within this model. |
| `infobox.results[].movie.actors[].thumbnail.bg_color` | `string?` | Bg color metadata/value within this model. |
| `infobox.results[].movie.actors[].thumbnail.original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `infobox.results[].movie.actors[].thumbnail.logo` | `bool?` | Logo metadata/value within this model. |
| `infobox.results[].movie.actors[].thumbnail.duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `infobox.results[].movie.actors[].thumbnail.theme` | `string?` | Theme metadata/value within this model. |
| `infobox.results[].movie.actors[].email` | `string?` | Email metadata/value within this model. |
| `infobox.results[].movie.rating` | `object?` | Rating metadata/value within this model. |
| `infobox.results[].movie.rating.ratingValue` | `number` | Source-specific rating value; compare only with that source and scale. |
| `infobox.results[].movie.rating.bestRating` | `number` | Source rating scale metadata; not global relevance. |
| `infobox.results[].movie.rating.reviewCount` | `int?` | Number of reviews reported by source. |
| `infobox.results[].movie.rating.profile` | `object?` | Profile metadata/value within this model. |
| `infobox.results[].movie.rating.profile.name` | `string` | Name metadata/value within this model. |
| `infobox.results[].movie.rating.profile.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `infobox.results[].movie.rating.profile.long_name` | `string?` | Long name metadata/value within this model. |
| `infobox.results[].movie.rating.profile.img` | `string?` | Img metadata/value within this model. |
| `infobox.results[].movie.rating.is_tripadvisor` | `bool?` | Attribution flag for the rating source. |
| `infobox.results[].movie.duration` | `string?` | Duration metadata/value within this model. |
| `infobox.results[].movie.genre` | `string[]?` | Genre collection. |
| `infobox.results[].movie.query` | `string?` | Query metadata object, or corrected/completed phrase on suggestion/spellcheck items. |
| `infobox.results[].subtype` | `string?` | Documented discriminator `generic` identifies this enclosing model; unknown future tags remain accepted. |
| `infobox.results[].found_in_urls` | `string[]?` | Found in urls collection. |
| `locations` | `object?` | Locations metadata/value within this model. |
| `locations.type` | `string?` | Documented discriminator `locations` identifies this enclosing model; unknown future tags remain accepted. |
| `locations.results` | `object[]` | Results collection. |
| `locations.results[].title` | `string` | Title metadata/value within this model. |
| `locations.results[].url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `locations.results[].is_source_local` | `bool?` | Is source local metadata/value within this model. |
| `locations.results[].is_source_both` | `bool?` | Is source both metadata/value within this model. |
| `locations.results[].description` | `string?` | Display description/excerpt; POI Descriptions text is generated and Context source description is query-independent. |
| `locations.results[].page_age` | `string?` | Content publication/modification date, not necessarily crawl time. |
| `locations.results[].page_fetched` | `string?` | Last-fetch date/time as provided by the vendor. |
| `locations.results[].fetched_content_timestamp` | `int?` | Fetch timestamp; do not infer units beyond the source contract. |
| `locations.results[].profile` | `object?` | Profile metadata/value within this model. |
| `locations.results[].profile.name` | `string` | Name metadata/value within this model. |
| `locations.results[].profile.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `locations.results[].profile.long_name` | `string?` | Long name metadata/value within this model. |
| `locations.results[].profile.img` | `string?` | Img metadata/value within this model. |
| `locations.results[].language` | `string?` | Language metadata/value within this model. |
| `locations.results[].family_friendly` | `bool?` | Vendor safety classification, not a content-safety guarantee. |
| `locations.results[].type` | `string?` | Documented discriminator `location_result` identifies this enclosing model; unknown future tags remain accepted. |
| `locations.results[].provider_url` | `string` | Attribution/provider destination URL. |
| `locations.results[].coordinates` | `array?` | Latitude/longitude pair as described by the location model; no geometry guarantee for unrelated arrays. |
| `locations.results[].zoom_level` | `int?` | Zoom level metadata/value within this model. |
| `locations.results[].thumbnail` | `object?` | Thumbnail metadata/value within this model. |
| `locations.results[].thumbnail.src` | `string` | Src metadata/value within this model. |
| `locations.results[].thumbnail.alt` | `string?` | Alt metadata/value within this model. |
| `locations.results[].thumbnail.height` | `int?` | Height metadata/value within this model. |
| `locations.results[].thumbnail.width` | `int?` | Width metadata/value within this model. |
| `locations.results[].thumbnail.bg_color` | `string?` | Bg color metadata/value within this model. |
| `locations.results[].thumbnail.original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `locations.results[].thumbnail.logo` | `bool?` | Logo metadata/value within this model. |
| `locations.results[].thumbnail.duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `locations.results[].thumbnail.theme` | `string?` | Theme metadata/value within this model. |
| `locations.results[].postal_address` | `object?` | Postal address metadata/value within this model. |
| `locations.results[].postal_address.type` | `string?` | Documented discriminator `PostalAddress` identifies this enclosing model; unknown future tags remain accepted. |
| `locations.results[].postal_address.country` | `string?` | Country metadata/value within this model. |
| `locations.results[].postal_address.postalCode` | `string?` | Postal code metadata/value within this model. |
| `locations.results[].postal_address.streetAddress` | `string?` | Street address metadata/value within this model. |
| `locations.results[].postal_address.addressRegion` | `string?` | Address region metadata/value within this model. |
| `locations.results[].postal_address.addressLocality` | `string?` | Address locality metadata/value within this model. |
| `locations.results[].postal_address.displayAddress` | `string` | Display address metadata/value within this model. |
| `locations.results[].opening_hours` | `object?` | Business hours; days is a nested array and a day may contain multiple intervals. |
| `locations.results[].opening_hours.current_day` | `object[]?` | Current day collection. |
| `locations.results[].opening_hours.current_day[].abbr_name` | `string` | Abbr name metadata/value within this model. |
| `locations.results[].opening_hours.current_day[].full_name` | `string` | Full name metadata/value within this model. |
| `locations.results[].opening_hours.current_day[].opens` | `string` | Local 24-hour opening-time string. |
| `locations.results[].opening_hours.current_day[].closes` | `string` | Local 24-hour closing-time string. |
| `locations.results[].opening_hours.days` | `object[][]?` | Days collection. |
| `locations.results[].contact` | `object?` | Contact metadata/value within this model. |
| `locations.results[].contact.email` | `string?` | Email metadata/value within this model. |
| `locations.results[].contact.telephone` | `string?` | Telephone metadata/value within this model. |
| `locations.results[].price_range` | `string?` | Price range metadata/value within this model. |
| `locations.results[].rating` | `object?` | Rating metadata/value within this model. |
| `locations.results[].rating.ratingValue` | `number` | Source-specific rating value; compare only with that source and scale. |
| `locations.results[].rating.bestRating` | `number` | Source rating scale metadata; not global relevance. |
| `locations.results[].rating.reviewCount` | `int?` | Number of reviews reported by source. |
| `locations.results[].rating.profile` | `object?` | Profile metadata/value within this model. |
| `locations.results[].rating.profile.name` | `string` | Name metadata/value within this model. |
| `locations.results[].rating.profile.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `locations.results[].rating.profile.long_name` | `string?` | Long name metadata/value within this model. |
| `locations.results[].rating.profile.img` | `string?` | Img metadata/value within this model. |
| `locations.results[].rating.is_tripadvisor` | `bool?` | Attribution flag for the rating source. |
| `locations.results[].distance` | `object?` | Distance metadata/value within this model. |
| `locations.results[].distance.value` | `number` | Value metadata/value within this model. |
| `locations.results[].distance.units` | `string` | Units metadata/value within this model. |
| `locations.results[].profiles` | `object[]?` | Profiles collection. |
| `locations.results[].profiles[].type` | `string?` | Source-origin type; external normally identifies an external data source, but is not a closed enum. |
| `locations.results[].profiles[].name` | `string` | Name metadata/value within this model. |
| `locations.results[].profiles[].url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `locations.results[].profiles[].long_name` | `string?` | Long name metadata/value within this model. |
| `locations.results[].profiles[].img` | `string?` | Img metadata/value within this model. |
| `locations.results[].reviews` | `object?` | Reviews metadata/value within this model. |
| `locations.results[].reviews.results` | `object[]` | Results collection. |
| `locations.results[].reviews.results[].title` | `string` | Title metadata/value within this model. |
| `locations.results[].reviews.results[].description` | `string` | Display description/excerpt; POI Descriptions text is generated and Context source description is query-independent. |
| `locations.results[].reviews.results[].date` | `string` | Date metadata/value within this model. |
| `locations.results[].reviews.results[].rating` | `object` | Rating metadata/value within this model. |
| `locations.results[].reviews.results[].rating.ratingValue` | `number` | Source-specific rating value; compare only with that source and scale. |
| `locations.results[].reviews.results[].rating.bestRating` | `number` | Source rating scale metadata; not global relevance. |
| `locations.results[].reviews.results[].rating.reviewCount` | `int?` | Number of reviews reported by source. |
| `locations.results[].reviews.results[].rating.profile` | `object?` | Profile metadata/value within this model. |
| `locations.results[].reviews.results[].rating.profile.name` | `string` | Name metadata/value within this model. |
| `locations.results[].reviews.results[].rating.profile.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `locations.results[].reviews.results[].rating.profile.long_name` | `string?` | Long name metadata/value within this model. |
| `locations.results[].reviews.results[].rating.profile.img` | `string?` | Img metadata/value within this model. |
| `locations.results[].reviews.results[].rating.is_tripadvisor` | `bool?` | Attribution flag for the rating source. |
| `locations.results[].reviews.results[].author` | `object` | Author metadata/value within this model. |
| `locations.results[].reviews.results[].author.type` | `string?` | Documented discriminator `person` identifies this enclosing model; unknown future tags remain accepted. |
| `locations.results[].reviews.results[].author.name` | `string` | Name metadata/value within this model. |
| `locations.results[].reviews.results[].author.url` | `string?` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `locations.results[].reviews.results[].author.thumbnail` | `object?` | Thumbnail metadata/value within this model. |
| `locations.results[].reviews.results[].author.thumbnail.src` | `string` | Src metadata/value within this model. |
| `locations.results[].reviews.results[].author.thumbnail.alt` | `string?` | Alt metadata/value within this model. |
| `locations.results[].reviews.results[].author.thumbnail.height` | `int?` | Height metadata/value within this model. |
| `locations.results[].reviews.results[].author.thumbnail.width` | `int?` | Width metadata/value within this model. |
| `locations.results[].reviews.results[].author.thumbnail.bg_color` | `string?` | Bg color metadata/value within this model. |
| `locations.results[].reviews.results[].author.thumbnail.original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `locations.results[].reviews.results[].author.thumbnail.logo` | `bool?` | Logo metadata/value within this model. |
| `locations.results[].reviews.results[].author.thumbnail.duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `locations.results[].reviews.results[].author.thumbnail.theme` | `string?` | Theme metadata/value within this model. |
| `locations.results[].reviews.results[].author.email` | `string?` | Email metadata/value within this model. |
| `locations.results[].reviews.results[].review_url` | `string` | Review url metadata/value within this model. |
| `locations.results[].reviews.results[].language` | `string` | Language metadata/value within this model. |
| `locations.results[].reviews.viewMoreUrl` | `string` | View more url metadata/value within this model. |
| `locations.results[].reviews.reviews_in_foreign_language` | `bool` | Reviews in foreign language metadata/value within this model. |
| `locations.results[].pictures` | `object?` | Pictures metadata/value within this model. |
| `locations.results[].pictures.viewMoreUrl` | `string?` | View more url metadata/value within this model. |
| `locations.results[].pictures.results` | `object[]` | Results collection. |
| `locations.results[].pictures.results[].src` | `string` | Src metadata/value within this model. |
| `locations.results[].pictures.results[].alt` | `string?` | Alt metadata/value within this model. |
| `locations.results[].pictures.results[].height` | `int?` | Height metadata/value within this model. |
| `locations.results[].pictures.results[].width` | `int?` | Width metadata/value within this model. |
| `locations.results[].pictures.results[].bg_color` | `string?` | Bg color metadata/value within this model. |
| `locations.results[].pictures.results[].original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `locations.results[].pictures.results[].logo` | `bool?` | Logo metadata/value within this model. |
| `locations.results[].pictures.results[].duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `locations.results[].pictures.results[].theme` | `string?` | Theme metadata/value within this model. |
| `locations.results[].action` | `object?` | Action metadata/value within this model. |
| `locations.results[].action.type` | `string` | Type metadata/value within this model. |
| `locations.results[].action.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `locations.results[].serves_cuisine` | `string[]?` | Serves cuisine collection. |
| `locations.results[].categories` | `string[]?` | Categories collection. |
| `locations.results[].icon_category` | `string?` | Icon category metadata/value within this model. |
| `locations.results[].timezone` | `string?` | Timezone metadata/value within this model. |
| `locations.results[].timezone_offset` | `int?` | Timezone offset metadata/value within this model. |
| `locations.results[].id` | `string?` | Identifier scoped to the enclosing result; local-place IDs expire in about eight hours. |
| `locations.results[].results` | `object[]?` | Results collection. |
| `locations.results[].results[].title` | `string` | Title metadata/value within this model. |
| `locations.results[].results[].url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `locations.results[].results[].is_source_local` | `bool?` | Is source local metadata/value within this model. |
| `locations.results[].results[].is_source_both` | `bool?` | Is source both metadata/value within this model. |
| `locations.results[].results[].description` | `string?` | Display description/excerpt; POI Descriptions text is generated and Context source description is query-independent. |
| `locations.results[].results[].page_age` | `string?` | Content publication/modification date, not necessarily crawl time. |
| `locations.results[].results[].page_fetched` | `string?` | Last-fetch date/time as provided by the vendor. |
| `locations.results[].results[].fetched_content_timestamp` | `int?` | Fetch timestamp; do not infer units beyond the source contract. |
| `locations.results[].results[].profile` | `object?` | Profile metadata/value within this model. |
| `locations.results[].results[].profile.name` | `string` | Name metadata/value within this model. |
| `locations.results[].results[].profile.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `locations.results[].results[].profile.long_name` | `string?` | Long name metadata/value within this model. |
| `locations.results[].results[].profile.img` | `string?` | Img metadata/value within this model. |
| `locations.results[].results[].language` | `string?` | Language metadata/value within this model. |
| `locations.results[].results[].family_friendly` | `bool?` | Vendor safety classification, not a content-safety guarantee. |
| `locations.results[].results[].meta_url` | `object` | Meta url metadata/value within this model. |
| `locations.results[].results[].meta_url.scheme` | `string` | Scheme metadata/value within this model. |
| `locations.results[].results[].meta_url.netloc` | `string` | Netloc metadata/value within this model. |
| `locations.results[].results[].meta_url.hostname` | `string?` | Hostname metadata/value within this model. |
| `locations.results[].results[].meta_url.favicon` | `string` | Favicon metadata/value within this model. |
| `locations.results[].results[].meta_url.path` | `string` | Path metadata/value within this model. |
| `locations.provider` | `object?` | Third-party data provenance; retain attribution in projections. |
| `mixed` | `object?` | Ordering references into vertical arrays; preserve type/index association when projecting. |
| `mixed.type` | `string?` | Documented discriminator `mixed` identifies this enclosing model; unknown future tags remain accepted. |
| `mixed.main` | `object[]?` | Main collection. |
| `mixed.main[].type` | `string` | Type metadata/value within this model. |
| `mixed.main[].index` | `int?` | Zero-based placement/index associated with the mixed-result type; preserve alongside type and all. |
| `mixed.main[].all` | `bool?` | All metadata/value within this model. |
| `mixed.top` | `object[]?` | Top collection. |
| `mixed.top[].type` | `string` | Type metadata/value within this model. |
| `mixed.top[].index` | `int?` | Zero-based placement/index associated with the mixed-result type; preserve alongside type and all. |
| `mixed.top[].all` | `bool?` | All metadata/value within this model. |
| `mixed.side` | `object[]?` | Side collection. |
| `mixed.side[].type` | `string` | Type metadata/value within this model. |
| `mixed.side[].index` | `int?` | Zero-based placement/index associated with the mixed-result type; preserve alongside type and all. |
| `mixed.side[].all` | `bool?` | All metadata/value within this model. |
| `news` | `object?` | News metadata/value within this model. |
| `news.type` | `string?` | Documented discriminator `news` identifies this enclosing model; unknown future tags remain accepted. |
| `news.results` | `object[]` | Results collection. |
| `news.results[].title` | `string` | Title metadata/value within this model. |
| `news.results[].url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `news.results[].is_source_local` | `bool?` | Is source local metadata/value within this model. |
| `news.results[].is_source_both` | `bool?` | Is source both metadata/value within this model. |
| `news.results[].description` | `string?` | Display description/excerpt; POI Descriptions text is generated and Context source description is query-independent. |
| `news.results[].page_age` | `string?` | Content publication/modification date, not necessarily crawl time. |
| `news.results[].page_fetched` | `string?` | Last-fetch date/time as provided by the vendor. |
| `news.results[].fetched_content_timestamp` | `int?` | Fetch timestamp; do not infer units beyond the source contract. |
| `news.results[].profile` | `object?` | Profile metadata/value within this model. |
| `news.results[].profile.name` | `string` | Name metadata/value within this model. |
| `news.results[].profile.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `news.results[].profile.long_name` | `string?` | Long name metadata/value within this model. |
| `news.results[].profile.img` | `string?` | Img metadata/value within this model. |
| `news.results[].language` | `string?` | Language metadata/value within this model. |
| `news.results[].family_friendly` | `bool?` | Vendor safety classification, not a content-safety guarantee. |
| `news.results[].meta_url` | `object?` | Meta url metadata/value within this model. |
| `news.results[].source` | `string?` | Source metadata/value within this model. |
| `news.results[].breaking` | `bool?` | Breaking metadata/value within this model. |
| `news.results[].is_live` | `bool?` | Is live metadata/value within this model. |
| `news.results[].thumbnail` | `object?` | Thumbnail metadata/value within this model. |
| `news.results[].thumbnail.src` | `string` | Src metadata/value within this model. |
| `news.results[].thumbnail.alt` | `string?` | Alt metadata/value within this model. |
| `news.results[].thumbnail.height` | `int?` | Height metadata/value within this model. |
| `news.results[].thumbnail.width` | `int?` | Width metadata/value within this model. |
| `news.results[].thumbnail.bg_color` | `string?` | Bg color metadata/value within this model. |
| `news.results[].thumbnail.original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `news.results[].thumbnail.logo` | `bool?` | Logo metadata/value within this model. |
| `news.results[].thumbnail.duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `news.results[].thumbnail.theme` | `string?` | Theme metadata/value within this model. |
| `news.results[].age` | `string?` | Human-readable age; Context sources use a separate four-position date array. |
| `news.results[].extra_snippets` | `string[]?` | Alternative page excerpts, not continuous full text. |
| `news.results[].icons` | `object[]?` | Icons collection. |
| `news.results[].icons[].href` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `news.results[].icons[].sizes` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `news.results[].icons[].rel` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `news.results[].icons[].type` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `news.results[].icons[].ext` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `news.mutated_by_goggles` | `bool?` | Whether Goggles modified these discussion results; documented default false. |
| `videos` | `object?` | Videos metadata/value within this model. |
| `videos.type` | `string?` | Documented discriminator `videos` identifies this enclosing model; unknown future tags remain accepted. |
| `videos.results` | `object[]` | Results collection. |
| `videos.results[].type` | `string?` | Documented discriminator `video_result` identifies this enclosing model; unknown future tags remain accepted. |
| `videos.results[].url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `videos.results[].title` | `string` | Title metadata/value within this model. |
| `videos.results[].description` | `string?` | Display description/excerpt; POI Descriptions text is generated and Context source description is query-independent. |
| `videos.results[].age` | `string?` | Human-readable age; Context sources use a separate four-position date array. |
| `videos.results[].page_age` | `string?` | Content publication/modification date, not necessarily crawl time. |
| `videos.results[].page_fetched` | `string?` | Last-fetch date/time as provided by the vendor. |
| `videos.results[].fetched_content_timestamp` | `int?` | Fetch timestamp; do not infer units beyond the source contract. |
| `videos.results[].video` | `object?` | Video metadata/value within this model. |
| `videos.results[].video.duration` | `string?` | Duration metadata/value within this model. |
| `videos.results[].video.views` | `int?` | Views metadata/value within this model. |
| `videos.results[].video.creator` | `string?` | Creator metadata/value within this model. |
| `videos.results[].video.publisher` | `string?` | Publisher metadata/value within this model. |
| `videos.results[].video.requires_subscription` | `bool?` | Source-content access restriction, independent of Brave API billing. |
| `videos.results[].video.tags` | `string[]?` | Tags collection. |
| `videos.results[].video.author` | `object?` | Author metadata/value within this model. |
| `videos.results[].video.author.name` | `string` | Name metadata/value within this model. |
| `videos.results[].video.author.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `videos.results[].video.author.long_name` | `string?` | Long name metadata/value within this model. |
| `videos.results[].video.author.img` | `string?` | Img metadata/value within this model. |
| `videos.results[].meta_url` | `object?` | Meta url metadata/value within this model. |
| `videos.results[].meta_url.scheme` | `string?` | Scheme metadata/value within this model. |
| `videos.results[].meta_url.netloc` | `string?` | Netloc metadata/value within this model. |
| `videos.results[].meta_url.hostname` | `string?` | Hostname metadata/value within this model. |
| `videos.results[].meta_url.favicon` | `string?` | Favicon metadata/value within this model. |
| `videos.results[].meta_url.path` | `string?` | Path metadata/value within this model. |
| `videos.results[].thumbnail` | `object?` | Thumbnail metadata/value within this model. |
| `videos.results[].thumbnail.src` | `string` | Src metadata/value within this model. |
| `videos.results[].thumbnail.original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `videos.mutated_by_goggles` | `bool?` | Whether Goggles modified these discussion results; documented default false. |
| `web` | `object?` | Web metadata/value within this model. |
| `web.type` | `string?` | Documented discriminator `search` identifies this enclosing model; unknown future tags remain accepted. |
| `web.results` | `object[]` | Results collection. |
| `web.results[].title` | `string` | Title metadata/value within this model. |
| `web.results[].url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `web.results[].is_source_local` | `bool?` | Is source local metadata/value within this model. |
| `web.results[].is_source_both` | `bool?` | Is source both metadata/value within this model. |
| `web.results[].description` | `string?` | Display description/excerpt; POI Descriptions text is generated and Context source description is query-independent. |
| `web.results[].page_age` | `string?` | Content publication/modification date, not necessarily crawl time. |
| `web.results[].page_fetched` | `string?` | Last-fetch date/time as provided by the vendor. |
| `web.results[].fetched_content_timestamp` | `int?` | Fetch timestamp; do not infer units beyond the source contract. |
| `web.results[].profile` | `object?` | Profile metadata/value within this model. |
| `web.results[].profile.name` | `string` | Name metadata/value within this model. |
| `web.results[].profile.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `web.results[].profile.long_name` | `string?` | Long name metadata/value within this model. |
| `web.results[].profile.img` | `string?` | Img metadata/value within this model. |
| `web.results[].language` | `string?` | Language metadata/value within this model. |
| `web.results[].family_friendly` | `bool?` | Vendor safety classification, not a content-safety guarantee. |
| `web.results[].type` | `string?` | Documented discriminator `search_result` identifies this enclosing model; unknown future tags remain accepted. |
| `web.results[].subtype` | `string?` | Subtype metadata/value within this model. |
| `web.results[].is_live` | `bool?` | Live/current-result indicator; vendor documents false as the default. |
| `web.results[].deep_results` | `object?` | Deep results metadata/value within this model. |
| `web.results[].deep_results.news` | `object[]?` | News collection. |
| `web.results[].deep_results.news[].title` | `string` | Title metadata/value within this model. |
| `web.results[].deep_results.news[].url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `web.results[].deep_results.news[].is_source_local` | `bool?` | Is source local metadata/value within this model. |
| `web.results[].deep_results.news[].is_source_both` | `bool?` | Is source both metadata/value within this model. |
| `web.results[].deep_results.news[].description` | `string?` | Display description/excerpt; POI Descriptions text is generated and Context source description is query-independent. |
| `web.results[].deep_results.news[].page_age` | `string?` | Content publication/modification date, not necessarily crawl time. |
| `web.results[].deep_results.news[].page_fetched` | `string?` | Last-fetch date/time as provided by the vendor. |
| `web.results[].deep_results.news[].fetched_content_timestamp` | `int?` | Fetch timestamp; do not infer units beyond the source contract. |
| `web.results[].deep_results.news[].profile` | `object?` | Profile metadata/value within this model. |
| `web.results[].deep_results.news[].profile.name` | `string` | Name metadata/value within this model. |
| `web.results[].deep_results.news[].profile.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `web.results[].deep_results.news[].profile.long_name` | `string?` | Long name metadata/value within this model. |
| `web.results[].deep_results.news[].profile.img` | `string?` | Img metadata/value within this model. |
| `web.results[].deep_results.news[].language` | `string?` | Language metadata/value within this model. |
| `web.results[].deep_results.news[].family_friendly` | `bool?` | Vendor safety classification, not a content-safety guarantee. |
| `web.results[].deep_results.news[].meta_url` | `object?` | Meta url metadata/value within this model. |
| `web.results[].deep_results.news[].source` | `string?` | Source metadata/value within this model. |
| `web.results[].deep_results.news[].breaking` | `bool?` | Breaking metadata/value within this model. |
| `web.results[].deep_results.news[].is_live` | `bool?` | Is live metadata/value within this model. |
| `web.results[].deep_results.news[].thumbnail` | `object?` | Thumbnail metadata/value within this model. |
| `web.results[].deep_results.news[].thumbnail.src` | `string` | Src metadata/value within this model. |
| `web.results[].deep_results.news[].thumbnail.alt` | `string?` | Alt metadata/value within this model. |
| `web.results[].deep_results.news[].thumbnail.height` | `int?` | Height metadata/value within this model. |
| `web.results[].deep_results.news[].thumbnail.width` | `int?` | Width metadata/value within this model. |
| `web.results[].deep_results.news[].thumbnail.bg_color` | `string?` | Bg color metadata/value within this model. |
| `web.results[].deep_results.news[].thumbnail.original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `web.results[].deep_results.news[].thumbnail.logo` | `bool?` | Logo metadata/value within this model. |
| `web.results[].deep_results.news[].thumbnail.duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `web.results[].deep_results.news[].thumbnail.theme` | `string?` | Theme metadata/value within this model. |
| `web.results[].deep_results.news[].age` | `string?` | Human-readable age; Context sources use a separate four-position date array. |
| `web.results[].deep_results.news[].extra_snippets` | `string[]?` | Alternative page excerpts, not continuous full text. |
| `web.results[].deep_results.news[].icons` | `object[]?` | Icons collection. |
| `web.results[].deep_results.news[].icons[].href` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `web.results[].deep_results.news[].icons[].sizes` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `web.results[].deep_results.news[].icons[].rel` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `web.results[].deep_results.news[].icons[].type` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `web.results[].deep_results.news[].icons[].ext` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `web.results[].deep_results.buttons` | `object[]?` | Buttons collection. |
| `web.results[].deep_results.buttons[].type` | `string?` | Documented discriminator `button_result` identifies this enclosing model; unknown future tags remain accepted. |
| `web.results[].deep_results.buttons[].title` | `string` | Title metadata/value within this model. |
| `web.results[].deep_results.buttons[].url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `web.results[].deep_results.videos` | `object[]?` | Videos collection. |
| `web.results[].deep_results.videos[].title` | `string` | Title metadata/value within this model. |
| `web.results[].deep_results.videos[].url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `web.results[].deep_results.videos[].is_source_local` | `bool?` | Is source local metadata/value within this model. |
| `web.results[].deep_results.videos[].is_source_both` | `bool?` | Is source both metadata/value within this model. |
| `web.results[].deep_results.videos[].description` | `string?` | Display description/excerpt; POI Descriptions text is generated and Context source description is query-independent. |
| `web.results[].deep_results.videos[].page_age` | `string?` | Content publication/modification date, not necessarily crawl time. |
| `web.results[].deep_results.videos[].page_fetched` | `string?` | Last-fetch date/time as provided by the vendor. |
| `web.results[].deep_results.videos[].fetched_content_timestamp` | `int?` | Fetch timestamp; do not infer units beyond the source contract. |
| `web.results[].deep_results.videos[].profile` | `object?` | Profile metadata/value within this model. |
| `web.results[].deep_results.videos[].profile.name` | `string` | Name metadata/value within this model. |
| `web.results[].deep_results.videos[].profile.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `web.results[].deep_results.videos[].profile.long_name` | `string?` | Long name metadata/value within this model. |
| `web.results[].deep_results.videos[].profile.img` | `string?` | Img metadata/value within this model. |
| `web.results[].deep_results.videos[].language` | `string?` | Language metadata/value within this model. |
| `web.results[].deep_results.videos[].family_friendly` | `bool?` | Vendor safety classification, not a content-safety guarantee. |
| `web.results[].deep_results.videos[].type` | `string?` | Documented discriminator `video_result` identifies this enclosing model; unknown future tags remain accepted. |
| `web.results[].deep_results.videos[].video` | `object` | Video metadata/value within this model. |
| `web.results[].deep_results.videos[].video.duration` | `string?` | Duration metadata/value within this model. |
| `web.results[].deep_results.videos[].video.views` | `string?` | Views metadata/value within this model. |
| `web.results[].deep_results.videos[].video.creator` | `string?` | Creator metadata/value within this model. |
| `web.results[].deep_results.videos[].video.publisher` | `string?` | Publisher metadata/value within this model. |
| `web.results[].deep_results.videos[].video.thumbnail` | `object?` | Thumbnail metadata/value within this model. |
| `web.results[].deep_results.videos[].video.thumbnail.src` | `string` | Src metadata/value within this model. |
| `web.results[].deep_results.videos[].video.thumbnail.alt` | `string?` | Alt metadata/value within this model. |
| `web.results[].deep_results.videos[].video.thumbnail.height` | `int?` | Height metadata/value within this model. |
| `web.results[].deep_results.videos[].video.thumbnail.width` | `int?` | Width metadata/value within this model. |
| `web.results[].deep_results.videos[].video.thumbnail.bg_color` | `string?` | Bg color metadata/value within this model. |
| `web.results[].deep_results.videos[].video.thumbnail.original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `web.results[].deep_results.videos[].video.thumbnail.logo` | `bool?` | Logo metadata/value within this model. |
| `web.results[].deep_results.videos[].video.thumbnail.duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `web.results[].deep_results.videos[].video.thumbnail.theme` | `string?` | Theme metadata/value within this model. |
| `web.results[].deep_results.videos[].video.tags` | `string[]?` | Tags collection. |
| `web.results[].deep_results.videos[].video.author` | `object?` | Author metadata/value within this model. |
| `web.results[].deep_results.videos[].video.author.name` | `string` | Name metadata/value within this model. |
| `web.results[].deep_results.videos[].video.author.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `web.results[].deep_results.videos[].video.author.long_name` | `string?` | Long name metadata/value within this model. |
| `web.results[].deep_results.videos[].video.author.img` | `string?` | Img metadata/value within this model. |
| `web.results[].deep_results.videos[].video.requires_subscription` | `bool?` | Source-content access restriction, independent of Brave API billing. |
| `web.results[].deep_results.videos[].meta_url` | `object?` | Meta url metadata/value within this model. |
| `web.results[].deep_results.videos[].meta_url.scheme` | `string` | Scheme metadata/value within this model. |
| `web.results[].deep_results.videos[].meta_url.netloc` | `string` | Netloc metadata/value within this model. |
| `web.results[].deep_results.videos[].meta_url.hostname` | `string?` | Hostname metadata/value within this model. |
| `web.results[].deep_results.videos[].meta_url.favicon` | `string` | Favicon metadata/value within this model. |
| `web.results[].deep_results.videos[].meta_url.path` | `string` | Path metadata/value within this model. |
| `web.results[].deep_results.videos[].thumbnail` | `object?` | Thumbnail metadata/value within this model. |
| `web.results[].deep_results.videos[].thumbnail.src` | `string` | Src metadata/value within this model. |
| `web.results[].deep_results.videos[].thumbnail.alt` | `string?` | Alt metadata/value within this model. |
| `web.results[].deep_results.videos[].thumbnail.height` | `int?` | Height metadata/value within this model. |
| `web.results[].deep_results.videos[].thumbnail.width` | `int?` | Width metadata/value within this model. |
| `web.results[].deep_results.videos[].thumbnail.bg_color` | `string?` | Bg color metadata/value within this model. |
| `web.results[].deep_results.videos[].thumbnail.original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `web.results[].deep_results.videos[].thumbnail.logo` | `bool?` | Logo metadata/value within this model. |
| `web.results[].deep_results.videos[].thumbnail.duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `web.results[].deep_results.videos[].thumbnail.theme` | `string?` | Theme metadata/value within this model. |
| `web.results[].deep_results.videos[].age` | `string?` | Human-readable age; Context sources use a separate four-position date array. |
| `web.results[].deep_results.videos[].publisher` | `string?` | Publisher metadata/value within this model. |
| `web.results[].deep_results.images` | `object[]?` | Images collection. |
| `web.results[].deep_results.images[].thumbnail` | `object` | Thumbnail metadata/value within this model. |
| `web.results[].deep_results.images[].thumbnail.src` | `string` | Src metadata/value within this model. |
| `web.results[].deep_results.images[].thumbnail.alt` | `string?` | Alt metadata/value within this model. |
| `web.results[].deep_results.images[].thumbnail.height` | `int?` | Height metadata/value within this model. |
| `web.results[].deep_results.images[].thumbnail.width` | `int?` | Width metadata/value within this model. |
| `web.results[].deep_results.images[].thumbnail.bg_color` | `string?` | Bg color metadata/value within this model. |
| `web.results[].deep_results.images[].thumbnail.original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `web.results[].deep_results.images[].thumbnail.logo` | `bool?` | Logo metadata/value within this model. |
| `web.results[].deep_results.images[].thumbnail.duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `web.results[].deep_results.images[].thumbnail.theme` | `string?` | Theme metadata/value within this model. |
| `web.results[].deep_results.images[].url` | `string?` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `web.results[].deep_results.images[].properties` | `object?` | Properties metadata/value within this model. |
| `web.results[].deep_results.images[].properties.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `web.results[].deep_results.images[].properties.resized` | `string` | Resized metadata/value within this model. |
| `web.results[].deep_results.images[].properties.placeholder` | `string` | Placeholder metadata/value within this model. |
| `web.results[].deep_results.images[].properties.height` | `int?` | Height metadata/value within this model. |
| `web.results[].deep_results.images[].properties.width` | `int?` | Width metadata/value within this model. |
| `web.results[].deep_results.images[].properties.format` | `string?` | Format metadata/value within this model. |
| `web.results[].deep_results.images[].properties.content_size` | `string?` | Content size metadata/value within this model. |
| `web.results[].schemas` | `any[]?` | Extracted structured data resembling schema.org; heterogeneous and not guaranteed to conform. |
| `web.results[].meta_url` | `object?` | Meta url metadata/value within this model. |
| `web.results[].thumbnail` | `object?` | Thumbnail metadata/value within this model. |
| `web.results[].thumbnail.src` | `string` | Src metadata/value within this model. |
| `web.results[].thumbnail.alt` | `string?` | Alt metadata/value within this model. |
| `web.results[].thumbnail.height` | `int?` | Height metadata/value within this model. |
| `web.results[].thumbnail.width` | `int?` | Width metadata/value within this model. |
| `web.results[].thumbnail.bg_color` | `string?` | Bg color metadata/value within this model. |
| `web.results[].thumbnail.original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `web.results[].thumbnail.logo` | `bool?` | Logo metadata/value within this model. |
| `web.results[].thumbnail.duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `web.results[].thumbnail.theme` | `string?` | Theme metadata/value within this model. |
| `web.results[].age` | `string?` | Human-readable age; Context sources use a separate four-position date array. |
| `web.results[].location` | `object?` | Location metadata/value within this model. |
| `web.results[].location.title` | `string` | Title metadata/value within this model. |
| `web.results[].location.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `web.results[].location.is_source_local` | `bool?` | Is source local metadata/value within this model. |
| `web.results[].location.is_source_both` | `bool?` | Is source both metadata/value within this model. |
| `web.results[].location.description` | `string?` | Display description/excerpt; POI Descriptions text is generated and Context source description is query-independent. |
| `web.results[].location.page_age` | `string?` | Content publication/modification date, not necessarily crawl time. |
| `web.results[].location.page_fetched` | `string?` | Last-fetch date/time as provided by the vendor. |
| `web.results[].location.fetched_content_timestamp` | `int?` | Fetch timestamp; do not infer units beyond the source contract. |
| `web.results[].location.profile` | `object?` | Profile metadata/value within this model. |
| `web.results[].location.profile.name` | `string` | Name metadata/value within this model. |
| `web.results[].location.profile.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `web.results[].location.profile.long_name` | `string?` | Long name metadata/value within this model. |
| `web.results[].location.profile.img` | `string?` | Img metadata/value within this model. |
| `web.results[].location.language` | `string?` | Language metadata/value within this model. |
| `web.results[].location.family_friendly` | `bool?` | Vendor safety classification, not a content-safety guarantee. |
| `web.results[].location.type` | `string?` | Documented discriminator `location_result` identifies this enclosing model; unknown future tags remain accepted. |
| `web.results[].location.provider_url` | `string` | Attribution/provider destination URL. |
| `web.results[].location.coordinates` | `array?` | Latitude/longitude pair as described by the location model; no geometry guarantee for unrelated arrays. |
| `web.results[].location.zoom_level` | `int?` | Zoom level metadata/value within this model. |
| `web.results[].location.thumbnail` | `object?` | Thumbnail metadata/value within this model. |
| `web.results[].location.thumbnail.src` | `string` | Src metadata/value within this model. |
| `web.results[].location.thumbnail.alt` | `string?` | Alt metadata/value within this model. |
| `web.results[].location.thumbnail.height` | `int?` | Height metadata/value within this model. |
| `web.results[].location.thumbnail.width` | `int?` | Width metadata/value within this model. |
| `web.results[].location.thumbnail.bg_color` | `string?` | Bg color metadata/value within this model. |
| `web.results[].location.thumbnail.original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `web.results[].location.thumbnail.logo` | `bool?` | Logo metadata/value within this model. |
| `web.results[].location.thumbnail.duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `web.results[].location.thumbnail.theme` | `string?` | Theme metadata/value within this model. |
| `web.results[].location.postal_address` | `object?` | Postal address metadata/value within this model. |
| `web.results[].location.postal_address.type` | `string?` | Documented discriminator `PostalAddress` identifies this enclosing model; unknown future tags remain accepted. |
| `web.results[].location.postal_address.country` | `string?` | Country metadata/value within this model. |
| `web.results[].location.postal_address.postalCode` | `string?` | Postal code metadata/value within this model. |
| `web.results[].location.postal_address.streetAddress` | `string?` | Street address metadata/value within this model. |
| `web.results[].location.postal_address.addressRegion` | `string?` | Address region metadata/value within this model. |
| `web.results[].location.postal_address.addressLocality` | `string?` | Address locality metadata/value within this model. |
| `web.results[].location.postal_address.displayAddress` | `string` | Display address metadata/value within this model. |
| `web.results[].location.opening_hours` | `object?` | Business hours; days is a nested array and a day may contain multiple intervals. |
| `web.results[].location.opening_hours.current_day` | `object[]?` | Current day collection. |
| `web.results[].location.opening_hours.current_day[].abbr_name` | `string` | Abbr name metadata/value within this model. |
| `web.results[].location.opening_hours.current_day[].full_name` | `string` | Full name metadata/value within this model. |
| `web.results[].location.opening_hours.current_day[].opens` | `string` | Local 24-hour opening-time string. |
| `web.results[].location.opening_hours.current_day[].closes` | `string` | Local 24-hour closing-time string. |
| `web.results[].location.opening_hours.days` | `object[][]?` | Days collection. |
| `web.results[].location.contact` | `object?` | Contact metadata/value within this model. |
| `web.results[].location.contact.email` | `string?` | Email metadata/value within this model. |
| `web.results[].location.contact.telephone` | `string?` | Telephone metadata/value within this model. |
| `web.results[].location.price_range` | `string?` | Price range metadata/value within this model. |
| `web.results[].location.rating` | `object?` | Rating metadata/value within this model. |
| `web.results[].location.rating.ratingValue` | `number` | Source-specific rating value; compare only with that source and scale. |
| `web.results[].location.rating.bestRating` | `number` | Source rating scale metadata; not global relevance. |
| `web.results[].location.rating.reviewCount` | `int?` | Number of reviews reported by source. |
| `web.results[].location.rating.profile` | `object?` | Profile metadata/value within this model. |
| `web.results[].location.rating.profile.name` | `string` | Name metadata/value within this model. |
| `web.results[].location.rating.profile.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `web.results[].location.rating.profile.long_name` | `string?` | Long name metadata/value within this model. |
| `web.results[].location.rating.profile.img` | `string?` | Img metadata/value within this model. |
| `web.results[].location.rating.is_tripadvisor` | `bool?` | Attribution flag for the rating source. |
| `web.results[].location.distance` | `object?` | Distance metadata/value within this model. |
| `web.results[].location.distance.value` | `number` | Value metadata/value within this model. |
| `web.results[].location.distance.units` | `string` | Units metadata/value within this model. |
| `web.results[].location.profiles` | `object[]?` | Profiles collection. |
| `web.results[].location.profiles[].type` | `string?` | Source-origin type; external normally identifies an external data source, but is not a closed enum. |
| `web.results[].location.profiles[].name` | `string` | Name metadata/value within this model. |
| `web.results[].location.profiles[].url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `web.results[].location.profiles[].long_name` | `string?` | Long name metadata/value within this model. |
| `web.results[].location.profiles[].img` | `string?` | Img metadata/value within this model. |
| `web.results[].location.reviews` | `object?` | Reviews metadata/value within this model. |
| `web.results[].location.reviews.results` | `object[]` | Results collection. |
| `web.results[].location.reviews.results[].title` | `string` | Title metadata/value within this model. |
| `web.results[].location.reviews.results[].description` | `string` | Display description/excerpt; POI Descriptions text is generated and Context source description is query-independent. |
| `web.results[].location.reviews.results[].date` | `string` | Date metadata/value within this model. |
| `web.results[].location.reviews.results[].rating` | `object` | Rating metadata/value within this model. |
| `web.results[].location.reviews.results[].rating.ratingValue` | `number` | Source-specific rating value; compare only with that source and scale. |
| `web.results[].location.reviews.results[].rating.bestRating` | `number` | Source rating scale metadata; not global relevance. |
| `web.results[].location.reviews.results[].rating.reviewCount` | `int?` | Number of reviews reported by source. |
| `web.results[].location.reviews.results[].rating.profile` | `object?` | Profile metadata/value within this model. |
| `web.results[].location.reviews.results[].rating.profile.name` | `string` | Name metadata/value within this model. |
| `web.results[].location.reviews.results[].rating.profile.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `web.results[].location.reviews.results[].rating.profile.long_name` | `string?` | Long name metadata/value within this model. |
| `web.results[].location.reviews.results[].rating.profile.img` | `string?` | Img metadata/value within this model. |
| `web.results[].location.reviews.results[].rating.is_tripadvisor` | `bool?` | Attribution flag for the rating source. |
| `web.results[].location.reviews.results[].author` | `object` | Author metadata/value within this model. |
| `web.results[].location.reviews.results[].author.type` | `string?` | Documented discriminator `person` identifies this enclosing model; unknown future tags remain accepted. |
| `web.results[].location.reviews.results[].author.name` | `string` | Name metadata/value within this model. |
| `web.results[].location.reviews.results[].author.url` | `string?` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `web.results[].location.reviews.results[].author.thumbnail` | `object?` | Thumbnail metadata/value within this model. |
| `web.results[].location.reviews.results[].author.thumbnail.src` | `string` | Src metadata/value within this model. |
| `web.results[].location.reviews.results[].author.thumbnail.alt` | `string?` | Alt metadata/value within this model. |
| `web.results[].location.reviews.results[].author.thumbnail.height` | `int?` | Height metadata/value within this model. |
| `web.results[].location.reviews.results[].author.thumbnail.width` | `int?` | Width metadata/value within this model. |
| `web.results[].location.reviews.results[].author.thumbnail.bg_color` | `string?` | Bg color metadata/value within this model. |
| `web.results[].location.reviews.results[].author.thumbnail.original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `web.results[].location.reviews.results[].author.thumbnail.logo` | `bool?` | Logo metadata/value within this model. |
| `web.results[].location.reviews.results[].author.thumbnail.duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `web.results[].location.reviews.results[].author.thumbnail.theme` | `string?` | Theme metadata/value within this model. |
| `web.results[].location.reviews.results[].author.email` | `string?` | Email metadata/value within this model. |
| `web.results[].location.reviews.results[].review_url` | `string` | Review url metadata/value within this model. |
| `web.results[].location.reviews.results[].language` | `string` | Language metadata/value within this model. |
| `web.results[].location.reviews.viewMoreUrl` | `string` | View more url metadata/value within this model. |
| `web.results[].location.reviews.reviews_in_foreign_language` | `bool` | Reviews in foreign language metadata/value within this model. |
| `web.results[].location.pictures` | `object?` | Pictures metadata/value within this model. |
| `web.results[].location.pictures.viewMoreUrl` | `string?` | View more url metadata/value within this model. |
| `web.results[].location.pictures.results` | `object[]` | Results collection. |
| `web.results[].location.pictures.results[].src` | `string` | Src metadata/value within this model. |
| `web.results[].location.pictures.results[].alt` | `string?` | Alt metadata/value within this model. |
| `web.results[].location.pictures.results[].height` | `int?` | Height metadata/value within this model. |
| `web.results[].location.pictures.results[].width` | `int?` | Width metadata/value within this model. |
| `web.results[].location.pictures.results[].bg_color` | `string?` | Bg color metadata/value within this model. |
| `web.results[].location.pictures.results[].original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `web.results[].location.pictures.results[].logo` | `bool?` | Logo metadata/value within this model. |
| `web.results[].location.pictures.results[].duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `web.results[].location.pictures.results[].theme` | `string?` | Theme metadata/value within this model. |
| `web.results[].location.action` | `object?` | Action metadata/value within this model. |
| `web.results[].location.action.type` | `string` | Type metadata/value within this model. |
| `web.results[].location.action.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `web.results[].location.serves_cuisine` | `string[]?` | Serves cuisine collection. |
| `web.results[].location.categories` | `string[]?` | Categories collection. |
| `web.results[].location.icon_category` | `string?` | Icon category metadata/value within this model. |
| `web.results[].location.timezone` | `string?` | Timezone metadata/value within this model. |
| `web.results[].location.timezone_offset` | `int?` | Timezone offset metadata/value within this model. |
| `web.results[].location.id` | `string?` | Identifier scoped to the enclosing result; local-place IDs expire in about eight hours. |
| `web.results[].location.results` | `object[]?` | Results collection. |
| `web.results[].location.results[].title` | `string` | Title metadata/value within this model. |
| `web.results[].location.results[].url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `web.results[].location.results[].is_source_local` | `bool?` | Is source local metadata/value within this model. |
| `web.results[].location.results[].is_source_both` | `bool?` | Is source both metadata/value within this model. |
| `web.results[].location.results[].description` | `string?` | Display description/excerpt; POI Descriptions text is generated and Context source description is query-independent. |
| `web.results[].location.results[].page_age` | `string?` | Content publication/modification date, not necessarily crawl time. |
| `web.results[].location.results[].page_fetched` | `string?` | Last-fetch date/time as provided by the vendor. |
| `web.results[].location.results[].fetched_content_timestamp` | `int?` | Fetch timestamp; do not infer units beyond the source contract. |
| `web.results[].location.results[].profile` | `object?` | Profile metadata/value within this model. |
| `web.results[].location.results[].profile.name` | `string` | Name metadata/value within this model. |
| `web.results[].location.results[].profile.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `web.results[].location.results[].profile.long_name` | `string?` | Long name metadata/value within this model. |
| `web.results[].location.results[].profile.img` | `string?` | Img metadata/value within this model. |
| `web.results[].location.results[].language` | `string?` | Language metadata/value within this model. |
| `web.results[].location.results[].family_friendly` | `bool?` | Vendor safety classification, not a content-safety guarantee. |
| `web.results[].location.results[].meta_url` | `object` | Meta url metadata/value within this model. |
| `web.results[].location.results[].meta_url.scheme` | `string` | Scheme metadata/value within this model. |
| `web.results[].location.results[].meta_url.netloc` | `string` | Netloc metadata/value within this model. |
| `web.results[].location.results[].meta_url.hostname` | `string?` | Hostname metadata/value within this model. |
| `web.results[].location.results[].meta_url.favicon` | `string` | Favicon metadata/value within this model. |
| `web.results[].location.results[].meta_url.path` | `string` | Path metadata/value within this model. |
| `web.results[].restaurant` | `object?` | Deprecated result property; prefer location. |
| `web.results[].restaurant.title` | `string` | Title metadata/value within this model. |
| `web.results[].restaurant.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `web.results[].restaurant.is_source_local` | `bool?` | Is source local metadata/value within this model. |
| `web.results[].restaurant.is_source_both` | `bool?` | Is source both metadata/value within this model. |
| `web.results[].restaurant.description` | `string?` | Display description/excerpt; POI Descriptions text is generated and Context source description is query-independent. |
| `web.results[].restaurant.page_age` | `string?` | Content publication/modification date, not necessarily crawl time. |
| `web.results[].restaurant.page_fetched` | `string?` | Last-fetch date/time as provided by the vendor. |
| `web.results[].restaurant.fetched_content_timestamp` | `int?` | Fetch timestamp; do not infer units beyond the source contract. |
| `web.results[].restaurant.profile` | `object?` | Profile metadata/value within this model. |
| `web.results[].restaurant.profile.name` | `string` | Name metadata/value within this model. |
| `web.results[].restaurant.profile.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `web.results[].restaurant.profile.long_name` | `string?` | Long name metadata/value within this model. |
| `web.results[].restaurant.profile.img` | `string?` | Img metadata/value within this model. |
| `web.results[].restaurant.language` | `string?` | Language metadata/value within this model. |
| `web.results[].restaurant.family_friendly` | `bool?` | Vendor safety classification, not a content-safety guarantee. |
| `web.results[].restaurant.type` | `string?` | Documented discriminator `location_result` identifies this enclosing model; unknown future tags remain accepted. |
| `web.results[].restaurant.provider_url` | `string` | Attribution/provider destination URL. |
| `web.results[].restaurant.coordinates` | `array?` | Latitude/longitude pair as described by the location model; no geometry guarantee for unrelated arrays. |
| `web.results[].restaurant.zoom_level` | `int?` | Zoom level metadata/value within this model. |
| `web.results[].restaurant.thumbnail` | `object?` | Thumbnail metadata/value within this model. |
| `web.results[].restaurant.thumbnail.src` | `string` | Src metadata/value within this model. |
| `web.results[].restaurant.thumbnail.alt` | `string?` | Alt metadata/value within this model. |
| `web.results[].restaurant.thumbnail.height` | `int?` | Height metadata/value within this model. |
| `web.results[].restaurant.thumbnail.width` | `int?` | Width metadata/value within this model. |
| `web.results[].restaurant.thumbnail.bg_color` | `string?` | Bg color metadata/value within this model. |
| `web.results[].restaurant.thumbnail.original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `web.results[].restaurant.thumbnail.logo` | `bool?` | Logo metadata/value within this model. |
| `web.results[].restaurant.thumbnail.duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `web.results[].restaurant.thumbnail.theme` | `string?` | Theme metadata/value within this model. |
| `web.results[].restaurant.postal_address` | `object?` | Postal address metadata/value within this model. |
| `web.results[].restaurant.postal_address.type` | `string?` | Documented discriminator `PostalAddress` identifies this enclosing model; unknown future tags remain accepted. |
| `web.results[].restaurant.postal_address.country` | `string?` | Country metadata/value within this model. |
| `web.results[].restaurant.postal_address.postalCode` | `string?` | Postal code metadata/value within this model. |
| `web.results[].restaurant.postal_address.streetAddress` | `string?` | Street address metadata/value within this model. |
| `web.results[].restaurant.postal_address.addressRegion` | `string?` | Address region metadata/value within this model. |
| `web.results[].restaurant.postal_address.addressLocality` | `string?` | Address locality metadata/value within this model. |
| `web.results[].restaurant.postal_address.displayAddress` | `string` | Display address metadata/value within this model. |
| `web.results[].restaurant.opening_hours` | `object?` | Business hours; days is a nested array and a day may contain multiple intervals. |
| `web.results[].restaurant.opening_hours.current_day` | `object[]?` | Current day collection. |
| `web.results[].restaurant.opening_hours.current_day[].abbr_name` | `string` | Abbr name metadata/value within this model. |
| `web.results[].restaurant.opening_hours.current_day[].full_name` | `string` | Full name metadata/value within this model. |
| `web.results[].restaurant.opening_hours.current_day[].opens` | `string` | Local 24-hour opening-time string. |
| `web.results[].restaurant.opening_hours.current_day[].closes` | `string` | Local 24-hour closing-time string. |
| `web.results[].restaurant.opening_hours.days` | `object[][]?` | Days collection. |
| `web.results[].restaurant.contact` | `object?` | Contact metadata/value within this model. |
| `web.results[].restaurant.contact.email` | `string?` | Email metadata/value within this model. |
| `web.results[].restaurant.contact.telephone` | `string?` | Telephone metadata/value within this model. |
| `web.results[].restaurant.price_range` | `string?` | Price range metadata/value within this model. |
| `web.results[].restaurant.rating` | `object?` | Rating metadata/value within this model. |
| `web.results[].restaurant.rating.ratingValue` | `number` | Source-specific rating value; compare only with that source and scale. |
| `web.results[].restaurant.rating.bestRating` | `number` | Source rating scale metadata; not global relevance. |
| `web.results[].restaurant.rating.reviewCount` | `int?` | Number of reviews reported by source. |
| `web.results[].restaurant.rating.profile` | `object?` | Profile metadata/value within this model. |
| `web.results[].restaurant.rating.profile.name` | `string` | Name metadata/value within this model. |
| `web.results[].restaurant.rating.profile.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `web.results[].restaurant.rating.profile.long_name` | `string?` | Long name metadata/value within this model. |
| `web.results[].restaurant.rating.profile.img` | `string?` | Img metadata/value within this model. |
| `web.results[].restaurant.rating.is_tripadvisor` | `bool?` | Attribution flag for the rating source. |
| `web.results[].restaurant.distance` | `object?` | Distance metadata/value within this model. |
| `web.results[].restaurant.distance.value` | `number` | Value metadata/value within this model. |
| `web.results[].restaurant.distance.units` | `string` | Units metadata/value within this model. |
| `web.results[].restaurant.profiles` | `object[]?` | Profiles collection. |
| `web.results[].restaurant.profiles[].type` | `string?` | Source-origin type; external normally identifies an external data source, but is not a closed enum. |
| `web.results[].restaurant.profiles[].name` | `string` | Name metadata/value within this model. |
| `web.results[].restaurant.profiles[].url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `web.results[].restaurant.profiles[].long_name` | `string?` | Long name metadata/value within this model. |
| `web.results[].restaurant.profiles[].img` | `string?` | Img metadata/value within this model. |
| `web.results[].restaurant.reviews` | `object?` | Reviews metadata/value within this model. |
| `web.results[].restaurant.reviews.results` | `object[]` | Results collection. |
| `web.results[].restaurant.reviews.results[].title` | `string` | Title metadata/value within this model. |
| `web.results[].restaurant.reviews.results[].description` | `string` | Display description/excerpt; POI Descriptions text is generated and Context source description is query-independent. |
| `web.results[].restaurant.reviews.results[].date` | `string` | Date metadata/value within this model. |
| `web.results[].restaurant.reviews.results[].rating` | `object` | Rating metadata/value within this model. |
| `web.results[].restaurant.reviews.results[].rating.ratingValue` | `number` | Source-specific rating value; compare only with that source and scale. |
| `web.results[].restaurant.reviews.results[].rating.bestRating` | `number` | Source rating scale metadata; not global relevance. |
| `web.results[].restaurant.reviews.results[].rating.reviewCount` | `int?` | Number of reviews reported by source. |
| `web.results[].restaurant.reviews.results[].rating.profile` | `object?` | Profile metadata/value within this model. |
| `web.results[].restaurant.reviews.results[].rating.profile.name` | `string` | Name metadata/value within this model. |
| `web.results[].restaurant.reviews.results[].rating.profile.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `web.results[].restaurant.reviews.results[].rating.profile.long_name` | `string?` | Long name metadata/value within this model. |
| `web.results[].restaurant.reviews.results[].rating.profile.img` | `string?` | Img metadata/value within this model. |
| `web.results[].restaurant.reviews.results[].rating.is_tripadvisor` | `bool?` | Attribution flag for the rating source. |
| `web.results[].restaurant.reviews.results[].author` | `object` | Author metadata/value within this model. |
| `web.results[].restaurant.reviews.results[].author.type` | `string?` | Documented discriminator `person` identifies this enclosing model; unknown future tags remain accepted. |
| `web.results[].restaurant.reviews.results[].author.name` | `string` | Name metadata/value within this model. |
| `web.results[].restaurant.reviews.results[].author.url` | `string?` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `web.results[].restaurant.reviews.results[].author.thumbnail` | `object?` | Thumbnail metadata/value within this model. |
| `web.results[].restaurant.reviews.results[].author.thumbnail.src` | `string` | Src metadata/value within this model. |
| `web.results[].restaurant.reviews.results[].author.thumbnail.alt` | `string?` | Alt metadata/value within this model. |
| `web.results[].restaurant.reviews.results[].author.thumbnail.height` | `int?` | Height metadata/value within this model. |
| `web.results[].restaurant.reviews.results[].author.thumbnail.width` | `int?` | Width metadata/value within this model. |
| `web.results[].restaurant.reviews.results[].author.thumbnail.bg_color` | `string?` | Bg color metadata/value within this model. |
| `web.results[].restaurant.reviews.results[].author.thumbnail.original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `web.results[].restaurant.reviews.results[].author.thumbnail.logo` | `bool?` | Logo metadata/value within this model. |
| `web.results[].restaurant.reviews.results[].author.thumbnail.duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `web.results[].restaurant.reviews.results[].author.thumbnail.theme` | `string?` | Theme metadata/value within this model. |
| `web.results[].restaurant.reviews.results[].author.email` | `string?` | Email metadata/value within this model. |
| `web.results[].restaurant.reviews.results[].review_url` | `string` | Review url metadata/value within this model. |
| `web.results[].restaurant.reviews.results[].language` | `string` | Language metadata/value within this model. |
| `web.results[].restaurant.reviews.viewMoreUrl` | `string` | View more url metadata/value within this model. |
| `web.results[].restaurant.reviews.reviews_in_foreign_language` | `bool` | Reviews in foreign language metadata/value within this model. |
| `web.results[].restaurant.pictures` | `object?` | Pictures metadata/value within this model. |
| `web.results[].restaurant.pictures.viewMoreUrl` | `string?` | View more url metadata/value within this model. |
| `web.results[].restaurant.pictures.results` | `object[]` | Results collection. |
| `web.results[].restaurant.pictures.results[].src` | `string` | Src metadata/value within this model. |
| `web.results[].restaurant.pictures.results[].alt` | `string?` | Alt metadata/value within this model. |
| `web.results[].restaurant.pictures.results[].height` | `int?` | Height metadata/value within this model. |
| `web.results[].restaurant.pictures.results[].width` | `int?` | Width metadata/value within this model. |
| `web.results[].restaurant.pictures.results[].bg_color` | `string?` | Bg color metadata/value within this model. |
| `web.results[].restaurant.pictures.results[].original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `web.results[].restaurant.pictures.results[].logo` | `bool?` | Logo metadata/value within this model. |
| `web.results[].restaurant.pictures.results[].duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `web.results[].restaurant.pictures.results[].theme` | `string?` | Theme metadata/value within this model. |
| `web.results[].restaurant.action` | `object?` | Action metadata/value within this model. |
| `web.results[].restaurant.action.type` | `string` | Type metadata/value within this model. |
| `web.results[].restaurant.action.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `web.results[].restaurant.serves_cuisine` | `string[]?` | Serves cuisine collection. |
| `web.results[].restaurant.categories` | `string[]?` | Categories collection. |
| `web.results[].restaurant.icon_category` | `string?` | Icon category metadata/value within this model. |
| `web.results[].restaurant.timezone` | `string?` | Timezone metadata/value within this model. |
| `web.results[].restaurant.timezone_offset` | `int?` | Timezone offset metadata/value within this model. |
| `web.results[].restaurant.id` | `string?` | Identifier scoped to the enclosing result; local-place IDs expire in about eight hours. |
| `web.results[].restaurant.results` | `object[]?` | Results collection. |
| `web.results[].restaurant.results[].title` | `string` | Title metadata/value within this model. |
| `web.results[].restaurant.results[].url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `web.results[].restaurant.results[].is_source_local` | `bool?` | Is source local metadata/value within this model. |
| `web.results[].restaurant.results[].is_source_both` | `bool?` | Is source both metadata/value within this model. |
| `web.results[].restaurant.results[].description` | `string?` | Display description/excerpt; POI Descriptions text is generated and Context source description is query-independent. |
| `web.results[].restaurant.results[].page_age` | `string?` | Content publication/modification date, not necessarily crawl time. |
| `web.results[].restaurant.results[].page_fetched` | `string?` | Last-fetch date/time as provided by the vendor. |
| `web.results[].restaurant.results[].fetched_content_timestamp` | `int?` | Fetch timestamp; do not infer units beyond the source contract. |
| `web.results[].restaurant.results[].profile` | `object?` | Profile metadata/value within this model. |
| `web.results[].restaurant.results[].profile.name` | `string` | Name metadata/value within this model. |
| `web.results[].restaurant.results[].profile.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `web.results[].restaurant.results[].profile.long_name` | `string?` | Long name metadata/value within this model. |
| `web.results[].restaurant.results[].profile.img` | `string?` | Img metadata/value within this model. |
| `web.results[].restaurant.results[].language` | `string?` | Language metadata/value within this model. |
| `web.results[].restaurant.results[].family_friendly` | `bool?` | Vendor safety classification, not a content-safety guarantee. |
| `web.results[].restaurant.results[].meta_url` | `object` | Meta url metadata/value within this model. |
| `web.results[].restaurant.results[].meta_url.scheme` | `string` | Scheme metadata/value within this model. |
| `web.results[].restaurant.results[].meta_url.netloc` | `string` | Netloc metadata/value within this model. |
| `web.results[].restaurant.results[].meta_url.hostname` | `string?` | Hostname metadata/value within this model. |
| `web.results[].restaurant.results[].meta_url.favicon` | `string` | Favicon metadata/value within this model. |
| `web.results[].restaurant.results[].meta_url.path` | `string` | Path metadata/value within this model. |
| `web.results[].video` | `object?` | Video metadata/value within this model. |
| `web.results[].video.duration` | `string?` | Duration metadata/value within this model. |
| `web.results[].video.views` | `string?` | Views metadata/value within this model. |
| `web.results[].video.creator` | `string?` | Creator metadata/value within this model. |
| `web.results[].video.publisher` | `string?` | Publisher metadata/value within this model. |
| `web.results[].video.thumbnail` | `object?` | Thumbnail metadata/value within this model. |
| `web.results[].video.thumbnail.src` | `string` | Src metadata/value within this model. |
| `web.results[].video.thumbnail.alt` | `string?` | Alt metadata/value within this model. |
| `web.results[].video.thumbnail.height` | `int?` | Height metadata/value within this model. |
| `web.results[].video.thumbnail.width` | `int?` | Width metadata/value within this model. |
| `web.results[].video.thumbnail.bg_color` | `string?` | Bg color metadata/value within this model. |
| `web.results[].video.thumbnail.original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `web.results[].video.thumbnail.logo` | `bool?` | Logo metadata/value within this model. |
| `web.results[].video.thumbnail.duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `web.results[].video.thumbnail.theme` | `string?` | Theme metadata/value within this model. |
| `web.results[].video.tags` | `string[]?` | Tags collection. |
| `web.results[].video.author` | `object?` | Author metadata/value within this model. |
| `web.results[].video.author.name` | `string` | Name metadata/value within this model. |
| `web.results[].video.author.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `web.results[].video.author.long_name` | `string?` | Long name metadata/value within this model. |
| `web.results[].video.author.img` | `string?` | Img metadata/value within this model. |
| `web.results[].video.requires_subscription` | `bool?` | Source-content access restriction, independent of Brave API billing. |
| `web.results[].movie` | `object?` | Movie metadata/value within this model. |
| `web.results[].movie.name` | `string?` | Name metadata/value within this model. |
| `web.results[].movie.description` | `string?` | Display description/excerpt; POI Descriptions text is generated and Context source description is query-independent. |
| `web.results[].movie.url` | `string?` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `web.results[].movie.thumbnail` | `object?` | Thumbnail metadata/value within this model. |
| `web.results[].movie.thumbnail.src` | `string` | Src metadata/value within this model. |
| `web.results[].movie.thumbnail.alt` | `string?` | Alt metadata/value within this model. |
| `web.results[].movie.thumbnail.height` | `int?` | Height metadata/value within this model. |
| `web.results[].movie.thumbnail.width` | `int?` | Width metadata/value within this model. |
| `web.results[].movie.thumbnail.bg_color` | `string?` | Bg color metadata/value within this model. |
| `web.results[].movie.thumbnail.original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `web.results[].movie.thumbnail.logo` | `bool?` | Logo metadata/value within this model. |
| `web.results[].movie.thumbnail.duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `web.results[].movie.thumbnail.theme` | `string?` | Theme metadata/value within this model. |
| `web.results[].movie.release` | `string?` | Release metadata/value within this model. |
| `web.results[].movie.directors` | `object[]?` | Directors collection. |
| `web.results[].movie.directors[].type` | `string?` | Documented discriminator `person` identifies this enclosing model; unknown future tags remain accepted. |
| `web.results[].movie.directors[].name` | `string` | Name metadata/value within this model. |
| `web.results[].movie.directors[].url` | `string?` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `web.results[].movie.directors[].thumbnail` | `object?` | Thumbnail metadata/value within this model. |
| `web.results[].movie.directors[].thumbnail.src` | `string` | Src metadata/value within this model. |
| `web.results[].movie.directors[].thumbnail.alt` | `string?` | Alt metadata/value within this model. |
| `web.results[].movie.directors[].thumbnail.height` | `int?` | Height metadata/value within this model. |
| `web.results[].movie.directors[].thumbnail.width` | `int?` | Width metadata/value within this model. |
| `web.results[].movie.directors[].thumbnail.bg_color` | `string?` | Bg color metadata/value within this model. |
| `web.results[].movie.directors[].thumbnail.original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `web.results[].movie.directors[].thumbnail.logo` | `bool?` | Logo metadata/value within this model. |
| `web.results[].movie.directors[].thumbnail.duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `web.results[].movie.directors[].thumbnail.theme` | `string?` | Theme metadata/value within this model. |
| `web.results[].movie.directors[].email` | `string?` | Email metadata/value within this model. |
| `web.results[].movie.actors` | `object[]?` | Actors collection. |
| `web.results[].movie.actors[].type` | `string?` | Documented discriminator `person` identifies this enclosing model; unknown future tags remain accepted. |
| `web.results[].movie.actors[].name` | `string` | Name metadata/value within this model. |
| `web.results[].movie.actors[].url` | `string?` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `web.results[].movie.actors[].thumbnail` | `object?` | Thumbnail metadata/value within this model. |
| `web.results[].movie.actors[].thumbnail.src` | `string` | Src metadata/value within this model. |
| `web.results[].movie.actors[].thumbnail.alt` | `string?` | Alt metadata/value within this model. |
| `web.results[].movie.actors[].thumbnail.height` | `int?` | Height metadata/value within this model. |
| `web.results[].movie.actors[].thumbnail.width` | `int?` | Width metadata/value within this model. |
| `web.results[].movie.actors[].thumbnail.bg_color` | `string?` | Bg color metadata/value within this model. |
| `web.results[].movie.actors[].thumbnail.original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `web.results[].movie.actors[].thumbnail.logo` | `bool?` | Logo metadata/value within this model. |
| `web.results[].movie.actors[].thumbnail.duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `web.results[].movie.actors[].thumbnail.theme` | `string?` | Theme metadata/value within this model. |
| `web.results[].movie.actors[].email` | `string?` | Email metadata/value within this model. |
| `web.results[].movie.rating` | `object?` | Rating metadata/value within this model. |
| `web.results[].movie.rating.ratingValue` | `number` | Source-specific rating value; compare only with that source and scale. |
| `web.results[].movie.rating.bestRating` | `number` | Source rating scale metadata; not global relevance. |
| `web.results[].movie.rating.reviewCount` | `int?` | Number of reviews reported by source. |
| `web.results[].movie.rating.profile` | `object?` | Profile metadata/value within this model. |
| `web.results[].movie.rating.profile.name` | `string` | Name metadata/value within this model. |
| `web.results[].movie.rating.profile.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `web.results[].movie.rating.profile.long_name` | `string?` | Long name metadata/value within this model. |
| `web.results[].movie.rating.profile.img` | `string?` | Img metadata/value within this model. |
| `web.results[].movie.rating.is_tripadvisor` | `bool?` | Attribution flag for the rating source. |
| `web.results[].movie.duration` | `string?` | Duration metadata/value within this model. |
| `web.results[].movie.genre` | `string[]?` | Genre collection. |
| `web.results[].movie.query` | `string?` | Query metadata object, or corrected/completed phrase on suggestion/spellcheck items. |
| `web.results[].faq` | `object?` | Faq metadata/value within this model. |
| `web.results[].faq.items` | `object[]` | Items collection. |
| `web.results[].faq.items[].question` | `string` | Question metadata/value within this model. |
| `web.results[].faq.items[].answer` | `string` | Calculator or question/answer result value according to enclosing type; not a general full-page field. |
| `web.results[].faq.items[].title` | `string` | Title metadata/value within this model. |
| `web.results[].faq.items[].url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `web.results[].faq.items[].meta_url` | `object?` | Meta url metadata/value within this model. |
| `web.results[].faq.items[].meta_url.scheme` | `string` | Scheme metadata/value within this model. |
| `web.results[].faq.items[].meta_url.netloc` | `string` | Netloc metadata/value within this model. |
| `web.results[].faq.items[].meta_url.hostname` | `string?` | Hostname metadata/value within this model. |
| `web.results[].faq.items[].meta_url.favicon` | `string` | Favicon metadata/value within this model. |
| `web.results[].faq.items[].meta_url.path` | `string` | Path metadata/value within this model. |
| `web.results[].qa` | `object?` | Qa metadata/value within this model. |
| `web.results[].qa.question` | `string` | Question metadata/value within this model. |
| `web.results[].qa.answer` | `object` | Calculator or question/answer result value according to enclosing type; not a general full-page field. |
| `web.results[].qa.answer.text` | `string` | Text metadata/value within this model. |
| `web.results[].qa.answer.author` | `string?` | Author metadata/value within this model. |
| `web.results[].qa.answer.upvoteCount` | `int?` | Upvote count metadata/value within this model. |
| `web.results[].qa.answer.downvoteCount` | `int?` | Downvote count metadata/value within this model. |
| `web.results[].book` | `object?` | Book metadata/value within this model. |
| `web.results[].book.title` | `string` | Title metadata/value within this model. |
| `web.results[].book.author` | `object[]` | Author collection. |
| `web.results[].book.author[].type` | `string?` | Documented discriminator `person` identifies this enclosing model; unknown future tags remain accepted. |
| `web.results[].book.author[].name` | `string` | Name metadata/value within this model. |
| `web.results[].book.author[].url` | `string?` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `web.results[].book.author[].thumbnail` | `object?` | Thumbnail metadata/value within this model. |
| `web.results[].book.author[].thumbnail.src` | `string` | Src metadata/value within this model. |
| `web.results[].book.author[].thumbnail.alt` | `string?` | Alt metadata/value within this model. |
| `web.results[].book.author[].thumbnail.height` | `int?` | Height metadata/value within this model. |
| `web.results[].book.author[].thumbnail.width` | `int?` | Width metadata/value within this model. |
| `web.results[].book.author[].thumbnail.bg_color` | `string?` | Bg color metadata/value within this model. |
| `web.results[].book.author[].thumbnail.original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `web.results[].book.author[].thumbnail.logo` | `bool?` | Logo metadata/value within this model. |
| `web.results[].book.author[].thumbnail.duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `web.results[].book.author[].thumbnail.theme` | `string?` | Theme metadata/value within this model. |
| `web.results[].book.author[].email` | `string?` | Email metadata/value within this model. |
| `web.results[].book.date` | `string?` | Date metadata/value within this model. |
| `web.results[].book.price` | `object?` | Price metadata/value within this model. |
| `web.results[].book.price.price` | `string` | Price metadata/value within this model. |
| `web.results[].book.price.priceCurrency` | `string` | Price currency metadata/value within this model. |
| `web.results[].book.pages` | `int?` | Pages metadata/value within this model. |
| `web.results[].book.publisher` | `object?` | Publisher metadata/value within this model. |
| `web.results[].book.publisher.type` | `string?` | Documented discriminator `person` identifies this enclosing model; unknown future tags remain accepted. |
| `web.results[].book.publisher.name` | `string` | Name metadata/value within this model. |
| `web.results[].book.publisher.url` | `string?` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `web.results[].book.publisher.thumbnail` | `object?` | Thumbnail metadata/value within this model. |
| `web.results[].book.publisher.thumbnail.src` | `string` | Src metadata/value within this model. |
| `web.results[].book.publisher.thumbnail.alt` | `string?` | Alt metadata/value within this model. |
| `web.results[].book.publisher.thumbnail.height` | `int?` | Height metadata/value within this model. |
| `web.results[].book.publisher.thumbnail.width` | `int?` | Width metadata/value within this model. |
| `web.results[].book.publisher.thumbnail.bg_color` | `string?` | Bg color metadata/value within this model. |
| `web.results[].book.publisher.thumbnail.original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `web.results[].book.publisher.thumbnail.logo` | `bool?` | Logo metadata/value within this model. |
| `web.results[].book.publisher.thumbnail.duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `web.results[].book.publisher.thumbnail.theme` | `string?` | Theme metadata/value within this model. |
| `web.results[].book.publisher.email` | `string?` | Email metadata/value within this model. |
| `web.results[].book.rating` | `object?` | Rating metadata/value within this model. |
| `web.results[].book.rating.ratingValue` | `number` | Source-specific rating value; compare only with that source and scale. |
| `web.results[].book.rating.bestRating` | `number` | Source rating scale metadata; not global relevance. |
| `web.results[].book.rating.reviewCount` | `int?` | Number of reviews reported by source. |
| `web.results[].book.rating.profile` | `object?` | Profile metadata/value within this model. |
| `web.results[].book.rating.profile.name` | `string` | Name metadata/value within this model. |
| `web.results[].book.rating.profile.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `web.results[].book.rating.profile.long_name` | `string?` | Long name metadata/value within this model. |
| `web.results[].book.rating.profile.img` | `string?` | Img metadata/value within this model. |
| `web.results[].book.rating.is_tripadvisor` | `bool?` | Attribution flag for the rating source. |
| `web.results[].rating` | `object?` | Rating metadata/value within this model. |
| `web.results[].rating.ratingValue` | `number` | Source-specific rating value; compare only with that source and scale. |
| `web.results[].rating.bestRating` | `number` | Source rating scale metadata; not global relevance. |
| `web.results[].rating.reviewCount` | `int?` | Number of reviews reported by source. |
| `web.results[].rating.profile` | `object?` | Profile metadata/value within this model. |
| `web.results[].rating.profile.name` | `string` | Name metadata/value within this model. |
| `web.results[].rating.profile.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `web.results[].rating.profile.long_name` | `string?` | Long name metadata/value within this model. |
| `web.results[].rating.profile.img` | `string?` | Img metadata/value within this model. |
| `web.results[].rating.is_tripadvisor` | `bool?` | Attribution flag for the rating source. |
| `web.results[].article` | `object?` | Article metadata/value within this model. |
| `web.results[].article.author` | `object[]?` | Author collection. |
| `web.results[].article.author[].type` | `string?` | Documented discriminator `person` identifies this enclosing model; unknown future tags remain accepted. |
| `web.results[].article.author[].name` | `string` | Name metadata/value within this model. |
| `web.results[].article.author[].url` | `string?` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `web.results[].article.author[].thumbnail` | `object?` | Thumbnail metadata/value within this model. |
| `web.results[].article.author[].thumbnail.src` | `string` | Src metadata/value within this model. |
| `web.results[].article.author[].thumbnail.alt` | `string?` | Alt metadata/value within this model. |
| `web.results[].article.author[].thumbnail.height` | `int?` | Height metadata/value within this model. |
| `web.results[].article.author[].thumbnail.width` | `int?` | Width metadata/value within this model. |
| `web.results[].article.author[].thumbnail.bg_color` | `string?` | Bg color metadata/value within this model. |
| `web.results[].article.author[].thumbnail.original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `web.results[].article.author[].thumbnail.logo` | `bool?` | Logo metadata/value within this model. |
| `web.results[].article.author[].thumbnail.duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `web.results[].article.author[].thumbnail.theme` | `string?` | Theme metadata/value within this model. |
| `web.results[].article.author[].email` | `string?` | Email metadata/value within this model. |
| `web.results[].article.date` | `string?` | Date metadata/value within this model. |
| `web.results[].article.publisher` | `object?` | Publisher metadata/value within this model. |
| `web.results[].article.publisher.type` | `string?` | Documented discriminator `organization` identifies this enclosing model; unknown future tags remain accepted. |
| `web.results[].article.publisher.name` | `string` | Name metadata/value within this model. |
| `web.results[].article.publisher.url` | `string?` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `web.results[].article.publisher.thumbnail` | `object?` | Thumbnail metadata/value within this model. |
| `web.results[].article.publisher.thumbnail.src` | `string` | Src metadata/value within this model. |
| `web.results[].article.publisher.thumbnail.alt` | `string?` | Alt metadata/value within this model. |
| `web.results[].article.publisher.thumbnail.height` | `int?` | Height metadata/value within this model. |
| `web.results[].article.publisher.thumbnail.width` | `int?` | Width metadata/value within this model. |
| `web.results[].article.publisher.thumbnail.bg_color` | `string?` | Bg color metadata/value within this model. |
| `web.results[].article.publisher.thumbnail.original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `web.results[].article.publisher.thumbnail.logo` | `bool?` | Logo metadata/value within this model. |
| `web.results[].article.publisher.thumbnail.duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `web.results[].article.publisher.thumbnail.theme` | `string?` | Theme metadata/value within this model. |
| `web.results[].article.publisher.contact_points` | `object[]?` | Contact points collection. |
| `web.results[].article.publisher.contact_points[].type` | `string?` | Documented discriminator `contact_point` identifies this enclosing model; unknown future tags remain accepted. |
| `web.results[].article.publisher.contact_points[].name` | `string` | Name metadata/value within this model. |
| `web.results[].article.publisher.contact_points[].url` | `string?` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `web.results[].article.publisher.contact_points[].thumbnail` | `object?` | Thumbnail metadata/value within this model. |
| `web.results[].article.publisher.contact_points[].thumbnail.src` | `string` | Src metadata/value within this model. |
| `web.results[].article.publisher.contact_points[].thumbnail.alt` | `string?` | Alt metadata/value within this model. |
| `web.results[].article.publisher.contact_points[].thumbnail.height` | `int?` | Height metadata/value within this model. |
| `web.results[].article.publisher.contact_points[].thumbnail.width` | `int?` | Width metadata/value within this model. |
| `web.results[].article.publisher.contact_points[].thumbnail.bg_color` | `string?` | Bg color metadata/value within this model. |
| `web.results[].article.publisher.contact_points[].thumbnail.original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `web.results[].article.publisher.contact_points[].thumbnail.logo` | `bool?` | Logo metadata/value within this model. |
| `web.results[].article.publisher.contact_points[].thumbnail.duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `web.results[].article.publisher.contact_points[].thumbnail.theme` | `string?` | Theme metadata/value within this model. |
| `web.results[].article.publisher.contact_points[].telephone` | `string?` | Telephone metadata/value within this model. |
| `web.results[].article.publisher.contact_points[].email` | `string?` | Email metadata/value within this model. |
| `web.results[].article.thumbnail` | `object?` | Thumbnail metadata/value within this model. |
| `web.results[].article.thumbnail.src` | `string` | Src metadata/value within this model. |
| `web.results[].article.thumbnail.alt` | `string?` | Alt metadata/value within this model. |
| `web.results[].article.thumbnail.height` | `int?` | Height metadata/value within this model. |
| `web.results[].article.thumbnail.width` | `int?` | Width metadata/value within this model. |
| `web.results[].article.thumbnail.bg_color` | `string?` | Bg color metadata/value within this model. |
| `web.results[].article.thumbnail.original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `web.results[].article.thumbnail.logo` | `bool?` | Logo metadata/value within this model. |
| `web.results[].article.thumbnail.duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `web.results[].article.thumbnail.theme` | `string?` | Theme metadata/value within this model. |
| `web.results[].article.isAccessibleForFree` | `bool?` | Is accessible for free metadata/value within this model. |
| `web.results[].product` | `any?` | Product metadata/value within this model. |
| `web.results[].product.type` | `string?` | Documented discriminator `Product` identifies this enclosing model; unknown future tags remain accepted. |
| `web.results[].product.name` | `string` | Name metadata/value within this model. |
| `web.results[].product.url` | `string?` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `web.results[].product.category` | `string?` | Category metadata/value within this model. |
| `web.results[].product.price` | `string` | Price metadata/value within this model. |
| `web.results[].product.thumbnail` | `object` | Thumbnail metadata/value within this model. |
| `web.results[].product.thumbnail.src` | `string` | Src metadata/value within this model. |
| `web.results[].product.thumbnail.alt` | `string?` | Alt metadata/value within this model. |
| `web.results[].product.thumbnail.height` | `int?` | Height metadata/value within this model. |
| `web.results[].product.thumbnail.width` | `int?` | Width metadata/value within this model. |
| `web.results[].product.thumbnail.bg_color` | `string?` | Bg color metadata/value within this model. |
| `web.results[].product.thumbnail.original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `web.results[].product.thumbnail.logo` | `bool?` | Logo metadata/value within this model. |
| `web.results[].product.thumbnail.duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `web.results[].product.thumbnail.theme` | `string?` | Theme metadata/value within this model. |
| `web.results[].product.description` | `string?` | Display description/excerpt; POI Descriptions text is generated and Context source description is query-independent. |
| `web.results[].product.offers` | `object[]?` | Offers collection. |
| `web.results[].product.offers[].url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `web.results[].product.offers[].priceCurrency` | `string` | Price currency metadata/value within this model. |
| `web.results[].product.offers[].price` | `string` | Price metadata/value within this model. |
| `web.results[].product.rating` | `object?` | Rating metadata/value within this model. |
| `web.results[].product.rating.ratingValue` | `number` | Source-specific rating value; compare only with that source and scale. |
| `web.results[].product.rating.bestRating` | `number` | Source rating scale metadata; not global relevance. |
| `web.results[].product.rating.reviewCount` | `int?` | Number of reviews reported by source. |
| `web.results[].product.rating.profile` | `object?` | Profile metadata/value within this model. |
| `web.results[].product.rating.profile.name` | `string` | Name metadata/value within this model. |
| `web.results[].product.rating.profile.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `web.results[].product.rating.profile.long_name` | `string?` | Long name metadata/value within this model. |
| `web.results[].product.rating.profile.img` | `string?` | Img metadata/value within this model. |
| `web.results[].product.rating.is_tripadvisor` | `bool?` | Attribution flag for the rating source. |
| `web.results[].product.gtin` | `string?` | Gtin metadata/value within this model. |
| `web.results[].product.gtin8` | `string?` | Gtin8 metadata/value within this model. |
| `web.results[].product.gtin12` | `string?` | Gtin12 metadata/value within this model. |
| `web.results[].product.gtin13` | `string?` | Gtin13 metadata/value within this model. |
| `web.results[].product.gtin14` | `string?` | Gtin14 metadata/value within this model. |
| `web.results[].product_cluster` | `any[]?` | Product cluster collection. |
| `web.results[].product_cluster[].type` | `string?` | Documented discriminator `Product` identifies this enclosing model; unknown future tags remain accepted. |
| `web.results[].product_cluster[].name` | `string` | Name metadata/value within this model. |
| `web.results[].product_cluster[].url` | `string?` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `web.results[].product_cluster[].category` | `string?` | Category metadata/value within this model. |
| `web.results[].product_cluster[].price` | `string` | Price metadata/value within this model. |
| `web.results[].product_cluster[].thumbnail` | `object` | Thumbnail metadata/value within this model. |
| `web.results[].product_cluster[].thumbnail.src` | `string` | Src metadata/value within this model. |
| `web.results[].product_cluster[].thumbnail.alt` | `string?` | Alt metadata/value within this model. |
| `web.results[].product_cluster[].thumbnail.height` | `int?` | Height metadata/value within this model. |
| `web.results[].product_cluster[].thumbnail.width` | `int?` | Width metadata/value within this model. |
| `web.results[].product_cluster[].thumbnail.bg_color` | `string?` | Bg color metadata/value within this model. |
| `web.results[].product_cluster[].thumbnail.original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `web.results[].product_cluster[].thumbnail.logo` | `bool?` | Logo metadata/value within this model. |
| `web.results[].product_cluster[].thumbnail.duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `web.results[].product_cluster[].thumbnail.theme` | `string?` | Theme metadata/value within this model. |
| `web.results[].product_cluster[].description` | `string?` | Display description/excerpt; POI Descriptions text is generated and Context source description is query-independent. |
| `web.results[].product_cluster[].offers` | `object[]?` | Offers collection. |
| `web.results[].product_cluster[].offers[].url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `web.results[].product_cluster[].offers[].priceCurrency` | `string` | Price currency metadata/value within this model. |
| `web.results[].product_cluster[].offers[].price` | `string` | Price metadata/value within this model. |
| `web.results[].product_cluster[].rating` | `object?` | Rating metadata/value within this model. |
| `web.results[].product_cluster[].rating.ratingValue` | `number` | Source-specific rating value; compare only with that source and scale. |
| `web.results[].product_cluster[].rating.bestRating` | `number` | Source rating scale metadata; not global relevance. |
| `web.results[].product_cluster[].rating.reviewCount` | `int?` | Number of reviews reported by source. |
| `web.results[].product_cluster[].rating.profile` | `object?` | Profile metadata/value within this model. |
| `web.results[].product_cluster[].rating.profile.name` | `string` | Name metadata/value within this model. |
| `web.results[].product_cluster[].rating.profile.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `web.results[].product_cluster[].rating.profile.long_name` | `string?` | Long name metadata/value within this model. |
| `web.results[].product_cluster[].rating.profile.img` | `string?` | Img metadata/value within this model. |
| `web.results[].product_cluster[].rating.is_tripadvisor` | `bool?` | Attribution flag for the rating source. |
| `web.results[].product_cluster[].gtin` | `string?` | Gtin metadata/value within this model. |
| `web.results[].product_cluster[].gtin8` | `string?` | Gtin8 metadata/value within this model. |
| `web.results[].product_cluster[].gtin12` | `string?` | Gtin12 metadata/value within this model. |
| `web.results[].product_cluster[].gtin13` | `string?` | Gtin13 metadata/value within this model. |
| `web.results[].product_cluster[].gtin14` | `string?` | Gtin14 metadata/value within this model. |
| `web.results[].cluster_type` | `string?` | Cluster type metadata/value within this model. |
| `web.results[].cluster` | `object[]?` | Cluster collection. |
| `web.results[].cluster[].title` | `string` | Title metadata/value within this model. |
| `web.results[].cluster[].url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `web.results[].cluster[].is_source_local` | `bool?` | Is source local metadata/value within this model. |
| `web.results[].cluster[].is_source_both` | `bool?` | Is source both metadata/value within this model. |
| `web.results[].cluster[].description` | `string?` | Display description/excerpt; POI Descriptions text is generated and Context source description is query-independent. |
| `web.results[].cluster[].page_age` | `string?` | Content publication/modification date, not necessarily crawl time. |
| `web.results[].cluster[].page_fetched` | `string?` | Last-fetch date/time as provided by the vendor. |
| `web.results[].cluster[].fetched_content_timestamp` | `int?` | Fetch timestamp; do not infer units beyond the source contract. |
| `web.results[].cluster[].profile` | `object?` | Profile metadata/value within this model. |
| `web.results[].cluster[].profile.name` | `string` | Name metadata/value within this model. |
| `web.results[].cluster[].profile.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `web.results[].cluster[].profile.long_name` | `string?` | Long name metadata/value within this model. |
| `web.results[].cluster[].profile.img` | `string?` | Img metadata/value within this model. |
| `web.results[].cluster[].language` | `string?` | Language metadata/value within this model. |
| `web.results[].cluster[].family_friendly` | `bool?` | Vendor safety classification, not a content-safety guarantee. |
| `web.results[].creative_work` | `object?` | Creative work metadata/value within this model. |
| `web.results[].creative_work.name` | `string` | Name metadata/value within this model. |
| `web.results[].creative_work.rating` | `object?` | Rating metadata/value within this model. |
| `web.results[].creative_work.rating.ratingValue` | `number` | Source-specific rating value; compare only with that source and scale. |
| `web.results[].creative_work.rating.bestRating` | `number` | Source rating scale metadata; not global relevance. |
| `web.results[].creative_work.rating.reviewCount` | `int?` | Number of reviews reported by source. |
| `web.results[].creative_work.rating.profile` | `object?` | Profile metadata/value within this model. |
| `web.results[].creative_work.rating.profile.name` | `string` | Name metadata/value within this model. |
| `web.results[].creative_work.rating.profile.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `web.results[].creative_work.rating.profile.long_name` | `string?` | Long name metadata/value within this model. |
| `web.results[].creative_work.rating.profile.img` | `string?` | Img metadata/value within this model. |
| `web.results[].creative_work.rating.is_tripadvisor` | `bool?` | Attribution flag for the rating source. |
| `web.results[].creative_work.thumbnail` | `object` | Thumbnail metadata/value within this model. |
| `web.results[].creative_work.thumbnail.src` | `string` | Src metadata/value within this model. |
| `web.results[].creative_work.thumbnail.alt` | `string?` | Alt metadata/value within this model. |
| `web.results[].creative_work.thumbnail.height` | `int?` | Height metadata/value within this model. |
| `web.results[].creative_work.thumbnail.width` | `int?` | Width metadata/value within this model. |
| `web.results[].creative_work.thumbnail.bg_color` | `string?` | Bg color metadata/value within this model. |
| `web.results[].creative_work.thumbnail.original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `web.results[].creative_work.thumbnail.logo` | `bool?` | Logo metadata/value within this model. |
| `web.results[].creative_work.thumbnail.duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `web.results[].creative_work.thumbnail.theme` | `string?` | Theme metadata/value within this model. |
| `web.results[].music_recording` | `object?` | Music recording metadata/value within this model. |
| `web.results[].music_recording.name` | `string` | Name metadata/value within this model. |
| `web.results[].music_recording.rating` | `object?` | Rating metadata/value within this model. |
| `web.results[].music_recording.rating.ratingValue` | `number` | Source-specific rating value; compare only with that source and scale. |
| `web.results[].music_recording.rating.bestRating` | `number` | Source rating scale metadata; not global relevance. |
| `web.results[].music_recording.rating.reviewCount` | `int?` | Number of reviews reported by source. |
| `web.results[].music_recording.rating.profile` | `object?` | Profile metadata/value within this model. |
| `web.results[].music_recording.rating.profile.name` | `string` | Name metadata/value within this model. |
| `web.results[].music_recording.rating.profile.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `web.results[].music_recording.rating.profile.long_name` | `string?` | Long name metadata/value within this model. |
| `web.results[].music_recording.rating.profile.img` | `string?` | Img metadata/value within this model. |
| `web.results[].music_recording.rating.is_tripadvisor` | `bool?` | Attribution flag for the rating source. |
| `web.results[].music_recording.thumbnail` | `object?` | Thumbnail metadata/value within this model. |
| `web.results[].music_recording.thumbnail.src` | `string` | Src metadata/value within this model. |
| `web.results[].music_recording.thumbnail.alt` | `string?` | Alt metadata/value within this model. |
| `web.results[].music_recording.thumbnail.height` | `int?` | Height metadata/value within this model. |
| `web.results[].music_recording.thumbnail.width` | `int?` | Width metadata/value within this model. |
| `web.results[].music_recording.thumbnail.bg_color` | `string?` | Bg color metadata/value within this model. |
| `web.results[].music_recording.thumbnail.original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `web.results[].music_recording.thumbnail.logo` | `bool?` | Logo metadata/value within this model. |
| `web.results[].music_recording.thumbnail.duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `web.results[].music_recording.thumbnail.theme` | `string?` | Theme metadata/value within this model. |
| `web.results[].review` | `object?` | Review metadata/value within this model. |
| `web.results[].review.type` | `string?` | Documented discriminator `Review` identifies this enclosing model; unknown future tags remain accepted. |
| `web.results[].review.name` | `string` | Name metadata/value within this model. |
| `web.results[].review.thumbnail` | `object` | Thumbnail metadata/value within this model. |
| `web.results[].review.thumbnail.src` | `string` | Src metadata/value within this model. |
| `web.results[].review.thumbnail.alt` | `string?` | Alt metadata/value within this model. |
| `web.results[].review.thumbnail.height` | `int?` | Height metadata/value within this model. |
| `web.results[].review.thumbnail.width` | `int?` | Width metadata/value within this model. |
| `web.results[].review.thumbnail.bg_color` | `string?` | Bg color metadata/value within this model. |
| `web.results[].review.thumbnail.original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `web.results[].review.thumbnail.logo` | `bool?` | Logo metadata/value within this model. |
| `web.results[].review.thumbnail.duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `web.results[].review.thumbnail.theme` | `string?` | Theme metadata/value within this model. |
| `web.results[].review.description` | `string` | Display description/excerpt; POI Descriptions text is generated and Context source description is query-independent. |
| `web.results[].review.rating` | `object` | Rating metadata/value within this model. |
| `web.results[].review.rating.ratingValue` | `number` | Source-specific rating value; compare only with that source and scale. |
| `web.results[].review.rating.bestRating` | `number` | Source rating scale metadata; not global relevance. |
| `web.results[].review.rating.reviewCount` | `int?` | Number of reviews reported by source. |
| `web.results[].review.rating.profile` | `object?` | Profile metadata/value within this model. |
| `web.results[].review.rating.profile.name` | `string` | Name metadata/value within this model. |
| `web.results[].review.rating.profile.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `web.results[].review.rating.profile.long_name` | `string?` | Long name metadata/value within this model. |
| `web.results[].review.rating.profile.img` | `string?` | Img metadata/value within this model. |
| `web.results[].review.rating.is_tripadvisor` | `bool?` | Attribution flag for the rating source. |
| `web.results[].recipe` | `object?` | Recipe metadata/value within this model. |
| `web.results[].recipe.title` | `string` | Title metadata/value within this model. |
| `web.results[].recipe.description` | `string` | Display description/excerpt; POI Descriptions text is generated and Context source description is query-independent. |
| `web.results[].recipe.thumbnail` | `object` | Thumbnail metadata/value within this model. |
| `web.results[].recipe.thumbnail.src` | `string` | Src metadata/value within this model. |
| `web.results[].recipe.thumbnail.alt` | `string?` | Alt metadata/value within this model. |
| `web.results[].recipe.thumbnail.height` | `int?` | Height metadata/value within this model. |
| `web.results[].recipe.thumbnail.width` | `int?` | Width metadata/value within this model. |
| `web.results[].recipe.thumbnail.bg_color` | `string?` | Bg color metadata/value within this model. |
| `web.results[].recipe.thumbnail.original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `web.results[].recipe.thumbnail.logo` | `bool?` | Logo metadata/value within this model. |
| `web.results[].recipe.thumbnail.duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `web.results[].recipe.thumbnail.theme` | `string?` | Theme metadata/value within this model. |
| `web.results[].recipe.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `web.results[].recipe.domain` | `string` | Domain metadata/value within this model. |
| `web.results[].recipe.favicon` | `string` | Favicon metadata/value within this model. |
| `web.results[].recipe.time` | `string?` | Time metadata/value within this model. |
| `web.results[].recipe.prep_time` | `string?` | Prep time metadata/value within this model. |
| `web.results[].recipe.cook_time` | `string?` | Cook time metadata/value within this model. |
| `web.results[].recipe.ingredients` | `string?` | Ingredients metadata/value within this model. |
| `web.results[].recipe.instructions` | `object[]?` | Instructions collection. |
| `web.results[].recipe.instructions[].text` | `string` | Text metadata/value within this model. |
| `web.results[].recipe.instructions[].name` | `string?` | Name metadata/value within this model. |
| `web.results[].recipe.instructions[].url` | `string?` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `web.results[].recipe.instructions[].image` | `string[]?` | Image collection. |
| `web.results[].recipe.servings` | `int?` | Servings metadata/value within this model. |
| `web.results[].recipe.calories` | `int?` | Calories metadata/value within this model. |
| `web.results[].recipe.publisher` | `string?` | Publisher metadata/value within this model. |
| `web.results[].recipe.rating` | `object?` | Rating metadata/value within this model. |
| `web.results[].recipe.rating.ratingValue` | `number` | Source-specific rating value; compare only with that source and scale. |
| `web.results[].recipe.rating.bestRating` | `number` | Source rating scale metadata; not global relevance. |
| `web.results[].recipe.rating.reviewCount` | `int?` | Number of reviews reported by source. |
| `web.results[].recipe.rating.profile` | `object?` | Profile metadata/value within this model. |
| `web.results[].recipe.rating.profile.name` | `string` | Name metadata/value within this model. |
| `web.results[].recipe.rating.profile.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `web.results[].recipe.rating.profile.long_name` | `string?` | Long name metadata/value within this model. |
| `web.results[].recipe.rating.profile.img` | `string?` | Img metadata/value within this model. |
| `web.results[].recipe.rating.is_tripadvisor` | `bool?` | Attribution flag for the rating source. |
| `web.results[].recipe.recipeCategory` | `string?` | Recipe category metadata/value within this model. |
| `web.results[].recipe.recipeCuisine` | `string?` | Recipe cuisine metadata/value within this model. |
| `web.results[].recipe.video` | `object?` | Video metadata/value within this model. |
| `web.results[].recipe.video.duration` | `string?` | Duration metadata/value within this model. |
| `web.results[].recipe.video.views` | `string?` | Views metadata/value within this model. |
| `web.results[].recipe.video.creator` | `string?` | Creator metadata/value within this model. |
| `web.results[].recipe.video.publisher` | `string?` | Publisher metadata/value within this model. |
| `web.results[].recipe.video.thumbnail` | `object?` | Thumbnail metadata/value within this model. |
| `web.results[].recipe.video.thumbnail.src` | `string` | Src metadata/value within this model. |
| `web.results[].recipe.video.thumbnail.alt` | `string?` | Alt metadata/value within this model. |
| `web.results[].recipe.video.thumbnail.height` | `int?` | Height metadata/value within this model. |
| `web.results[].recipe.video.thumbnail.width` | `int?` | Width metadata/value within this model. |
| `web.results[].recipe.video.thumbnail.bg_color` | `string?` | Bg color metadata/value within this model. |
| `web.results[].recipe.video.thumbnail.original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `web.results[].recipe.video.thumbnail.logo` | `bool?` | Logo metadata/value within this model. |
| `web.results[].recipe.video.thumbnail.duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `web.results[].recipe.video.thumbnail.theme` | `string?` | Theme metadata/value within this model. |
| `web.results[].recipe.video.tags` | `string[]?` | Tags collection. |
| `web.results[].recipe.video.author` | `object?` | Author metadata/value within this model. |
| `web.results[].recipe.video.author.name` | `string` | Name metadata/value within this model. |
| `web.results[].recipe.video.author.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `web.results[].recipe.video.author.long_name` | `string?` | Long name metadata/value within this model. |
| `web.results[].recipe.video.author.img` | `string?` | Img metadata/value within this model. |
| `web.results[].recipe.video.requires_subscription` | `bool?` | Source-content access restriction, independent of Brave API billing. |
| `web.results[].software` | `object?` | Software metadata/value within this model. |
| `web.results[].software.name` | `string?` | Name metadata/value within this model. |
| `web.results[].software.author` | `string?` | Author metadata/value within this model. |
| `web.results[].software.version` | `string?` | Version metadata/value within this model. |
| `web.results[].software.codeRepository` | `string?` | Code repository metadata/value within this model. |
| `web.results[].software.homepage` | `string?` | Homepage metadata/value within this model. |
| `web.results[].software.datePublished` | `string?` | Date published metadata/value within this model. |
| `web.results[].software.is_npm` | `bool?` | Is npm metadata/value within this model. |
| `web.results[].software.is_pypi` | `bool?` | Is pypi metadata/value within this model. |
| `web.results[].software.stars` | `int?` | Stars metadata/value within this model. |
| `web.results[].software.forks` | `int?` | Forks metadata/value within this model. |
| `web.results[].software.programmingLanguage` | `string?` | Programming language metadata/value within this model. |
| `web.results[].organization` | `object?` | Organization metadata/value within this model. |
| `web.results[].organization.type` | `string?` | Documented discriminator `organization` identifies this enclosing model; unknown future tags remain accepted. |
| `web.results[].organization.name` | `string` | Name metadata/value within this model. |
| `web.results[].organization.url` | `string?` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `web.results[].organization.thumbnail` | `object?` | Thumbnail metadata/value within this model. |
| `web.results[].organization.thumbnail.src` | `string` | Src metadata/value within this model. |
| `web.results[].organization.thumbnail.alt` | `string?` | Alt metadata/value within this model. |
| `web.results[].organization.thumbnail.height` | `int?` | Height metadata/value within this model. |
| `web.results[].organization.thumbnail.width` | `int?` | Width metadata/value within this model. |
| `web.results[].organization.thumbnail.bg_color` | `string?` | Bg color metadata/value within this model. |
| `web.results[].organization.thumbnail.original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `web.results[].organization.thumbnail.logo` | `bool?` | Logo metadata/value within this model. |
| `web.results[].organization.thumbnail.duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `web.results[].organization.thumbnail.theme` | `string?` | Theme metadata/value within this model. |
| `web.results[].organization.contact_points` | `object[]?` | Contact points collection. |
| `web.results[].organization.contact_points[].type` | `string?` | Documented discriminator `contact_point` identifies this enclosing model; unknown future tags remain accepted. |
| `web.results[].organization.contact_points[].name` | `string` | Name metadata/value within this model. |
| `web.results[].organization.contact_points[].url` | `string?` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `web.results[].organization.contact_points[].thumbnail` | `object?` | Thumbnail metadata/value within this model. |
| `web.results[].organization.contact_points[].thumbnail.src` | `string` | Src metadata/value within this model. |
| `web.results[].organization.contact_points[].thumbnail.alt` | `string?` | Alt metadata/value within this model. |
| `web.results[].organization.contact_points[].thumbnail.height` | `int?` | Height metadata/value within this model. |
| `web.results[].organization.contact_points[].thumbnail.width` | `int?` | Width metadata/value within this model. |
| `web.results[].organization.contact_points[].thumbnail.bg_color` | `string?` | Bg color metadata/value within this model. |
| `web.results[].organization.contact_points[].thumbnail.original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `web.results[].organization.contact_points[].thumbnail.logo` | `bool?` | Logo metadata/value within this model. |
| `web.results[].organization.contact_points[].thumbnail.duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `web.results[].organization.contact_points[].thumbnail.theme` | `string?` | Theme metadata/value within this model. |
| `web.results[].organization.contact_points[].telephone` | `string?` | Telephone metadata/value within this model. |
| `web.results[].organization.contact_points[].email` | `string?` | Email metadata/value within this model. |
| `web.results[].content_type` | `string?` | Content type metadata/value within this model. |
| `web.results[].extra_snippets` | `string[]?` | Alternative page excerpts, not continuous full text. |
| `web.results[].icons` | `object[]?` | Icons collection. |
| `web.results[].icons[].href` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `web.results[].icons[].sizes` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `web.results[].icons[].rel` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `web.results[].icons[].type` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `web.results[].icons[].ext` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `web.family_friendly` | `bool?` | Vendor safety classification, not a content-safety guarantee. |
| `summarizer` | `object?` | Summarizer metadata/value within this model. |
| `summarizer.type` | `string?` | Documented discriminator `summarizer` identifies this enclosing model; unknown future tags remain accepted. |
| `summarizer.key` | `string` | Key metadata/value within this model. |
| `rich` | `object?` | Rich metadata/value within this model. |
| `rich.type` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `rich.hint` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `rich.hint.vertical` | `string` | Vertical metadata/value within this model. |
| `rich.hint.callback_key` | `string` | Opaque key for explicit fixed-endpoint Rich Search continuation. |
