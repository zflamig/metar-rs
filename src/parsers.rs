use super::types::*;

/// A result with an error case of a 3-tuple containing the start offset, the length and the error
/// information.
type ParserResult<T, E> = Result<T, (usize, usize, E)>;

pub mod errors {
    use std::fmt;

    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    /// An error caused whilst parsing the weather station
    pub enum StationError {
        /// The station ID is not the correct length
        IncorrectLength,
        /// A character was found to be not alphabetic
        NonAlphabeticCharacter,
    }

    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    /// An error caused when parsing the observation time
    pub enum ObservationTimeError {
        /// The observation time is not the correct length
        IncorrectLength,
        /// The observation date is not valid
        DateNotValid,
        /// The observation hour is not valid
        HourNotValid,
        /// The observation minute is not valid
        MinuteNotValid,
        /// The specified time zone is not within the ICAO METAR standard
        InvalidTimeZone,
    }

    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    /// An error caused when parsing the wind
    pub enum WindError {
        /// The wind heading is not valid
        HeadingNotValid,
        /// The wind speed was not valid
        SpeedNotValid,
        /// The wind gusting speed was not valid
        GustingNotValid,
        /// An unknown unit was read
        UnitNotValid,
    }

    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    /// An error caused when parsing the wind varying
    pub enum WindVaryingError {
        /// The wind heading is not valid
        HeadingNotValid,
        /// Mostly an internal error - informs the calling function that this is not a wind varying
        /// and should be attempted to be parsed as cloud/visibility information
        NotWindVarying,
    }

    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    /// An error caused when parsing the temperature
    pub enum TemperatureError {
        /// The temperature is not valid
        TemperatureNotValid,
        /// The dewpoint is not valid
        DewpointNotValid,
        /// Mostly an internal error - informs the calling function that this is not a
        /// temperature/dewpoint pair and should be attempted to be parsed as cloud/visibility
        /// information
        NotTemperatureDewpointPair,
    }

    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    /// An error caused when parsing the pressure
    pub enum PressureError {
        /// The pressure is not valid
        PressureNotValid,
        /// The unit is not valid
        UnitNotValid,
    }

    impl fmt::Display for StationError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::IncorrectLength => write!(f, "The station ID was not the correct length."),
                Self::NonAlphabeticCharacter => write!(f, "Found a non-alphabetic character."),
            }
        }
    }

    impl fmt::Display for ObservationTimeError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::IncorrectLength => write!(f, "The observation time was not the correct length."),
                Self::DateNotValid => write!(f, "The date was invalid."),
                Self::HourNotValid => write!(f, "The hour was invalid."),
                Self::MinuteNotValid => write!(f, "The minute was invalid."),
                Self::InvalidTimeZone => write!(f, "The time zone was invalid (not Zulu)."),
            }
        }
    }

    impl fmt::Display for WindError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::HeadingNotValid => write!(f, "The heading is invalid."),
                Self::SpeedNotValid => write!(f, "The speed is invalid."),
                Self::GustingNotValid => write!(f, "The gusting speed is invalid."),
                Self::UnitNotValid => write!(f, "The unit is not valid."),
            }
        }
    }

    impl fmt::Display for WindVaryingError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::HeadingNotValid => write!(f, "The heading is invalid."),
                Self::NotWindVarying => unreachable!(),
            }
        }
    }

    impl fmt::Display for TemperatureError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::TemperatureNotValid => write!(f, "The temperature is invalid."),
                Self::DewpointNotValid => write!(f, "The dewpoint is invalid."),
                Self::NotTemperatureDewpointPair => unreachable!(),
            }
        }
    }

    impl fmt::Display for PressureError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::PressureNotValid => write!(f, "The pressure is invalid."),
                Self::UnitNotValid => write!(f, "The unit is invalid."),
            }
        }
    }
}

use errors::*;

pub fn parse_station<'a>(s: &'a str) -> ParserResult<&'a str, StationError> {
    if s.len() != 4 {
        // Not correct length
        return Err((0, s.len(), StationError::IncorrectLength));
    }

    let chs: Vec<_> = s.chars().collect();
    for i in 0..chs.len() {
        let c = chs[i];
        if !c.is_alphabetic() {
            return Err((i, 1, StationError::NonAlphabeticCharacter));
        }
    }
    Ok(s)
}

pub fn parse_obs_time<'a>(s: &'a str) -> ParserResult<Time, ObservationTimeError> {
    let mut time = Time {
        date: 0,
        hour: 0,
        minute: 0,
    };

    if s.len() != 7 {
        // Not correct length
        return Err((0, s.len(), ObservationTimeError::IncorrectLength));
    }

    let chs: Vec<_> = s.chars().collect();

    if !chs[0].is_digit(10) {
        // Not valid digit
        return Err((0, 1, ObservationTimeError::DateNotValid));
    } else if !chs[1].is_digit(10) {
        // Not valid digit
        return Err((1, 1, ObservationTimeError::DateNotValid));
    } else {
        // Date in range
        let date = s[0..2].parse().unwrap();
        if date > 31 {
            return Err((0, 2, ObservationTimeError::DateNotValid));
        }
        time.date = date;
    }

    if !chs[2].is_digit(10) {
        // Not valid digit
        return Err((2, 1, ObservationTimeError::HourNotValid));
    } else if !chs[3].is_digit(10) {
        // Not valid digit
        return Err((3, 1, ObservationTimeError::HourNotValid));
    } else {
        // Hour in range
        let hour = s[2..4].parse().unwrap();
        if hour > 23 {
            return Err((2, 2, ObservationTimeError::HourNotValid));
        }
        time.hour = hour;
    }

    if !chs[4].is_digit(10) {
        // Minute valid digit
        return Err((4, 1, ObservationTimeError::MinuteNotValid));
    } else if !chs[5].is_digit(10) {
        // Minute valid digit
        return Err((5, 1, ObservationTimeError::MinuteNotValid));
    } else {
        // Minute in range
        let minute = s[4..6].parse().unwrap();
        if minute > 59 {
            return Err((4, 2, ObservationTimeError::MinuteNotValid));
        }
        time.minute = minute;
    }

    if chs[6] != 'Z' {
        return Err((6, 1, ObservationTimeError::InvalidTimeZone));
    }

    Ok(time)
}

pub fn parse_wind<'a>(s: &'a str) -> ParserResult<Wind, WindError> {
    let mut wind = Wind {
        dir: WindDirection::Heading(0),
        speed: WindSpeed::Knot(0),
        varying: None,
        gusting: None,
    };

    let chs: Vec<_> = s.chars().collect();

    if &s[0..3] == "VRB" {
        wind.dir = WindDirection::Variable;
    } else if &s[0..3] == "ABV" {
        wind.dir = WindDirection::Above;
    } else if !chs[0].is_digit(10) {
        return Err((0, 1, WindError::HeadingNotValid));
    } else if !chs[1].is_digit(10) {
        return Err((1, 1, WindError::HeadingNotValid));
    } else if !chs[2].is_digit(10) {
        return Err((2, 1, WindError::HeadingNotValid));
    } else {
        let heading = s[0..3].parse().unwrap();
        if heading >= 360 {
            return Err((0, 3, WindError::HeadingNotValid));
        }
        wind.dir = WindDirection::Heading(heading);
    }

    if !chs[3].is_digit(10) {
        return Err((3, 1, WindError::SpeedNotValid));
    } else if !chs[4].is_digit(10) {
        return Err((4, 1, WindError::SpeedNotValid));
    } else {
        let speed = s[3..5].parse().unwrap();

        if chs[5] == 'G' {
            if !chs[6].is_digit(10) {
                return Err((6, 1, WindError::GustingNotValid));
            } else if !chs[7].is_digit(10) {
                return Err((7, 1, WindError::GustingNotValid));
            }
            let g_speed = s[6..8].parse().unwrap();

            let unit = &s[8..];
            if unit == "KT" {
                wind.speed = WindSpeed::Knot(speed);
                wind.gusting = Some(WindSpeed::Knot(g_speed));
            } else if unit == "MPS" {
                wind.speed = WindSpeed::MetresPerSecond(speed);
                wind.gusting = Some(WindSpeed::MetresPerSecond(g_speed));
            } else {
                return Err((8, unit.len(), WindError::UnitNotValid));
            }
        } else {
            let unit = &s[5..];
            if unit == "KT" {
                wind.speed = WindSpeed::Knot(speed);
            } else if unit == "MPS" {
                wind.speed = WindSpeed::MetresPerSecond(speed);
            } else {
                return Err((5, unit.len(), WindError::UnitNotValid));
            }
        }
    }

    Ok(wind)
}

pub fn parse_wind_varying<'a>(s: &'a str) -> ParserResult<(u32, u32), WindVaryingError> {
    let chs: Vec<_> = s.chars().collect();

    if chs[3] != 'V' {
        return Err((3, 1, WindVaryingError::NotWindVarying));
    }

    if !chs[0].is_digit(10) {
        return Err((0, 1, WindVaryingError::HeadingNotValid));
    } else if !chs[1].is_digit(10) {
        return Err((1, 1, WindVaryingError::HeadingNotValid));
    } else if !chs[2].is_digit(10) {
        return Err((2, 1, WindVaryingError::HeadingNotValid));
    } else if !chs[4].is_digit(10) {
        return Err((4, 1, WindVaryingError::HeadingNotValid));
    } else if !chs[5].is_digit(10) {
        return Err((5, 1, WindVaryingError::HeadingNotValid));
    } else if !chs[6].is_digit(10) {
        return Err((6, 1, WindVaryingError::HeadingNotValid));
    } else {
        let heading_from = s[0..3].parse().unwrap();
        let heading_to = s[4..7].parse().unwrap();
        if heading_from >= 360 {
            return Err((0, 3, WindVaryingError::HeadingNotValid));
        }
        if heading_to >= 360 {
            return Err((4, 3, WindVaryingError::HeadingNotValid));
        }
        return Ok((heading_from, heading_to));
    }
}

pub fn parse_temperatures<'a>(s: &'a str) -> ParserResult<(i32, i32), TemperatureError> {
    let chs: Vec<_> = s.chars().collect();

    if s.contains("///") {
        return Err((0, s.len(), TemperatureError::NotTemperatureDewpointPair));
    }
    if !s.contains('/') {
        return Err((0, s.len(), TemperatureError::NotTemperatureDewpointPair));
    }
    if s.contains('R') {
        // To protect against RVRs being interpreted as temperatures
        return Err((0, s.len(), TemperatureError::NotTemperatureDewpointPair));
    }

    let temp;
    let dewp;

    let mut i = 0;
    if chs[i] == 'M' {
        if !chs[i + 1].is_digit(10) {
            return Err((i + 1, 1, TemperatureError::TemperatureNotValid));
        }
        if !chs[i + 2].is_digit(10) {
            return Err((i + 2, 1, TemperatureError::TemperatureNotValid));
        }
        temp = -1 * s[i + 1 .. i + 3].parse::<i32>().unwrap();
        i += 4;
    } else {
        if !chs[i].is_digit(10) {
            return Err((i, 1, TemperatureError::TemperatureNotValid));
        }
        if !chs[i + 1].is_digit(10) {
            return Err((i + 1, 1, TemperatureError::TemperatureNotValid));
        }
        temp = s[i .. i + 2].parse().unwrap();
        i += 3;
    }

    if chs[i] == 'M' {
        if !chs[i + 1].is_digit(10) {
            return Err((i + 1, 1, TemperatureError::DewpointNotValid));
        }
        if !chs[i + 2].is_digit(10) {
            return Err((i + 2, 1, TemperatureError::DewpointNotValid));
        }
        dewp = -1 * s[i + 1 .. i + 3].parse::<i32>().unwrap();
    } else {
        if !chs[i].is_digit(10) {
            return Err((i, 1, TemperatureError::DewpointNotValid));
        }
        if !chs[i + 1].is_digit(10) {
            return Err((i + 1, 1, TemperatureError::DewpointNotValid));
        }
        dewp = s[i .. i + 2].parse().unwrap();
    }

    Ok((temp, dewp))
}

pub fn parse_pressure<'a>(s: &'a str) -> ParserResult<Pressure, PressureError> {
    let chs: Vec<_> = s.chars().collect();

    if !chs[1].is_digit(10) {
        return Err((1, 1, PressureError::PressureNotValid));
    }
    if !chs[2].is_digit(10) {
        return Err((2, 1, PressureError::PressureNotValid));
    }
    if !chs[3].is_digit(10) {
        return Err((3, 1, PressureError::PressureNotValid));
    }
    if !chs[4].is_digit(10) {
        return Err((4, 1, PressureError::PressureNotValid));
    }

    let pressure = s[1..5].parse().unwrap();

    if chs[0] == 'Q' {
        return Ok(Pressure::Hectopascals(pressure));
    } else if chs[0] == 'A' {
        return Ok(Pressure::InchesMercury(pressure));
    } else {
        return Err((0, 1, PressureError::UnitNotValid));
    }
}
