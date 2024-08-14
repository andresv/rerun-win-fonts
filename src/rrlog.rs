use anyhow::Result;

pub const VOLTAGE_LED1: &str = "voltage/led1";
pub const VOLTAGE_LED2: &str = "voltage/led2";
pub const VOLTAGE_LED3: &str = "voltage/led3";
pub const VOLTAGE_LED4: &str = "voltage/led4";
pub const VOLTAGE_OUT: &str = "voltage/out";

pub const CURRENT_LED12: &str = "current/led12";
pub const CURRENT_LED34: &str = "current/led34";
pub const CURRENT_I54V: &str = "current/i_54v";

pub const TEMPERATURE_1: &str = "temperature/1";
pub const TEMPERATURE_2: &str = "temperature/2";

pub fn recording_stream() -> Result<rerun::RecordingStream> {
    let rr = rerun::RecordingStreamBuilder::new("uledware").connect()?;

    rr.log(VOLTAGE_LED1, &rerun::SeriesLine::new().with_name("LED1"))?;
    rr.log(VOLTAGE_LED2, &rerun::SeriesLine::new().with_name("LED2"))?;
    rr.log(VOLTAGE_LED3, &rerun::SeriesLine::new().with_name("LED3"))?;
    rr.log(VOLTAGE_LED4, &rerun::SeriesLine::new().with_name("LED4"))?;
    rr.log(VOLTAGE_OUT, &rerun::SeriesLine::new().with_name("OUT"))?;

    rr.log(CURRENT_LED12, &rerun::SeriesLine::new().with_name("LED12"))?;
    rr.log(CURRENT_LED34, &rerun::SeriesLine::new().with_name("LED34"))?;
    rr.log(CURRENT_I54V, &rerun::SeriesLine::new().with_name("I_54V"))?;

    rr.log(TEMPERATURE_1, &rerun::SeriesLine::new().with_name("TEMP1"))?;
    rr.log(TEMPERATURE_2, &rerun::SeriesLine::new().with_name("TEMP2"))?;

    Ok(rr)
}
