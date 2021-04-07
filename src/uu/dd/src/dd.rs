// This file is part of the uutils coreutils package.
//
// (c) Derek Chiang <derekchiang93@gmail.com>
// (c) Christopher Brown <ccbrown112@gmail.com>
// (c) Justin Ryan <justin.alan.ryan@gmail.com>
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

#[macro_use]
extern crate uucore;

use clap::{crate_version, App, Arg};
use std::io::{self, Write};
use std::iter::Peekable;
use std::str::Chars;

const NAME: &str = "dd";
const SUMMARY: &str = "disk to disk copy";
const USAGE: &str = "[OPTIONS]... [STRING]...";
const AFTER_HELP: &str = r#"
 Echo the STRING(s) to standard output.

 If -e is in effect, the following sequences are recognized:

 \\\\      backslash
 \\a      alert (BEL)
 \\b      backspace
 \\c      produce no further output
 \\e      escape
 \\f      form feed
 \\n      new line
 \\r      carriage return
 \\t      horizontal tab
 \\v      vertical tab
 \\0NNN   byte with octal value NNN (1 to 3 digits)
 \\xHH    byte with hexadecimal value HH (1 to 2 digits)
"#;

mod options {
    pub const STRING: &str = "STRING";
    pub const NO_NEWLINE: &str = "no_newline";
    pub const ENABLE_BACKSLASH_ESCAPE: &str = "enable_backslash_escape";
    pub const DISABLE_BACKSLASH_ESCAPE: &str = "disable_backslash_escape";
}

fn parse_code(
    input: &mut Peekable<Chars>,
    base: u32,
    max_digits: u32,
    bits_per_digit: u32,
) -> Option<char> {
    let mut ret = 0x8000_0000;
    for _ in 0..max_digits {
        match input.peek().and_then(|c| c.to_digit(base)) {
            Some(n) => ret = (ret << bits_per_digit) | n,
            None => break,
        }
        input.next();
    }
    std::char::from_u32(ret)
}

### BEGIN RUST STUBS FROM C DD CODE
### Source: https://github.com/coreutils/coreutils/blob/master/src/dd.c


/* True if we need to close the standard output *stream*.  */
static close_stdout_required: bool = true;

/* The only reason to close the standard output *stream* is if
   parse_long_options fails (as it does for --help or --version).
   In any other case, dd uses only the STDOUT_FILENO file descriptor,
   and the "cleanup" function calls "close (STDOUT_FILENO)".
   Closing the file descriptor and then letting the usual atexit-run
   close_stdout function call "fclose (stdout)" would result in a
   harmless failure of the close syscall (with errno EBADF).
   This function serves solely to avoid the unnecessary close_stdout
   call, once parse_long_options has succeeded.
   Meanwhile, we guarantee that the standard error stream is flushed,
   by inlining the last half of close_stdout as needed.  */

fn maybe_close_stdout() {}

/* Like the 'error' function but handle any pending newline.  */

fn _GL_ATTRIBUTE_FORMAT() {}

/* fn usage() {} */

/* Common options to use when displaying sizes and rates.  */

enum human_opts {
  human_autoscale,
  human_round_to_nearest,
  human_space_before_unit,
  human_SI,
  human_Bi
}

/* Ensure input buffer IBUF is allocated.  */

fn alloc_ibuf() {}

fn translate_charset() {}

/* Return true if I has more than one bit set.  I must be nonnegative.  */

fn multiple_bits_set() {}

fn abbreviation_lacks_prefix() {}

/* Print transfer statistics.  */

fn print_xfer_stats() {}

fn print_stats() {}

/* An ordinary signal was received; arrange for the program to exit.  */

fn interrupt_handler() {}

/* An info signal was received; arrange for the program to print status.  */

fn siginfo_handler() {}

/* Install the signal handlers.  */

fn install_signal_handlers() {}

/* Close FD.  Return 0 if successful, -1 (setting errno) otherwise.
   If close fails with errno == EINTR, POSIX says the file descriptor
   is in an unspecified state, so keep trying to close FD but do not
   consider EBADF to be an error.  Do not process signals.  This all
   differs somewhat from functions like ifdatasync and ifsync.  */

fn iclose() {}

fn cleanup() {}

/* Process any pending signals.  If signals are caught, this function
   should be called periodically.  Ideally there should never be an
   unbounded amount of time when signals are not being processed.  */

fn process_signals() {}

fn finish_up() {}

fn quit() {}

/* Return LEN rounded down to a multiple of IO_BUFSIZE
   (to minimize calls to the expensive posix_fadvise(,POSIX_FADV_DONTNEED),
   while storing the remainder internally per FD.
   Pass LEN == 0 to get the current remainder.  */

fn cache_round() {}

/* Discard the cache from the current offset of either
   STDIN_FILENO or STDOUT_FILENO.
   Return true on success.  */

fn invalidate_cache() {}

/* Read from FD into the buffer BUF of size SIZE, processing any
   signals that arrive before bytes are read.  Return the number of
   bytes read if successful, -1 (setting errno) on failure.  */

fn iread() {}

/* Wrapper around iread function to accumulate full blocks.  */

fn iread_fullblock() {}

/* Write to FD the buffer BUF of size SIZE, processing any signals
   that arrive.  Return the number of bytes written, setting errno if
   this is less than SIZE.  Keep trying if there are partial
   writes.  */

fn iwrite() {}

/* Write, then empty, the output buffer 'obuf'. */

fn write_output() {}

/* Restart on EINTR from fdatasync.  */

fn ifdatasync() {}

/* Restart on EINTR from fd_reopen.  */

fn ifd_reopen() {}

/* Restart on EINTR from fstat.  */

fn ifstat() {}

/* Restart on EINTR from fsync.  */

fn ifsync() {}

/* Restart on EINTR from ftruncate.  */

fn iftruncate() {}

/* Return true if STR is of the form "PATTERN" or "PATTERNDELIM...".  */

fn operand_matches() {}

/* Interpret one "conv=..." or similar operand STR according to the
   symbols in TABLE, returning the flags specified.  If the operand
   cannot be parsed, use ERROR_MSGID to generate a diagnostic.  */

fn parse_symbols() {}

/* Return the value of STR, interpreted as a non-negative decimal integer,
   optionally multiplied by various values.
   Set *INVALID to a nonzero error value if STR does not represent a
   number in this format.  */

fn parse_integer() {}

/* OPERAND is of the form "X=...".  Return true if X is NAME.  */

fn operand_is() {}

fn scanargs() {}

/* Fix up translation table. */

fn apply_translations() {}

/* Apply the character-set translations specified by the user
   to the NREAD bytes in BUF.  */

fn translate_buffer() {}

/* If true, the last char from the previous call to 'swab_buffer'
   is saved in 'saved_char'.  */
static char_is_saved: bool = false;

/* Odd char from previous call.  */
static saved_char: char;

/* Swap NREAD bytes in BUF, plus possibly an initial char from the
   previous call.  If NREAD is odd, save the last char for the
   next call.   Return the new start of the BUF buffer.  */

fn swab_buffer() {}

/* Add OFFSET to the input offset, setting the overflow flag if
   necessary.  */

fn advance_input_offset() {}

/* Throw away RECORDS blocks of BLOCKSIZE bytes plus BYTES bytes on
   file descriptor FDESC, which is open with read permission for FILE.
   Store up to BLOCKSIZE bytes of the data at a time in IBUF or OBUF, if
   necessary. RECORDS or BYTES must be nonzero. If FDESC is
   STDIN_FILENO, advance the input offset. Return the number of
   records remaining, i.e., that were not skipped because EOF was
   reached.  If FDESC is STDOUT_FILENO, on return, BYTES is the
   remaining bytes in addition to the remaining records.  */

fn skip() {}

/* Advance the input by NBYTES if possible, after a read error.
   The input file offset may or may not have advanced after the failed
   read; adjust it to point just after the bad record regardless.
   Return true if successful, or if the input is already known to not
   be seekable.  */

fn advance_input_after_read_error() {}

/* Copy NREAD bytes of BUF, with no conversions.  */

fn copy_simple() {}

/* Copy NREAD bytes of BUF, doing conv=block
   (pad newline-terminated records to 'conversion_blocksize',
   replacing the newline with trailing spaces).  */

fn copy_with_block() {}

/* Copy NREAD bytes of BUF, doing conv=unblock
   (replace trailing spaces in 'conversion_blocksize'-sized records
   with a newline).  */

fn copy_with_unblock() {}

/* Set the file descriptor flags for FD that correspond to the nonzero bits
   in ADD_FLAGS.  The file's name is NAME.  */

fn set_fd_flags() {}

/* The main loop.  */

fn dd_copy() {}

ub fn uumain(args: impl uucore::Args) -> i32 {
    let matches = App::new(executable!())
        .name(NAME)
        // TrailingVarArg specifies the final positional argument is a VarArg
        // and it doesn't attempts the parse any further args.
        // Final argument must have multiple(true) or the usage string equivalent.
        .setting(clap::AppSettings::TrailingVarArg)
        .setting(clap::AppSettings::AllowLeadingHyphen)
        .version(crate_version!())
        .about(SUMMARY)
        .after_help(AFTER_HELP)
        .usage(USAGE)
        .arg(
            Arg::with_name(options::NO_NEWLINE)
                .short("n")
                .help("do not output the trailing newline")
                .takes_value(false)
                .display_order(1),
        )
        .arg(
            Arg::with_name(options::ENABLE_BACKSLASH_ESCAPE)
                .short("e")
                .help("enable interpretation of backslash escapes")
                .takes_value(false)
                .display_order(2),
        )
        .arg(
            Arg::with_name(options::DISABLE_BACKSLASH_ESCAPE)
                .short("E")
                .help("disable interpretation of backslash escapes (default)")
                .takes_value(false)
                .display_order(3),
        )
        .arg(
            Arg::with_name(options::STRING)
                .multiple(true)
                .allow_hyphen_values(true),
        )
        .get_matches_from(args);

    let no_newline = matches.is_present(options::NO_NEWLINE);
    let escaped = matches.is_present(options::ENABLE_BACKSLASH_ESCAPE);
    let values: Vec<String> = match matches.values_of(options::STRING) {
        Some(s) => s.map(|s| s.to_string()).collect(),
        None => vec!["".to_string()],
    };

    match execute(no_newline, escaped, values) {
        Ok(_) => 0,
        Err(f) => {
            show_error!("{}", f);
            1
        }
    }
}

fn execute(no_newline: bool, escaped: bool, free: Vec<String>) -> io::Result<()> {
    let stdout = io::stdout();
    let mut output = stdout.lock();

    for (i, input) in free.iter().enumerate() {
        if i > 0 {
            write!(output, " ")?;
        }
        if escaped {
            let should_stop = print_escaped(&input, &mut output)?;
            if should_stop {
                break;
            }
        } else {
            write!(output, "{}", input)?;
        }
    }

    if !no_newline {
        writeln!(output)?;
    }

    Ok(())
}
