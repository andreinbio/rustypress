pub trait Handler: Send + Sync + 'static {
    fn handle(self, request: Request) -> Response;
}