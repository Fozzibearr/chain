(function() {var implementors = {};
implementors["beefy_primitives"] = [{"text":"impl&lt;Block:&nbsp;<a class=\"trait\" href=\"sp_runtime/traits/trait.Block.html\" title=\"trait sp_runtime::traits::Block\">BlockT</a>&gt; <a class=\"trait\" href=\"sp_api/trait.RuntimeApiInfo.html\" title=\"trait sp_api::RuntimeApiInfo\">RuntimeApiInfo</a> for dyn <a class=\"trait\" href=\"beefy_primitives/trait.BeefyApi.html\" title=\"trait beefy_primitives::BeefyApi\">BeefyApi</a>&lt;Block&gt;","synthetic":false,"types":[]}];
implementors["cumulus_primitives_core"] = [{"text":"impl&lt;Block:&nbsp;<a class=\"trait\" href=\"sp_runtime/traits/trait.Block.html\" title=\"trait sp_runtime::traits::Block\">BlockT</a>&gt; <a class=\"trait\" href=\"sp_api/trait.RuntimeApiInfo.html\" title=\"trait sp_api::RuntimeApiInfo\">RuntimeApiInfo</a> for dyn <a class=\"trait\" href=\"cumulus_primitives_core/trait.CollectCollationInfo.html\" title=\"trait cumulus_primitives_core::CollectCollationInfo\">CollectCollationInfo</a>&lt;Block&gt;","synthetic":false,"types":[]}];
implementors["frame_benchmarking"] = [{"text":"impl&lt;Block:&nbsp;<a class=\"trait\" href=\"sp_runtime/traits/trait.Block.html\" title=\"trait sp_runtime::traits::Block\">BlockT</a>&gt; <a class=\"trait\" href=\"sp_api/trait.RuntimeApiInfo.html\" title=\"trait sp_api::RuntimeApiInfo\">RuntimeApiInfo</a> for dyn <a class=\"trait\" href=\"frame_benchmarking/trait.Benchmark.html\" title=\"trait frame_benchmarking::Benchmark\">Benchmark</a>&lt;Block&gt;","synthetic":false,"types":[]}];
implementors["frame_system_rpc_runtime_api"] = [{"text":"impl&lt;Block:&nbsp;<a class=\"trait\" href=\"sp_runtime/traits/trait.Block.html\" title=\"trait sp_runtime::traits::Block\">BlockT</a>, AccountId, Index&gt; <a class=\"trait\" href=\"sp_api/trait.RuntimeApiInfo.html\" title=\"trait sp_api::RuntimeApiInfo\">RuntimeApiInfo</a> for dyn <a class=\"trait\" href=\"frame_system_rpc_runtime_api/trait.AccountNonceApi.html\" title=\"trait frame_system_rpc_runtime_api::AccountNonceApi\">AccountNonceApi</a>&lt;Block, AccountId, Index&gt;","synthetic":false,"types":[]}];
implementors["frame_try_runtime"] = [{"text":"impl&lt;Block:&nbsp;<a class=\"trait\" href=\"sp_runtime/traits/trait.Block.html\" title=\"trait sp_runtime::traits::Block\">BlockT</a>&gt; <a class=\"trait\" href=\"sp_api/trait.RuntimeApiInfo.html\" title=\"trait sp_api::RuntimeApiInfo\">RuntimeApiInfo</a> for dyn <a class=\"trait\" href=\"frame_try_runtime/trait.TryRuntime.html\" title=\"trait frame_try_runtime::TryRuntime\">TryRuntime</a>&lt;Block&gt;","synthetic":false,"types":[]}];
implementors["pallet_contracts_rpc_runtime_api"] = [{"text":"impl&lt;Block:&nbsp;<a class=\"trait\" href=\"sp_runtime/traits/trait.Block.html\" title=\"trait sp_runtime::traits::Block\">BlockT</a>, AccountId, Balance, BlockNumber, Hash&gt; <a class=\"trait\" href=\"sp_api/trait.RuntimeApiInfo.html\" title=\"trait sp_api::RuntimeApiInfo\">RuntimeApiInfo</a> for dyn <a class=\"trait\" href=\"pallet_contracts_rpc_runtime_api/trait.ContractsApi.html\" title=\"trait pallet_contracts_rpc_runtime_api::ContractsApi\">ContractsApi</a>&lt;Block, AccountId, Balance, BlockNumber, Hash&gt;","synthetic":false,"types":[]}];
implementors["pallet_mmr_primitives"] = [{"text":"impl&lt;Block:&nbsp;<a class=\"trait\" href=\"sp_runtime/traits/trait.Block.html\" title=\"trait sp_runtime::traits::Block\">BlockT</a>, Hash:&nbsp;<a class=\"trait\" href=\"parity_scale_codec/codec/trait.Codec.html\" title=\"trait parity_scale_codec::codec::Codec\">Codec</a>&gt; <a class=\"trait\" href=\"sp_api/trait.RuntimeApiInfo.html\" title=\"trait sp_api::RuntimeApiInfo\">RuntimeApiInfo</a> for dyn <a class=\"trait\" href=\"pallet_mmr_primitives/trait.MmrApi.html\" title=\"trait pallet_mmr_primitives::MmrApi\">MmrApi</a>&lt;Block, Hash&gt;","synthetic":false,"types":[]}];
implementors["pallet_root_of_trust_runtime_api"] = [{"text":"impl&lt;Block:&nbsp;<a class=\"trait\" href=\"sp_runtime/traits/trait.Block.html\" title=\"trait sp_runtime::traits::Block\">BlockT</a>, CertificateId&gt; <a class=\"trait\" href=\"sp_api/trait.RuntimeApiInfo.html\" title=\"trait sp_api::RuntimeApiInfo\">RuntimeApiInfo</a> for dyn <a class=\"trait\" href=\"pallet_root_of_trust_runtime_api/trait.RootOfTrustApi.html\" title=\"trait pallet_root_of_trust_runtime_api::RootOfTrustApi\">RootOfTrustApi</a>&lt;Block, CertificateId&gt;","synthetic":false,"types":[]}];
implementors["pallet_transaction_payment_rpc_runtime_api"] = [{"text":"impl&lt;Block:&nbsp;<a class=\"trait\" href=\"sp_runtime/traits/trait.Block.html\" title=\"trait sp_runtime::traits::Block\">BlockT</a>, Balance&gt; <a class=\"trait\" href=\"sp_api/trait.RuntimeApiInfo.html\" title=\"trait sp_api::RuntimeApiInfo\">RuntimeApiInfo</a> for dyn <a class=\"trait\" href=\"pallet_transaction_payment_rpc_runtime_api/trait.TransactionPaymentApi.html\" title=\"trait pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi\">TransactionPaymentApi</a>&lt;Block, Balance&gt;","synthetic":false,"types":[]}];
implementors["polkadot_primitives"] = [{"text":"impl&lt;Block:&nbsp;<a class=\"trait\" href=\"sp_runtime/traits/trait.Block.html\" title=\"trait sp_runtime::traits::Block\">BlockT</a>&gt; <a class=\"trait\" href=\"sp_api/trait.RuntimeApiInfo.html\" title=\"trait sp_api::RuntimeApiInfo\">RuntimeApiInfo</a> for dyn <a class=\"trait\" href=\"polkadot_primitives/v0/trait.ParachainHost.html\" title=\"trait polkadot_primitives::v0::ParachainHost\">ParachainHost</a>&lt;Block&gt;","synthetic":false,"types":[]},{"text":"impl&lt;Block:&nbsp;<a class=\"trait\" href=\"sp_runtime/traits/trait.Block.html\" title=\"trait sp_runtime::traits::Block\">BlockT</a>, H:&nbsp;<a class=\"trait\" href=\"parity_scale_codec/codec/trait.Encode.html\" title=\"trait parity_scale_codec::codec::Encode\">Encode</a> + <a class=\"trait\" href=\"parity_scale_codec/codec/trait.Decode.html\" title=\"trait parity_scale_codec::codec::Decode\">Decode</a>, N:&nbsp;<a class=\"trait\" href=\"parity_scale_codec/codec/trait.Encode.html\" title=\"trait parity_scale_codec::codec::Encode\">Encode</a> + <a class=\"trait\" href=\"parity_scale_codec/codec/trait.Decode.html\" title=\"trait parity_scale_codec::codec::Decode\">Decode</a>&gt; <a class=\"trait\" href=\"sp_api/trait.RuntimeApiInfo.html\" title=\"trait sp_api::RuntimeApiInfo\">RuntimeApiInfo</a> for dyn <a class=\"trait\" href=\"polkadot_primitives/v1/trait.ParachainHost.html\" title=\"trait polkadot_primitives::v1::ParachainHost\">ParachainHost</a>&lt;Block, H, N&gt;","synthetic":false,"types":[]}];
implementors["sp_api"] = [];
implementors["sp_authority_discovery"] = [{"text":"impl&lt;Block:&nbsp;<a class=\"trait\" href=\"sp_runtime/traits/trait.Block.html\" title=\"trait sp_runtime::traits::Block\">BlockT</a>&gt; <a class=\"trait\" href=\"sp_api/trait.RuntimeApiInfo.html\" title=\"trait sp_api::RuntimeApiInfo\">RuntimeApiInfo</a> for dyn <a class=\"trait\" href=\"sp_authority_discovery/trait.AuthorityDiscoveryApi.html\" title=\"trait sp_authority_discovery::AuthorityDiscoveryApi\">AuthorityDiscoveryApi</a>&lt;Block&gt;","synthetic":false,"types":[]}];
implementors["sp_block_builder"] = [{"text":"impl&lt;Block:&nbsp;<a class=\"trait\" href=\"sp_runtime/traits/trait.Block.html\" title=\"trait sp_runtime::traits::Block\">BlockT</a>&gt; <a class=\"trait\" href=\"sp_api/trait.RuntimeApiInfo.html\" title=\"trait sp_api::RuntimeApiInfo\">RuntimeApiInfo</a> for dyn <a class=\"trait\" href=\"sp_block_builder/trait.BlockBuilder.html\" title=\"trait sp_block_builder::BlockBuilder\">BlockBuilder</a>&lt;Block&gt;","synthetic":false,"types":[]}];
implementors["sp_consensus_aura"] = [{"text":"impl&lt;Block:&nbsp;<a class=\"trait\" href=\"sp_runtime/traits/trait.Block.html\" title=\"trait sp_runtime::traits::Block\">BlockT</a>, AuthorityId:&nbsp;<a class=\"trait\" href=\"parity_scale_codec/codec/trait.Codec.html\" title=\"trait parity_scale_codec::codec::Codec\">Codec</a>&gt; <a class=\"trait\" href=\"sp_api/trait.RuntimeApiInfo.html\" title=\"trait sp_api::RuntimeApiInfo\">RuntimeApiInfo</a> for dyn <a class=\"trait\" href=\"sp_consensus_aura/trait.AuraApi.html\" title=\"trait sp_consensus_aura::AuraApi\">AuraApi</a>&lt;Block, AuthorityId&gt;","synthetic":false,"types":[]}];
implementors["sp_consensus_babe"] = [{"text":"impl&lt;Block:&nbsp;<a class=\"trait\" href=\"sp_runtime/traits/trait.Block.html\" title=\"trait sp_runtime::traits::Block\">BlockT</a>&gt; <a class=\"trait\" href=\"sp_api/trait.RuntimeApiInfo.html\" title=\"trait sp_api::RuntimeApiInfo\">RuntimeApiInfo</a> for dyn <a class=\"trait\" href=\"sp_consensus_babe/trait.BabeApi.html\" title=\"trait sp_consensus_babe::BabeApi\">BabeApi</a>&lt;Block&gt;","synthetic":false,"types":[]}];
implementors["sp_finality_grandpa"] = [{"text":"impl&lt;Block:&nbsp;<a class=\"trait\" href=\"sp_runtime/traits/trait.Block.html\" title=\"trait sp_runtime::traits::Block\">BlockT</a>&gt; <a class=\"trait\" href=\"sp_api/trait.RuntimeApiInfo.html\" title=\"trait sp_api::RuntimeApiInfo\">RuntimeApiInfo</a> for dyn <a class=\"trait\" href=\"sp_finality_grandpa/trait.GrandpaApi.html\" title=\"trait sp_finality_grandpa::GrandpaApi\">GrandpaApi</a>&lt;Block&gt;","synthetic":false,"types":[]}];
implementors["sp_offchain"] = [{"text":"impl&lt;Block:&nbsp;<a class=\"trait\" href=\"sp_runtime/traits/trait.Block.html\" title=\"trait sp_runtime::traits::Block\">BlockT</a>&gt; <a class=\"trait\" href=\"sp_api/trait.RuntimeApiInfo.html\" title=\"trait sp_api::RuntimeApiInfo\">RuntimeApiInfo</a> for dyn <a class=\"trait\" href=\"sp_offchain/trait.OffchainWorkerApi.html\" title=\"trait sp_offchain::OffchainWorkerApi\">OffchainWorkerApi</a>&lt;Block&gt;","synthetic":false,"types":[]}];
implementors["sp_session"] = [{"text":"impl&lt;Block:&nbsp;<a class=\"trait\" href=\"sp_runtime/traits/trait.Block.html\" title=\"trait sp_runtime::traits::Block\">BlockT</a>&gt; <a class=\"trait\" href=\"sp_api/trait.RuntimeApiInfo.html\" title=\"trait sp_api::RuntimeApiInfo\">RuntimeApiInfo</a> for dyn <a class=\"trait\" href=\"sp_session/trait.SessionKeys.html\" title=\"trait sp_session::SessionKeys\">SessionKeys</a>&lt;Block&gt;","synthetic":false,"types":[]}];
implementors["sp_transaction_pool"] = [{"text":"impl&lt;Block:&nbsp;<a class=\"trait\" href=\"sp_runtime/traits/trait.Block.html\" title=\"trait sp_runtime::traits::Block\">BlockT</a>&gt; <a class=\"trait\" href=\"sp_api/trait.RuntimeApiInfo.html\" title=\"trait sp_api::RuntimeApiInfo\">RuntimeApiInfo</a> for dyn <a class=\"trait\" href=\"sp_transaction_pool/runtime_api/trait.TaggedTransactionQueue.html\" title=\"trait sp_transaction_pool::runtime_api::TaggedTransactionQueue\">TaggedTransactionQueue</a>&lt;Block&gt;","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()