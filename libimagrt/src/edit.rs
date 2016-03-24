use std::ops::DerefMut;

use runtime::Runtime;
use error::RuntimeError;
use error::RuntimeErrorKind;

use libimagstore::store::FileLockEntry;
use libimagstore::store::Entry;

pub type EditResult<T> = Result<T, RuntimeError>;

pub trait Edit {
    fn edit_content(&mut self, rt: &Runtime) -> EditResult<()>;
}

impl Edit for Entry {

    fn edit_content(&mut self, rt: &Runtime) -> EditResult<()> {
        edit_in_tmpfile(rt, self.get_content_mut())
            .map(|_| ())
    }

}

impl<'a> Edit for FileLockEntry<'a> {

    fn edit_content(&mut self, rt: &Runtime) -> EditResult<()> {
        self.deref_mut().edit_content(rt)
    }

}

pub fn edit_in_tmpfile(rt: &Runtime, s: &mut String) -> EditResult<()> {
    use tempfile::NamedTempFile;
    use std::io::Seek;
    use std::io::Read;
    use std::io::SeekFrom;
    use std::io::Write;

    let file = NamedTempFile::new();
    if file.is_err() {
        return Err(RuntimeError::new(RuntimeErrorKind::Instantiate, None));
    }
    let file      = file.unwrap();
    let file_path = file.path();
    let file      = file.reopen();

    if file.is_err() {
        return Err(RuntimeError::new(RuntimeErrorKind::IOError, Some(Box::new(file.err().unwrap()))));
    }

    let mut file = file.unwrap();
    file.write_all(&s.clone().into_bytes()[..]);

    if let Err(e) = file.sync_data() {
        return Err(RuntimeError::new(RuntimeErrorKind::IOError, Some(Box::new(e))));
    }

    if let Some(mut editor) = rt.editor() {
        let exit_status = editor.arg(file_path).status();

        match exit_status.map(|s| s.success()) {
            Ok(true) => {
                file.sync_data()
                    .and_then(|_| file.seek(SeekFrom::Start(0)))
                    .and_then(|_| file.read_to_string(s))
                    .map(|_| ())
                    .map_err(|e| RuntimeError::new(RuntimeErrorKind::IOError, Some(Box::new(e))))
            },
            Ok(_)    => Err(RuntimeError::new(RuntimeErrorKind::ProcessExitFailure, None)),
            Err(e)   => Err(RuntimeError::new(RuntimeErrorKind::IOError, Some(Box::new(e)))),
        }
    } else {
        Err(RuntimeError::new(RuntimeErrorKind::Instantiate, None))
    }
}
