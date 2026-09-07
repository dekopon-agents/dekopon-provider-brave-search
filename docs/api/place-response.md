# Place Search response field catalog

All fetched **200** reference paths are listed below; arrays use `[]`. A `?` records vendor optional notation, not proof of when a field is sent. The schema conservatively accepts null for these fields; it does not assert every optional field is actually nullable. Required-looking fields are not required by the permissive response schema. Unknown fields survive bounded raw projection.

See [response semantics](../response-semantics.md), [endpoint](./place.md), and [upstream schema](../../schemas/upstream/place.response.schema.json). Unspecified rich-field semantics are explicitly unknown, not inferred.

| Field path | Vendor type | Meaning / interpretation |
|---|---|---|
| `type` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `query` | `object?` | Query metadata object, or corrected/completed phrase on suggestion/spellcheck items. |
| `query.original` | `string` | Original user phrase (or original asset URL within a thumbnail). |
| `query.altered` | `string?` | Spelling-corrected phrase actually used upstream. |
| `query.spellcheck_off` | `bool?` | Spellcheck off metadata/value within this model. |
| `query.show_strict_warning` | `bool?` | Indicates results were reduced by strict safety filtering. |
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
| `cities` | `object[]?` | Cities collection. |
| `cities[].type` | `string?` | Documented discriminator `city` identifies this enclosing model; unknown future tags remain accepted. |
| `cities[].name` | `string` | Name metadata/value within this model. |
| `cities[].country` | `string` | Country metadata/value within this model. |
| `cities[].coordinates` | `array` | Latitude/longitude pair as described by the location model; no geometry guarantee for unrelated arrays. |
| `cities[].thumbnail` | `object` | Thumbnail metadata/value within this model. |
| `cities[].thumbnail.src` | `string` | Src metadata/value within this model. |
| `cities[].thumbnail.alt` | `string?` | Alt metadata/value within this model. |
| `cities[].thumbnail.height` | `int?` | Height metadata/value within this model. |
| `cities[].thumbnail.width` | `int?` | Width metadata/value within this model. |
| `cities[].thumbnail.bg_color` | `string?` | Bg color metadata/value within this model. |
| `cities[].thumbnail.original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `cities[].thumbnail.logo` | `bool?` | Logo metadata/value within this model. |
| `cities[].thumbnail.duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `cities[].thumbnail.theme` | `string?` | Theme metadata/value within this model. |
| `countries` | `object[]?` | Countries collection. |
| `countries[].type` | `string?` | Documented discriminator `country` identifies this enclosing model; unknown future tags remain accepted. |
| `countries[].name` | `string` | Name metadata/value within this model. |
| `countries[].country` | `string` | Country metadata/value within this model. |
| `countries[].coordinates` | `array` | Latitude/longitude pair as described by the location model; no geometry guarantee for unrelated arrays. |
| `countries[].thumbnail` | `object` | Thumbnail metadata/value within this model. |
| `countries[].thumbnail.src` | `string` | Src metadata/value within this model. |
| `countries[].thumbnail.alt` | `string?` | Alt metadata/value within this model. |
| `countries[].thumbnail.height` | `int?` | Height metadata/value within this model. |
| `countries[].thumbnail.width` | `int?` | Width metadata/value within this model. |
| `countries[].thumbnail.bg_color` | `string?` | Bg color metadata/value within this model. |
| `countries[].thumbnail.original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `countries[].thumbnail.logo` | `bool?` | Logo metadata/value within this model. |
| `countries[].thumbnail.duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `countries[].thumbnail.theme` | `string?` | Theme metadata/value within this model. |
| `regions` | `object[]?` | Regions collection. |
| `regions[].type` | `string?` | Documented discriminator `region` identifies this enclosing model; unknown future tags remain accepted. |
| `regions[].name` | `string` | Name metadata/value within this model. |
| `regions[].country` | `string` | Country metadata/value within this model. |
| `regions[].coordinates` | `array` | Latitude/longitude pair as described by the location model; no geometry guarantee for unrelated arrays. |
| `regions[].thumbnail` | `object` | Thumbnail metadata/value within this model. |
| `regions[].thumbnail.src` | `string` | Src metadata/value within this model. |
| `regions[].thumbnail.alt` | `string?` | Alt metadata/value within this model. |
| `regions[].thumbnail.height` | `int?` | Height metadata/value within this model. |
| `regions[].thumbnail.width` | `int?` | Width metadata/value within this model. |
| `regions[].thumbnail.bg_color` | `string?` | Bg color metadata/value within this model. |
| `regions[].thumbnail.original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `regions[].thumbnail.logo` | `bool?` | Logo metadata/value within this model. |
| `regions[].thumbnail.duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `regions[].thumbnail.theme` | `string?` | Theme metadata/value within this model. |
| `neighborhoods` | `object[]?` | Neighborhoods collection. |
| `neighborhoods[].type` | `string?` | Documented discriminator `neighborhood` identifies this enclosing model; unknown future tags remain accepted. |
| `neighborhoods[].name` | `string` | Name metadata/value within this model. |
| `neighborhoods[].country` | `string` | Country metadata/value within this model. |
| `neighborhoods[].coordinates` | `array` | Latitude/longitude pair as described by the location model; no geometry guarantee for unrelated arrays. |
| `neighborhoods[].thumbnail` | `object` | Thumbnail metadata/value within this model. |
| `neighborhoods[].thumbnail.src` | `string` | Src metadata/value within this model. |
| `neighborhoods[].thumbnail.alt` | `string?` | Alt metadata/value within this model. |
| `neighborhoods[].thumbnail.height` | `int?` | Height metadata/value within this model. |
| `neighborhoods[].thumbnail.width` | `int?` | Width metadata/value within this model. |
| `neighborhoods[].thumbnail.bg_color` | `string?` | Bg color metadata/value within this model. |
| `neighborhoods[].thumbnail.original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `neighborhoods[].thumbnail.logo` | `bool?` | Logo metadata/value within this model. |
| `neighborhoods[].thumbnail.duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `neighborhoods[].thumbnail.theme` | `string?` | Theme metadata/value within this model. |
| `addresses` | `object[]?` | Addresses collection. |
| `addresses[].type` | `string?` | address means a specific street plus number; street means the street without a specific numbered address. |
| `addresses[].name` | `string` | Name metadata/value within this model. |
| `addresses[].coordinates` | `array` | Latitude/longitude pair as described by the location model; no geometry guarantee for unrelated arrays. |
| `addresses[].pois` | `object[]?` | Pois collection. |
| `addresses[].pois[].title` | `string` | Title metadata/value within this model. |
| `addresses[].pois[].url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `addresses[].pois[].is_source_local` | `bool?` | Is source local metadata/value within this model. |
| `addresses[].pois[].is_source_both` | `bool?` | Is source both metadata/value within this model. |
| `addresses[].pois[].description` | `string?` | Display description/excerpt; POI Descriptions text is generated and Context source description is query-independent. |
| `addresses[].pois[].page_age` | `string?` | Content publication/modification date, not necessarily crawl time. |
| `addresses[].pois[].page_fetched` | `string?` | Last-fetch date/time as provided by the vendor. |
| `addresses[].pois[].fetched_content_timestamp` | `int?` | Fetch timestamp; do not infer units beyond the source contract. |
| `addresses[].pois[].profile` | `object?` | Profile metadata/value within this model. |
| `addresses[].pois[].profile.name` | `string` | Name metadata/value within this model. |
| `addresses[].pois[].profile.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `addresses[].pois[].profile.long_name` | `string?` | Long name metadata/value within this model. |
| `addresses[].pois[].profile.img` | `string?` | Img metadata/value within this model. |
| `addresses[].pois[].language` | `string?` | Language metadata/value within this model. |
| `addresses[].pois[].family_friendly` | `bool?` | Vendor safety classification, not a content-safety guarantee. |
| `addresses[].pois[].type` | `string?` | Documented discriminator `location_result` identifies this enclosing model; unknown future tags remain accepted. |
| `addresses[].pois[].provider_url` | `string` | Attribution/provider destination URL. |
| `addresses[].pois[].coordinates` | `array?` | Latitude/longitude pair as described by the location model; no geometry guarantee for unrelated arrays. |
| `addresses[].pois[].zoom_level` | `int?` | Zoom level metadata/value within this model. |
| `addresses[].pois[].thumbnail` | `object?` | Thumbnail metadata/value within this model. |
| `addresses[].pois[].thumbnail.src` | `string` | Src metadata/value within this model. |
| `addresses[].pois[].thumbnail.alt` | `string?` | Alt metadata/value within this model. |
| `addresses[].pois[].thumbnail.height` | `int?` | Height metadata/value within this model. |
| `addresses[].pois[].thumbnail.width` | `int?` | Width metadata/value within this model. |
| `addresses[].pois[].thumbnail.bg_color` | `string?` | Bg color metadata/value within this model. |
| `addresses[].pois[].thumbnail.original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `addresses[].pois[].thumbnail.logo` | `bool?` | Logo metadata/value within this model. |
| `addresses[].pois[].thumbnail.duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `addresses[].pois[].thumbnail.theme` | `string?` | Theme metadata/value within this model. |
| `addresses[].pois[].postal_address` | `object?` | Postal address metadata/value within this model. |
| `addresses[].pois[].postal_address.type` | `string?` | Documented discriminator `PostalAddress` identifies this enclosing model; unknown future tags remain accepted. |
| `addresses[].pois[].postal_address.country` | `string?` | Country metadata/value within this model. |
| `addresses[].pois[].postal_address.postalCode` | `string?` | Postal code metadata/value within this model. |
| `addresses[].pois[].postal_address.streetAddress` | `string?` | Street address metadata/value within this model. |
| `addresses[].pois[].postal_address.addressRegion` | `string?` | Address region metadata/value within this model. |
| `addresses[].pois[].postal_address.addressLocality` | `string?` | Address locality metadata/value within this model. |
| `addresses[].pois[].postal_address.displayAddress` | `string` | Display address metadata/value within this model. |
| `addresses[].pois[].opening_hours` | `object?` | Business hours; days is a nested array and a day may contain multiple intervals. |
| `addresses[].pois[].opening_hours.current_day` | `object[]?` | Current day collection. |
| `addresses[].pois[].opening_hours.current_day[].abbr_name` | `string` | Abbr name metadata/value within this model. |
| `addresses[].pois[].opening_hours.current_day[].full_name` | `string` | Full name metadata/value within this model. |
| `addresses[].pois[].opening_hours.current_day[].opens` | `string` | Local 24-hour opening-time string. |
| `addresses[].pois[].opening_hours.current_day[].closes` | `string` | Local 24-hour closing-time string. |
| `addresses[].pois[].opening_hours.days` | `object[][]?` | Days collection. |
| `addresses[].pois[].contact` | `object?` | Contact metadata/value within this model. |
| `addresses[].pois[].contact.email` | `string?` | Email metadata/value within this model. |
| `addresses[].pois[].contact.telephone` | `string?` | Telephone metadata/value within this model. |
| `addresses[].pois[].price_range` | `string?` | Price range metadata/value within this model. |
| `addresses[].pois[].rating` | `object?` | Rating metadata/value within this model. |
| `addresses[].pois[].rating.ratingValue` | `number` | Source-specific rating value; compare only with that source and scale. |
| `addresses[].pois[].rating.bestRating` | `number` | Source rating scale metadata; not global relevance. |
| `addresses[].pois[].rating.reviewCount` | `int?` | Number of reviews reported by source. |
| `addresses[].pois[].rating.profile` | `object?` | Profile metadata/value within this model. |
| `addresses[].pois[].rating.profile.name` | `string` | Name metadata/value within this model. |
| `addresses[].pois[].rating.profile.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `addresses[].pois[].rating.profile.long_name` | `string?` | Long name metadata/value within this model. |
| `addresses[].pois[].rating.profile.img` | `string?` | Img metadata/value within this model. |
| `addresses[].pois[].rating.is_tripadvisor` | `bool?` | Attribution flag for the rating source. |
| `addresses[].pois[].distance` | `object?` | Distance metadata/value within this model. |
| `addresses[].pois[].distance.value` | `number` | Value metadata/value within this model. |
| `addresses[].pois[].distance.units` | `string` | Units metadata/value within this model. |
| `addresses[].pois[].profiles` | `object[]?` | Profiles collection. |
| `addresses[].pois[].profiles[].type` | `string?` | Source-origin type; external normally identifies an external data source, but is not a closed enum. |
| `addresses[].pois[].profiles[].name` | `string` | Name metadata/value within this model. |
| `addresses[].pois[].profiles[].url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `addresses[].pois[].profiles[].long_name` | `string?` | Long name metadata/value within this model. |
| `addresses[].pois[].profiles[].img` | `string?` | Img metadata/value within this model. |
| `addresses[].pois[].reviews` | `object?` | Reviews metadata/value within this model. |
| `addresses[].pois[].reviews.results` | `object[]` | Results collection. |
| `addresses[].pois[].reviews.results[].title` | `string` | Title metadata/value within this model. |
| `addresses[].pois[].reviews.results[].description` | `string` | Display description/excerpt; POI Descriptions text is generated and Context source description is query-independent. |
| `addresses[].pois[].reviews.results[].date` | `string` | Date metadata/value within this model. |
| `addresses[].pois[].reviews.results[].rating` | `object` | Rating metadata/value within this model. |
| `addresses[].pois[].reviews.results[].rating.ratingValue` | `number` | Source-specific rating value; compare only with that source and scale. |
| `addresses[].pois[].reviews.results[].rating.bestRating` | `number` | Source rating scale metadata; not global relevance. |
| `addresses[].pois[].reviews.results[].rating.reviewCount` | `int?` | Number of reviews reported by source. |
| `addresses[].pois[].reviews.results[].rating.profile` | `object?` | Profile metadata/value within this model. |
| `addresses[].pois[].reviews.results[].rating.profile.name` | `string` | Name metadata/value within this model. |
| `addresses[].pois[].reviews.results[].rating.profile.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `addresses[].pois[].reviews.results[].rating.profile.long_name` | `string?` | Long name metadata/value within this model. |
| `addresses[].pois[].reviews.results[].rating.profile.img` | `string?` | Img metadata/value within this model. |
| `addresses[].pois[].reviews.results[].rating.is_tripadvisor` | `bool?` | Attribution flag for the rating source. |
| `addresses[].pois[].reviews.results[].author` | `object` | Author metadata/value within this model. |
| `addresses[].pois[].reviews.results[].author.type` | `string?` | Documented discriminator `person` identifies this enclosing model; unknown future tags remain accepted. |
| `addresses[].pois[].reviews.results[].author.name` | `string` | Name metadata/value within this model. |
| `addresses[].pois[].reviews.results[].author.url` | `string?` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `addresses[].pois[].reviews.results[].author.thumbnail` | `object?` | Thumbnail metadata/value within this model. |
| `addresses[].pois[].reviews.results[].author.thumbnail.src` | `string` | Src metadata/value within this model. |
| `addresses[].pois[].reviews.results[].author.thumbnail.alt` | `string?` | Alt metadata/value within this model. |
| `addresses[].pois[].reviews.results[].author.thumbnail.height` | `int?` | Height metadata/value within this model. |
| `addresses[].pois[].reviews.results[].author.thumbnail.width` | `int?` | Width metadata/value within this model. |
| `addresses[].pois[].reviews.results[].author.thumbnail.bg_color` | `string?` | Bg color metadata/value within this model. |
| `addresses[].pois[].reviews.results[].author.thumbnail.original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `addresses[].pois[].reviews.results[].author.thumbnail.logo` | `bool?` | Logo metadata/value within this model. |
| `addresses[].pois[].reviews.results[].author.thumbnail.duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `addresses[].pois[].reviews.results[].author.thumbnail.theme` | `string?` | Theme metadata/value within this model. |
| `addresses[].pois[].reviews.results[].author.email` | `string?` | Email metadata/value within this model. |
| `addresses[].pois[].reviews.results[].review_url` | `string` | Review url metadata/value within this model. |
| `addresses[].pois[].reviews.results[].language` | `string` | Language metadata/value within this model. |
| `addresses[].pois[].reviews.viewMoreUrl` | `string` | View more url metadata/value within this model. |
| `addresses[].pois[].reviews.reviews_in_foreign_language` | `bool` | Reviews in foreign language metadata/value within this model. |
| `addresses[].pois[].pictures` | `object?` | Pictures metadata/value within this model. |
| `addresses[].pois[].pictures.viewMoreUrl` | `string?` | View more url metadata/value within this model. |
| `addresses[].pois[].pictures.results` | `object[]` | Results collection. |
| `addresses[].pois[].pictures.results[].src` | `string` | Src metadata/value within this model. |
| `addresses[].pois[].pictures.results[].alt` | `string?` | Alt metadata/value within this model. |
| `addresses[].pois[].pictures.results[].height` | `int?` | Height metadata/value within this model. |
| `addresses[].pois[].pictures.results[].width` | `int?` | Width metadata/value within this model. |
| `addresses[].pois[].pictures.results[].bg_color` | `string?` | Bg color metadata/value within this model. |
| `addresses[].pois[].pictures.results[].original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `addresses[].pois[].pictures.results[].logo` | `bool?` | Logo metadata/value within this model. |
| `addresses[].pois[].pictures.results[].duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `addresses[].pois[].pictures.results[].theme` | `string?` | Theme metadata/value within this model. |
| `addresses[].pois[].action` | `object?` | Action metadata/value within this model. |
| `addresses[].pois[].action.type` | `string` | Type metadata/value within this model. |
| `addresses[].pois[].action.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `addresses[].pois[].serves_cuisine` | `string[]?` | Serves cuisine collection. |
| `addresses[].pois[].categories` | `string[]?` | Categories collection. |
| `addresses[].pois[].icon_category` | `string?` | Icon category metadata/value within this model. |
| `addresses[].pois[].timezone` | `string?` | Timezone metadata/value within this model. |
| `addresses[].pois[].timezone_offset` | `int?` | Timezone offset metadata/value within this model. |
| `addresses[].pois[].id` | `string?` | Identifier scoped to the enclosing result; local-place IDs expire in about eight hours. |
| `addresses[].pois[].results` | `object[]?` | Results collection. |
| `addresses[].pois[].results[].title` | `string` | Title metadata/value within this model. |
| `addresses[].pois[].results[].url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `addresses[].pois[].results[].is_source_local` | `bool?` | Is source local metadata/value within this model. |
| `addresses[].pois[].results[].is_source_both` | `bool?` | Is source both metadata/value within this model. |
| `addresses[].pois[].results[].description` | `string?` | Display description/excerpt; POI Descriptions text is generated and Context source description is query-independent. |
| `addresses[].pois[].results[].page_age` | `string?` | Content publication/modification date, not necessarily crawl time. |
| `addresses[].pois[].results[].page_fetched` | `string?` | Last-fetch date/time as provided by the vendor. |
| `addresses[].pois[].results[].fetched_content_timestamp` | `int?` | Fetch timestamp; do not infer units beyond the source contract. |
| `addresses[].pois[].results[].profile` | `object?` | Profile metadata/value within this model. |
| `addresses[].pois[].results[].profile.name` | `string` | Name metadata/value within this model. |
| `addresses[].pois[].results[].profile.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `addresses[].pois[].results[].profile.long_name` | `string?` | Long name metadata/value within this model. |
| `addresses[].pois[].results[].profile.img` | `string?` | Img metadata/value within this model. |
| `addresses[].pois[].results[].language` | `string?` | Language metadata/value within this model. |
| `addresses[].pois[].results[].family_friendly` | `bool?` | Vendor safety classification, not a content-safety guarantee. |
| `addresses[].pois[].results[].meta_url` | `object` | Meta url metadata/value within this model. |
| `addresses[].pois[].results[].meta_url.scheme` | `string` | Scheme metadata/value within this model. |
| `addresses[].pois[].results[].meta_url.netloc` | `string` | Netloc metadata/value within this model. |
| `addresses[].pois[].results[].meta_url.hostname` | `string?` | Hostname metadata/value within this model. |
| `addresses[].pois[].results[].meta_url.favicon` | `string` | Favicon metadata/value within this model. |
| `addresses[].pois[].results[].meta_url.path` | `string` | Path metadata/value within this model. |
| `addresses[].pois_nearby` | `object[]?` | Pois nearby collection. |
| `addresses[].pois_nearby[].title` | `string` | Title metadata/value within this model. |
| `addresses[].pois_nearby[].url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `addresses[].pois_nearby[].is_source_local` | `bool?` | Is source local metadata/value within this model. |
| `addresses[].pois_nearby[].is_source_both` | `bool?` | Is source both metadata/value within this model. |
| `addresses[].pois_nearby[].description` | `string?` | Display description/excerpt; POI Descriptions text is generated and Context source description is query-independent. |
| `addresses[].pois_nearby[].page_age` | `string?` | Content publication/modification date, not necessarily crawl time. |
| `addresses[].pois_nearby[].page_fetched` | `string?` | Last-fetch date/time as provided by the vendor. |
| `addresses[].pois_nearby[].fetched_content_timestamp` | `int?` | Fetch timestamp; do not infer units beyond the source contract. |
| `addresses[].pois_nearby[].profile` | `object?` | Profile metadata/value within this model. |
| `addresses[].pois_nearby[].profile.name` | `string` | Name metadata/value within this model. |
| `addresses[].pois_nearby[].profile.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `addresses[].pois_nearby[].profile.long_name` | `string?` | Long name metadata/value within this model. |
| `addresses[].pois_nearby[].profile.img` | `string?` | Img metadata/value within this model. |
| `addresses[].pois_nearby[].language` | `string?` | Language metadata/value within this model. |
| `addresses[].pois_nearby[].family_friendly` | `bool?` | Vendor safety classification, not a content-safety guarantee. |
| `addresses[].pois_nearby[].type` | `string?` | Documented discriminator `location_result` identifies this enclosing model; unknown future tags remain accepted. |
| `addresses[].pois_nearby[].provider_url` | `string` | Attribution/provider destination URL. |
| `addresses[].pois_nearby[].coordinates` | `array?` | Latitude/longitude pair as described by the location model; no geometry guarantee for unrelated arrays. |
| `addresses[].pois_nearby[].zoom_level` | `int?` | Zoom level metadata/value within this model. |
| `addresses[].pois_nearby[].thumbnail` | `object?` | Thumbnail metadata/value within this model. |
| `addresses[].pois_nearby[].thumbnail.src` | `string` | Src metadata/value within this model. |
| `addresses[].pois_nearby[].thumbnail.alt` | `string?` | Alt metadata/value within this model. |
| `addresses[].pois_nearby[].thumbnail.height` | `int?` | Height metadata/value within this model. |
| `addresses[].pois_nearby[].thumbnail.width` | `int?` | Width metadata/value within this model. |
| `addresses[].pois_nearby[].thumbnail.bg_color` | `string?` | Bg color metadata/value within this model. |
| `addresses[].pois_nearby[].thumbnail.original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `addresses[].pois_nearby[].thumbnail.logo` | `bool?` | Logo metadata/value within this model. |
| `addresses[].pois_nearby[].thumbnail.duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `addresses[].pois_nearby[].thumbnail.theme` | `string?` | Theme metadata/value within this model. |
| `addresses[].pois_nearby[].postal_address` | `object?` | Postal address metadata/value within this model. |
| `addresses[].pois_nearby[].postal_address.type` | `string?` | Documented discriminator `PostalAddress` identifies this enclosing model; unknown future tags remain accepted. |
| `addresses[].pois_nearby[].postal_address.country` | `string?` | Country metadata/value within this model. |
| `addresses[].pois_nearby[].postal_address.postalCode` | `string?` | Postal code metadata/value within this model. |
| `addresses[].pois_nearby[].postal_address.streetAddress` | `string?` | Street address metadata/value within this model. |
| `addresses[].pois_nearby[].postal_address.addressRegion` | `string?` | Address region metadata/value within this model. |
| `addresses[].pois_nearby[].postal_address.addressLocality` | `string?` | Address locality metadata/value within this model. |
| `addresses[].pois_nearby[].postal_address.displayAddress` | `string` | Display address metadata/value within this model. |
| `addresses[].pois_nearby[].opening_hours` | `object?` | Business hours; days is a nested array and a day may contain multiple intervals. |
| `addresses[].pois_nearby[].opening_hours.current_day` | `object[]?` | Current day collection. |
| `addresses[].pois_nearby[].opening_hours.current_day[].abbr_name` | `string` | Abbr name metadata/value within this model. |
| `addresses[].pois_nearby[].opening_hours.current_day[].full_name` | `string` | Full name metadata/value within this model. |
| `addresses[].pois_nearby[].opening_hours.current_day[].opens` | `string` | Local 24-hour opening-time string. |
| `addresses[].pois_nearby[].opening_hours.current_day[].closes` | `string` | Local 24-hour closing-time string. |
| `addresses[].pois_nearby[].opening_hours.days` | `object[][]?` | Days collection. |
| `addresses[].pois_nearby[].contact` | `object?` | Contact metadata/value within this model. |
| `addresses[].pois_nearby[].contact.email` | `string?` | Email metadata/value within this model. |
| `addresses[].pois_nearby[].contact.telephone` | `string?` | Telephone metadata/value within this model. |
| `addresses[].pois_nearby[].price_range` | `string?` | Price range metadata/value within this model. |
| `addresses[].pois_nearby[].rating` | `object?` | Rating metadata/value within this model. |
| `addresses[].pois_nearby[].rating.ratingValue` | `number` | Source-specific rating value; compare only with that source and scale. |
| `addresses[].pois_nearby[].rating.bestRating` | `number` | Source rating scale metadata; not global relevance. |
| `addresses[].pois_nearby[].rating.reviewCount` | `int?` | Number of reviews reported by source. |
| `addresses[].pois_nearby[].rating.profile` | `object?` | Profile metadata/value within this model. |
| `addresses[].pois_nearby[].rating.profile.name` | `string` | Name metadata/value within this model. |
| `addresses[].pois_nearby[].rating.profile.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `addresses[].pois_nearby[].rating.profile.long_name` | `string?` | Long name metadata/value within this model. |
| `addresses[].pois_nearby[].rating.profile.img` | `string?` | Img metadata/value within this model. |
| `addresses[].pois_nearby[].rating.is_tripadvisor` | `bool?` | Attribution flag for the rating source. |
| `addresses[].pois_nearby[].distance` | `object?` | Distance metadata/value within this model. |
| `addresses[].pois_nearby[].distance.value` | `number` | Value metadata/value within this model. |
| `addresses[].pois_nearby[].distance.units` | `string` | Units metadata/value within this model. |
| `addresses[].pois_nearby[].profiles` | `object[]?` | Profiles collection. |
| `addresses[].pois_nearby[].profiles[].type` | `string?` | Source-origin type; external normally identifies an external data source, but is not a closed enum. |
| `addresses[].pois_nearby[].profiles[].name` | `string` | Name metadata/value within this model. |
| `addresses[].pois_nearby[].profiles[].url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `addresses[].pois_nearby[].profiles[].long_name` | `string?` | Long name metadata/value within this model. |
| `addresses[].pois_nearby[].profiles[].img` | `string?` | Img metadata/value within this model. |
| `addresses[].pois_nearby[].reviews` | `object?` | Reviews metadata/value within this model. |
| `addresses[].pois_nearby[].reviews.results` | `object[]` | Results collection. |
| `addresses[].pois_nearby[].reviews.results[].title` | `string` | Title metadata/value within this model. |
| `addresses[].pois_nearby[].reviews.results[].description` | `string` | Display description/excerpt; POI Descriptions text is generated and Context source description is query-independent. |
| `addresses[].pois_nearby[].reviews.results[].date` | `string` | Date metadata/value within this model. |
| `addresses[].pois_nearby[].reviews.results[].rating` | `object` | Rating metadata/value within this model. |
| `addresses[].pois_nearby[].reviews.results[].rating.ratingValue` | `number` | Source-specific rating value; compare only with that source and scale. |
| `addresses[].pois_nearby[].reviews.results[].rating.bestRating` | `number` | Source rating scale metadata; not global relevance. |
| `addresses[].pois_nearby[].reviews.results[].rating.reviewCount` | `int?` | Number of reviews reported by source. |
| `addresses[].pois_nearby[].reviews.results[].rating.profile` | `object?` | Profile metadata/value within this model. |
| `addresses[].pois_nearby[].reviews.results[].rating.profile.name` | `string` | Name metadata/value within this model. |
| `addresses[].pois_nearby[].reviews.results[].rating.profile.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `addresses[].pois_nearby[].reviews.results[].rating.profile.long_name` | `string?` | Long name metadata/value within this model. |
| `addresses[].pois_nearby[].reviews.results[].rating.profile.img` | `string?` | Img metadata/value within this model. |
| `addresses[].pois_nearby[].reviews.results[].rating.is_tripadvisor` | `bool?` | Attribution flag for the rating source. |
| `addresses[].pois_nearby[].reviews.results[].author` | `object` | Author metadata/value within this model. |
| `addresses[].pois_nearby[].reviews.results[].author.type` | `string?` | Documented discriminator `person` identifies this enclosing model; unknown future tags remain accepted. |
| `addresses[].pois_nearby[].reviews.results[].author.name` | `string` | Name metadata/value within this model. |
| `addresses[].pois_nearby[].reviews.results[].author.url` | `string?` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `addresses[].pois_nearby[].reviews.results[].author.thumbnail` | `object?` | Thumbnail metadata/value within this model. |
| `addresses[].pois_nearby[].reviews.results[].author.thumbnail.src` | `string` | Src metadata/value within this model. |
| `addresses[].pois_nearby[].reviews.results[].author.thumbnail.alt` | `string?` | Alt metadata/value within this model. |
| `addresses[].pois_nearby[].reviews.results[].author.thumbnail.height` | `int?` | Height metadata/value within this model. |
| `addresses[].pois_nearby[].reviews.results[].author.thumbnail.width` | `int?` | Width metadata/value within this model. |
| `addresses[].pois_nearby[].reviews.results[].author.thumbnail.bg_color` | `string?` | Bg color metadata/value within this model. |
| `addresses[].pois_nearby[].reviews.results[].author.thumbnail.original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `addresses[].pois_nearby[].reviews.results[].author.thumbnail.logo` | `bool?` | Logo metadata/value within this model. |
| `addresses[].pois_nearby[].reviews.results[].author.thumbnail.duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `addresses[].pois_nearby[].reviews.results[].author.thumbnail.theme` | `string?` | Theme metadata/value within this model. |
| `addresses[].pois_nearby[].reviews.results[].author.email` | `string?` | Email metadata/value within this model. |
| `addresses[].pois_nearby[].reviews.results[].review_url` | `string` | Review url metadata/value within this model. |
| `addresses[].pois_nearby[].reviews.results[].language` | `string` | Language metadata/value within this model. |
| `addresses[].pois_nearby[].reviews.viewMoreUrl` | `string` | View more url metadata/value within this model. |
| `addresses[].pois_nearby[].reviews.reviews_in_foreign_language` | `bool` | Reviews in foreign language metadata/value within this model. |
| `addresses[].pois_nearby[].pictures` | `object?` | Pictures metadata/value within this model. |
| `addresses[].pois_nearby[].pictures.viewMoreUrl` | `string?` | View more url metadata/value within this model. |
| `addresses[].pois_nearby[].pictures.results` | `object[]` | Results collection. |
| `addresses[].pois_nearby[].pictures.results[].src` | `string` | Src metadata/value within this model. |
| `addresses[].pois_nearby[].pictures.results[].alt` | `string?` | Alt metadata/value within this model. |
| `addresses[].pois_nearby[].pictures.results[].height` | `int?` | Height metadata/value within this model. |
| `addresses[].pois_nearby[].pictures.results[].width` | `int?` | Width metadata/value within this model. |
| `addresses[].pois_nearby[].pictures.results[].bg_color` | `string?` | Bg color metadata/value within this model. |
| `addresses[].pois_nearby[].pictures.results[].original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `addresses[].pois_nearby[].pictures.results[].logo` | `bool?` | Logo metadata/value within this model. |
| `addresses[].pois_nearby[].pictures.results[].duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `addresses[].pois_nearby[].pictures.results[].theme` | `string?` | Theme metadata/value within this model. |
| `addresses[].pois_nearby[].action` | `object?` | Action metadata/value within this model. |
| `addresses[].pois_nearby[].action.type` | `string` | Type metadata/value within this model. |
| `addresses[].pois_nearby[].action.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `addresses[].pois_nearby[].serves_cuisine` | `string[]?` | Serves cuisine collection. |
| `addresses[].pois_nearby[].categories` | `string[]?` | Categories collection. |
| `addresses[].pois_nearby[].icon_category` | `string?` | Icon category metadata/value within this model. |
| `addresses[].pois_nearby[].timezone` | `string?` | Timezone metadata/value within this model. |
| `addresses[].pois_nearby[].timezone_offset` | `int?` | Timezone offset metadata/value within this model. |
| `addresses[].pois_nearby[].id` | `string?` | Identifier scoped to the enclosing result; local-place IDs expire in about eight hours. |
| `addresses[].pois_nearby[].results` | `object[]?` | Results collection. |
| `addresses[].pois_nearby[].results[].title` | `string` | Title metadata/value within this model. |
| `addresses[].pois_nearby[].results[].url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `addresses[].pois_nearby[].results[].is_source_local` | `bool?` | Is source local metadata/value within this model. |
| `addresses[].pois_nearby[].results[].is_source_both` | `bool?` | Is source both metadata/value within this model. |
| `addresses[].pois_nearby[].results[].description` | `string?` | Display description/excerpt; POI Descriptions text is generated and Context source description is query-independent. |
| `addresses[].pois_nearby[].results[].page_age` | `string?` | Content publication/modification date, not necessarily crawl time. |
| `addresses[].pois_nearby[].results[].page_fetched` | `string?` | Last-fetch date/time as provided by the vendor. |
| `addresses[].pois_nearby[].results[].fetched_content_timestamp` | `int?` | Fetch timestamp; do not infer units beyond the source contract. |
| `addresses[].pois_nearby[].results[].profile` | `object?` | Profile metadata/value within this model. |
| `addresses[].pois_nearby[].results[].profile.name` | `string` | Name metadata/value within this model. |
| `addresses[].pois_nearby[].results[].profile.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `addresses[].pois_nearby[].results[].profile.long_name` | `string?` | Long name metadata/value within this model. |
| `addresses[].pois_nearby[].results[].profile.img` | `string?` | Img metadata/value within this model. |
| `addresses[].pois_nearby[].results[].language` | `string?` | Language metadata/value within this model. |
| `addresses[].pois_nearby[].results[].family_friendly` | `bool?` | Vendor safety classification, not a content-safety guarantee. |
| `addresses[].pois_nearby[].results[].meta_url` | `object` | Meta url metadata/value within this model. |
| `addresses[].pois_nearby[].results[].meta_url.scheme` | `string` | Scheme metadata/value within this model. |
| `addresses[].pois_nearby[].results[].meta_url.netloc` | `string` | Netloc metadata/value within this model. |
| `addresses[].pois_nearby[].results[].meta_url.hostname` | `string?` | Hostname metadata/value within this model. |
| `addresses[].pois_nearby[].results[].meta_url.favicon` | `string` | Favicon metadata/value within this model. |
| `addresses[].pois_nearby[].results[].meta_url.path` | `string` | Path metadata/value within this model. |
| `addresses[].zoom_level` | `int?` | Zoom level metadata/value within this model. |
| `addresses[].distance` | `object?` | Distance metadata/value within this model. |
| `addresses[].distance.value` | `number` | Value metadata/value within this model. |
| `addresses[].distance.units` | `string` | Units metadata/value within this model. |
| `addresses[].postal_address` | `object?` | Postal address metadata/value within this model. |
| `addresses[].postal_address.type` | `string?` | Documented discriminator `PostalAddress` identifies this enclosing model; unknown future tags remain accepted. |
| `addresses[].postal_address.country` | `string?` | Country metadata/value within this model. |
| `addresses[].postal_address.postalCode` | `string?` | Postal code metadata/value within this model. |
| `addresses[].postal_address.streetAddress` | `string?` | Street address metadata/value within this model. |
| `addresses[].postal_address.addressRegion` | `string?` | Address region metadata/value within this model. |
| `addresses[].postal_address.addressLocality` | `string?` | Address locality metadata/value within this model. |
| `addresses[].postal_address.displayAddress` | `string` | Display address metadata/value within this model. |
| `streets` | `object[]?` | Streets collection. |
| `streets[].type` | `string?` | address means a specific street plus number; street means the street without a specific numbered address. |
| `streets[].name` | `string` | Name metadata/value within this model. |
| `streets[].coordinates` | `array` | Latitude/longitude pair as described by the location model; no geometry guarantee for unrelated arrays. |
| `streets[].pois` | `object[]?` | Pois collection. |
| `streets[].pois[].title` | `string` | Title metadata/value within this model. |
| `streets[].pois[].url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `streets[].pois[].is_source_local` | `bool?` | Is source local metadata/value within this model. |
| `streets[].pois[].is_source_both` | `bool?` | Is source both metadata/value within this model. |
| `streets[].pois[].description` | `string?` | Display description/excerpt; POI Descriptions text is generated and Context source description is query-independent. |
| `streets[].pois[].page_age` | `string?` | Content publication/modification date, not necessarily crawl time. |
| `streets[].pois[].page_fetched` | `string?` | Last-fetch date/time as provided by the vendor. |
| `streets[].pois[].fetched_content_timestamp` | `int?` | Fetch timestamp; do not infer units beyond the source contract. |
| `streets[].pois[].profile` | `object?` | Profile metadata/value within this model. |
| `streets[].pois[].profile.name` | `string` | Name metadata/value within this model. |
| `streets[].pois[].profile.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `streets[].pois[].profile.long_name` | `string?` | Long name metadata/value within this model. |
| `streets[].pois[].profile.img` | `string?` | Img metadata/value within this model. |
| `streets[].pois[].language` | `string?` | Language metadata/value within this model. |
| `streets[].pois[].family_friendly` | `bool?` | Vendor safety classification, not a content-safety guarantee. |
| `streets[].pois[].type` | `string?` | Documented discriminator `location_result` identifies this enclosing model; unknown future tags remain accepted. |
| `streets[].pois[].provider_url` | `string` | Attribution/provider destination URL. |
| `streets[].pois[].coordinates` | `array?` | Latitude/longitude pair as described by the location model; no geometry guarantee for unrelated arrays. |
| `streets[].pois[].zoom_level` | `int?` | Zoom level metadata/value within this model. |
| `streets[].pois[].thumbnail` | `object?` | Thumbnail metadata/value within this model. |
| `streets[].pois[].thumbnail.src` | `string` | Src metadata/value within this model. |
| `streets[].pois[].thumbnail.alt` | `string?` | Alt metadata/value within this model. |
| `streets[].pois[].thumbnail.height` | `int?` | Height metadata/value within this model. |
| `streets[].pois[].thumbnail.width` | `int?` | Width metadata/value within this model. |
| `streets[].pois[].thumbnail.bg_color` | `string?` | Bg color metadata/value within this model. |
| `streets[].pois[].thumbnail.original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `streets[].pois[].thumbnail.logo` | `bool?` | Logo metadata/value within this model. |
| `streets[].pois[].thumbnail.duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `streets[].pois[].thumbnail.theme` | `string?` | Theme metadata/value within this model. |
| `streets[].pois[].postal_address` | `object?` | Postal address metadata/value within this model. |
| `streets[].pois[].postal_address.type` | `string?` | Documented discriminator `PostalAddress` identifies this enclosing model; unknown future tags remain accepted. |
| `streets[].pois[].postal_address.country` | `string?` | Country metadata/value within this model. |
| `streets[].pois[].postal_address.postalCode` | `string?` | Postal code metadata/value within this model. |
| `streets[].pois[].postal_address.streetAddress` | `string?` | Street address metadata/value within this model. |
| `streets[].pois[].postal_address.addressRegion` | `string?` | Address region metadata/value within this model. |
| `streets[].pois[].postal_address.addressLocality` | `string?` | Address locality metadata/value within this model. |
| `streets[].pois[].postal_address.displayAddress` | `string` | Display address metadata/value within this model. |
| `streets[].pois[].opening_hours` | `object?` | Business hours; days is a nested array and a day may contain multiple intervals. |
| `streets[].pois[].opening_hours.current_day` | `object[]?` | Current day collection. |
| `streets[].pois[].opening_hours.current_day[].abbr_name` | `string` | Abbr name metadata/value within this model. |
| `streets[].pois[].opening_hours.current_day[].full_name` | `string` | Full name metadata/value within this model. |
| `streets[].pois[].opening_hours.current_day[].opens` | `string` | Local 24-hour opening-time string. |
| `streets[].pois[].opening_hours.current_day[].closes` | `string` | Local 24-hour closing-time string. |
| `streets[].pois[].opening_hours.days` | `object[][]?` | Days collection. |
| `streets[].pois[].contact` | `object?` | Contact metadata/value within this model. |
| `streets[].pois[].contact.email` | `string?` | Email metadata/value within this model. |
| `streets[].pois[].contact.telephone` | `string?` | Telephone metadata/value within this model. |
| `streets[].pois[].price_range` | `string?` | Price range metadata/value within this model. |
| `streets[].pois[].rating` | `object?` | Rating metadata/value within this model. |
| `streets[].pois[].rating.ratingValue` | `number` | Source-specific rating value; compare only with that source and scale. |
| `streets[].pois[].rating.bestRating` | `number` | Source rating scale metadata; not global relevance. |
| `streets[].pois[].rating.reviewCount` | `int?` | Number of reviews reported by source. |
| `streets[].pois[].rating.profile` | `object?` | Profile metadata/value within this model. |
| `streets[].pois[].rating.profile.name` | `string` | Name metadata/value within this model. |
| `streets[].pois[].rating.profile.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `streets[].pois[].rating.profile.long_name` | `string?` | Long name metadata/value within this model. |
| `streets[].pois[].rating.profile.img` | `string?` | Img metadata/value within this model. |
| `streets[].pois[].rating.is_tripadvisor` | `bool?` | Attribution flag for the rating source. |
| `streets[].pois[].distance` | `object?` | Distance metadata/value within this model. |
| `streets[].pois[].distance.value` | `number` | Value metadata/value within this model. |
| `streets[].pois[].distance.units` | `string` | Units metadata/value within this model. |
| `streets[].pois[].profiles` | `object[]?` | Profiles collection. |
| `streets[].pois[].profiles[].type` | `string?` | Source-origin type; external normally identifies an external data source, but is not a closed enum. |
| `streets[].pois[].profiles[].name` | `string` | Name metadata/value within this model. |
| `streets[].pois[].profiles[].url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `streets[].pois[].profiles[].long_name` | `string?` | Long name metadata/value within this model. |
| `streets[].pois[].profiles[].img` | `string?` | Img metadata/value within this model. |
| `streets[].pois[].reviews` | `object?` | Reviews metadata/value within this model. |
| `streets[].pois[].reviews.results` | `object[]` | Results collection. |
| `streets[].pois[].reviews.results[].title` | `string` | Title metadata/value within this model. |
| `streets[].pois[].reviews.results[].description` | `string` | Display description/excerpt; POI Descriptions text is generated and Context source description is query-independent. |
| `streets[].pois[].reviews.results[].date` | `string` | Date metadata/value within this model. |
| `streets[].pois[].reviews.results[].rating` | `object` | Rating metadata/value within this model. |
| `streets[].pois[].reviews.results[].rating.ratingValue` | `number` | Source-specific rating value; compare only with that source and scale. |
| `streets[].pois[].reviews.results[].rating.bestRating` | `number` | Source rating scale metadata; not global relevance. |
| `streets[].pois[].reviews.results[].rating.reviewCount` | `int?` | Number of reviews reported by source. |
| `streets[].pois[].reviews.results[].rating.profile` | `object?` | Profile metadata/value within this model. |
| `streets[].pois[].reviews.results[].rating.profile.name` | `string` | Name metadata/value within this model. |
| `streets[].pois[].reviews.results[].rating.profile.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `streets[].pois[].reviews.results[].rating.profile.long_name` | `string?` | Long name metadata/value within this model. |
| `streets[].pois[].reviews.results[].rating.profile.img` | `string?` | Img metadata/value within this model. |
| `streets[].pois[].reviews.results[].rating.is_tripadvisor` | `bool?` | Attribution flag for the rating source. |
| `streets[].pois[].reviews.results[].author` | `object` | Author metadata/value within this model. |
| `streets[].pois[].reviews.results[].author.type` | `string?` | Documented discriminator `person` identifies this enclosing model; unknown future tags remain accepted. |
| `streets[].pois[].reviews.results[].author.name` | `string` | Name metadata/value within this model. |
| `streets[].pois[].reviews.results[].author.url` | `string?` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `streets[].pois[].reviews.results[].author.thumbnail` | `object?` | Thumbnail metadata/value within this model. |
| `streets[].pois[].reviews.results[].author.thumbnail.src` | `string` | Src metadata/value within this model. |
| `streets[].pois[].reviews.results[].author.thumbnail.alt` | `string?` | Alt metadata/value within this model. |
| `streets[].pois[].reviews.results[].author.thumbnail.height` | `int?` | Height metadata/value within this model. |
| `streets[].pois[].reviews.results[].author.thumbnail.width` | `int?` | Width metadata/value within this model. |
| `streets[].pois[].reviews.results[].author.thumbnail.bg_color` | `string?` | Bg color metadata/value within this model. |
| `streets[].pois[].reviews.results[].author.thumbnail.original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `streets[].pois[].reviews.results[].author.thumbnail.logo` | `bool?` | Logo metadata/value within this model. |
| `streets[].pois[].reviews.results[].author.thumbnail.duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `streets[].pois[].reviews.results[].author.thumbnail.theme` | `string?` | Theme metadata/value within this model. |
| `streets[].pois[].reviews.results[].author.email` | `string?` | Email metadata/value within this model. |
| `streets[].pois[].reviews.results[].review_url` | `string` | Review url metadata/value within this model. |
| `streets[].pois[].reviews.results[].language` | `string` | Language metadata/value within this model. |
| `streets[].pois[].reviews.viewMoreUrl` | `string` | View more url metadata/value within this model. |
| `streets[].pois[].reviews.reviews_in_foreign_language` | `bool` | Reviews in foreign language metadata/value within this model. |
| `streets[].pois[].pictures` | `object?` | Pictures metadata/value within this model. |
| `streets[].pois[].pictures.viewMoreUrl` | `string?` | View more url metadata/value within this model. |
| `streets[].pois[].pictures.results` | `object[]` | Results collection. |
| `streets[].pois[].pictures.results[].src` | `string` | Src metadata/value within this model. |
| `streets[].pois[].pictures.results[].alt` | `string?` | Alt metadata/value within this model. |
| `streets[].pois[].pictures.results[].height` | `int?` | Height metadata/value within this model. |
| `streets[].pois[].pictures.results[].width` | `int?` | Width metadata/value within this model. |
| `streets[].pois[].pictures.results[].bg_color` | `string?` | Bg color metadata/value within this model. |
| `streets[].pois[].pictures.results[].original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `streets[].pois[].pictures.results[].logo` | `bool?` | Logo metadata/value within this model. |
| `streets[].pois[].pictures.results[].duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `streets[].pois[].pictures.results[].theme` | `string?` | Theme metadata/value within this model. |
| `streets[].pois[].action` | `object?` | Action metadata/value within this model. |
| `streets[].pois[].action.type` | `string` | Type metadata/value within this model. |
| `streets[].pois[].action.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `streets[].pois[].serves_cuisine` | `string[]?` | Serves cuisine collection. |
| `streets[].pois[].categories` | `string[]?` | Categories collection. |
| `streets[].pois[].icon_category` | `string?` | Icon category metadata/value within this model. |
| `streets[].pois[].timezone` | `string?` | Timezone metadata/value within this model. |
| `streets[].pois[].timezone_offset` | `int?` | Timezone offset metadata/value within this model. |
| `streets[].pois[].id` | `string?` | Identifier scoped to the enclosing result; local-place IDs expire in about eight hours. |
| `streets[].pois[].results` | `object[]?` | Results collection. |
| `streets[].pois[].results[].title` | `string` | Title metadata/value within this model. |
| `streets[].pois[].results[].url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `streets[].pois[].results[].is_source_local` | `bool?` | Is source local metadata/value within this model. |
| `streets[].pois[].results[].is_source_both` | `bool?` | Is source both metadata/value within this model. |
| `streets[].pois[].results[].description` | `string?` | Display description/excerpt; POI Descriptions text is generated and Context source description is query-independent. |
| `streets[].pois[].results[].page_age` | `string?` | Content publication/modification date, not necessarily crawl time. |
| `streets[].pois[].results[].page_fetched` | `string?` | Last-fetch date/time as provided by the vendor. |
| `streets[].pois[].results[].fetched_content_timestamp` | `int?` | Fetch timestamp; do not infer units beyond the source contract. |
| `streets[].pois[].results[].profile` | `object?` | Profile metadata/value within this model. |
| `streets[].pois[].results[].profile.name` | `string` | Name metadata/value within this model. |
| `streets[].pois[].results[].profile.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `streets[].pois[].results[].profile.long_name` | `string?` | Long name metadata/value within this model. |
| `streets[].pois[].results[].profile.img` | `string?` | Img metadata/value within this model. |
| `streets[].pois[].results[].language` | `string?` | Language metadata/value within this model. |
| `streets[].pois[].results[].family_friendly` | `bool?` | Vendor safety classification, not a content-safety guarantee. |
| `streets[].pois[].results[].meta_url` | `object` | Meta url metadata/value within this model. |
| `streets[].pois[].results[].meta_url.scheme` | `string` | Scheme metadata/value within this model. |
| `streets[].pois[].results[].meta_url.netloc` | `string` | Netloc metadata/value within this model. |
| `streets[].pois[].results[].meta_url.hostname` | `string?` | Hostname metadata/value within this model. |
| `streets[].pois[].results[].meta_url.favicon` | `string` | Favicon metadata/value within this model. |
| `streets[].pois[].results[].meta_url.path` | `string` | Path metadata/value within this model. |
| `streets[].pois_nearby` | `object[]?` | Pois nearby collection. |
| `streets[].pois_nearby[].title` | `string` | Title metadata/value within this model. |
| `streets[].pois_nearby[].url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `streets[].pois_nearby[].is_source_local` | `bool?` | Is source local metadata/value within this model. |
| `streets[].pois_nearby[].is_source_both` | `bool?` | Is source both metadata/value within this model. |
| `streets[].pois_nearby[].description` | `string?` | Display description/excerpt; POI Descriptions text is generated and Context source description is query-independent. |
| `streets[].pois_nearby[].page_age` | `string?` | Content publication/modification date, not necessarily crawl time. |
| `streets[].pois_nearby[].page_fetched` | `string?` | Last-fetch date/time as provided by the vendor. |
| `streets[].pois_nearby[].fetched_content_timestamp` | `int?` | Fetch timestamp; do not infer units beyond the source contract. |
| `streets[].pois_nearby[].profile` | `object?` | Profile metadata/value within this model. |
| `streets[].pois_nearby[].profile.name` | `string` | Name metadata/value within this model. |
| `streets[].pois_nearby[].profile.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `streets[].pois_nearby[].profile.long_name` | `string?` | Long name metadata/value within this model. |
| `streets[].pois_nearby[].profile.img` | `string?` | Img metadata/value within this model. |
| `streets[].pois_nearby[].language` | `string?` | Language metadata/value within this model. |
| `streets[].pois_nearby[].family_friendly` | `bool?` | Vendor safety classification, not a content-safety guarantee. |
| `streets[].pois_nearby[].type` | `string?` | Documented discriminator `location_result` identifies this enclosing model; unknown future tags remain accepted. |
| `streets[].pois_nearby[].provider_url` | `string` | Attribution/provider destination URL. |
| `streets[].pois_nearby[].coordinates` | `array?` | Latitude/longitude pair as described by the location model; no geometry guarantee for unrelated arrays. |
| `streets[].pois_nearby[].zoom_level` | `int?` | Zoom level metadata/value within this model. |
| `streets[].pois_nearby[].thumbnail` | `object?` | Thumbnail metadata/value within this model. |
| `streets[].pois_nearby[].thumbnail.src` | `string` | Src metadata/value within this model. |
| `streets[].pois_nearby[].thumbnail.alt` | `string?` | Alt metadata/value within this model. |
| `streets[].pois_nearby[].thumbnail.height` | `int?` | Height metadata/value within this model. |
| `streets[].pois_nearby[].thumbnail.width` | `int?` | Width metadata/value within this model. |
| `streets[].pois_nearby[].thumbnail.bg_color` | `string?` | Bg color metadata/value within this model. |
| `streets[].pois_nearby[].thumbnail.original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `streets[].pois_nearby[].thumbnail.logo` | `bool?` | Logo metadata/value within this model. |
| `streets[].pois_nearby[].thumbnail.duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `streets[].pois_nearby[].thumbnail.theme` | `string?` | Theme metadata/value within this model. |
| `streets[].pois_nearby[].postal_address` | `object?` | Postal address metadata/value within this model. |
| `streets[].pois_nearby[].postal_address.type` | `string?` | Documented discriminator `PostalAddress` identifies this enclosing model; unknown future tags remain accepted. |
| `streets[].pois_nearby[].postal_address.country` | `string?` | Country metadata/value within this model. |
| `streets[].pois_nearby[].postal_address.postalCode` | `string?` | Postal code metadata/value within this model. |
| `streets[].pois_nearby[].postal_address.streetAddress` | `string?` | Street address metadata/value within this model. |
| `streets[].pois_nearby[].postal_address.addressRegion` | `string?` | Address region metadata/value within this model. |
| `streets[].pois_nearby[].postal_address.addressLocality` | `string?` | Address locality metadata/value within this model. |
| `streets[].pois_nearby[].postal_address.displayAddress` | `string` | Display address metadata/value within this model. |
| `streets[].pois_nearby[].opening_hours` | `object?` | Business hours; days is a nested array and a day may contain multiple intervals. |
| `streets[].pois_nearby[].opening_hours.current_day` | `object[]?` | Current day collection. |
| `streets[].pois_nearby[].opening_hours.current_day[].abbr_name` | `string` | Abbr name metadata/value within this model. |
| `streets[].pois_nearby[].opening_hours.current_day[].full_name` | `string` | Full name metadata/value within this model. |
| `streets[].pois_nearby[].opening_hours.current_day[].opens` | `string` | Local 24-hour opening-time string. |
| `streets[].pois_nearby[].opening_hours.current_day[].closes` | `string` | Local 24-hour closing-time string. |
| `streets[].pois_nearby[].opening_hours.days` | `object[][]?` | Days collection. |
| `streets[].pois_nearby[].contact` | `object?` | Contact metadata/value within this model. |
| `streets[].pois_nearby[].contact.email` | `string?` | Email metadata/value within this model. |
| `streets[].pois_nearby[].contact.telephone` | `string?` | Telephone metadata/value within this model. |
| `streets[].pois_nearby[].price_range` | `string?` | Price range metadata/value within this model. |
| `streets[].pois_nearby[].rating` | `object?` | Rating metadata/value within this model. |
| `streets[].pois_nearby[].rating.ratingValue` | `number` | Source-specific rating value; compare only with that source and scale. |
| `streets[].pois_nearby[].rating.bestRating` | `number` | Source rating scale metadata; not global relevance. |
| `streets[].pois_nearby[].rating.reviewCount` | `int?` | Number of reviews reported by source. |
| `streets[].pois_nearby[].rating.profile` | `object?` | Profile metadata/value within this model. |
| `streets[].pois_nearby[].rating.profile.name` | `string` | Name metadata/value within this model. |
| `streets[].pois_nearby[].rating.profile.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `streets[].pois_nearby[].rating.profile.long_name` | `string?` | Long name metadata/value within this model. |
| `streets[].pois_nearby[].rating.profile.img` | `string?` | Img metadata/value within this model. |
| `streets[].pois_nearby[].rating.is_tripadvisor` | `bool?` | Attribution flag for the rating source. |
| `streets[].pois_nearby[].distance` | `object?` | Distance metadata/value within this model. |
| `streets[].pois_nearby[].distance.value` | `number` | Value metadata/value within this model. |
| `streets[].pois_nearby[].distance.units` | `string` | Units metadata/value within this model. |
| `streets[].pois_nearby[].profiles` | `object[]?` | Profiles collection. |
| `streets[].pois_nearby[].profiles[].type` | `string?` | Source-origin type; external normally identifies an external data source, but is not a closed enum. |
| `streets[].pois_nearby[].profiles[].name` | `string` | Name metadata/value within this model. |
| `streets[].pois_nearby[].profiles[].url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `streets[].pois_nearby[].profiles[].long_name` | `string?` | Long name metadata/value within this model. |
| `streets[].pois_nearby[].profiles[].img` | `string?` | Img metadata/value within this model. |
| `streets[].pois_nearby[].reviews` | `object?` | Reviews metadata/value within this model. |
| `streets[].pois_nearby[].reviews.results` | `object[]` | Results collection. |
| `streets[].pois_nearby[].reviews.results[].title` | `string` | Title metadata/value within this model. |
| `streets[].pois_nearby[].reviews.results[].description` | `string` | Display description/excerpt; POI Descriptions text is generated and Context source description is query-independent. |
| `streets[].pois_nearby[].reviews.results[].date` | `string` | Date metadata/value within this model. |
| `streets[].pois_nearby[].reviews.results[].rating` | `object` | Rating metadata/value within this model. |
| `streets[].pois_nearby[].reviews.results[].rating.ratingValue` | `number` | Source-specific rating value; compare only with that source and scale. |
| `streets[].pois_nearby[].reviews.results[].rating.bestRating` | `number` | Source rating scale metadata; not global relevance. |
| `streets[].pois_nearby[].reviews.results[].rating.reviewCount` | `int?` | Number of reviews reported by source. |
| `streets[].pois_nearby[].reviews.results[].rating.profile` | `object?` | Profile metadata/value within this model. |
| `streets[].pois_nearby[].reviews.results[].rating.profile.name` | `string` | Name metadata/value within this model. |
| `streets[].pois_nearby[].reviews.results[].rating.profile.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `streets[].pois_nearby[].reviews.results[].rating.profile.long_name` | `string?` | Long name metadata/value within this model. |
| `streets[].pois_nearby[].reviews.results[].rating.profile.img` | `string?` | Img metadata/value within this model. |
| `streets[].pois_nearby[].reviews.results[].rating.is_tripadvisor` | `bool?` | Attribution flag for the rating source. |
| `streets[].pois_nearby[].reviews.results[].author` | `object` | Author metadata/value within this model. |
| `streets[].pois_nearby[].reviews.results[].author.type` | `string?` | Documented discriminator `person` identifies this enclosing model; unknown future tags remain accepted. |
| `streets[].pois_nearby[].reviews.results[].author.name` | `string` | Name metadata/value within this model. |
| `streets[].pois_nearby[].reviews.results[].author.url` | `string?` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `streets[].pois_nearby[].reviews.results[].author.thumbnail` | `object?` | Thumbnail metadata/value within this model. |
| `streets[].pois_nearby[].reviews.results[].author.thumbnail.src` | `string` | Src metadata/value within this model. |
| `streets[].pois_nearby[].reviews.results[].author.thumbnail.alt` | `string?` | Alt metadata/value within this model. |
| `streets[].pois_nearby[].reviews.results[].author.thumbnail.height` | `int?` | Height metadata/value within this model. |
| `streets[].pois_nearby[].reviews.results[].author.thumbnail.width` | `int?` | Width metadata/value within this model. |
| `streets[].pois_nearby[].reviews.results[].author.thumbnail.bg_color` | `string?` | Bg color metadata/value within this model. |
| `streets[].pois_nearby[].reviews.results[].author.thumbnail.original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `streets[].pois_nearby[].reviews.results[].author.thumbnail.logo` | `bool?` | Logo metadata/value within this model. |
| `streets[].pois_nearby[].reviews.results[].author.thumbnail.duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `streets[].pois_nearby[].reviews.results[].author.thumbnail.theme` | `string?` | Theme metadata/value within this model. |
| `streets[].pois_nearby[].reviews.results[].author.email` | `string?` | Email metadata/value within this model. |
| `streets[].pois_nearby[].reviews.results[].review_url` | `string` | Review url metadata/value within this model. |
| `streets[].pois_nearby[].reviews.results[].language` | `string` | Language metadata/value within this model. |
| `streets[].pois_nearby[].reviews.viewMoreUrl` | `string` | View more url metadata/value within this model. |
| `streets[].pois_nearby[].reviews.reviews_in_foreign_language` | `bool` | Reviews in foreign language metadata/value within this model. |
| `streets[].pois_nearby[].pictures` | `object?` | Pictures metadata/value within this model. |
| `streets[].pois_nearby[].pictures.viewMoreUrl` | `string?` | View more url metadata/value within this model. |
| `streets[].pois_nearby[].pictures.results` | `object[]` | Results collection. |
| `streets[].pois_nearby[].pictures.results[].src` | `string` | Src metadata/value within this model. |
| `streets[].pois_nearby[].pictures.results[].alt` | `string?` | Alt metadata/value within this model. |
| `streets[].pois_nearby[].pictures.results[].height` | `int?` | Height metadata/value within this model. |
| `streets[].pois_nearby[].pictures.results[].width` | `int?` | Width metadata/value within this model. |
| `streets[].pois_nearby[].pictures.results[].bg_color` | `string?` | Bg color metadata/value within this model. |
| `streets[].pois_nearby[].pictures.results[].original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `streets[].pois_nearby[].pictures.results[].logo` | `bool?` | Logo metadata/value within this model. |
| `streets[].pois_nearby[].pictures.results[].duplicated` | `bool?` | Duplicated metadata/value within this model. |
| `streets[].pois_nearby[].pictures.results[].theme` | `string?` | Theme metadata/value within this model. |
| `streets[].pois_nearby[].action` | `object?` | Action metadata/value within this model. |
| `streets[].pois_nearby[].action.type` | `string` | Type metadata/value within this model. |
| `streets[].pois_nearby[].action.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `streets[].pois_nearby[].serves_cuisine` | `string[]?` | Serves cuisine collection. |
| `streets[].pois_nearby[].categories` | `string[]?` | Categories collection. |
| `streets[].pois_nearby[].icon_category` | `string?` | Icon category metadata/value within this model. |
| `streets[].pois_nearby[].timezone` | `string?` | Timezone metadata/value within this model. |
| `streets[].pois_nearby[].timezone_offset` | `int?` | Timezone offset metadata/value within this model. |
| `streets[].pois_nearby[].id` | `string?` | Identifier scoped to the enclosing result; local-place IDs expire in about eight hours. |
| `streets[].pois_nearby[].results` | `object[]?` | Results collection. |
| `streets[].pois_nearby[].results[].title` | `string` | Title metadata/value within this model. |
| `streets[].pois_nearby[].results[].url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `streets[].pois_nearby[].results[].is_source_local` | `bool?` | Is source local metadata/value within this model. |
| `streets[].pois_nearby[].results[].is_source_both` | `bool?` | Is source both metadata/value within this model. |
| `streets[].pois_nearby[].results[].description` | `string?` | Display description/excerpt; POI Descriptions text is generated and Context source description is query-independent. |
| `streets[].pois_nearby[].results[].page_age` | `string?` | Content publication/modification date, not necessarily crawl time. |
| `streets[].pois_nearby[].results[].page_fetched` | `string?` | Last-fetch date/time as provided by the vendor. |
| `streets[].pois_nearby[].results[].fetched_content_timestamp` | `int?` | Fetch timestamp; do not infer units beyond the source contract. |
| `streets[].pois_nearby[].results[].profile` | `object?` | Profile metadata/value within this model. |
| `streets[].pois_nearby[].results[].profile.name` | `string` | Name metadata/value within this model. |
| `streets[].pois_nearby[].results[].profile.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `streets[].pois_nearby[].results[].profile.long_name` | `string?` | Long name metadata/value within this model. |
| `streets[].pois_nearby[].results[].profile.img` | `string?` | Img metadata/value within this model. |
| `streets[].pois_nearby[].results[].language` | `string?` | Language metadata/value within this model. |
| `streets[].pois_nearby[].results[].family_friendly` | `bool?` | Vendor safety classification, not a content-safety guarantee. |
| `streets[].pois_nearby[].results[].meta_url` | `object` | Meta url metadata/value within this model. |
| `streets[].pois_nearby[].results[].meta_url.scheme` | `string` | Scheme metadata/value within this model. |
| `streets[].pois_nearby[].results[].meta_url.netloc` | `string` | Netloc metadata/value within this model. |
| `streets[].pois_nearby[].results[].meta_url.hostname` | `string?` | Hostname metadata/value within this model. |
| `streets[].pois_nearby[].results[].meta_url.favicon` | `string` | Favicon metadata/value within this model. |
| `streets[].pois_nearby[].results[].meta_url.path` | `string` | Path metadata/value within this model. |
| `streets[].zoom_level` | `int?` | Zoom level metadata/value within this model. |
| `streets[].distance` | `object?` | Distance metadata/value within this model. |
| `streets[].distance.value` | `number` | Value metadata/value within this model. |
| `streets[].distance.units` | `string` | Units metadata/value within this model. |
| `streets[].postal_address` | `object?` | Postal address metadata/value within this model. |
| `streets[].postal_address.type` | `string?` | Documented discriminator `PostalAddress` identifies this enclosing model; unknown future tags remain accepted. |
| `streets[].postal_address.country` | `string?` | Country metadata/value within this model. |
| `streets[].postal_address.postalCode` | `string?` | Postal code metadata/value within this model. |
| `streets[].postal_address.streetAddress` | `string?` | Street address metadata/value within this model. |
| `streets[].postal_address.addressRegion` | `string?` | Address region metadata/value within this model. |
| `streets[].postal_address.addressLocality` | `string?` | Address locality metadata/value within this model. |
| `streets[].postal_address.displayAddress` | `string` | Display address metadata/value within this model. |
| `mixed` | `object[]?` | Ordering references into vertical arrays; preserve type/index association when projecting. |
| `mixed[].type` | `string` | Type metadata/value within this model. |
| `mixed[].index` | `int?` | Index scoped to the enclosing vendor model; no general cross-model ordering guarantee. |
| `mixed[].all` | `bool?` | All metadata/value within this model. |
| `location` | `object?` | Location metadata/value within this model. |
| `location.coordinates` | `array` | Latitude/longitude pair as described by the location model; no geometry guarantee for unrelated arrays. |
| `location.name` | `string?` | Name metadata/value within this model. |
| `location.country` | `string?` | Country metadata/value within this model. |
