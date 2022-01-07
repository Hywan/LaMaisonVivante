// This code is a shameless re-implementation of the excellent
// `rust-sunrise` crate
// https://github.com/nathan-osman/rust-sunrise/. Nathan Osman, you've
// all my gratitude.

const SECONDS_IN_A_DAY = 86400.;
const UNIX_EPOCH_JULIAN_DAY = 2440587.5;

function unix_to_julian(timestamp) {
    return timestamp / SECONDS_IN_A_DAY + UNIX_EPOCH_JULIAN_DAY;
}

function julian_to_unix(day) {
    return ((day - UNIX_EPOCH_JULIAN_DAY) * SECONDS_IN_A_DAY);
}

function mean_solar_noon(longitude, year, month, day) {
    const date = new Date(year, month - 1, day, 12, 00, 00);

    return unix_to_julian(date.getTime() / 1000) - longitude / 360.;
}

const J2000 = 2451545.;

function solar_mean_anomaly(day) {
    const v = (357.5291 + 0.98560028 * (day - J2000)) % 360.;

    if (v < 0.) {
        return v + 360.;
    }

    return v;
}

function equation_of_center(solar_anomaly) {
    let anomaly_in_rad = solar_anomaly * (Math.PI / 180.);
    let anomaly_sin = Math.sin(anomaly_in_rad);
    let anomaly_2_sin = Math.sin(2. * anomaly_in_rad);
    let anomaly_3_sin = Math.sin(3. * anomaly_in_rad);

    return 1.9148 * anomaly_sin + 0.02 * anomaly_2_sin + 0.0003 * anomaly_3_sin;
}

function argument_of_perihelion(day) {
    return 102.93005 + 0.3179526 * (day - 2451545.) / 36525.;
}

function ecliptic_longitude(solar_anomaly, equation_of_center, day) {
    return (solar_anomaly
        + equation_of_center
        + 180.
        + argument_of_perihelion(day) % 360.
        + 360.)
        % 360.;
}

const DEGREE = Math.PI / 180.;

function solar_transit(day, solar_anomaly, ecliptic_longitude) {
    return day + (0.0053 * Math.sin(solar_anomaly * DEGREE)
                  - 0.0069 * Math.sin(2. * ecliptic_longitude * DEGREE));
}

function declination(ecliptic_longitude) {
    return Math.asin(Math.sin(ecliptic_longitude * DEGREE) * 0.39779) / DEGREE;
}

function hour_angle(latitude, declination) {
    const latitude_rad = latitude * DEGREE;
    const declination_rad = declination * DEGREE;
    const numerator = -0.01449 - Math.sin(latitude_rad) * Math.sin(declination_rad);
    const denominator = Math.cos(latitude_rad) * Math.cos(declination_rad);

    return Math.acos(numerator / denominator) / DEGREE;
}

function adjust_time_to_local(timestamp) {
    const local_date = new Date();
    const difference_to_utc_in_ms = -1 * local_date.getTimezoneOffset() * 60 * 1000;

    return new Date(timestamp + difference_to_utc_in_ms);
}

function sunrise_sunset(latitude, longitude, year, month, day) {
    const day_ = mean_solar_noon(longitude, year, month, day);
    const solar_anomaly_ = solar_mean_anomaly(day_);
    const equation_of_center_ = equation_of_center(solar_anomaly_);
    const ecliptic_longitude_ = ecliptic_longitude(solar_anomaly_, equation_of_center_, day_);
    const solar_transit_= solar_transit(day_, solar_anomaly_, ecliptic_longitude_);
    const declination_ = declination(ecliptic_longitude_);
    const hour_angle_ = hour_angle(latitude, declination_);
    const frac = hour_angle_ / 360.;

    return {
        sunrise: adjust_time_to_local(julian_to_unix(solar_transit_ - frac) * 1000),
        sunset: adjust_time_to_local(julian_to_unix(solar_transit_ + frac) * 1000),
    };
}
