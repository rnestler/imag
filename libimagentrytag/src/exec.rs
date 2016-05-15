use clap::ArgMatches;

use libimagstore::store::FileLockEntry;

use result::Result;
use tagable::*;
use ui::{get_add_tags, get_remove_tags};

pub fn exec_cli_for_entry(matches: &ArgMatches, entry: &mut FileLockEntry) -> Result<()> {
    if let Some(ts) = get_add_tags(matches) {
        for t in ts {
            if let Err(e) = entry.add_tag(t) {
                return Err(e);
            }
        }
    }

    if let Some(ts) = get_remove_tags(matches) {
        for t in ts {
            if let Err(e) = entry.remove_tag(t) {
                return Err(e);
            }
        }
    }

    Ok(())
}
