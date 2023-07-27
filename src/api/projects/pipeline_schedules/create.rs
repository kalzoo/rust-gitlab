// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::str::FromStr;

use derive_builder::Builder;
use thiserror::Error;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;
use crate::api::ParamValue;

/// Errors when parsing cron sequences.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum PipelineScheduleCronError {
    /// Failure to parse a cron expression.
    #[error("parse error: {}", reason)]
    ParseError {
        /// The reason for the parse error.
        reason: String,
    },
}

/// A cron schedule for a pipeline.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineScheduleCron {
    cron: String,
}

impl PipelineScheduleCron {
    /// Create a cron expression from a string.
    pub fn new<E>(expression: E) -> Result<Self, PipelineScheduleCronError>
    where
        E: AsRef<str>,
    {
        Self::new_impl(expression.as_ref())
    }

    fn new_impl(expression: &str) -> Result<Self, PipelineScheduleCronError> {
        if cron::Schedule::from_str(expression).is_err() {
            // Not needed if seconds ever become optional. https://github.com/zslayton/cron/issues/13
            let compat_expression = format!("* {}", expression);
            let _ = cron::Schedule::from_str(&compat_expression).map_err(|err| {
                PipelineScheduleCronError::ParseError {
                    reason: err.to_string(),
                }
            })?;
        };

        Ok(Self {
            cron: expression.into(),
        })
    }
}

impl<'a> ParamValue<'a> for &'a PipelineScheduleCron {
    fn as_value(&self) -> Cow<'a, str> {
        self.cron.as_str().into()
    }
}

/// Timezone selection for a pipeline schedule.
///
/// GitLab uses [ActiveRecord's TimeZone names][activerecord-timezone] to map to official
/// timezones.
///
/// https://api.rubyonrails.org/classes/ActiveSupport/TimeZone.html
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum PipelineScheduleTimeZone<'a> {
    /// Etc/GMT+12
    InternationalDateLineWest,
    /// Pacific/Pago_Pago
    AmericanSamoa,
    /// Pacific/Midway
    MidwayIsland,
    /// Pacific/Honolulu
    Hawaii,
    /// America/Juneau
    Alaska,
    /// America/Los_Angeles
    PacificTimeUSCanada,
    /// America/Tijuana
    Tijuana,
    /// America/Phoenix
    Arizona,
    /// America/Mazatlan
    Mazatlan,
    /// America/Denver
    MountainTimeUSCanada,
    /// America/Guatemala
    CentralAmerica,
    /// America/Chicago
    CentralTimeUSCanada,
    /// America/Chihuahua
    Chihuahua,
    /// America/Mexico_City
    Guadalajara,
    /// America/Mexico_City
    MexicoCity,
    /// America/Monterrey
    Monterrey,
    /// America/Regina
    Saskatchewan,
    /// America/Bogota
    Bogota,
    /// America/New_York
    EasternTimeUSCanada,
    /// America/Indiana/Indianapolis
    IndianaEast,
    /// America/Lima
    Lima,
    /// America/Lima
    Quito,
    /// America/Halifax
    AtlanticTimeCanada,
    /// America/Caracas
    Caracas,
    /// America/Guyana
    Georgetown,
    /// America/La_Paz
    LaPaz,
    /// America/Puerto_Rico
    PuertoRico,
    /// America/Santiago
    Santiago,
    /// America/St_Johns
    Newfoundland,
    /// America/Sao_Paulo
    Brasilia,
    /// America/Argentina/Buenos_Aires
    BuenosAires,
    /// America/Godthab
    Greenland,
    /// America/Montevideo
    Montevideo,
    /// Atlantic/South_Georgia
    MidAtlantic,
    /// Atlantic/Azores
    Azores,
    /// Atlantic/Cape_Verde
    CapeVerdeIslands,
    /// Europe/London
    Edinburgh,
    /// Europe/Lisbon
    Lisbon,
    /// Europe/London
    London,
    /// Africa/Monrovia
    Monrovia,
    /// Etc/UTC
    UTC,
    /// Europe/Amsterdam
    Amsterdam,
    /// Europe/Belgrade
    Belgrade,
    /// Europe/Berlin
    Berlin,
    /// Europe/Zurich
    Bern,
    /// Europe/Bratislava
    Bratislava,
    /// Europe/Brussels
    Brussels,
    /// Europe/Budapest
    Budapest,
    /// Africa/Casablanca
    Casablanca,
    /// Europe/Copenhagen
    Copenhagen,
    /// Europe/Dublin
    Dublin,
    /// Europe/Ljubljana
    Ljubljana,
    /// Europe/Madrid
    Madrid,
    /// Europe/Paris
    Paris,
    /// Europe/Prague
    Prague,
    /// Europe/Rome
    Rome,
    /// Europe/Sarajevo
    Sarajevo,
    /// Europe/Skopje
    Skopje,
    /// Europe/Stockholm
    Stockholm,
    /// Europe/Vienna
    Vienna,
    /// Europe/Warsaw
    Warsaw,
    /// Africa/Algiers
    WestCentralAfrica,
    /// Europe/Zagreb
    Zagreb,
    /// Europe/Zurich
    Zurich,
    /// Europe/Athens
    Athens,
    /// Europe/Bucharest
    Bucharest,
    /// Africa/Cairo
    Cairo,
    /// Africa/Harare
    Harare,
    /// Europe/Helsinki
    Helsinki,
    /// Asia/Jerusalem
    Jerusalem,
    /// Asia/Kaliningrad
    Kaliningrad,
    /// Europe/Kiev
    Kyiv,
    /// Africa/Johannesburg
    Pretoria,
    /// Europe/Eiga
    Riga,
    /// Europe/Sofia
    Sofia,
    /// Europe/Tallinn
    Tallinn,
    /// Europe/Vilnius
    Vilnius,
    /// Asia/Baghdad
    Baghdad,
    /// Europe/Istanbul
    Istanbul,
    /// Asia/Kuwait
    Kuwait,
    /// Europe/Minsk
    Minsk,
    /// Europe/Moscow
    Moscow,
    /// Asia/Nairobi
    Nairobi,
    /// Asia/Riyadh
    Riyadh,
    /// Europe/Moscow
    StPetersburg,
    /// Europe/Volgograd
    Volgograd,
    /// Asia/Tehran
    Tehran,
    /// Asia/Muscat
    AbuDhabi,
    /// Asia/Baku
    Baku,
    /// Asia/Muscat
    Muscat,
    /// Europe/Samara
    Samara,
    /// Asia/Tbilisi
    Tbilisi,
    /// Asia/Yerevan
    Yerevan,
    /// Asia/Kabul
    Kabul,
    /// Asia/Yekaterinburg
    Ekaterinburg,
    /// Asia/Karachi
    Islamabad,
    /// Asia/Karachi
    Karachi,
    /// Asia/Tashkent
    Tashkent,
    /// Asia/Kolkata
    Chennai,
    /// Asia/Kolkata
    Kolkata,
    /// Asia/Kolkata
    Mumbai,
    /// Asia/Kolkata
    NewDelhi,
    /// Asia/Colombo
    SriJayawardenepura,
    /// Asia/Kathmandu
    Kathmandu,
    /// Asia/Almaty
    Almaty,
    /// Asia/Dhaka
    Astana,
    /// Asia/Dhaka
    Dhaka,
    /// Asia/Urumqi
    Urumqi,
    /// Asia/Rangoon
    Rangoon,
    /// Asia/Bangkok
    Bangkok,
    /// Asia/Bangkok
    Hanoi,
    /// Asia/Jakarta
    Jakarta,
    /// Asia/Krasnoyarsk
    Krasnoyarsk,
    /// Asia/Novosibirsk
    Novosibirsk,
    /// Asia/Shanghai
    Beijing,
    /// Asia/Chongqing
    Chongqing,
    /// Asia/Hong_Kong
    HongKong,
    /// Asia/Irkutsk
    Irkutsk,
    /// Asia/Kuala_Lumpur
    KualaLumpur,
    /// Australia/Perth
    Perth,
    /// Asia/Singapore
    Singapore,
    /// Asia/Taipei
    Taipei,
    /// Asia/Ulaanbaatar
    Ulaanbaatar,
    /// Asia/Tokyo
    Osaka,
    /// Asia/Tokyo
    Sapporo,
    /// Asia/Seoul
    Seoul,
    /// Asia/Tokyo
    Tokyo,
    /// Asia/Yakutsk
    Yakutsk,
    ///Australia/Adelaide
    Adelaide,
    ///Australia/Darwin
    Darwin,
    ///Australia/Brisbane
    Brisbane,
    ///Australia/Melbourne
    Canberra,
    /// Pacific/Guam
    Guam,
    /// Australia/Hobart
    Hobart,
    ///Australia/Melbourne
    Melbourne,
    /// Pacific/Port_Moresby
    PortMoresby,
    /// Australia/Sydney
    Sydney,
    /// Asia/Vladivostok
    Vladivostok,
    /// Asia/Magadan
    Magadan,
    /// Pacific/Noumea
    NewCaledonia,
    /// Pacific/Guadalcanal
    SolomonIslands,
    /// Asia/Srednekolymsk
    Srednekolymsk,
    /// Pacific/Auckland
    Auckland,
    /// Pacific/Fiji
    Fiji,
    /// Asia/Kamchatka
    Kamchatka,
    /// Pacific/Majuro
    MarshallIslands,
    /// Pacific/Auckland
    Wellington,
    /// Pacific/Chatham
    ChathamIslands,
    /// Pacific/Tongatapu
    Nukualofa,
    /// Pacific/Apia
    Samoa,
    /// Pacific/Fakaofo
    TokelauIslands,
    /// Coverage of any unhandled time zone.
    Custom(Cow<'a, str>),
}

impl<'a> PipelineScheduleTimeZone<'a> {
    fn as_str(&self) -> &str {
        match *self {
            Self::InternationalDateLineWest => "International Date Line West",
            Self::AmericanSamoa => "American Samoa",
            Self::MidwayIsland => "Midway Island",
            Self::Hawaii => "Hawaii",
            Self::Alaska => "Alaska",
            Self::PacificTimeUSCanada => "Pacific Time (US & Canada)",
            Self::Tijuana => "Tijuana",
            Self::Arizona => "Arizona",
            Self::Mazatlan => "Mazatlan",
            Self::MountainTimeUSCanada => "Mountain Time (US & Canada)",
            Self::CentralAmerica => "Central America",
            Self::CentralTimeUSCanada => "Central Time (US & Canada)",
            Self::Chihuahua => "Chihuahua",
            Self::Guadalajara => "Guadalajara",
            Self::MexicoCity => "Mexico City",
            Self::Monterrey => "Monterrey",
            Self::Saskatchewan => "Saskatchewan",
            Self::Bogota => "Bogota",
            Self::EasternTimeUSCanada => "Eastern Time (US & Canada)",
            Self::IndianaEast => "Indiana (East)",
            Self::Lima => "Lima",
            Self::Quito => "Quito",
            Self::AtlanticTimeCanada => "Atlantic Time (Canada)",
            Self::Caracas => "Caracas",
            Self::Georgetown => "Georgetown",
            Self::LaPaz => "La Paz",
            Self::PuertoRico => "Puerto Rico",
            Self::Santiago => "Santiago",
            Self::Newfoundland => "Newfoundland",
            Self::Brasilia => "Brasilia",
            Self::BuenosAires => "Buenos Aires",
            Self::Greenland => "Greenland",
            Self::Montevideo => "Montevideo",
            Self::MidAtlantic => "Mid-Atlantic",
            Self::Azores => "Azores",
            Self::CapeVerdeIslands => "Cape Verde Is.",
            Self::Edinburgh => "Edinburgh",
            Self::Lisbon => "Lisbon",
            Self::London => "London",
            Self::Monrovia => "Monrovia",
            Self::UTC => "UTC",
            Self::Amsterdam => "Amsterdam",
            Self::Belgrade => "Belgrade",
            Self::Berlin => "Berlin",
            Self::Bern => "Bern",
            Self::Bratislava => "Bratislava",
            Self::Brussels => "Brussels",
            Self::Budapest => "Budapest",
            Self::Casablanca => "Casablanca",
            Self::Copenhagen => "Copenhagen",
            Self::Dublin => "Dublin",
            Self::Ljubljana => "Ljubljana",
            Self::Madrid => "Madrid",
            Self::Paris => "Paris",
            Self::Prague => "Prague",
            Self::Rome => "Rome",
            Self::Sarajevo => "Sarajevo",
            Self::Skopje => "Skopje",
            Self::Stockholm => "Stockholm",
            Self::Vienna => "Vienna",
            Self::Warsaw => "Warsaw",
            Self::WestCentralAfrica => "West Central Africa",
            Self::Zagreb => "Zagreb",
            Self::Zurich => "Zurich",
            Self::Athens => "Athens",
            Self::Bucharest => "Bucharest",
            Self::Cairo => "Cairo",
            Self::Harare => "Harare",
            Self::Helsinki => "Helsinki",
            Self::Jerusalem => "Jerusalem",
            Self::Kaliningrad => "Kaliningrad",
            Self::Kyiv => "Kyiv",
            Self::Pretoria => "Pretoria",
            Self::Riga => "Riga",
            Self::Sofia => "Sofia",
            Self::Tallinn => "Tallinn",
            Self::Vilnius => "Vilnius",
            Self::Baghdad => "Baghdad",
            Self::Istanbul => "Istanbul",
            Self::Kuwait => "Kuwait",
            Self::Minsk => "Minsk",
            Self::Moscow => "Moscow",
            Self::Nairobi => "Nairobi",
            Self::Riyadh => "Riyadh",
            Self::StPetersburg => "St. Petersburg",
            Self::Volgograd => "Volgograd",
            Self::Tehran => "Tehran",
            Self::AbuDhabi => "Abu Dhabi",
            Self::Baku => "Baku",
            Self::Muscat => "Muscat",
            Self::Samara => "Samara",
            Self::Tbilisi => "Tbilisi",
            Self::Yerevan => "Yerevan",
            Self::Kabul => "Kabul",
            Self::Ekaterinburg => "Ekaterinburg",
            Self::Islamabad => "Islamabad",
            Self::Karachi => "Karachi",
            Self::Tashkent => "Tashkent",
            Self::Chennai => "Chennai",
            Self::Kolkata => "Kolkata",
            Self::Mumbai => "Mumbai",
            Self::NewDelhi => "New Delhi",
            Self::SriJayawardenepura => "Sri Jayawardenepura",
            Self::Kathmandu => "Kathmandu",
            Self::Almaty => "Almaty",
            Self::Astana => "Astana",
            Self::Dhaka => "Dhaka",
            Self::Urumqi => "Urumqi",
            Self::Rangoon => "Rangoon",
            Self::Bangkok => "Bangkok",
            Self::Hanoi => "Hanoi",
            Self::Jakarta => "Jakarta",
            Self::Krasnoyarsk => "Krasnoyarsk",
            Self::Novosibirsk => "Novosibirsk",
            Self::Beijing => "Beijing",
            Self::Chongqing => "Chongqing",
            Self::HongKong => "Hong Kong",
            Self::Irkutsk => "Irkutsk",
            Self::KualaLumpur => "Kuala Lumpur",
            Self::Perth => "Perth",
            Self::Singapore => "Singapore",
            Self::Taipei => "Taipei",
            Self::Ulaanbaatar => "Ulaanbaatar",
            Self::Osaka => "Osaka",
            Self::Sapporo => "Sapporo",
            Self::Seoul => "Seoul",
            Self::Tokyo => "Tokyo",
            Self::Yakutsk => "Yakutsk",
            Self::Adelaide => "Adelaide",
            Self::Darwin => "Darwin",
            Self::Brisbane => "Brisbane",
            Self::Canberra => "Canberra",
            Self::Guam => "Guam",
            Self::Hobart => "Hobart",
            Self::Melbourne => "Melbourne",
            Self::PortMoresby => "Port Moresby",
            Self::Sydney => "Sydney",
            Self::Vladivostok => "Vladivostok",
            Self::Magadan => "Magadan",
            Self::NewCaledonia => "New Caledonia",
            Self::SolomonIslands => "Solomon Is.",
            Self::Srednekolymsk => "Srednekolymsk",
            Self::Auckland => "Auckland",
            Self::Fiji => "Fiji",
            Self::Kamchatka => "Kamchatka",
            Self::MarshallIslands => "Marshall Is.",
            Self::Wellington => "Wellington",
            Self::ChathamIslands => "Chatham Is.",
            Self::Nukualofa => "Nuku'alofa",
            Self::Samoa => "Samoa",
            Self::TokelauIslands => "Tokelau Is.",
            Self::Custom(ref s) => s.as_ref(),
        }
    }
}

impl<'a> ParamValue<'a> for &'a PipelineScheduleTimeZone<'a> {
    fn as_value(&self) -> Cow<'a, str> {
        self.as_str().into()
    }
}

/// Create a new pipeline schedule on a project.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct CreatePipelineSchedule<'a> {
    /// The project to create the pipeline schedule within.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The description of the pipeline schedule.
    #[builder(setter(into))]
    description: Cow<'a, str>,
    /// The ref to create the pipeline schedule for.
    #[builder(setter(into))]
    ref_: Cow<'a, str>,
    /// The `cron` schedule.
    cron: PipelineScheduleCron,

    /// The timezone to use.
    ///
    /// Defaults to `UTC`.
    #[builder(default)]
    cron_timezone: Option<PipelineScheduleTimeZone<'a>>,
    /// Whether the pipeline is active or not.
    #[builder(default)]
    active: Option<bool>,
}

impl<'a> CreatePipelineSchedule<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> CreatePipelineScheduleBuilder<'a> {
        CreatePipelineScheduleBuilder::default()
    }
}

impl<'a> Endpoint for CreatePipelineSchedule<'a> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/pipeline_schedules", self.project).into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = FormParams::default();

        params
            .push("description", &self.description)
            .push("ref", &self.ref_)
            .push("cron", &self.cron)
            .push_opt("cron_timezone", self.cron_timezone.as_ref())
            .push_opt("active", self.active);

        params.into_body()
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::api::projects::pipeline_schedules::{
        CreatePipelineSchedule, CreatePipelineScheduleBuilderError, PipelineScheduleCron,
        PipelineScheduleCronError, PipelineScheduleTimeZone,
    };
    use crate::api::{self, ParamValue, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn pipeline_schedule_cron_parse() {
        PipelineScheduleCron::new("0 1 * * *").unwrap();
        let PipelineScheduleCronError::ParseError {
            reason,
        } = PipelineScheduleCron::new("").unwrap_err();
        assert!(!reason.is_empty());
    }

    #[test]
    fn pipeline_schedule_cron_as_param() {
        let items = &[("0 1 * * *", "0 1 * * *"), ("* 4,5 * * *", "* 4,5 * * *")];

        for (i, s) in items {
            let cron = PipelineScheduleCron::new(i).unwrap();
            assert_eq!((&cron).as_value(), *s);
        }
    }

    #[test]
    fn pipeline_timezone_as_str() {
        let items = &[
            (
                PipelineScheduleTimeZone::InternationalDateLineWest,
                "International Date Line West",
            ),
            (PipelineScheduleTimeZone::AmericanSamoa, "American Samoa"),
            (PipelineScheduleTimeZone::MidwayIsland, "Midway Island"),
            (PipelineScheduleTimeZone::Hawaii, "Hawaii"),
            (PipelineScheduleTimeZone::Alaska, "Alaska"),
            (
                PipelineScheduleTimeZone::PacificTimeUSCanada,
                "Pacific Time (US & Canada)",
            ),
            (PipelineScheduleTimeZone::Tijuana, "Tijuana"),
            (PipelineScheduleTimeZone::Arizona, "Arizona"),
            (PipelineScheduleTimeZone::Mazatlan, "Mazatlan"),
            (
                PipelineScheduleTimeZone::MountainTimeUSCanada,
                "Mountain Time (US & Canada)",
            ),
            (PipelineScheduleTimeZone::CentralAmerica, "Central America"),
            (
                PipelineScheduleTimeZone::CentralTimeUSCanada,
                "Central Time (US & Canada)",
            ),
            (PipelineScheduleTimeZone::Chihuahua, "Chihuahua"),
            (PipelineScheduleTimeZone::Guadalajara, "Guadalajara"),
            (PipelineScheduleTimeZone::MexicoCity, "Mexico City"),
            (PipelineScheduleTimeZone::Monterrey, "Monterrey"),
            (PipelineScheduleTimeZone::Saskatchewan, "Saskatchewan"),
            (PipelineScheduleTimeZone::Bogota, "Bogota"),
            (
                PipelineScheduleTimeZone::EasternTimeUSCanada,
                "Eastern Time (US & Canada)",
            ),
            (PipelineScheduleTimeZone::IndianaEast, "Indiana (East)"),
            (PipelineScheduleTimeZone::Lima, "Lima"),
            (PipelineScheduleTimeZone::Quito, "Quito"),
            (
                PipelineScheduleTimeZone::AtlanticTimeCanada,
                "Atlantic Time (Canada)",
            ),
            (PipelineScheduleTimeZone::Caracas, "Caracas"),
            (PipelineScheduleTimeZone::Georgetown, "Georgetown"),
            (PipelineScheduleTimeZone::LaPaz, "La Paz"),
            (PipelineScheduleTimeZone::PuertoRico, "Puerto Rico"),
            (PipelineScheduleTimeZone::Santiago, "Santiago"),
            (PipelineScheduleTimeZone::Newfoundland, "Newfoundland"),
            (PipelineScheduleTimeZone::Brasilia, "Brasilia"),
            (PipelineScheduleTimeZone::BuenosAires, "Buenos Aires"),
            (PipelineScheduleTimeZone::Greenland, "Greenland"),
            (PipelineScheduleTimeZone::Montevideo, "Montevideo"),
            (PipelineScheduleTimeZone::MidAtlantic, "Mid-Atlantic"),
            (PipelineScheduleTimeZone::Azores, "Azores"),
            (PipelineScheduleTimeZone::CapeVerdeIslands, "Cape Verde Is."),
            (PipelineScheduleTimeZone::Edinburgh, "Edinburgh"),
            (PipelineScheduleTimeZone::Lisbon, "Lisbon"),
            (PipelineScheduleTimeZone::London, "London"),
            (PipelineScheduleTimeZone::Monrovia, "Monrovia"),
            (PipelineScheduleTimeZone::UTC, "UTC"),
            (PipelineScheduleTimeZone::Amsterdam, "Amsterdam"),
            (PipelineScheduleTimeZone::Belgrade, "Belgrade"),
            (PipelineScheduleTimeZone::Berlin, "Berlin"),
            (PipelineScheduleTimeZone::Bern, "Bern"),
            (PipelineScheduleTimeZone::Bratislava, "Bratislava"),
            (PipelineScheduleTimeZone::Brussels, "Brussels"),
            (PipelineScheduleTimeZone::Budapest, "Budapest"),
            (PipelineScheduleTimeZone::Casablanca, "Casablanca"),
            (PipelineScheduleTimeZone::Copenhagen, "Copenhagen"),
            (PipelineScheduleTimeZone::Dublin, "Dublin"),
            (PipelineScheduleTimeZone::Ljubljana, "Ljubljana"),
            (PipelineScheduleTimeZone::Madrid, "Madrid"),
            (PipelineScheduleTimeZone::Paris, "Paris"),
            (PipelineScheduleTimeZone::Prague, "Prague"),
            (PipelineScheduleTimeZone::Rome, "Rome"),
            (PipelineScheduleTimeZone::Sarajevo, "Sarajevo"),
            (PipelineScheduleTimeZone::Skopje, "Skopje"),
            (PipelineScheduleTimeZone::Stockholm, "Stockholm"),
            (PipelineScheduleTimeZone::Vienna, "Vienna"),
            (PipelineScheduleTimeZone::Warsaw, "Warsaw"),
            (
                PipelineScheduleTimeZone::WestCentralAfrica,
                "West Central Africa",
            ),
            (PipelineScheduleTimeZone::Zagreb, "Zagreb"),
            (PipelineScheduleTimeZone::Zurich, "Zurich"),
            (PipelineScheduleTimeZone::Athens, "Athens"),
            (PipelineScheduleTimeZone::Bucharest, "Bucharest"),
            (PipelineScheduleTimeZone::Cairo, "Cairo"),
            (PipelineScheduleTimeZone::Harare, "Harare"),
            (PipelineScheduleTimeZone::Helsinki, "Helsinki"),
            (PipelineScheduleTimeZone::Jerusalem, "Jerusalem"),
            (PipelineScheduleTimeZone::Kaliningrad, "Kaliningrad"),
            (PipelineScheduleTimeZone::Kyiv, "Kyiv"),
            (PipelineScheduleTimeZone::Pretoria, "Pretoria"),
            (PipelineScheduleTimeZone::Riga, "Riga"),
            (PipelineScheduleTimeZone::Sofia, "Sofia"),
            (PipelineScheduleTimeZone::Tallinn, "Tallinn"),
            (PipelineScheduleTimeZone::Vilnius, "Vilnius"),
            (PipelineScheduleTimeZone::Baghdad, "Baghdad"),
            (PipelineScheduleTimeZone::Istanbul, "Istanbul"),
            (PipelineScheduleTimeZone::Kuwait, "Kuwait"),
            (PipelineScheduleTimeZone::Minsk, "Minsk"),
            (PipelineScheduleTimeZone::Moscow, "Moscow"),
            (PipelineScheduleTimeZone::Nairobi, "Nairobi"),
            (PipelineScheduleTimeZone::Riyadh, "Riyadh"),
            (PipelineScheduleTimeZone::StPetersburg, "St. Petersburg"),
            (PipelineScheduleTimeZone::Volgograd, "Volgograd"),
            (PipelineScheduleTimeZone::Tehran, "Tehran"),
            (PipelineScheduleTimeZone::AbuDhabi, "Abu Dhabi"),
            (PipelineScheduleTimeZone::Baku, "Baku"),
            (PipelineScheduleTimeZone::Muscat, "Muscat"),
            (PipelineScheduleTimeZone::Samara, "Samara"),
            (PipelineScheduleTimeZone::Tbilisi, "Tbilisi"),
            (PipelineScheduleTimeZone::Yerevan, "Yerevan"),
            (PipelineScheduleTimeZone::Kabul, "Kabul"),
            (PipelineScheduleTimeZone::Ekaterinburg, "Ekaterinburg"),
            (PipelineScheduleTimeZone::Islamabad, "Islamabad"),
            (PipelineScheduleTimeZone::Karachi, "Karachi"),
            (PipelineScheduleTimeZone::Tashkent, "Tashkent"),
            (PipelineScheduleTimeZone::Chennai, "Chennai"),
            (PipelineScheduleTimeZone::Kolkata, "Kolkata"),
            (PipelineScheduleTimeZone::Mumbai, "Mumbai"),
            (PipelineScheduleTimeZone::NewDelhi, "New Delhi"),
            (
                PipelineScheduleTimeZone::SriJayawardenepura,
                "Sri Jayawardenepura",
            ),
            (PipelineScheduleTimeZone::Kathmandu, "Kathmandu"),
            (PipelineScheduleTimeZone::Almaty, "Almaty"),
            (PipelineScheduleTimeZone::Astana, "Astana"),
            (PipelineScheduleTimeZone::Dhaka, "Dhaka"),
            (PipelineScheduleTimeZone::Urumqi, "Urumqi"),
            (PipelineScheduleTimeZone::Rangoon, "Rangoon"),
            (PipelineScheduleTimeZone::Bangkok, "Bangkok"),
            (PipelineScheduleTimeZone::Hanoi, "Hanoi"),
            (PipelineScheduleTimeZone::Jakarta, "Jakarta"),
            (PipelineScheduleTimeZone::Krasnoyarsk, "Krasnoyarsk"),
            (PipelineScheduleTimeZone::Novosibirsk, "Novosibirsk"),
            (PipelineScheduleTimeZone::Beijing, "Beijing"),
            (PipelineScheduleTimeZone::Chongqing, "Chongqing"),
            (PipelineScheduleTimeZone::HongKong, "Hong Kong"),
            (PipelineScheduleTimeZone::Irkutsk, "Irkutsk"),
            (PipelineScheduleTimeZone::KualaLumpur, "Kuala Lumpur"),
            (PipelineScheduleTimeZone::Perth, "Perth"),
            (PipelineScheduleTimeZone::Singapore, "Singapore"),
            (PipelineScheduleTimeZone::Taipei, "Taipei"),
            (PipelineScheduleTimeZone::Ulaanbaatar, "Ulaanbaatar"),
            (PipelineScheduleTimeZone::Osaka, "Osaka"),
            (PipelineScheduleTimeZone::Sapporo, "Sapporo"),
            (PipelineScheduleTimeZone::Seoul, "Seoul"),
            (PipelineScheduleTimeZone::Tokyo, "Tokyo"),
            (PipelineScheduleTimeZone::Yakutsk, "Yakutsk"),
            (PipelineScheduleTimeZone::Adelaide, "Adelaide"),
            (PipelineScheduleTimeZone::Darwin, "Darwin"),
            (PipelineScheduleTimeZone::Brisbane, "Brisbane"),
            (PipelineScheduleTimeZone::Canberra, "Canberra"),
            (PipelineScheduleTimeZone::Guam, "Guam"),
            (PipelineScheduleTimeZone::Hobart, "Hobart"),
            (PipelineScheduleTimeZone::Melbourne, "Melbourne"),
            (PipelineScheduleTimeZone::PortMoresby, "Port Moresby"),
            (PipelineScheduleTimeZone::Sydney, "Sydney"),
            (PipelineScheduleTimeZone::Vladivostok, "Vladivostok"),
            (PipelineScheduleTimeZone::Magadan, "Magadan"),
            (PipelineScheduleTimeZone::NewCaledonia, "New Caledonia"),
            (PipelineScheduleTimeZone::SolomonIslands, "Solomon Is."),
            (PipelineScheduleTimeZone::Srednekolymsk, "Srednekolymsk"),
            (PipelineScheduleTimeZone::Auckland, "Auckland"),
            (PipelineScheduleTimeZone::Fiji, "Fiji"),
            (PipelineScheduleTimeZone::Kamchatka, "Kamchatka"),
            (PipelineScheduleTimeZone::MarshallIslands, "Marshall Is."),
            (PipelineScheduleTimeZone::Wellington, "Wellington"),
            (PipelineScheduleTimeZone::ChathamIslands, "Chatham Is."),
            (PipelineScheduleTimeZone::Nukualofa, "Nuku'alofa"),
            (PipelineScheduleTimeZone::Samoa, "Samoa"),
            (PipelineScheduleTimeZone::TokelauIslands, "Tokelau Is."),
            (PipelineScheduleTimeZone::Custom("custom".into()), "custom"),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn all_required_params_are_necessary() {
        let err = CreatePipelineSchedule::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, CreatePipelineScheduleBuilderError, "project");
    }

    #[test]
    fn project_is_necessary() {
        let err = CreatePipelineSchedule::builder()
            .description("desc")
            .ref_("master")
            .cron(PipelineScheduleCron::new("0 1 * * *").unwrap())
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, CreatePipelineScheduleBuilderError, "project");
    }

    #[test]
    fn description_is_necessary() {
        let err = CreatePipelineSchedule::builder()
            .project(1)
            .ref_("master")
            .cron(PipelineScheduleCron::new("0 1 * * *").unwrap())
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, CreatePipelineScheduleBuilderError, "description");
    }

    #[test]
    fn ref_is_necessary() {
        let err = CreatePipelineSchedule::builder()
            .project(1)
            .description("desc")
            .cron(PipelineScheduleCron::new("0 1 * * *").unwrap())
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, CreatePipelineScheduleBuilderError, "ref_");
    }

    #[test]
    fn cron_is_necessary() {
        let err = CreatePipelineSchedule::builder()
            .project(1)
            .description("desc")
            .ref_("master")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, CreatePipelineScheduleBuilderError, "cron");
    }

    #[test]
    fn all_required_parameters() {
        CreatePipelineSchedule::builder()
            .project(1)
            .description("desc")
            .ref_("master")
            .cron(PipelineScheduleCron::new("0 1 * * *").unwrap())
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/pipeline_schedules")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "description=desc",
                "&ref=master",
                "&cron=0+1+*+*+*",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreatePipelineSchedule::builder()
            .project("simple/project")
            .description("desc")
            .ref_("master")
            .cron(PipelineScheduleCron::new("0 1 * * *").unwrap())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_cron_timezone() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/pipeline_schedules")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "description=desc",
                "&ref=master",
                "&cron=0+1+*+*+*",
                "&cron_timezone=Newfoundland",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreatePipelineSchedule::builder()
            .project("simple/project")
            .description("desc")
            .ref_("master")
            .cron(PipelineScheduleCron::new("0 1 * * *").unwrap())
            .cron_timezone(PipelineScheduleTimeZone::Newfoundland)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_active() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/pipeline_schedules")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "description=desc",
                "&ref=master",
                "&cron=0+1+*+*+*",
                "&active=false",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreatePipelineSchedule::builder()
            .project("simple/project")
            .description("desc")
            .ref_("master")
            .cron(PipelineScheduleCron::new("0 1 * * *").unwrap())
            .active(false)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
