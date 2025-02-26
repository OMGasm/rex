use std::{
    fs::File,
    io::{self, BufRead, BufReader, Seek, SeekFrom},
    num::TryFromIntError,
};

pub struct FileCursor {
    file: BufReader<File>,
    window_bytes: usize,
    window_line_bytes: u64,
}

impl FileCursor {
    pub fn new(file: File, window_bytes: usize, window_line_bytes: u64) -> FileCursor {
        assert_ne!(window_bytes, 0);
        assert_ne!(window_line_bytes, 0);
        assert_eq!(
            window_bytes as u64 % window_line_bytes,
            0,
            "Cursor's window_bytes must be divisible by window_line_bytes"
        );
        FileCursor {
            file: BufReader::with_capacity(window_bytes, file),
            window_bytes,
            window_line_bytes,
        }
    }

    pub fn set_window(self, window_bytes: usize, window_line_bytes: u64) -> FileCursor {
        assert_ne!(window_bytes, 0);
        assert_ne!(window_line_bytes, 0);
        assert_eq!(
            window_bytes as u64 % window_line_bytes,
            0,
            "Cursor's window_bytes must be divisible by window_line_bytes"
        );
        FileCursor {
            file: BufReader::with_capacity(window_bytes, self.file.into_inner()),
            window_bytes,
            window_line_bytes,
        }
    }
}
impl FileCursor {
    pub fn position(&mut self) -> Result<u64, CursorError> {
        Ok(self.file.stream_position()? / self.window_line_bytes)
    }

    pub fn scroll(&mut self, direction: CursorDirection) -> Result<(), CursorError> {
        assert_eq!(direction.is_valid(), true);
        let file = &mut self.file;
        let len = file.buffer().len();
        file.consume(len);

        match direction {
            CursorDirection::Down(rows) => {
                file.rewind()?;
                file.seek_relative((rows * self.window_line_bytes).try_into()?)?;
                file.fill_buf()?;
            }
            CursorDirection::Up(rows) => {
                file.seek_relative(-(rows * self.window_line_bytes).try_into()?)?;
            }
        }

        Ok(())
    }

    pub fn buffer(&mut self) -> &[u8] {
        self.file.buffer()
    }
}

pub enum CursorError {
    InvalidMovement(TryFromIntError),
    IOError(io::Error),
}

impl From<io::Error> for CursorError {
    fn from(value: io::Error) -> Self {
        Self::IOError(value)
    }
}

impl From<TryFromIntError> for CursorError {
    fn from(value: TryFromIntError) -> Self {
        Self::InvalidMovement(value)
    }
}

/// The direction and amount to move the cursor by.
/// # Remarks
/// Amounts should be non-zero.
pub enum CursorDirection {
    Down(u64),
    Up(u64),
}

impl CursorDirection {
    fn is_valid(&self) -> bool {
        const I64_MAX: u64 = i64::MAX as u64;
        match self {
            Self::Down(1..=I64_MAX) => true,
            Self::Up(1..=I64_MAX) => true,
            _ => false,
        }
    }
}
