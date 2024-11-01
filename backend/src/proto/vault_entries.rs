// This file is @generated by prost-build.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewVaultEntryRequest {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub vault_id: ::prost::alloc::string::String,
    #[prost(map = "string, string", tag = "4")]
    pub more_fields: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    #[prost(string, tag = "5")]
    pub user_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetVaultEntryResponse {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub created_at: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "4")]
    pub updated_at: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag = "5")]
    pub vault_id: ::prost::alloc::string::String,
    #[prost(map = "string, string", tag = "6")]
    pub more_fields: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    #[prost(string, tag = "7")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub vault_entry_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetVaultEntryRequest {
    #[prost(string, tag = "1")]
    pub vault_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub vault_entry_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub user_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateVaultEntryRequest {
    #[prost(string, tag = "1")]
    pub vault_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteVaultEntryResponse {
    #[prost(string, tag = "1")]
    pub vault_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub vault_entry_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub status: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListVaultEntrysRequest {
    #[prost(string, tag = "1")]
    pub vault_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub vault_entry_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(int32, tag = "4")]
    pub page: i32,
    #[prost(int32, tag = "5")]
    pub page_size: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListVaultEntrysResponse {
    #[prost(message, repeated, tag = "1")]
    pub vault_entries: ::prost::alloc::vec::Vec<GetVaultEntryResponse>,
    #[prost(int32, tag = "2")]
    pub total_count: i32,
}
/// Generated server implementations.
pub mod vault_entries_manager_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with VaultEntriesManagerServer.
    #[async_trait]
    pub trait VaultEntriesManager: std::marker::Send + std::marker::Sync + 'static {
        async fn create_new_vault(
            &self,
            request: tonic::Request<super::NewVaultEntryRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetVaultEntryResponse>,
            tonic::Status,
        >;
        async fn get_vault_entry(
            &self,
            request: tonic::Request<super::GetVaultEntryRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetVaultEntryResponse>,
            tonic::Status,
        >;
        async fn update_vault_entry(
            &self,
            request: tonic::Request<super::UpdateVaultEntryRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetVaultEntryResponse>,
            tonic::Status,
        >;
        async fn delete_vault_entry(
            &self,
            request: tonic::Request<super::UpdateVaultEntryRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteVaultEntryResponse>,
            tonic::Status,
        >;
        async fn list_vault_entrys(
            &self,
            request: tonic::Request<super::ListVaultEntrysRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListVaultEntrysResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct VaultEntriesManagerServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> VaultEntriesManagerServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for VaultEntriesManagerServer<T>
    where
        T: VaultEntriesManager,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/vault_entries.VaultEntriesManager/CreateNewVault" => {
                    #[allow(non_camel_case_types)]
                    struct CreateNewVaultSvc<T: VaultEntriesManager>(pub Arc<T>);
                    impl<
                        T: VaultEntriesManager,
                    > tonic::server::UnaryService<super::NewVaultEntryRequest>
                    for CreateNewVaultSvc<T> {
                        type Response = super::GetVaultEntryResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NewVaultEntryRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as VaultEntriesManager>::create_new_vault(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = CreateNewVaultSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/vault_entries.VaultEntriesManager/GetVaultEntry" => {
                    #[allow(non_camel_case_types)]
                    struct GetVaultEntrySvc<T: VaultEntriesManager>(pub Arc<T>);
                    impl<
                        T: VaultEntriesManager,
                    > tonic::server::UnaryService<super::GetVaultEntryRequest>
                    for GetVaultEntrySvc<T> {
                        type Response = super::GetVaultEntryResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetVaultEntryRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as VaultEntriesManager>::get_vault_entry(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = GetVaultEntrySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/vault_entries.VaultEntriesManager/UpdateVaultEntry" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateVaultEntrySvc<T: VaultEntriesManager>(pub Arc<T>);
                    impl<
                        T: VaultEntriesManager,
                    > tonic::server::UnaryService<super::UpdateVaultEntryRequest>
                    for UpdateVaultEntrySvc<T> {
                        type Response = super::GetVaultEntryResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateVaultEntryRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as VaultEntriesManager>::update_vault_entry(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = UpdateVaultEntrySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/vault_entries.VaultEntriesManager/DeleteVaultEntry" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteVaultEntrySvc<T: VaultEntriesManager>(pub Arc<T>);
                    impl<
                        T: VaultEntriesManager,
                    > tonic::server::UnaryService<super::UpdateVaultEntryRequest>
                    for DeleteVaultEntrySvc<T> {
                        type Response = super::DeleteVaultEntryResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateVaultEntryRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as VaultEntriesManager>::delete_vault_entry(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = DeleteVaultEntrySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/vault_entries.VaultEntriesManager/ListVaultEntrys" => {
                    #[allow(non_camel_case_types)]
                    struct ListVaultEntrysSvc<T: VaultEntriesManager>(pub Arc<T>);
                    impl<
                        T: VaultEntriesManager,
                    > tonic::server::UnaryService<super::ListVaultEntrysRequest>
                    for ListVaultEntrysSvc<T> {
                        type Response = super::ListVaultEntrysResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListVaultEntrysRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as VaultEntriesManager>::list_vault_entrys(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ListVaultEntrysSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        let mut response = http::Response::new(empty_body());
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for VaultEntriesManagerServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "vault_entries.VaultEntriesManager";
    impl<T> tonic::server::NamedService for VaultEntriesManagerServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
