pub mod file_path;
pub mod function;
pub mod math;
pub mod time;
pub mod unit;

#[cfg(test)]
mod tests {
    use std::{thread::sleep, time::Duration};

    use crate::{
        file_path::file_path::FilePath,
        math::math,
        time::stopwatch::stopwatch::Stopwatch,
        unit::{
            geom::{
                angle::{
                    angle_measure::Angle,
                    angle_unit::{self, DEGREES},
                },
                distance::{
                    distance_measure::Distance,
                    distance_unit::{self, DistanceUnit, METERS},
                },
            },
            measure::Measure,
            motion::{
                motion_unit::MotionUnit,
                velocity::{
                    angular::{
                        angular_velocity_measure::AngularVelocity,
                        angular_velocity_unit::{AngularVelocityUnit, RADIANS_PER_SECOND},
                    },
                    linear::{
                        linear_velocity_measure::LinearVelocity,
                        linear_velocity_unit::{LinearVelocityUnit, METERS_PER_SECOND},
                    },
                },
            },
            time::{
                time_measure::Time,
                time_unit::{self, MINUTES, SECONDS},
            },
        },
    };

    use super::*;

    #[test]
    fn file_extension() {
        let input = "test.jpg";
        let output = file_path::file_extension::FileExtension::get(String::from(input));
        assert!(output.is_valid());
    }

    #[test]
    fn file_path() {
        let mut path = FilePath::new();
        path.append_entry("test");
        path.append_entry("light");
        path.set_extension("jpg");
        assert!(path.raw() == "test/light.jpg")
    }

    #[test]
    fn stopwatch() {
        let mut stopwatch = Stopwatch::new();
        let start_result = stopwatch.start();

        match start_result {
            Ok(_) => {}
            Err(error) => {
                panic!("Unable to start stopwatch due to error {}", error);
            }
        }

        let iterations = 3;
        let epsilon = 0.5;
        let sleep_delay = Duration::from_secs(1);

        for i in 0..iterations {
            let expected_time = ((i + 1) as f64) * sleep_delay.as_secs_f64();
            sleep(sleep_delay);
            match stopwatch.get_sec() {
                Ok(secs) => {
                    assert!(
                        ((secs - epsilon) <= expected_time) && ((secs + epsilon) >= expected_time)
                    );
                }
                Err(error) => {
                    panic!("Failed when fetching stopwatch value due to {}", error);
                }
            }
        }
    }

    #[test]
    pub fn distance_test() {
        let distance = Distance::from(1.0, distance_unit::METERS);
        println!("Distance in feet is {}", distance.to(&distance_unit::FEET));
        assert!(math::epsilon_equals(
            distance.to(&distance_unit::FEET),
            3.2808,
            0.2
        ));
        let half = distance - Distance::from(0.5, distance_unit::METERS);
        assert!(math::epsilon_equals(
            half.to(&distance_unit::METERS),
            0.5,
            0.05
        ))
    }

    #[test]
    pub fn angle_test() {
        let angle = Angle::from(1.0, angle_unit::ROTATIONS);
        assert!(math::epsilon_equals(
            angle.to(&angle_unit::DEGREES),
            360.0,
            0.5
        ))
    }

    #[test]
    pub fn time_test() {
        let time = Time::from(1.0, time_unit::WEEKS);
        assert!(math::epsilon_equals(time.to(&time_unit::DAYS), 7.0, 0.5))
    }

    #[test]
    pub fn linear_velocity_check() {
        let velo = LinearVelocity::from(1.0, METERS_PER_SECOND);
        let next_unit = &LinearVelocityUnit::derive_units(METERS, MINUTES);
        println!("{}", velo.to(next_unit));
        assert!(math::epsilon_equals(
            velo.to(next_unit),
            1.0 / next_unit.get_scale_to_base(),
            0.5
        ))
    }

    #[test]
    pub fn angular_velocity_check() {
        let velo = AngularVelocity::from(1.0, RADIANS_PER_SECOND);
        let next_unit = &AngularVelocityUnit::derive_units(DEGREES, SECONDS);
        assert!(math::epsilon_equals(
            velo.to(next_unit),
            1.0 / next_unit.get_scale_to_base(),
            0.5
        ))
    }
}
