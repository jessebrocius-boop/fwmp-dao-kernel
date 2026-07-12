#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use tower::ServiceExt; // oneshot
    use serde_json::json;
    use hyper;

    #[tokio::test]
    async fn accepted_flow() {
        let app = router();
        let payload = json!({
            "directive": "do_something",
            "origin_node": "TESTNODE",
            "data": { "k": "v" }
        });

        let req = Request::builder()
            .method("POST")
            .uri("/submit")
            .header("content-type", "application/json")
            .header("x-idempotency-key", "abc123")
            .body(Body::from(payload.to_string()))
            .unwrap();

        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), 200);

        let body_bytes = hyper::body::to_bytes(resp.into_body()).await.unwrap();
        let v: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
        assert_eq!(v["status"], "ACCEPTED");
    }

    #[tokio::test]
    async fn rejected_flow_missing_idempotency() {
        let app = router();
        let payload = json!({
            "directive": "do_something"
        });

        let req = Request::builder()
            .method("POST")
            .uri("/submit")
            .header("content-type", "application/json")
            .body(Body::from(payload.to_string()))
            .unwrap();

        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), 400);

        let body_bytes = hyper::body::to_bytes(resp.into_body()).await.unwrap();
        let v: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
        assert_eq!(v["status"], "REJECTED");
    }
}
