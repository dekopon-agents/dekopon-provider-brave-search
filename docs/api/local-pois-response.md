# Local POIs response field catalog

All fetched **200** reference paths are listed below; arrays use `[]`. A `?` records vendor optional notation, not proof of when a field is sent. The schema conservatively accepts null for these fields; it does not assert every optional field is actually nullable. Required-looking fields are not required by the permissive response schema. Unknown fields survive bounded raw projection.

See [response semantics](../response-semantics.md), [endpoint](./local-pois.md), and [upstream schema](../../schemas/upstream/local-pois.response.schema.json). Unspecified rich-field semantics are explicitly unknown, not inferred.

| Field path | Vendor type | Meaning / interpretation |
|---|---|---|
| `type` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results` | `object[]?` | Results collection. |
| `results[].title` | `string` | Title metadata/value within this model. |
| `results[].url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `results[].is_source_local` | `bool?` | Is source local metadata/value within this model. |
| `results[].is_source_both` | `bool?` | Is source both metadata/value within this model. |
| `results[].description` | `string?` | Display description/excerpt; POI Descriptions text is generated and Context source description is query-independent. |
| `results[].page_age` | `string?` | Content publication/modification date, not necessarily crawl time. |
| `results[].page_fetched` | `string?` | Last-fetch date/time as provided by the vendor. |
| `results[].fetched_content_timestamp` | `int?` | Fetch timestamp; do not infer units beyond the source contract. |
| `results[].profile` | `object?` | Profile metadata/value within this model. |
| `results[].profile.name` | `string` | Name metadata/value within this model. |
| `results[].profile.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `results[].profile.long_name` | `string?` | Long name metadata/value within this model. |
| `results[].profile.img` | `string?` | Img metadata/value within this model. |
| `results[].language` | `string?` | Language metadata/value within this model. |
| `results[].family_friendly` | `bool?` | Vendor safety classification, not a content-safety guarantee. |
| `results[].type` | `string?` | Documented discriminator `location_result` identifies this enclosing model; unknown future tags remain accepted. |
| `results[].provider_url` | `string` | Attribution/provider destination URL. |
| `results[].coordinates` | `array?` | Latitude/longitude pair as described by the location model; no geometry guarantee for unrelated arrays. |
| `results[].zoom_level` | `int?` | Zoom level metadata/value within this model. |
| `results[].thumbnail` | `object?` | Thumbnail metadata/value within this model. |
| `results[].thumbnail.src` | `string` | Src metadata/value within this model. |
| `results[].thumbnail.alt` | `string?` | Alt metadata/value within this model. |
| `results[].thumbnail.height` | `int?` | Height metadata/value within this model. |
| `results[].thumbnail.width` | `int?` | Width metadata/value within this model. |
| `results[].thumbnail.bg_color` | `string?` | Bg color metadata/value within this model. |
| `results[].thumbnail.original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `results[].thumbnail.logo` | `bool?` | Logo metadata/value within this model. |
| `results[].thumbnail.duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `results[].thumbnail.theme` | `string?` | Theme metadata/value within this model. |
| `results[].postal_address` | `object?` | Postal address metadata/value within this model. |
| `results[].postal_address.type` | `string?` | Documented discriminator `PostalAddress` identifies this enclosing model; unknown future tags remain accepted. |
| `results[].postal_address.country` | `string?` | Country metadata/value within this model. |
| `results[].postal_address.postalCode` | `string?` | Postal code metadata/value within this model. |
| `results[].postal_address.streetAddress` | `string?` | Street address metadata/value within this model. |
| `results[].postal_address.addressRegion` | `string?` | Address region metadata/value within this model. |
| `results[].postal_address.addressLocality` | `string?` | Address locality metadata/value within this model. |
| `results[].postal_address.displayAddress` | `string` | Display address metadata/value within this model. |
| `results[].opening_hours` | `object?` | Business hours; days is a nested array and a day may contain multiple intervals. |
| `results[].opening_hours.current_day` | `object[]?` | Current day collection. |
| `results[].opening_hours.current_day[].abbr_name` | `string` | Abbr name metadata/value within this model. |
| `results[].opening_hours.current_day[].full_name` | `string` | Full name metadata/value within this model. |
| `results[].opening_hours.current_day[].opens` | `string` | Local 24-hour opening-time string. |
| `results[].opening_hours.current_day[].closes` | `string` | Local 24-hour closing-time string. |
| `results[].opening_hours.days` | `object[][]?` | Days collection. |
| `results[].contact` | `object?` | Contact metadata/value within this model. |
| `results[].contact.email` | `string?` | Email metadata/value within this model. |
| `results[].contact.telephone` | `string?` | Telephone metadata/value within this model. |
| `results[].price_range` | `string?` | Price range metadata/value within this model. |
| `results[].rating` | `object?` | Rating metadata/value within this model. |
| `results[].rating.ratingValue` | `number` | Source-specific rating value; compare only with that source and scale. |
| `results[].rating.bestRating` | `number` | Source rating scale metadata; not global relevance. |
| `results[].rating.reviewCount` | `int?` | Number of reviews reported by source. |
| `results[].rating.profile` | `object?` | Profile metadata/value within this model. |
| `results[].rating.profile.name` | `string` | Name metadata/value within this model. |
| `results[].rating.profile.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `results[].rating.profile.long_name` | `string?` | Long name metadata/value within this model. |
| `results[].rating.profile.img` | `string?` | Img metadata/value within this model. |
| `results[].rating.is_tripadvisor` | `bool?` | Attribution flag for the rating source. |
| `results[].distance` | `object?` | Distance metadata/value within this model. |
| `results[].distance.value` | `number` | Value metadata/value within this model. |
| `results[].distance.units` | `string` | Units metadata/value within this model. |
| `results[].profiles` | `object[]?` | Profiles collection. |
| `results[].profiles[].type` | `string?` | Source-origin type; external normally identifies an external data source, but is not a closed enum. |
| `results[].profiles[].name` | `string` | Name metadata/value within this model. |
| `results[].profiles[].url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `results[].profiles[].long_name` | `string?` | Long name metadata/value within this model. |
| `results[].profiles[].img` | `string?` | Img metadata/value within this model. |
| `results[].reviews` | `object?` | Reviews metadata/value within this model. |
| `results[].reviews.results` | `object[]` | Results collection. |
| `results[].reviews.results[].title` | `string` | Title metadata/value within this model. |
| `results[].reviews.results[].description` | `string` | Display description/excerpt; POI Descriptions text is generated and Context source description is query-independent. |
| `results[].reviews.results[].date` | `string` | Date metadata/value within this model. |
| `results[].reviews.results[].rating` | `object` | Rating metadata/value within this model. |
| `results[].reviews.results[].rating.ratingValue` | `number` | Source-specific rating value; compare only with that source and scale. |
| `results[].reviews.results[].rating.bestRating` | `number` | Source rating scale metadata; not global relevance. |
| `results[].reviews.results[].rating.reviewCount` | `int?` | Number of reviews reported by source. |
| `results[].reviews.results[].rating.profile` | `object?` | Profile metadata/value within this model. |
| `results[].reviews.results[].rating.profile.name` | `string` | Name metadata/value within this model. |
| `results[].reviews.results[].rating.profile.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `results[].reviews.results[].rating.profile.long_name` | `string?` | Long name metadata/value within this model. |
| `results[].reviews.results[].rating.profile.img` | `string?` | Img metadata/value within this model. |
| `results[].reviews.results[].rating.is_tripadvisor` | `bool?` | Attribution flag for the rating source. |
| `results[].reviews.results[].author` | `object` | Author metadata/value within this model. |
| `results[].reviews.results[].author.type` | `string?` | Documented discriminator `person` identifies this enclosing model; unknown future tags remain accepted. |
| `results[].reviews.results[].author.name` | `string` | Name metadata/value within this model. |
| `results[].reviews.results[].author.url` | `string?` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `results[].reviews.results[].author.thumbnail` | `object?` | Thumbnail metadata/value within this model. |
| `results[].reviews.results[].author.thumbnail.src` | `string` | Src metadata/value within this model. |
| `results[].reviews.results[].author.thumbnail.alt` | `string?` | Alt metadata/value within this model. |
| `results[].reviews.results[].author.thumbnail.height` | `int?` | Height metadata/value within this model. |
| `results[].reviews.results[].author.thumbnail.width` | `int?` | Width metadata/value within this model. |
| `results[].reviews.results[].author.thumbnail.bg_color` | `string?` | Bg color metadata/value within this model. |
| `results[].reviews.results[].author.thumbnail.original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `results[].reviews.results[].author.thumbnail.logo` | `bool?` | Logo metadata/value within this model. |
| `results[].reviews.results[].author.thumbnail.duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `results[].reviews.results[].author.thumbnail.theme` | `string?` | Theme metadata/value within this model. |
| `results[].reviews.results[].author.email` | `string?` | Email metadata/value within this model. |
| `results[].reviews.results[].review_url` | `string` | Review url metadata/value within this model. |
| `results[].reviews.results[].language` | `string` | Language metadata/value within this model. |
| `results[].reviews.viewMoreUrl` | `string` | View more url metadata/value within this model. |
| `results[].reviews.reviews_in_foreign_language` | `bool` | Reviews in foreign language metadata/value within this model. |
| `results[].pictures` | `object?` | Pictures metadata/value within this model. |
| `results[].pictures.viewMoreUrl` | `string?` | View more url metadata/value within this model. |
| `results[].pictures.results` | `object[]` | Results collection. |
| `results[].pictures.results[].src` | `string` | Src metadata/value within this model. |
| `results[].pictures.results[].alt` | `string?` | Alt metadata/value within this model. |
| `results[].pictures.results[].height` | `int?` | Height metadata/value within this model. |
| `results[].pictures.results[].width` | `int?` | Width metadata/value within this model. |
| `results[].pictures.results[].bg_color` | `string?` | Bg color metadata/value within this model. |
| `results[].pictures.results[].original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `results[].pictures.results[].logo` | `bool?` | Logo metadata/value within this model. |
| `results[].pictures.results[].duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `results[].pictures.results[].theme` | `string?` | Theme metadata/value within this model. |
| `results[].action` | `object?` | Action metadata/value within this model. |
| `results[].action.type` | `string` | Type metadata/value within this model. |
| `results[].action.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `results[].serves_cuisine` | `string[]?` | Serves cuisine collection. |
| `results[].categories` | `string[]?` | Categories collection. |
| `results[].icon_category` | `string?` | Icon category metadata/value within this model. |
| `results[].timezone` | `string?` | Timezone metadata/value within this model. |
| `results[].timezone_offset` | `int?` | Timezone offset metadata/value within this model. |
| `results[].id` | `string?` | Identifier scoped to the enclosing result; local-place IDs expire in about eight hours. |
| `results[].results` | `object[]?` | Results collection. |
| `results[].results[].title` | `string` | Title metadata/value within this model. |
| `results[].results[].url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `results[].results[].is_source_local` | `bool?` | Is source local metadata/value within this model. |
| `results[].results[].is_source_both` | `bool?` | Is source both metadata/value within this model. |
| `results[].results[].description` | `string?` | Display description/excerpt; POI Descriptions text is generated and Context source description is query-independent. |
| `results[].results[].page_age` | `string?` | Content publication/modification date, not necessarily crawl time. |
| `results[].results[].page_fetched` | `string?` | Last-fetch date/time as provided by the vendor. |
| `results[].results[].fetched_content_timestamp` | `int?` | Fetch timestamp; do not infer units beyond the source contract. |
| `results[].results[].profile` | `object?` | Profile metadata/value within this model. |
| `results[].results[].profile.name` | `string` | Name metadata/value within this model. |
| `results[].results[].profile.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `results[].results[].profile.long_name` | `string?` | Long name metadata/value within this model. |
| `results[].results[].profile.img` | `string?` | Img metadata/value within this model. |
| `results[].results[].language` | `string?` | Language metadata/value within this model. |
| `results[].results[].family_friendly` | `bool?` | Vendor safety classification, not a content-safety guarantee. |
| `results[].results[].meta_url` | `object` | Meta url metadata/value within this model. |
| `results[].results[].meta_url.scheme` | `string` | Scheme metadata/value within this model. |
| `results[].results[].meta_url.netloc` | `string` | Netloc metadata/value within this model. |
| `results[].results[].meta_url.hostname` | `string?` | Hostname metadata/value within this model. |
| `results[].results[].meta_url.favicon` | `string` | Favicon metadata/value within this model. |
| `results[].results[].meta_url.path` | `string` | Path metadata/value within this model. |
