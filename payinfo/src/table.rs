use std::{num::NonZeroU8, str::FromStr};

#[derive(Debug)]
pub struct EventTable {
    events: Vec<Event>,
}

#[derive(Debug)]
pub struct Event {
    name: String,
    date: String,
    adder: Option<String>,
    hours: f32,
    msr: f32,
    time_in: Time,
    time_out: Time,
    meals: Option<NonZeroU8>,
    travel_meet_time: Option<Time>,
    travel_rtn_time: Option<Time>,
}

#[derive(Debug)]
pub struct Time {
    hour: u8,
    minute: u8,
}

impl EventTable {
    pub fn parse_stub(stub: &str) -> EventTable {
        let (_, tables): (bool, Vec<Vec<&str>>) =
            stub.lines()
                .fold((false, vec![vec![]]), |(mut in_table, mut tables), line| {
                    if line.ends_with("Pay Details") {
                        in_table = true;
                    }

                    if in_table {
                        tables.last_mut().unwrap().push(line);
                    }

                    if line.ends_with("Note: Travel Pay is paid at the prevailing minimum wage.") {
                        tables.push(vec![]);
                        in_table = false;
                    }

                    (in_table, tables)
                });

        let events = tables
            .into_iter()
            .map(|v| v.join("\n"))
            .filter_map(|table| table.parse().ok())
            .flat_map(|table: EventTable| table.events.into_iter())
            .collect();

        Self { events }
    }

    pub fn to_csv(&self) -> String {
        let mut csv: Vec<String> = Vec::with_capacity(self.events.len() + 1);
        csv.push("Customer,Date,Adder,Hours,MSR,Time In,Time Out,Meals,Travel Meet Time,Travel Return Time".to_owned());

        //Convert the events to CSV and extend the buffer to include them
        csv.extend(self.events.iter().map(Event::to_csv));

        csv.join("\n")
    }
}

impl Event {
    fn to_csv(&self) -> String {
        fn fmt_float(input: f32) -> String {
            format!("{:.2}", input)
        }

        fn fmt_time(input: &Time) -> String {
            format!("{:02}:{:02}", input.hour, input.minute)
        }

        let meals = self
            .meals
            .map(|meals| meals.to_string())
            .unwrap_or("0".to_owned());

        let travel_meet_time = self
            .travel_meet_time
            .as_ref()
            .map(fmt_time)
            .unwrap_or("".to_owned());
        let travel_rtn_time = self
            .travel_rtn_time
            .as_ref()
            .map(fmt_time)
            .unwrap_or("".to_owned());

        let fields: [&str; 10] = [
            &self.name,
            &self.date,
            match self.adder {
                None => "",
                Some(ref adder) => adder,
            },
            &fmt_float(self.hours),
            &fmt_float(self.msr),
            &fmt_time(&self.time_in),
            &fmt_time(&self.time_out),
            &meals,
            &travel_meet_time,
            &travel_rtn_time,
        ];

        fields.join(",")
    }
}

impl FromStr for EventTable {
    type Err = ();

    fn from_str(lines: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = lines
            .lines()
            .skip(4)
            .map(str::trim)
            .take_while(|line| {
                !line.starts_with("Note: Travel Pay is paid at the prevailing minimum wage.")
            })
            .collect();

        fn is_event_line(line: &&str) -> bool {
            !line.starts_with("Week") && !line.chars().skip(5).all(|ch| ch.is_digit(10))
        }

        let events = lines
            .into_iter()
            .filter(is_event_line)
            .filter_map(|event| event.parse().ok())
            .collect();

        Ok(Self { events })
    }
}

pub(super) mod fields {
    pub(super) const CUSTOMER: usize = 0;
    pub(super) const DATE: usize = 41;
    pub(super) const ADDER: usize = 54;
    pub(super) const HOURS: usize = 76;
    pub(super) const MSR: usize = 89;
    pub(super) const TIME_IN: usize = 101;
    pub(super) const TIME_OUT: usize = 115;
    pub(super) const MEALS: usize = 129;
    pub(super) const TRAVEL_MEET_TIME: usize = 139;
    pub(super) const TRAVEL_RTN_TIME: usize = 154;
}

impl FromStr for Event {
    type Err = ();

    fn from_str(event: &str) -> Result<Self, Self::Err> {
        //Ensure the event line has full width in order to avoid indexing out of bounds.
        //Fields after time_out may not exist, cutting the line off there.
        let event = format!("{}{}", event, " ".repeat(80));
        let chars: Vec<char> = event.chars().collect();

        // println!("{event}");
        let name = String::from_iter(&chars.as_slice()[fields::CUSTOMER..fields::DATE]);
        let date = String::from_iter(&chars.as_slice()[fields::DATE..fields::ADDER]);
        let adder = String::from_iter(&chars.as_slice()[fields::ADDER..fields::HOURS]);
        let hours = String::from_iter(&chars.as_slice()[fields::HOURS..fields::MSR]);
        let msr = String::from_iter(&chars.as_slice()[fields::MSR..fields::TIME_IN]);
        let time_in = String::from_iter(&chars.as_slice()[fields::TIME_IN..fields::TIME_OUT]);
        let time_out = String::from_iter(&chars.as_slice()[fields::TIME_OUT..fields::MEALS]);
        let meals = String::from_iter(&chars.as_slice()[fields::MEALS..fields::TRAVEL_MEET_TIME]);
        let travel_meet_time =
            String::from_iter(&chars.as_slice()[fields::TRAVEL_MEET_TIME..fields::TRAVEL_RTN_TIME]);
        let travel_return_time = String::from_iter(&chars.as_slice()[fields::TRAVEL_RTN_TIME..]);

        // dbg!(
        //     &name,
        //     &date,
        //     &adder,
        //     &hours,
        //     &msr,
        //     &time_in,
        //     &time_out,
        //     &meals,
        //     &travel_meet_time,
        //     &travel_return_time
        // );

        let name = name.trim().to_owned();
        let date = date.trim().to_owned();
        let adder = match adder.trim() {
            "" => None,
            adder => Some(adder.to_owned()),
        };
        let hours = hours.trim().parse().unwrap_or(0.0f32);
        let msr = msr.trim().parse().unwrap_or(0.0f32);
        let time_in = time_in
            .trim()
            .parse()
            .unwrap_or(Time { hour: 0, minute: 0 });
        let time_out = time_out
            .trim()
            .parse()
            .unwrap_or(Time { hour: 0, minute: 0 });
        let meals = meals.trim().parse().ok();
        let travel_meet_time = travel_meet_time.trim().parse().ok();
        let travel_rtn_time = travel_return_time.trim().parse().ok();

        Ok(Self {
            name,
            date,
            adder,
            hours,
            msr,
            time_in,
            time_out,
            meals,
            travel_meet_time,
            travel_rtn_time,
        })
    }
}

impl FromStr for Time {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hour, minute) = s.split_once(':').ok_or(())?;

        let (hour, minute) = match (hour.parse(), minute.parse()) {
            (Err(_), _) | (_, Err(_)) => return Err(()),
            (Ok(hour), Ok(minute)) => (hour, minute),
        };

        Ok(Self { hour, minute })
    }
}