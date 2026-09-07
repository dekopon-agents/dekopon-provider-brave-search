# Rich Search response field catalog

All fetched **200** reference paths are listed below; arrays use `[]`. A `?` records vendor optional notation, not proof of when a field is sent. The schema conservatively accepts null for these fields; it does not assert every optional field is actually nullable. Required-looking fields are not required by the permissive response schema. Unknown fields survive bounded raw projection.

See [response semantics](../response-semantics.md), [endpoint](./rich.md), and [upstream schema](../../schemas/upstream/rich.response.schema.json). Unspecified rich-field semantics are explicitly unknown, not inferred.

| Field path | Vendor type | Meaning / interpretation |
|---|---|---|
| `type` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results` | `object[]?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].type` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].subtype` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].provider` | `object?` | Third-party data provenance; retain attribution in projections. |
| `results[].provider.type` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].provider.name` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].provider.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `results[].provider.long_name` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].provider.img` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].language` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].calculator` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].calculator.expression` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].calculator.answer` | `any?` | Calculator or question/answer result value according to enclosing type; not a general full-page field. |
| `results[].cryptocurrency` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cryptocurrency.intent_type` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cryptocurrency.quote` | `object` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cryptocurrency.quote.id` | `string` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].cryptocurrency.quote.symbol` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cryptocurrency.quote.name` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cryptocurrency.quote.image` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cryptocurrency.quote.current_price` | `number?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cryptocurrency.quote.market_cap` | `number?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cryptocurrency.quote.market_cap_rank` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cryptocurrency.quote.fully_diluted_valuation` | `number?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cryptocurrency.quote.total_volume` | `number` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cryptocurrency.quote.high_24h` | `number?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cryptocurrency.quote.low_24h` | `number?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cryptocurrency.quote.price_change_24h` | `number?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cryptocurrency.quote.price_change_percentage_24h` | `number?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cryptocurrency.quote.market_cap_change_24h` | `number?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cryptocurrency.quote.market_cap_change_percentage_24h` | `number?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cryptocurrency.quote.circulating_supply` | `number?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cryptocurrency.quote.total_supply` | `number?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cryptocurrency.quote.max_supply` | `number?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cryptocurrency.quote.ath` | `number` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cryptocurrency.quote.ath_change_percentage` | `number` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cryptocurrency.quote.ath_date` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cryptocurrency.quote.atl` | `number` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cryptocurrency.quote.atl_change_percentage` | `number` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cryptocurrency.quote.atl_date` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cryptocurrency.quote.last_updated` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cryptocurrency.timeseries` | `object` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cryptocurrency.timeseries.time_range` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cryptocurrency.timeseries.ts_price` | `array[]` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cryptocurrency.timeseries.ts_market_cap` | `array[]` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cryptocurrency.timeseries.ts_volume` | `array[]` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cryptocurrency.vs_currency` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cryptocurrency.vs_currencies` | `object[]?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cryptocurrency.vs_currencies[].label` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cryptocurrency.vs_currencies[].value` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cryptocurrency.top100_crypto` | `object[]?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cryptocurrency.top100_crypto[].label` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cryptocurrency.top100_crypto[].value` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cryptocurrency.top100_crypto[].symbol` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cryptocurrency.amount` | `any?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cryptocurrency.currency` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cryptocurrency.currency.full_name` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cryptocurrency.currency.code` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cryptocurrency.currency.decimals` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].currency` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].currency.conversion` | `object` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].currency.conversion.query` | `object` | Variant-specific query data; no automatic follow-up search is authorized. |
| `results[].currency.conversion.query.from_currency` | `object` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].currency.conversion.query.from_currency.full_name` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].currency.conversion.query.from_currency.code` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].currency.conversion.query.from_currency.decimals` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].currency.conversion.query.to_currency` | `object` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].currency.conversion.query.to_currency.full_name` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].currency.conversion.query.to_currency.code` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].currency.conversion.query.to_currency.decimals` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].currency.conversion.query.amount` | `number` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].currency.conversion.info` | `object` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].currency.conversion.info.rate` | `number` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].currency.conversion.info.timestamp` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].currency.conversion.date` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].currency.conversion.result` | `number` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].currency.conversion.amount` | `number` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].currency.timeseries` | `object` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].currency.timeseries.time_range` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].currency.timeseries.ts_exchange_rate` | `array[]?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].currency.supported_currencies` | `object[]?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].currency.supported_currencies[].full_name` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].currency.supported_currencies[].code` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].currency.supported_currencies[].decimals` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].definitions` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].definitions.word` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].definitions.language` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].definitions.pronounciation` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].definitions.definitions` | `object[]` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].definitions.definitions[].part_of_speech` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].definitions.definitions[].definitions` | `any[]` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].definitions.definitions[].definitions[].text` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].definitions.definitions[].definitions[].example_uses` | `string[]` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].definitions.definitions[].definitions[].related_words` | `string[]` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].definitions.definitions[].definitions[].labels` | `object[]` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].definitions.definitions[].definitions[].labels[].text` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].definitions.definitions[].definitions[].labels[].type` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].definitions.source_dict` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].definitions.attribution_text` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].definitions.attribution_url` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].definitions.audio_available` | `bool?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.default_view` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.calendar` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.calendar.races` | `object[]?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.calendar.races[].id` | `int` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].formula1.calendar.races[].name` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.calendar.races[].date` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.calendar.next_race` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.calendar.next_race.id` | `int` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].formula1.calendar.next_race.competition` | `object` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.calendar.next_race.competition.id` | `int` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].formula1.calendar.next_race.competition.name` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.calendar.next_race.competition.location` | `object` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.calendar.next_race.competition.location.country` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.calendar.next_race.competition.location.city` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.calendar.next_race.circuit` | `object` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.calendar.next_race.circuit.id` | `int` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].formula1.calendar.next_race.circuit.name` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.calendar.next_race.circuit.image` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.calendar.next_race.season` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.calendar.next_race.type` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.calendar.next_race.laps` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.calendar.next_race.laps.current` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.calendar.next_race.laps.total` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.calendar.next_race.fastest_lap` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.calendar.next_race.fastest_lap.driver` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.calendar.next_race.fastest_lap.driver.id` | `int?` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].formula1.calendar.next_race.fastest_lap.time` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.calendar.next_race.distance` | `any?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.calendar.next_race.date` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.calendar.next_race.status` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.calendar.previous_race` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.calendar.previous_race.id` | `int` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].formula1.calendar.previous_race.competition` | `object` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.calendar.previous_race.competition.id` | `int` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].formula1.calendar.previous_race.competition.name` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.calendar.previous_race.competition.location` | `object` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.calendar.previous_race.competition.location.country` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.calendar.previous_race.competition.location.city` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.calendar.previous_race.circuit` | `object` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.calendar.previous_race.circuit.id` | `int` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].formula1.calendar.previous_race.circuit.name` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.calendar.previous_race.circuit.image` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.calendar.previous_race.season` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.calendar.previous_race.type` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.calendar.previous_race.laps` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.calendar.previous_race.laps.current` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.calendar.previous_race.laps.total` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.calendar.previous_race.fastest_lap` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.calendar.previous_race.fastest_lap.driver` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.calendar.previous_race.fastest_lap.driver.id` | `int?` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].formula1.calendar.previous_race.fastest_lap.time` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.calendar.previous_race.distance` | `any?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.calendar.previous_race.date` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.calendar.previous_race.status` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.event` | `object` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.event.id` | `int` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].formula1.next_race.event.competition` | `object` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.event.competition.id` | `int` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].formula1.next_race.event.competition.name` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.event.competition.location` | `object` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.event.competition.location.country` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.event.competition.location.city` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.event.circuit` | `object` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.event.circuit.id` | `int` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].formula1.next_race.event.circuit.name` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.event.circuit.image` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.event.season` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.event.type` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.event.laps` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.event.laps.current` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.event.laps.total` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.event.fastest_lap` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.event.fastest_lap.driver` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.event.fastest_lap.driver.id` | `int?` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].formula1.next_race.event.fastest_lap.time` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.event.distance` | `any?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.event.date` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.event.status` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.standings` | `object[]?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.standings[].position` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.standings[].driver` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.standings[].driver.id` | `int` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].formula1.next_race.standings[].driver.name` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.standings[].driver.abbr` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.standings[].driver.number` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.standings[].driver.image` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.standings[].team` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.standings[].team.id` | `int` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].formula1.next_race.standings[].team.name` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.standings[].team.logo` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.standings[].points` | `number?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.standings[].wins` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.standings[].behind` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.standings[].season` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.standings[].time` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.standings[].laps` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.standings[].grid` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.standings[].gap` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.qualifying` | `object[]?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.qualifying[].position` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.qualifying[].driver` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.qualifying[].driver.id` | `int` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].formula1.next_race.qualifying[].driver.name` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.qualifying[].driver.abbr` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.qualifying[].driver.number` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.qualifying[].driver.image` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.qualifying[].team` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.qualifying[].team.id` | `int` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].formula1.next_race.qualifying[].team.name` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.qualifying[].team.logo` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.qualifying[].points` | `number?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.qualifying[].wins` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.qualifying[].behind` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.qualifying[].season` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.qualifying[].time` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.qualifying[].laps` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.qualifying[].grid` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.qualifying[].gap` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.sprint_shootout` | `object[]?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.sprint_shootout[].position` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.sprint_shootout[].driver` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.sprint_shootout[].driver.id` | `int` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].formula1.next_race.sprint_shootout[].driver.name` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.sprint_shootout[].driver.abbr` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.sprint_shootout[].driver.number` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.sprint_shootout[].driver.image` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.sprint_shootout[].team` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.sprint_shootout[].team.id` | `int` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].formula1.next_race.sprint_shootout[].team.name` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.sprint_shootout[].team.logo` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.sprint_shootout[].points` | `number?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.sprint_shootout[].wins` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.sprint_shootout[].behind` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.sprint_shootout[].season` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.sprint_shootout[].time` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.sprint_shootout[].laps` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.sprint_shootout[].grid` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.sprint_shootout[].gap` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.sprint_event` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.sprint_event.id` | `int` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].formula1.next_race.sprint_event.competition` | `object` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.sprint_event.competition.id` | `int` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].formula1.next_race.sprint_event.competition.name` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.sprint_event.competition.location` | `object` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.sprint_event.competition.location.country` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.sprint_event.competition.location.city` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.sprint_event.circuit` | `object` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.sprint_event.circuit.id` | `int` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].formula1.next_race.sprint_event.circuit.name` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.sprint_event.circuit.image` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.sprint_event.season` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.sprint_event.type` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.sprint_event.laps` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.sprint_event.laps.current` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.sprint_event.laps.total` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.sprint_event.fastest_lap` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.sprint_event.fastest_lap.driver` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.sprint_event.fastest_lap.driver.id` | `int?` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].formula1.next_race.sprint_event.fastest_lap.time` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.sprint_event.distance` | `any?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.sprint_event.date` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.sprint_event.status` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.sprint_standings` | `object[]?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.sprint_standings[].position` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.sprint_standings[].driver` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.sprint_standings[].driver.id` | `int` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].formula1.next_race.sprint_standings[].driver.name` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.sprint_standings[].driver.abbr` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.sprint_standings[].driver.number` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.sprint_standings[].driver.image` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.sprint_standings[].team` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.sprint_standings[].team.id` | `int` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].formula1.next_race.sprint_standings[].team.name` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.sprint_standings[].team.logo` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.sprint_standings[].points` | `number?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.sprint_standings[].wins` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.sprint_standings[].behind` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.sprint_standings[].season` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.sprint_standings[].time` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.sprint_standings[].laps` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.sprint_standings[].grid` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.sprint_standings[].gap` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.weekend_events` | `object[]?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.weekend_events[].type` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.weekend_events[].date` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.next_race.weekend_events[].status` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.event` | `object` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.event.id` | `int` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].formula1.previous_race.event.competition` | `object` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.event.competition.id` | `int` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].formula1.previous_race.event.competition.name` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.event.competition.location` | `object` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.event.competition.location.country` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.event.competition.location.city` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.event.circuit` | `object` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.event.circuit.id` | `int` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].formula1.previous_race.event.circuit.name` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.event.circuit.image` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.event.season` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.event.type` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.event.laps` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.event.laps.current` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.event.laps.total` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.event.fastest_lap` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.event.fastest_lap.driver` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.event.fastest_lap.driver.id` | `int?` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].formula1.previous_race.event.fastest_lap.time` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.event.distance` | `any?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.event.date` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.event.status` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.standings` | `object[]?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.standings[].position` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.standings[].driver` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.standings[].driver.id` | `int` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].formula1.previous_race.standings[].driver.name` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.standings[].driver.abbr` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.standings[].driver.number` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.standings[].driver.image` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.standings[].team` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.standings[].team.id` | `int` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].formula1.previous_race.standings[].team.name` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.standings[].team.logo` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.standings[].points` | `number?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.standings[].wins` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.standings[].behind` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.standings[].season` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.standings[].time` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.standings[].laps` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.standings[].grid` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.standings[].gap` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.qualifying` | `object[]?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.qualifying[].position` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.qualifying[].driver` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.qualifying[].driver.id` | `int` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].formula1.previous_race.qualifying[].driver.name` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.qualifying[].driver.abbr` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.qualifying[].driver.number` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.qualifying[].driver.image` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.qualifying[].team` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.qualifying[].team.id` | `int` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].formula1.previous_race.qualifying[].team.name` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.qualifying[].team.logo` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.qualifying[].points` | `number?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.qualifying[].wins` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.qualifying[].behind` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.qualifying[].season` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.qualifying[].time` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.qualifying[].laps` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.qualifying[].grid` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.qualifying[].gap` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.sprint_shootout` | `object[]?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.sprint_shootout[].position` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.sprint_shootout[].driver` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.sprint_shootout[].driver.id` | `int` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].formula1.previous_race.sprint_shootout[].driver.name` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.sprint_shootout[].driver.abbr` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.sprint_shootout[].driver.number` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.sprint_shootout[].driver.image` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.sprint_shootout[].team` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.sprint_shootout[].team.id` | `int` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].formula1.previous_race.sprint_shootout[].team.name` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.sprint_shootout[].team.logo` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.sprint_shootout[].points` | `number?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.sprint_shootout[].wins` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.sprint_shootout[].behind` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.sprint_shootout[].season` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.sprint_shootout[].time` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.sprint_shootout[].laps` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.sprint_shootout[].grid` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.sprint_shootout[].gap` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.sprint_event` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.sprint_event.id` | `int` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].formula1.previous_race.sprint_event.competition` | `object` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.sprint_event.competition.id` | `int` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].formula1.previous_race.sprint_event.competition.name` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.sprint_event.competition.location` | `object` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.sprint_event.competition.location.country` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.sprint_event.competition.location.city` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.sprint_event.circuit` | `object` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.sprint_event.circuit.id` | `int` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].formula1.previous_race.sprint_event.circuit.name` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.sprint_event.circuit.image` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.sprint_event.season` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.sprint_event.type` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.sprint_event.laps` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.sprint_event.laps.current` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.sprint_event.laps.total` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.sprint_event.fastest_lap` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.sprint_event.fastest_lap.driver` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.sprint_event.fastest_lap.driver.id` | `int?` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].formula1.previous_race.sprint_event.fastest_lap.time` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.sprint_event.distance` | `any?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.sprint_event.date` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.sprint_event.status` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.sprint_standings` | `object[]?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.sprint_standings[].position` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.sprint_standings[].driver` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.sprint_standings[].driver.id` | `int` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].formula1.previous_race.sprint_standings[].driver.name` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.sprint_standings[].driver.abbr` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.sprint_standings[].driver.number` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.sprint_standings[].driver.image` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.sprint_standings[].team` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.sprint_standings[].team.id` | `int` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].formula1.previous_race.sprint_standings[].team.name` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.sprint_standings[].team.logo` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.sprint_standings[].points` | `number?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.sprint_standings[].wins` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.sprint_standings[].behind` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.sprint_standings[].season` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.sprint_standings[].time` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.sprint_standings[].laps` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.sprint_standings[].grid` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.sprint_standings[].gap` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.weekend_events` | `object[]?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.weekend_events[].type` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.weekend_events[].date` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.previous_race.weekend_events[].status` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.event` | `object` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.event.id` | `int` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].formula1.live_race.event.competition` | `object` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.event.competition.id` | `int` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].formula1.live_race.event.competition.name` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.event.competition.location` | `object` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.event.competition.location.country` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.event.competition.location.city` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.event.circuit` | `object` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.event.circuit.id` | `int` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].formula1.live_race.event.circuit.name` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.event.circuit.image` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.event.season` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.event.type` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.event.laps` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.event.laps.current` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.event.laps.total` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.event.fastest_lap` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.event.fastest_lap.driver` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.event.fastest_lap.driver.id` | `int?` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].formula1.live_race.event.fastest_lap.time` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.event.distance` | `any?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.event.date` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.event.status` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.standings` | `object[]?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.standings[].position` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.standings[].driver` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.standings[].driver.id` | `int` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].formula1.live_race.standings[].driver.name` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.standings[].driver.abbr` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.standings[].driver.number` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.standings[].driver.image` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.standings[].team` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.standings[].team.id` | `int` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].formula1.live_race.standings[].team.name` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.standings[].team.logo` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.standings[].points` | `number?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.standings[].wins` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.standings[].behind` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.standings[].season` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.standings[].time` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.standings[].laps` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.standings[].grid` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.standings[].gap` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.qualifying` | `object[]?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.qualifying[].position` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.qualifying[].driver` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.qualifying[].driver.id` | `int` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].formula1.live_race.qualifying[].driver.name` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.qualifying[].driver.abbr` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.qualifying[].driver.number` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.qualifying[].driver.image` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.qualifying[].team` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.qualifying[].team.id` | `int` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].formula1.live_race.qualifying[].team.name` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.qualifying[].team.logo` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.qualifying[].points` | `number?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.qualifying[].wins` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.qualifying[].behind` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.qualifying[].season` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.qualifying[].time` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.qualifying[].laps` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.qualifying[].grid` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.qualifying[].gap` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.sprint_shootout` | `object[]?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.sprint_shootout[].position` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.sprint_shootout[].driver` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.sprint_shootout[].driver.id` | `int` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].formula1.live_race.sprint_shootout[].driver.name` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.sprint_shootout[].driver.abbr` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.sprint_shootout[].driver.number` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.sprint_shootout[].driver.image` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.sprint_shootout[].team` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.sprint_shootout[].team.id` | `int` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].formula1.live_race.sprint_shootout[].team.name` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.sprint_shootout[].team.logo` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.sprint_shootout[].points` | `number?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.sprint_shootout[].wins` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.sprint_shootout[].behind` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.sprint_shootout[].season` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.sprint_shootout[].time` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.sprint_shootout[].laps` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.sprint_shootout[].grid` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.sprint_shootout[].gap` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.sprint_event` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.sprint_event.id` | `int` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].formula1.live_race.sprint_event.competition` | `object` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.sprint_event.competition.id` | `int` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].formula1.live_race.sprint_event.competition.name` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.sprint_event.competition.location` | `object` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.sprint_event.competition.location.country` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.sprint_event.competition.location.city` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.sprint_event.circuit` | `object` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.sprint_event.circuit.id` | `int` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].formula1.live_race.sprint_event.circuit.name` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.sprint_event.circuit.image` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.sprint_event.season` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.sprint_event.type` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.sprint_event.laps` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.sprint_event.laps.current` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.sprint_event.laps.total` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.sprint_event.fastest_lap` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.sprint_event.fastest_lap.driver` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.sprint_event.fastest_lap.driver.id` | `int?` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].formula1.live_race.sprint_event.fastest_lap.time` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.sprint_event.distance` | `any?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.sprint_event.date` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.sprint_event.status` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.sprint_standings` | `object[]?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.sprint_standings[].position` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.sprint_standings[].driver` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.sprint_standings[].driver.id` | `int` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].formula1.live_race.sprint_standings[].driver.name` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.sprint_standings[].driver.abbr` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.sprint_standings[].driver.number` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.sprint_standings[].driver.image` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.sprint_standings[].team` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.sprint_standings[].team.id` | `int` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].formula1.live_race.sprint_standings[].team.name` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.sprint_standings[].team.logo` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.sprint_standings[].points` | `number?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.sprint_standings[].wins` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.sprint_standings[].behind` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.sprint_standings[].season` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.sprint_standings[].time` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.sprint_standings[].laps` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.sprint_standings[].grid` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.sprint_standings[].gap` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.weekend_events` | `object[]?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.weekend_events[].type` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.weekend_events[].date` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.live_race.weekend_events[].status` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.drivers` | `object[]?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.drivers[].position` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.drivers[].driver` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.drivers[].driver.id` | `int` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].formula1.drivers[].driver.name` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.drivers[].driver.abbr` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.drivers[].driver.number` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.drivers[].driver.image` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.drivers[].team` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.drivers[].team.id` | `int` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].formula1.drivers[].team.name` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.drivers[].team.logo` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.drivers[].points` | `number?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.drivers[].wins` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.drivers[].behind` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.drivers[].season` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.drivers[].time` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.drivers[].laps` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.drivers[].grid` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.drivers[].gap` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.constructors` | `object[]?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.constructors[].position` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.constructors[].driver` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.constructors[].driver.id` | `int` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].formula1.constructors[].driver.name` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.constructors[].driver.abbr` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.constructors[].driver.number` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.constructors[].driver.image` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.constructors[].team` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.constructors[].team.id` | `int` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].formula1.constructors[].team.name` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.constructors[].team.logo` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.constructors[].points` | `number?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.constructors[].wins` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.constructors[].behind` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.constructors[].season` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.constructors[].time` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.constructors[].laps` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.constructors[].grid` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].formula1.constructors[].gap` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].news` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].news.is_topical` | `bool` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].news.topic_id` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].news.topics` | `object[]?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].news.topics[].index` | `int` | Index scoped to the enclosing vendor model; no general cross-model ordering guarantee. |
| `results[].news.topics[].title` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].news.topics[].score` | `number` | Domain-specific score within this variant; scale, meaning and cross-query comparability are not specified. Sports objects contain game scoring, not relevance. |
| `results[].news.topics[].query` | `string` | Variant-specific query data; no automatic follow-up search is authorized. |
| `results[].news.articles` | `object[]?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].news.articles[].title` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].news.articles[].img` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].news.articles[].img_small` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].news.articles[].url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `results[].news.articles[].meta_url` | `object` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].news.articles[].meta_url.scheme` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].news.articles[].meta_url.netloc` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].news.articles[].meta_url.hostname` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].news.articles[].meta_url.favicon` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].news.articles[].meta_url.path` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].news.articles[].publisher_name` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].news.articles[].publish_time` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].news.articles[].page_age` | `string?` | Content publication/modification date, not necessarily crawl time. |
| `results[].news.articles[].is_live` | `bool?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].news.articles[].description` | `string?` | Display description/excerpt; POI Descriptions text is generated and Context source description is query-independent. |
| `results[].news.articles[].extra_snippets` | `string[]?` | Alternative page excerpts, not continuous full text. |
| `results[].news.articles[].bo_debug` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].packagetracker` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].packagetracker.matched` | `object[]?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].packagetracker.matched[].tracking_number` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].packagetracker.matched[].tracking_url` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].packagetracker.matched[].description` | `string?` | Display description/excerpt; POI Descriptions text is generated and Context source description is query-independent. |
| `results[].packagetracker.matched[].courier` | `object` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].packagetracker.matched[].courier.name` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].packagetracker.matched[].courier.service` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].packagetracker.matched[].courier.code` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].packagetracker.matched[].courier.domain` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].packagetracker.matched[].courier.thumbnail` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].american_football` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].american_football.sport` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].american_football.type` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].american_football.view` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].american_football.content` | `any` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].american_football.content.type` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].american_football.content.league` | `object` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].american_football.content.league.id` | `any` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].american_football.content.league.name` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].american_football.content.league.logo` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].american_football.content.league.season` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].american_football.content.league.round` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].american_football.content.league.has_standings` | `bool?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].american_football.content.game_times` | `int[]` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].american_football.content.game_time` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].american_football.content.date` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].american_football.content.date_kind` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].american_football.content.games` | `object[]` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].american_football.content.games[].id` | `any` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].american_football.content.games[].teams` | `object` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].american_football.content.games[].teams.home` | `object` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].american_football.content.games[].teams.home.id` | `any?` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].american_football.content.games[].teams.home.nickname` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].american_football.content.games[].teams.home.location` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].american_football.content.games[].teams.home.name` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].american_football.content.games[].teams.home.logo` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].american_football.content.games[].teams.away` | `object` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].american_football.content.games[].teams.away.id` | `any?` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].american_football.content.games[].teams.away.nickname` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].american_football.content.games[].teams.away.location` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].american_football.content.games[].teams.away.name` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].american_football.content.games[].teams.away.logo` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].american_football.content.games[].score` | `object?` | Domain-specific score within this variant; scale, meaning and cross-query comparability are not specified. Sports objects contain game scoring, not relevance. |
| `results[].american_football.content.games[].score.home` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].american_football.content.games[].score.away` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].american_football.content.games[].start_time` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].american_football.content.games[].status` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].american_football.content.games[].status.code` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].american_football.content.games[].status.elapsed` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].american_football.content.games[].video` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].american_football.content.games[].video.duration` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].american_football.content.games[].video.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `results[].american_football.content.games[].video.thumbnail` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].american_football.content.games[].video.type` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].american_football.content.games[].score_table` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].american_football.content.games[].score_table.q1` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].american_football.content.games[].score_table.q1.home` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].american_football.content.games[].score_table.q1.away` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].american_football.content.games[].score_table.q2` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].american_football.content.games[].score_table.q2.home` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].american_football.content.games[].score_table.q2.away` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].american_football.content.games[].score_table.q3` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].american_football.content.games[].score_table.q3.home` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].american_football.content.games[].score_table.q3.away` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].american_football.content.games[].score_table.q4` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].american_football.content.games[].score_table.q4.home` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].american_football.content.games[].score_table.q4.away` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].american_football.content.games[].score_table.ot` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].american_football.content.games[].score_table.ot.home` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].american_football.content.games[].score_table.ot.away` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].american_football.content.games[].phase` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].football` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].football.sport` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].football.type` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].football.view` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].football.content` | `any` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].football.content.type` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].football.content.league` | `object` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].football.content.league.id` | `any` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].football.content.league.name` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].football.content.league.logo` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].football.content.league.season` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].football.content.league.round` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].football.content.league.has_standings` | `bool?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].football.content.id` | `any` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].football.content.start_time` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].football.content.teams` | `object` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].football.content.teams.home` | `object` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].football.content.teams.home.id` | `any?` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].football.content.teams.home.nickname` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].football.content.teams.home.location` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].football.content.teams.home.name` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].football.content.teams.home.logo` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].football.content.teams.away` | `object` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].football.content.teams.away.id` | `any?` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].football.content.teams.away.nickname` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].football.content.teams.away.location` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].football.content.teams.away.name` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].football.content.teams.away.logo` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].football.content.status` | `object` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].football.content.status.code` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].football.content.status.elapsed` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].football.content.status.extra` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].football.content.score` | `object?` | Domain-specific score within this variant; scale, meaning and cross-query comparability are not specified. Sports objects contain game scoring, not relevance. |
| `results[].football.content.score.home` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].football.content.score.away` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].football.content.ps_score` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].football.content.ps_score.home` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].football.content.ps_score.away` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].football.content.statistics` | `object[]?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].football.content.statistics[].name` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].football.content.statistics[].home` | `any?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].football.content.statistics[].away` | `any?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].football.content.events` | `object[]?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].football.content.events[].team_id` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].football.content.events[].type` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].football.content.events[].detail` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].football.content.events[].player_name` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].football.content.events[].assist_name` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].football.content.events[].elapsed` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].football.content.events[].extra` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].football.content.lineups` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].football.content.lineups.home` | `object` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].football.content.lineups.home.formation` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].football.content.lineups.home.startXI` | `object[]` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].football.content.lineups.home.startXI[].name` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].football.content.lineups.home.startXI[].number` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].football.content.lineups.home.startXI[].pos` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].football.content.lineups.home.startXI[].grid` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].football.content.lineups.home.startXI[].photo` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].football.content.lineups.home.substitutes` | `object[]` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].football.content.lineups.home.substitutes[].name` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].football.content.lineups.home.substitutes[].number` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].football.content.lineups.home.substitutes[].pos` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].football.content.lineups.home.substitutes[].grid` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].football.content.lineups.home.substitutes[].photo` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].football.content.lineups.home.coach` | `object` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].football.content.lineups.home.coach.name` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].football.content.lineups.home.coach.photo` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].football.content.lineups.away` | `object` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].football.content.lineups.away.formation` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].football.content.lineups.away.startXI` | `object[]` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].football.content.lineups.away.startXI[].name` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].football.content.lineups.away.startXI[].number` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].football.content.lineups.away.startXI[].pos` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].football.content.lineups.away.startXI[].grid` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].football.content.lineups.away.startXI[].photo` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].football.content.lineups.away.substitutes` | `object[]` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].football.content.lineups.away.substitutes[].name` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].football.content.lineups.away.substitutes[].number` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].football.content.lineups.away.substitutes[].pos` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].football.content.lineups.away.substitutes[].grid` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].football.content.lineups.away.substitutes[].photo` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].football.content.lineups.away.coach` | `object` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].football.content.lineups.away.coach.name` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].football.content.lineups.away.coach.photo` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].football.content.aggregate_score` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].football.content.aggregate_score.scores` | `object` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].baseball` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].baseball.sport` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].baseball.type` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].baseball.view` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].baseball.content` | `any` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].baseball.content.type` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].baseball.content.league` | `object` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].baseball.content.league.id` | `any` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].baseball.content.league.name` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].baseball.content.league.logo` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].baseball.content.league.season` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].baseball.content.league.round` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].baseball.content.league.has_standings` | `bool?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].baseball.content.game_times` | `int[]` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].baseball.content.game_time` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].baseball.content.date` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].baseball.content.date_kind` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].baseball.content.games` | `object[]` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].baseball.content.games[].id` | `any` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].baseball.content.games[].teams` | `object` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].baseball.content.games[].teams.home` | `object` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].baseball.content.games[].teams.home.id` | `any?` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].baseball.content.games[].teams.home.nickname` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].baseball.content.games[].teams.home.location` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].baseball.content.games[].teams.home.name` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].baseball.content.games[].teams.home.logo` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].baseball.content.games[].teams.away` | `object` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].baseball.content.games[].teams.away.id` | `any?` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].baseball.content.games[].teams.away.nickname` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].baseball.content.games[].teams.away.location` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].baseball.content.games[].teams.away.name` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].baseball.content.games[].teams.away.logo` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].baseball.content.games[].score` | `object?` | Domain-specific score within this variant; scale, meaning and cross-query comparability are not specified. Sports objects contain game scoring, not relevance. |
| `results[].baseball.content.games[].score.home` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].baseball.content.games[].score.away` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].baseball.content.games[].start_time` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].baseball.content.games[].status` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].baseball.content.games[].status.code` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].baseball.content.games[].video` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].baseball.content.games[].video.duration` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].baseball.content.games[].video.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `results[].baseball.content.games[].video.thumbnail` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].baseball.content.games[].video.type` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].basketball` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].basketball.sport` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].basketball.type` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].basketball.view` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].basketball.content` | `any` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].basketball.content.type` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].basketball.content.league` | `object` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].basketball.content.league.id` | `any` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].basketball.content.league.name` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].basketball.content.league.logo` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].basketball.content.league.season` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].basketball.content.league.round` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].basketball.content.league.has_standings` | `bool?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].basketball.content.game_times` | `int[]` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].basketball.content.game_time` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].basketball.content.date` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].basketball.content.date_kind` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].basketball.content.games` | `object[]` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].basketball.content.games[].id` | `any` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].basketball.content.games[].teams` | `object` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].basketball.content.games[].teams.home` | `object` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].basketball.content.games[].teams.home.id` | `any?` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].basketball.content.games[].teams.home.nickname` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].basketball.content.games[].teams.home.location` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].basketball.content.games[].teams.home.name` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].basketball.content.games[].teams.home.logo` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].basketball.content.games[].teams.away` | `object` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].basketball.content.games[].teams.away.id` | `any?` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].basketball.content.games[].teams.away.nickname` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].basketball.content.games[].teams.away.location` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].basketball.content.games[].teams.away.name` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].basketball.content.games[].teams.away.logo` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].basketball.content.games[].score` | `object?` | Domain-specific score within this variant; scale, meaning and cross-query comparability are not specified. Sports objects contain game scoring, not relevance. |
| `results[].basketball.content.games[].score.home` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].basketball.content.games[].score.away` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].basketball.content.games[].start_time` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].basketball.content.games[].status` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].basketball.content.games[].status.code` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].basketball.content.games[].video` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].basketball.content.games[].video.duration` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].basketball.content.games[].video.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `results[].basketball.content.games[].video.thumbnail` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].basketball.content.games[].video.type` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].basketball.content.games[].score_table` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].basketball.content.games[].score_table.q1` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].basketball.content.games[].score_table.q1.home` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].basketball.content.games[].score_table.q1.away` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].basketball.content.games[].score_table.q2` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].basketball.content.games[].score_table.q2.home` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].basketball.content.games[].score_table.q2.away` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].basketball.content.games[].score_table.q3` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].basketball.content.games[].score_table.q3.home` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].basketball.content.games[].score_table.q3.away` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].basketball.content.games[].score_table.q4` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].basketball.content.games[].score_table.q4.home` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].basketball.content.games[].score_table.q4.away` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].basketball.content.games[].score_table.ot` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].basketball.content.games[].score_table.ot.home` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].basketball.content.games[].score_table.ot.away` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].ice_hockey` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].ice_hockey.sport` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].ice_hockey.type` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].ice_hockey.view` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].ice_hockey.content` | `any` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].ice_hockey.content.type` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].ice_hockey.content.league` | `object` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].ice_hockey.content.league.id` | `any` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].ice_hockey.content.league.name` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].ice_hockey.content.league.logo` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].ice_hockey.content.league.season` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].ice_hockey.content.league.round` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].ice_hockey.content.league.has_standings` | `bool?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].ice_hockey.content.game_times` | `int[]` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].ice_hockey.content.game_time` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].ice_hockey.content.date` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].ice_hockey.content.date_kind` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].ice_hockey.content.games` | `object[]` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].ice_hockey.content.games[].id` | `any` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].ice_hockey.content.games[].teams` | `object` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].ice_hockey.content.games[].teams.home` | `object` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].ice_hockey.content.games[].teams.home.id` | `any?` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].ice_hockey.content.games[].teams.home.nickname` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].ice_hockey.content.games[].teams.home.location` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].ice_hockey.content.games[].teams.home.name` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].ice_hockey.content.games[].teams.home.logo` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].ice_hockey.content.games[].teams.away` | `object` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].ice_hockey.content.games[].teams.away.id` | `any?` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].ice_hockey.content.games[].teams.away.nickname` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].ice_hockey.content.games[].teams.away.location` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].ice_hockey.content.games[].teams.away.name` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].ice_hockey.content.games[].teams.away.logo` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].ice_hockey.content.games[].score` | `object?` | Domain-specific score within this variant; scale, meaning and cross-query comparability are not specified. Sports objects contain game scoring, not relevance. |
| `results[].ice_hockey.content.games[].score.home` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].ice_hockey.content.games[].score.away` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].ice_hockey.content.games[].start_time` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].ice_hockey.content.games[].status` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].ice_hockey.content.games[].status.code` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].ice_hockey.content.games[].status.elapsed` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].ice_hockey.content.games[].video` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].ice_hockey.content.games[].video.duration` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].ice_hockey.content.games[].video.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `results[].ice_hockey.content.games[].video.thumbnail` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].ice_hockey.content.games[].video.type` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].ice_hockey.content.games[].score_table` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].ice_hockey.content.games[].score_table.p1` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].ice_hockey.content.games[].score_table.p1.home` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].ice_hockey.content.games[].score_table.p1.away` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].ice_hockey.content.games[].score_table.p2` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].ice_hockey.content.games[].score_table.p2.home` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].ice_hockey.content.games[].score_table.p2.away` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].ice_hockey.content.games[].score_table.p3` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].ice_hockey.content.games[].score_table.p3.home` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].ice_hockey.content.games[].score_table.p3.away` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].ice_hockey.content.games[].score_table.ot` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].ice_hockey.content.games[].score_table.ot.home` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].ice_hockey.content.games[].score_table.ot.away` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].ice_hockey.content.games[].score_table.so` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].ice_hockey.content.games[].score_table.so.home` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].ice_hockey.content.games[].score_table.so.away` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].ice_hockey.content.games[].league_id` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cricket` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cricket.sport` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cricket.type` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cricket.view` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cricket.content` | `any` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cricket.content.type` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cricket.content.league` | `object` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cricket.content.league.id` | `any` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].cricket.content.league.name` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cricket.content.league.logo` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cricket.content.league.season` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cricket.content.league.round` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cricket.content.league.has_standings` | `bool?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cricket.content.game_times` | `int[]` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cricket.content.game_time` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cricket.content.date` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cricket.content.date_kind` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cricket.content.games` | `object[]` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cricket.content.games[].id` | `any` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].cricket.content.games[].teams` | `object` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cricket.content.games[].teams.home` | `object` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cricket.content.games[].teams.home.id` | `any?` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].cricket.content.games[].teams.home.nickname` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cricket.content.games[].teams.home.location` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cricket.content.games[].teams.home.name` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cricket.content.games[].teams.home.logo` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cricket.content.games[].teams.away` | `object` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cricket.content.games[].teams.away.id` | `any?` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].cricket.content.games[].teams.away.nickname` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cricket.content.games[].teams.away.location` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cricket.content.games[].teams.away.name` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cricket.content.games[].teams.away.logo` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cricket.content.games[].score` | `object?` | Domain-specific score within this variant; scale, meaning and cross-query comparability are not specified. Sports objects contain game scoring, not relevance. |
| `results[].cricket.content.games[].score.home` | `any` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cricket.content.games[].score.away` | `any` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cricket.content.games[].score.home_runs` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cricket.content.games[].score.home_overs` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cricket.content.games[].score.away_runs` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cricket.content.games[].score.away_overs` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cricket.content.games[].start_time` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cricket.content.games[].status` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cricket.content.games[].status.code` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cricket.content.games[].video` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cricket.content.games[].video.duration` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cricket.content.games[].video.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `results[].cricket.content.games[].video.thumbnail` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].cricket.content.games[].video.type` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].stock` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].stock.asset_info` | `object` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].stock.asset_info.symbol` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].stock.asset_info.name` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].stock.asset_info.category` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].stock.asset_info.currency` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].stock.asset_info.country` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].stock.asset_info.country_code` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].stock.asset_info.exchange` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].stock.asset_info.type` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].stock.asset_info.mic_code` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].stock.asset_info.score` | `number?` | Domain-specific score within this variant; scale, meaning and cross-query comparability are not specified. Sports objects contain game scoring, not relevance. |
| `results[].stock.time_range` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].stock.quote` | `object` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].stock.quote.symbol` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].stock.quote.company_name` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].stock.quote.primary_exchange` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].stock.quote.market_cap` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].stock.quote.pe_ratio` | `number?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].stock.quote.week_52_high` | `number?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].stock.quote.week_52_low` | `number?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].stock.quote.latest_price` | `number` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].stock.quote.open` | `number?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].stock.quote.close` | `number` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].stock.quote.high` | `number?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].stock.quote.low` | `number?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].stock.quote.change` | `number?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].stock.quote.change_percent` | `number?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].stock.quote.volume` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].stock.quote.latest_update` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].stock.quote.currency` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].stock.timeseries` | `object` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].stock.timeseries.time_range` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].stock.timeseries.timeseries` | `object[]` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].stock.timeseries.timeseries[].date` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].stock.timeseries.timeseries[].minute` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].stock.timeseries.timeseries[].label` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].stock.timeseries.timeseries[].timestamp` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].stock.timeseries.timeseries[].high` | `number?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].stock.timeseries.timeseries[].low` | `number?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].stock.timeseries.timeseries[].close` | `number?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].stock.timeseries.timeseries[].previous_close` | `number?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].stock.timeseries.timeseries[].average` | `number?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].stock.timeseries.timeseries[].volume` | `number?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].stock.exchange_info` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].stock.exchange_info.timezone` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].stock.exchange_info.open_time` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].stock.exchange_info.close_time` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].unitconversion` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].unitconversion.amount` | `number` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].unitconversion.from_unit` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].unitconversion.to_unit` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].unitconversion.dimensionality` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].unixtimestamp` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].unixtimestamp.conversion` | `any` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].unixtimestamp.conversion.intent_type` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].unixtimestamp.conversion.input_ts` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.location` | `object` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.location.id` | `int` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].weather.location.name` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.location.country` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.location.state` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.location.coords` | `object` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.location.coords.lat` | `number` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.location.coords.lon` | `number` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.location.population` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.location.sunrise` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.location.sunset` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.location.tzoffset` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.location.implicit_location` | `bool?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.current_time_iso` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.current_weather` | `object` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.current_weather.ts` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.current_weather.sunrise` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.current_weather.sunset` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.current_weather.temp` | `number` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.current_weather.feels_like` | `number` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.current_weather.pressure` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.current_weather.humidity` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.current_weather.dew_point` | `number` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.current_weather.uvi` | `number` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.current_weather.clouds` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.current_weather.visibility` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.current_weather.wind` | `object` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.current_weather.wind.speed` | `number` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.current_weather.wind.deg` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.current_weather.wind.gust` | `number?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.current_weather.weather` | `object` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.current_weather.weather.id` | `int` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].weather.current_weather.weather.main` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.current_weather.weather.description` | `string` | Display description/excerpt; POI Descriptions text is generated and Context source description is query-independent. |
| `results[].weather.current_weather.weather.icon` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.current_weather.rain` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.current_weather.snow` | `int?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.daily` | `object[]` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.daily[].ts` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.daily[].date_i18n` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.daily[].sunrise` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.daily[].sunset` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.daily[].moonrise` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.daily[].moonset` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.daily[].temperature` | `object` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.daily[].temperature.day` | `number` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.daily[].temperature.min` | `number` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.daily[].temperature.max` | `number` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.daily[].temperature.night` | `number` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.daily[].temperature.evening` | `number` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.daily[].temperature.morning` | `number` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.daily[].feels_like` | `object` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.daily[].feels_like.day` | `number` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.daily[].feels_like.night` | `number` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.daily[].feels_like.evening` | `number` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.daily[].feels_like.morning` | `number` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.daily[].pressure` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.daily[].humidity` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.daily[].dew_point` | `number` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.daily[].wind` | `object` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.daily[].wind.speed` | `number` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.daily[].wind.deg` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.daily[].wind.gust` | `number?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.daily[].clouds` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.daily[].pop` | `number` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.daily[].rain` | `number?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.daily[].uvi` | `number` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.daily[].weather` | `object` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.daily[].weather.id` | `int` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].weather.daily[].weather.main` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.daily[].weather.description` | `string` | Display description/excerpt; POI Descriptions text is generated and Context source description is query-independent. |
| `results[].weather.daily[].weather.icon` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.hours3` | `object[]` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.hours3[].ts` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.hours3[].temperature` | `object` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.hours3[].temperature.temp` | `number` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.hours3[].temperature.feels_like` | `number` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.hours3[].temperature.min` | `number` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.hours3[].temperature.max` | `number` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.hours3[].pressure` | `int` | Pressure metadata/value within this model. |
| `results[].weather.hours3[].weather` | `object` | Weather metadata/value within this model. |
| `results[].weather.hours3[].weather.id` | `int` | Identifier for the enclosing asset/entity/event; no expiry is documented for this rich field. |
| `results[].weather.hours3[].weather.main` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.hours3[].weather.description` | `string` | Display description/excerpt; POI Descriptions text is generated and Context source description is query-independent. |
| `results[].weather.hours3[].weather.icon` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.hours3[].wind` | `object` | Wind metadata/value within this model. |
| `results[].weather.hours3[].wind.speed` | `number` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.hours3[].wind.deg` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.hours3[].wind.gust` | `number?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.hours3[].humidity` | `int` | Humidity metadata/value within this model. |
| `results[].weather.hours3[].clouds` | `int` | Clouds metadata/value within this model. |
| `results[].weather.hours3[].pop` | `number` | Pop metadata/value within this model. |
| `results[].weather.hours3[].visibility` | `int?` | Visibility metadata/value within this model. |
| `results[].weather.hours3[].rain` | `number?` | Rain metadata/value within this model. |
| `results[].weather.hours3[].snow` | `number?` | Snow metadata/value within this model. |
| `results[].weather.alerts` | `object[]?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.alerts[].sender` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.alerts[].event` | `string` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.alerts[].ts_start` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.alerts[].ts_end` | `int` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.alerts[].start_relative_i18n` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results[].weather.alerts[].description` | `string` | Display description/excerpt; POI Descriptions text is generated and Context source description is query-independent. |
| `results[].weather.alerts[].tags` | `string[]` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `response_callback_info` | `object?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `response_callback_info.vertical` | `string?` | Vertical metadata/value within this model. |
| `response_callback_info.callback_key` | `string` | Opaque key for explicit fixed-endpoint Rich Search continuation. |
| `response_callback_info.callback_status` | `string` | success indicates the callback succeeded; failure indicates a callback failure even within HTTP 200. |
| `response_callback_info.search_lang` | `string?` | Search lang metadata/value within this model. |
