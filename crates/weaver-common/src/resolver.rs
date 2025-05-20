use atrium_identity::handle::DnsTxtResolver;
use hickory_resolver::TokioAsyncResolver;

pub struct HickoryDnsTxtResolver {
    resolver: TokioAsyncResolver,
}

impl Default for HickoryDnsTxtResolver {
    fn default() -> Self {
        Self {
            resolver: TokioAsyncResolver::tokio_from_system_conf()
                .expect("failed to create resolver"),
        }
    }
}

impl DnsTxtResolver for HickoryDnsTxtResolver {
    async fn resolve(
        &self,
        query: &str,
    ) -> core::result::Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
        Ok(self
            .resolver
            .txt_lookup(query)
            .await
            .map_err(crate::error::NetworkError::from)?
            .iter()
            .map(|txt| txt.to_string())
            .collect())
    }
}
