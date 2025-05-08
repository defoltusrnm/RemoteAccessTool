pub trait AsyncAndThen<TOk, Err, UOk, F>
where
    F: AsyncFn(TOk) -> Result<UOk, Err>
{
    fn and_then_async(self, map: F) -> impl Future<Output = Result<UOk, Err>>;
}

impl<TOk, Err, UOk, F> AsyncAndThen<TOk, Err, UOk, F> for Result<TOk, Err>
where
    TOk: Send,
    UOk: Send,
    F: AsyncFn(TOk) -> Result<UOk, Err>
{
    async fn and_then_async(self, map: F) -> Result<UOk, Err> {
        match self {
            Ok(val) => {
                let x = map(val).await;
                x
            },
            Err(err) => Err(err)
        }
    }
}
