use std::time::Duration;
use tokio::time::sleep;

pub async fn with_retry<F, Fut, T>(
    max_retries: u32,
    mut operation: F,
) -> Result<T, crate::VelixError>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<T, crate::VelixError>>,
{
    let mut attempt = 0;
    loop {
        match operation().await {
            Ok(val) => return Ok(val),
            Err(e) => {
                let retryable = matches!(
                    &e,
                    crate::VelixError::Http { status, .. } if *status == 429 || *status == 503
                ) || matches!(&e, crate::VelixError::RateLimit { .. });

                if !retryable || attempt >= max_retries {
                    return Err(e);
                }

                let backoff = Duration::from_millis(200 * 2u64.pow(attempt));
                sleep(backoff).await;
                attempt += 1;
            }
        }
    }
}
