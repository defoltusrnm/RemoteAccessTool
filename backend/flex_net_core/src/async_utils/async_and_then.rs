pub trait AsyncAndThen<TOk, Err, UOk> {
    fn and_then_async(
        self,
        map: impl AsyncFn(TOk) -> Result<UOk, Err>,
    ) -> impl Future<Output = Result<UOk, Err>>;
}

impl<TOk, Err, UOk> AsyncAndThen<TOk, Err, UOk> for Result<TOk, Err> {
    async fn and_then_async(self, map: impl AsyncFn(TOk) -> Result<UOk, Err>) -> Result<UOk, Err> {
        let val = self?;
        map(val).await
    }
}
