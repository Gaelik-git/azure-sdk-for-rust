use crate::prelude::*;
use azure_core::{
    error::Error, headers::Headers, CollectedResponse, Continuable, Method, Pageable,
};
use url::Url;

operation! {
    #[stream]
    ListCertificates,
    client: CertificateClient,
}

impl ListCertificatesBuilder {
    pub fn into_stream(self) -> Pageable<KeyVaultGetCertificatesResponse, Error> {
        let make_request = move |continuation: Option<String>| {
            let this = self.clone();
            let ctx = self.context.clone();
            async move {
                let url = if let Some(url) = continuation {
                    Url::parse(&url)?
                } else {
                    let mut url = this.client.keyvault_client.vault_url.clone();
                    url.set_path("certificates");
                    url
                };

                let headers = Headers::new();
                let mut request = this.client.keyvault_client.finalize_request(
                    url,
                    Method::Get,
                    headers,
                    None,
                )?;

                let response = this.client.keyvault_client.send(&ctx, &mut request).await?;

                let response = CollectedResponse::from_response(response).await?;
                let body = response.body();

                let response = serde_json::from_slice::<KeyVaultGetCertificatesResponse>(body)?;
                Ok(response)
            }
        };
        Pageable::new(make_request)
    }
}

type ListCertificatesResponse = KeyVaultGetCertificatesResponse;

impl Continuable for ListCertificatesResponse {
    type Continuation = String;

    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
