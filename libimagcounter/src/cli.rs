use std::str::FromStr;

use error::CounterErrorKind as CEK;
use error::MapErrInto;
use result::Result;

use clap::{Arg, ArgMatches, App, SubCommand};
use toml::Value;
use regex::Regex;

use libimagerror::into::IntoError;
use libimagentryfilter::builtin::header::field_gt::FieldGt;
use libimagentryfilter::builtin::header::field_lt::FieldLt;
use libimagentryfilter::builtin::header::field_grep::FieldGrep;
use libimagentryfilter::builtin::header::field_eq::FieldEq;
use libimagentryfilter::builtin::bool_filter::BoolFilter;
use libimagentryfilter::filter::Filter;

pub fn build_cli_filter<F: Filter>(matches: &ArgMatches) -> Result<Option<Box<F>>> {
    if let Some(matches) = matches.subcommand_matches("filter") {
        let gt_filter = matches.value_of("greater-than").map(|v| {
            try!(FromStr::from_str(v)
                .map_err(|e| CEK::CliValueTransformError.into_error_with_cause(Box::new(e)))
                .map(|i| FieldGt::new(String::from("counter.value"), Value::Integer(i)))
                .map_err_into(CEK::CliError))
        });

        let lt_filter = matches.value_of("lower-than").map(|v| {
            try!(FromStr::from_str(v)
                .map_err(|e| CEK::CliValueTransformError.into_error_with_cause(Box::new(e)))
                .map(|i| FieldLt::new(String::from("counter.value"), Value::Integer(i)))
                .map_err_into(CEK::CliError))
        });

        let eq_filter = matches.value_of("equals").map(|v| {
            try!(FromStr::from_str(v)
                .map_err(|e| CEK::CliValueTransformError.into_error_with_cause(Box::new(e)))
                .map(|i| FieldEq::new(String::from("counter.value"), Value::Integer(i)))
                .map_err_into(CEK::CliError))
        });

        let grep_filter = matches.value_of("name").map(|v| {
            let r = try!(Regex::new(v).map_err(|e| CEK::CliError.into_error_with_cause(Box::new(e))));
            FieldGrep::new(String::from("counter.name"), r)
        });

        let mut filter = Box::new(BoolFilter::new(true));

        if let Some(f) = gt_filter {
            filter = Box::new(filter.and(Box::new(f)));
        }

        if let Some(f) = lt_filter {
            filter = Box::new(filter.and(Box::new(f)));
        }

        if let Some(f) = eq_filter {
            filter = Box::new(filter.and(Box::new(f)));
        }

        if let Some(f) = grep_filter {
            filter = Box::new(filter.and(Box::new(f)));
        }

        Ok(Box::new(filter))
    }
    else {
        Ok(None)
    }
}

pub fn build_subcommand_filter<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(filter_subcommand_name())
        .author("Matthias Beyer <mail@beyermatthias.de>")
        .version("0.1")
        .about("Filter entries, multiple filter are connected with logical AND")
        .arg(Arg::with_name("name")
             .long("name")
             .short("n")
             .takes_value(true)
             .required(false)
             .help("Filter for this name (Regex)")
             .value_name("NAME"))

        .arg(Arg::with_name("greater-than")
             .long("greater")
             .short("g")
             .takes_value(true)
             .required(false)
             .help("Filter Counters which are greater than VALUE")
             .value_name("VALUE"))

        .arg(Arg::with_name("lower-than")
             .long("lower")
             .short("l")
             .takes_value(true)
             .required(false)
             .help("Filter counters which are lower than VALUE")
             .value_name("VALUE"))

        .arg(Arg::with_name("equals")
             .long("equal")
             .short("e")
             .takes_value(true)
             .required(false)
             .help("Filter counters which equal VALUE")
             .value_name("VALUE"))
}

pub fn filter_subcommand_name() -> &'static str {
    "filter"
}

