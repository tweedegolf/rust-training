use error::StrompyError;
use futures::AsyncRead;
use heapless::Vec as HeaplessVec;
use nalgebra as na;
use struson::reader::{JsonReader, JsonStreamReader};

pub mod error;

type StrompyResult<T> = core::result::Result<T, StrompyError>;

/// A [nalgebra::Matrix] that is backed by some other means of storage.
/// Allows for backing [nalgebra::Matrix] with some stack-based
/// storage, like [HeaplessVec]
pub type MatrixView<'buf> = na::Matrix<
    f64,
    na::Dyn,
    na::Dyn,
    na::ViewStorage<'buf, f64, na::Dyn, na::Dyn, na::Const<1>, na::Dyn>,
>;

pub type ConstMatrixView<'buf, const R: usize, const C: usize> = na::Matrix<
    f64,
    na::Const<R>,
    na::Const<C>,
    na::ViewStorage<'buf, f64, na::Const<R>, na::Const<C>, na::Const<1>, na::Const<R>>,
>;

/// A buffer into which matrix data can be stored
#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct MatrixBuf {
    d: HeaplessVec<f64, { 6 * 6 }>,
    n: usize,
}

impl MatrixBuf {
    pub fn with_data(d: HeaplessVec<f64, { 6 * 6 }>, n: usize) -> Self {
        Self { d, n }
    }

    pub fn view<'buf>(&'buf self) -> MatrixView<'buf> {
        let rows = self.d.len() / self.n;
        let cols = self.n;
        MatrixView::from_slice_generic(&self.d, na::Dyn(rows), na::Dyn(cols))
    }

    pub fn try_const_view<'buf, const R: usize, const C: usize>(
        &'buf self,
    ) -> Result<ConstMatrixView<'buf, R, C>, StrompyError> {
        let rows = self.d.len() / self.n;
        let cols = self.n;
        if (rows, cols) != (R, C) {
            return Err(StrompyError::InvalidDimensions);
        }

        Ok(ConstMatrixView::from_slice(&self.d))
    }

    pub async fn deserialize<R: AsyncRead + Unpin>(
        reader: &mut JsonStreamReader<R>,
    ) -> StrompyResult<Self> {
        reader.begin_object().await?;

        // First, read in the data
        let "d" = reader.next_name().await? else {
            return Err(StrompyError::Json(
                r#"Unexpected key encountered, expected "d""#,
            ));
        };
        reader.begin_array().await?;
        let mut d = HeaplessVec::new();
        while reader.has_next().await? {
            d.push(reader.next_number().await??).unwrap();
        }
        reader.end_array().await?;

        // Then, read the number of columns
        let "n" = reader.next_name().await? else {
            return Err(StrompyError::Json(
                r#"Unexpected key encountered, expected "n""#,
            ));
        };
        let n = reader.next_number().await??;

        reader.end_object().await?;

        Ok(Self { d, n })
    }
}

/// An operation that can be performed on a Matrix
#[derive(serde::Deserialize, Debug)]
#[serde(tag = "code", rename_all = "lowercase")]
enum Operation {
    /// Perform the dot product of some matrix with `rhs`
    Dot {
        rhs: MatrixBuf,
    },
    Add {
        rhs: MatrixBuf,
    },
    // TODO for part C: support other operations
}

impl Operation {
    /// Evaluate the operation, given a [MatrixBuf]
    fn eval(self, lhs: MatrixBuf) -> Result<MatrixBuf, StrompyError> {
        match self {
            Operation::Dot { rhs } => {
                let dot = lhs.view().dot(&rhs.view());
                Ok(MatrixBuf {
                    d: HeaplessVec::from_slice(&[dot]).unwrap(),
                    n: 1,
                })
            }
            Operation::Add { rhs } => {
                todo!()
            }
        }
    }

    pub async fn deserialize<R: AsyncRead + Unpin>(
        reader: &mut JsonStreamReader<R>,
    ) -> StrompyResult<Self> {
        // Reads the rhs field as a MatrixBuf
        async fn read_rhs<R: AsyncRead + Unpin>(
            reader: &mut JsonStreamReader<R>,
        ) -> StrompyResult<MatrixBuf> {
            let "rhs" = reader.next_name().await? else {
                return Err(StrompyError::Json(
                    r#"Unexpected key encountered, expected "rhs""#,
                ));
            };

            let rhs = MatrixBuf::deserialize(reader).await?;
            Ok(rhs)
        }

        reader.begin_object().await?;

        // Read op code
        let "code" = reader.next_name().await? else {
            return Err(StrompyError::Json(
                r#"Unexpected key encountered, expected "code""#,
            ));
        };
        let code = reader.next_str().await?;

        // Depending on op code, read further data
        let op = match code {
            "dot" => Self::Dot {
                rhs: read_rhs(reader).await?,
            },
            _ => return Err(StrompyError::Json("Unexpected Operation code")),
        };

        reader.end_object().await?;

        Ok(op)
    }
}

/// A single piece of work
#[derive(serde::Deserialize, Debug)]
pub struct PieceOfWork {
    lhs: MatrixBuf,
    op: HeaplessVec<Operation, 5>,
}

impl PieceOfWork {
    /// Execute a single [PieceOfWork] that
    /// has already been read fully into memory.
    pub fn exec(self) -> Result<MatrixBuf, StrompyError> {
        let res = self
            .op
            .into_iter()
            .try_fold(self.lhs, |rhs: MatrixBuf, op| op.eval(rhs));

        res
    }

    /// Read and execute a single [PieceOfWork]
    pub async fn exec_streamingly<R: AsyncRead + Unpin>(
        reader: &mut JsonStreamReader<R>,
    ) -> StrompyResult<MatrixBuf> {
        reader.begin_object().await?;

        // First, we need the `lhs` object
        let "lhs" = reader.next_name().await? else {
            return Err(StrompyError::Json(
                r#"Unexpected key encountered, expected "lhs""#,
            ));
        };
        let lhs: MatrixBuf = MatrixBuf::deserialize(reader).await?;

        // Then, we read the `op` array element-by-element
        let "op" = reader.next_name().await? else {
            return Err(StrompyError::Json(
                r#"Unexpected key encountered, expected "op""#,
            ));
        };

        reader.begin_array().await?;

        // We execute operations as they come in
        let mut res = lhs;
        while reader.has_next().await? {
            let op: Operation = Operation::deserialize(reader).await?;
            res = op.eval(res)?;
        }

        reader.end_array().await?;

        reader.end_object().await?;

        Ok(res)
    }
}

mod strompychan {
    use pyo3::{prelude::*, types::PyList};
    use std::sync::Arc;

    use futures::lock::Mutex;
    use pychan::reader::PyBytesReader;
    use struson::reader::{JsonReader, JsonStreamReader};

    use crate::{MatrixBuf, PieceOfWork, StrompyResult};

    struct StrompyJsonReaderInner {
        reader: JsonStreamReader<PyBytesReader>,
        in_array: bool,
    }

    #[pyclass]
    #[derive(Clone)]
    /// Wraps a PyBytesReader, and allows for deserializing
    /// incoming bytes from that reader to MatrixBufs and Ops,
    /// in a streaming fashion, as well as executing
    pub struct StrompyJsonReader {
        // Wrapping the actual object in Arc<Mutex> makes
        // this thing Send, Sync and allows for shared ownership
        // of StrompyJsonReader, which again makes it ergonomic
        // to use from Python
        inner: Arc<Mutex<StrompyJsonReaderInner>>,
    }

    impl StrompyJsonReader {
        pub fn new(reader: PyBytesReader) -> Self {
            let reader = JsonStreamReader::new(reader);
            let inner = StrompyJsonReaderInner {
                reader,
                in_array: false,
            };

            Self {
                inner: Arc::new(Mutex::new(inner)),
            }
        }

        pub async fn next(&mut self) -> StrompyResult<Option<MatrixBuf>> {
            let mut inner = self.inner.lock().await;
            if !inner.in_array {
                inner.reader.begin_array().await.unwrap();
                inner.in_array = true;
            }
            if inner.reader.has_next().await? {
                let next = PieceOfWork::exec_streamingly(&mut inner.reader).await?;
                Ok(Some(next))
            } else {
                Ok(None)
            }
        }
    }

    #[pymethods]
    impl StrompyJsonReader {
        /*
            TODO PART B: add an async method to StrompyJsonReader that yields a PyResult<Option<Vec<Vec<f64>>>>
            and is exposed with the name '`next`'
        */
        #[pyo3(name = "next")]
        async fn next_py(&mut self) -> PyResult<Option<Py<PyList>>> {
            let m = self.next().await?;
            let pylist = m.map(|m| {
                Python::with_gil(|py| {
                    PyList::new_bound(
                        py,
                        m.d.chunks_exact(m.n)
                            .into_iter()
                            .map(|c| PyList::new_bound(py, c).unbind()),
                    )
                    .unbind()
                })
            });
            Ok(pylist)
        }
    }
}

mod py {
    use pychan::py_bytes::PyBytesSender;

    /* Take these unused import as a hint ;) */
    use futures::SinkExt;
    use pyo3::{exceptions, prelude::*, types::PyBytes};

    use crate::{strompychan::StrompyJsonReader, MatrixBuf, PieceOfWork};

    impl From<MatrixBuf> for Vec<Vec<f64>> {
        fn from(MatrixBuf { d, n }: MatrixBuf) -> Self {
            d.chunks_exact(n).into_iter().map(|c| c.to_vec()).collect()
        }
    }

    #[pyfunction]
    fn exec(json_bytes: &[u8]) -> PyResult<Vec<Vec<Vec<f64>>>> {
        let work: Vec<PieceOfWork> = serde_json::from_reader(json_bytes).unwrap();

        let mut results: Vec<MatrixBuf> = todo!("PART A: Execute each piece of work");

        // Do you have an idea what this incantation does?
        Ok(results.into_iter().map(Into::into).collect())
    }

    #[pyfunction]
    fn channel() -> (PyBytesSender, StrompyJsonReader) {
        let (tx, rx) = pychan::py_bytes::channel(16);
        let reader = rx.into_reader();
        let reader = StrompyJsonReader::new(reader);

        (tx, reader)
    }

    #[pyfunction]
    async fn feed_bytes(mut writer: PyBytesSender, bytes: Py<PyBytes>) -> PyResult<()> {
        todo!("PART B: Send the bytes to the writer")
    }

    #[pymodule]
    fn strompy(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
        todo!("Add functions and classes to the exposed modue as needed");
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use struson::reader::{JsonReader, JsonStreamReader};

    use crate::PieceOfWork;

    #[test]
    fn it_deserializes() {
        let json = include_str!("../op.json");
        let [_work]: [PieceOfWork; 1] = dbg!(serde_json::from_str(json).unwrap());
    }

    #[test]
    fn it_works() {
        let json = include_str!("../op.json");
        let [work]: [PieceOfWork; 1] = serde_json::from_str(json).unwrap();
        let res = work.exec().unwrap();
        assert_eq!(res.view(), nalgebra::matrix![1587.0]);
    }

    #[tokio::test]
    async fn it_works_streamingly() {
        use tokio_util::compat::TokioAsyncReadCompatExt;
        let file = tokio::fs::File::open("op.json").await.unwrap().compat();
        let mut json_reader = JsonStreamReader::new(file);

        json_reader.begin_array().await.unwrap();

        let res = PieceOfWork::exec_streamingly(&mut json_reader)
            .await
            .unwrap();
        assert_eq!(res.view(), nalgebra::matrix![1586.0]);

        assert!(!json_reader.has_next().await.unwrap());

        json_reader.end_array().await.unwrap();
    }
}

/// Perform some operation on two MatrixBufs that have the same dimensions.
///
/// Usage:
///
/// ```
/// # use strompy::error::StrompyError;
/// # use strompy::{ConstMatrixView, MatrixBuf};
/// # use strompy::matrix_op;
/// #
/// fn add_matrices(lhs: &MatrixBuf, rhs: &MatrixBuf) -> Result<MatrixBuf, StrompyError> {
///     let (rows, cols) = lhs.view().shape();
///     if (rows, cols) != rhs.view().shape() {
///         return Err(StrompyError::InvalidDimensions);
///     };
///     let result = matrix_op!(&lhs, &rhs, R, C, |lhs: &MatrixBuf, rhs: &MatrixBuf| {
///         let lhs: ConstMatrixView<R, C> = lhs.try_const_view().unwrap();
///         let rhs: ConstMatrixView<R, C> = rhs.try_const_view().unwrap();
///         lhs + rhs
///     });
///
///     result
/// }
/// ```
#[macro_export]
macro_rules! matrix_op {
    ($lhs:expr, $rhs:expr, $R:ident, $C:ident, $f:expr) => {{
        fn do_perform_op<const R: usize>(
            cols: usize,
            lhs: &MatrixBuf,
            rhs: &MatrixBuf,
        ) -> Result<MatrixBuf, StrompyError> {
            fn do_perform_op_inner<const $R: usize, const $C: usize>(
                lhs: &MatrixBuf,
                rhs: &MatrixBuf,
            ) -> Result<MatrixBuf, StrompyError> {
                let res = ($f)(lhs, rhs);
                Ok(MatrixBuf::with_data(
                    ::heapless::Vec::from_slice(res.as_slice()).unwrap(),
                    R * C,
                ))
            }

            match cols {
                1 => do_perform_op_inner::<R, 1>(lhs, rhs),
                2 => do_perform_op_inner::<R, 2>(lhs, rhs),
                3 => do_perform_op_inner::<R, 3>(lhs, rhs),
                4 => do_perform_op_inner::<R, 4>(lhs, rhs),
                5 => do_perform_op_inner::<R, 5>(lhs, rhs),
                6 => do_perform_op_inner::<R, 6>(lhs, rhs),
                _ => unreachable!("MatrixBufs can have at most 6 cols"),
            }
        }

        let (rows, cols) = $lhs.view().shape();
        if (rows, cols) != $rhs.view().shape() {
            return Err(StrompyError::InvalidDimensions);
        };

        match rows {
            1 => do_perform_op::<1>(cols, $lhs, $rhs),
            2 => do_perform_op::<2>(cols, $lhs, $rhs),
            3 => do_perform_op::<3>(cols, $lhs, $rhs),
            4 => do_perform_op::<4>(cols, $lhs, $rhs),
            5 => do_perform_op::<5>(cols, $lhs, $rhs),
            6 => do_perform_op::<6>(cols, $lhs, $rhs),
            _ => unreachable!("MatrixBufs can have at most 6 rows"),
        }
    }};
}