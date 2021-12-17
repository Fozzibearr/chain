(function() {var implementors = {};
implementors["pallet_xcm"] = [{"text":"impl&lt;Origin:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/1.57.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"pallet_xcm/pallet/enum.Origin.html\" title=\"enum pallet_xcm::pallet::Origin\">Origin</a>&gt;&gt; <a class=\"trait\" href=\"xcm_executor/traits/conversion/trait.ConvertOrigin.html\" title=\"trait xcm_executor::traits::conversion::ConvertOrigin\">ConvertOrigin</a>&lt;Origin&gt; for <a class=\"struct\" href=\"pallet_xcm/struct.XcmPassthrough.html\" title=\"struct pallet_xcm::XcmPassthrough\">XcmPassthrough</a>&lt;Origin&gt;","synthetic":false,"types":["pallet_xcm::XcmPassthrough"]}];
implementors["xcm_builder"] = [{"text":"impl&lt;LocationConverter:&nbsp;<a class=\"trait\" href=\"xcm_executor/traits/conversion/trait.Convert.html\" title=\"trait xcm_executor::traits::conversion::Convert\">Convert</a>&lt;<a class=\"struct\" href=\"xcm/v1/multilocation/struct.MultiLocation.html\" title=\"struct xcm::v1::multilocation::MultiLocation\">MultiLocation</a>, Origin::<a class=\"type\" href=\"frame_support/traits/dispatch/trait.OriginTrait.html#associatedtype.AccountId\" title=\"type frame_support::traits::dispatch::OriginTrait::AccountId\">AccountId</a>&gt;, Origin:&nbsp;<a class=\"trait\" href=\"frame_support/traits/dispatch/trait.OriginTrait.html\" title=\"trait frame_support::traits::dispatch::OriginTrait\">OriginTrait</a>&gt; <a class=\"trait\" href=\"xcm_executor/traits/conversion/trait.ConvertOrigin.html\" title=\"trait xcm_executor::traits::conversion::ConvertOrigin\">ConvertOrigin</a>&lt;Origin&gt; for <a class=\"struct\" href=\"xcm_builder/struct.SovereignSignedViaLocation.html\" title=\"struct xcm_builder::SovereignSignedViaLocation\">SovereignSignedViaLocation</a>&lt;LocationConverter, Origin&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Origin::<a class=\"type\" href=\"frame_support/traits/dispatch/trait.OriginTrait.html#associatedtype.AccountId\" title=\"type frame_support::traits::dispatch::OriginTrait::AccountId\">AccountId</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.57.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,&nbsp;</span>","synthetic":false,"types":["xcm_builder::origin_conversion::SovereignSignedViaLocation"]},{"text":"impl&lt;Origin:&nbsp;<a class=\"trait\" href=\"frame_support/traits/dispatch/trait.OriginTrait.html\" title=\"trait frame_support::traits::dispatch::OriginTrait\">OriginTrait</a>&gt; <a class=\"trait\" href=\"xcm_executor/traits/conversion/trait.ConvertOrigin.html\" title=\"trait xcm_executor::traits::conversion::ConvertOrigin\">ConvertOrigin</a>&lt;Origin&gt; for <a class=\"struct\" href=\"xcm_builder/struct.ParentAsSuperuser.html\" title=\"struct xcm_builder::ParentAsSuperuser\">ParentAsSuperuser</a>&lt;Origin&gt;","synthetic":false,"types":["xcm_builder::origin_conversion::ParentAsSuperuser"]},{"text":"impl&lt;ParaId:&nbsp;<a class=\"trait\" href=\"polkadot_parachain/primitives/trait.IsSystem.html\" title=\"trait polkadot_parachain::primitives::IsSystem\">IsSystem</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.57.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.57.0/std/primitive.u32.html\">u32</a>&gt;, Origin:&nbsp;<a class=\"trait\" href=\"frame_support/traits/dispatch/trait.OriginTrait.html\" title=\"trait frame_support::traits::dispatch::OriginTrait\">OriginTrait</a>&gt; <a class=\"trait\" href=\"xcm_executor/traits/conversion/trait.ConvertOrigin.html\" title=\"trait xcm_executor::traits::conversion::ConvertOrigin\">ConvertOrigin</a>&lt;Origin&gt; for <a class=\"struct\" href=\"xcm_builder/struct.ChildSystemParachainAsSuperuser.html\" title=\"struct xcm_builder::ChildSystemParachainAsSuperuser\">ChildSystemParachainAsSuperuser</a>&lt;ParaId, Origin&gt;","synthetic":false,"types":["xcm_builder::origin_conversion::ChildSystemParachainAsSuperuser"]},{"text":"impl&lt;ParaId:&nbsp;<a class=\"trait\" href=\"polkadot_parachain/primitives/trait.IsSystem.html\" title=\"trait polkadot_parachain::primitives::IsSystem\">IsSystem</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.57.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.57.0/std/primitive.u32.html\">u32</a>&gt;, Origin:&nbsp;<a class=\"trait\" href=\"frame_support/traits/dispatch/trait.OriginTrait.html\" title=\"trait frame_support::traits::dispatch::OriginTrait\">OriginTrait</a>&gt; <a class=\"trait\" href=\"xcm_executor/traits/conversion/trait.ConvertOrigin.html\" title=\"trait xcm_executor::traits::conversion::ConvertOrigin\">ConvertOrigin</a>&lt;Origin&gt; for <a class=\"struct\" href=\"xcm_builder/struct.SiblingSystemParachainAsSuperuser.html\" title=\"struct xcm_builder::SiblingSystemParachainAsSuperuser\">SiblingSystemParachainAsSuperuser</a>&lt;ParaId, Origin&gt;","synthetic":false,"types":["xcm_builder::origin_conversion::SiblingSystemParachainAsSuperuser"]},{"text":"impl&lt;ParachainOrigin:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/1.57.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.57.0/std/primitive.u32.html\">u32</a>&gt;, Origin:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/1.57.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;ParachainOrigin&gt;&gt; <a class=\"trait\" href=\"xcm_executor/traits/conversion/trait.ConvertOrigin.html\" title=\"trait xcm_executor::traits::conversion::ConvertOrigin\">ConvertOrigin</a>&lt;Origin&gt; for <a class=\"struct\" href=\"xcm_builder/struct.ChildParachainAsNative.html\" title=\"struct xcm_builder::ChildParachainAsNative\">ChildParachainAsNative</a>&lt;ParachainOrigin, Origin&gt;","synthetic":false,"types":["xcm_builder::origin_conversion::ChildParachainAsNative"]},{"text":"impl&lt;ParachainOrigin:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/1.57.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.57.0/std/primitive.u32.html\">u32</a>&gt;, Origin:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/1.57.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;ParachainOrigin&gt;&gt; <a class=\"trait\" href=\"xcm_executor/traits/conversion/trait.ConvertOrigin.html\" title=\"trait xcm_executor::traits::conversion::ConvertOrigin\">ConvertOrigin</a>&lt;Origin&gt; for <a class=\"struct\" href=\"xcm_builder/struct.SiblingParachainAsNative.html\" title=\"struct xcm_builder::SiblingParachainAsNative\">SiblingParachainAsNative</a>&lt;ParachainOrigin, Origin&gt;","synthetic":false,"types":["xcm_builder::origin_conversion::SiblingParachainAsNative"]},{"text":"impl&lt;RelayOrigin:&nbsp;<a class=\"trait\" href=\"frame_support/traits/misc/trait.Get.html\" title=\"trait frame_support::traits::misc::Get\">Get</a>&lt;Origin&gt;, Origin&gt; <a class=\"trait\" href=\"xcm_executor/traits/conversion/trait.ConvertOrigin.html\" title=\"trait xcm_executor::traits::conversion::ConvertOrigin\">ConvertOrigin</a>&lt;Origin&gt; for <a class=\"struct\" href=\"xcm_builder/struct.RelayChainAsNative.html\" title=\"struct xcm_builder::RelayChainAsNative\">RelayChainAsNative</a>&lt;RelayOrigin, Origin&gt;","synthetic":false,"types":["xcm_builder::origin_conversion::RelayChainAsNative"]},{"text":"impl&lt;Network:&nbsp;<a class=\"trait\" href=\"frame_support/traits/misc/trait.Get.html\" title=\"trait frame_support::traits::misc::Get\">Get</a>&lt;<a class=\"enum\" href=\"xcm/v0/junction/enum.NetworkId.html\" title=\"enum xcm::v0::junction::NetworkId\">NetworkId</a>&gt;, Origin:&nbsp;<a class=\"trait\" href=\"frame_support/traits/dispatch/trait.OriginTrait.html\" title=\"trait frame_support::traits::dispatch::OriginTrait\">OriginTrait</a>&gt; <a class=\"trait\" href=\"xcm_executor/traits/conversion/trait.ConvertOrigin.html\" title=\"trait xcm_executor::traits::conversion::ConvertOrigin\">ConvertOrigin</a>&lt;Origin&gt; for <a class=\"struct\" href=\"xcm_builder/struct.SignedAccountId32AsNative.html\" title=\"struct xcm_builder::SignedAccountId32AsNative\">SignedAccountId32AsNative</a>&lt;Network, Origin&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Origin::<a class=\"type\" href=\"frame_support/traits/dispatch/trait.OriginTrait.html#associatedtype.AccountId\" title=\"type frame_support::traits::dispatch::OriginTrait::AccountId\">AccountId</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.57.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.57.0/std/primitive.array.html\">[</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/1.57.0/std/primitive.u8.html\">u8</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/1.57.0/std/primitive.array.html\">; 32]</a>&gt;,&nbsp;</span>","synthetic":false,"types":["xcm_builder::origin_conversion::SignedAccountId32AsNative"]},{"text":"impl&lt;Network:&nbsp;<a class=\"trait\" href=\"frame_support/traits/misc/trait.Get.html\" title=\"trait frame_support::traits::misc::Get\">Get</a>&lt;<a class=\"enum\" href=\"xcm/v0/junction/enum.NetworkId.html\" title=\"enum xcm::v0::junction::NetworkId\">NetworkId</a>&gt;, Origin:&nbsp;<a class=\"trait\" href=\"frame_support/traits/dispatch/trait.OriginTrait.html\" title=\"trait frame_support::traits::dispatch::OriginTrait\">OriginTrait</a>&gt; <a class=\"trait\" href=\"xcm_executor/traits/conversion/trait.ConvertOrigin.html\" title=\"trait xcm_executor::traits::conversion::ConvertOrigin\">ConvertOrigin</a>&lt;Origin&gt; for <a class=\"struct\" href=\"xcm_builder/struct.SignedAccountKey20AsNative.html\" title=\"struct xcm_builder::SignedAccountKey20AsNative\">SignedAccountKey20AsNative</a>&lt;Network, Origin&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Origin::<a class=\"type\" href=\"frame_support/traits/dispatch/trait.OriginTrait.html#associatedtype.AccountId\" title=\"type frame_support::traits::dispatch::OriginTrait::AccountId\">AccountId</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.57.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.57.0/std/primitive.array.html\">[</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/1.57.0/std/primitive.u8.html\">u8</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/1.57.0/std/primitive.array.html\">; 20]</a>&gt;,&nbsp;</span>","synthetic":false,"types":["xcm_builder::origin_conversion::SignedAccountKey20AsNative"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()