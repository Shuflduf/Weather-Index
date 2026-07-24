pub async fn get(
    State(state): State<Arc<WIState>>,
    Path(username): Path<String>,
) -> Result<Json<ListRetur>, WIError> {
}
