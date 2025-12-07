pub mod arg_util;
pub mod file_path;
pub mod function;
pub mod math;
pub mod time;
pub mod unit;

#[cfg(test)]
mod tests {
    use std::{iter::Inspect, thread::sleep, time::Duration};

    use crate::{
        file_path::file_path::FilePath,
        function::UnaryOperator,
        math::math,
        time::{
            stopwatch::{self, stopwatch::Stopwatch},
            time,
        },
        unit::distance::{Distance, DistanceUnit},
    };

    use super::*;

    #[test]
    fn supplier() {
        let supplier: function::Supplier<String> = || return "input".to_string();
        assert!(supplier().eq("input"));
    }

    #[test]
    fn consumer() {
        let consumer: function::Consumer<&mut Option<String>> = |x: &mut Option<String>| {
            x.take();
        };
        let input = &mut Option::Some("input".to_string());
        consumer(input);
        assert!(input.is_none());
    }

    #[test]
    fn unary_operator() {
        let operation: UnaryOperator<u32> = |x| return x * 2;
        let input = 3;
        let output = operation(input);
        println!("{}", output);
        assert!(output == (input * 2));
    }

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
        let distance = Distance::from(1.0, DistanceUnit::Meters);
        assert!(math::epsilon_equals(
            distance.to(DistanceUnit::Feet),
            3.2808,
            0.2
        ))
    }
}
