pub struct Constants {
    pub bsc_gton: &'static str,
    pub bsc_pancake_factory: &'static str,
    pub eth_dodo_dvm: &'static str,
    pub eth_gton: &'static str,
    pub eth_sushi_factory: &'static str,
    pub eth_anyv4_vault: &'static str,
    pub ftm_gton: &'static str,
    pub ftm_spirit_factory: &'static str,
    pub ftm_spooky_factory: &'static str,
    pub plg_gton: &'static str,
    pub plg_quick_factory: &'static str,
    pub plg_sushi_factory: &'static str,
    pub topic0_anyv4_swapin: &'static str,
    pub topic0_anyv4_swapout: &'static str,
    pub topic0_erc20_approve: &'static str,
    pub topic0_erc20_transfer: &'static str,
    pub topic0_univ2_burn: &'static str,
    pub topic0_univ2_mint: &'static str,
    pub topic0_univ2_pair_created: &'static str,
    pub topic0_univ2_swap: &'static str,
}

pub static C: Constants = Constants {
    ftm_gton: "0xc1be9a4d5d45beeacae296a7bd5fadbfc14602c4",
    eth_gton: "0x01e0E2e61f554eCAaeC0cC933E739Ad90f24a86d",
    plg_gton: "0xf480f38c366daac4305dc484b2ad7a496ff00cea",
    bsc_gton: "0x64D5BaF5ac030e2b7c435aDD967f787ae94D0205",
    eth_sushi_factory: "0xc0aee478e3658e2610c5f7a4a2e1777ce9e4f2ac",
    plg_sushi_factory: "0xc35dadb65012ec5796536bd9864ed8773abc74c4",
    plg_quick_factory: "0x5757371414417b8c6caad45baef941abc7d3ab32",
    ftm_spooky_factory: "0x152ee697f2e276fa89e96742e9bb9ab1f2e61be3",
    ftm_spirit_factory: "0xef45d134b73241eda7703fa787148d9c9f4950b0",
    bsc_pancake_factory: "0xca143ce32fe78f1f7019d7d551a6402fc5350c73",
    eth_dodo_dvm: "0x9f99bd1cb3a51bb6356b5de6598c5a2548fba29b",
    eth_anyv4_vault: "0x533e3c0e6b48010873B947bddC4721b1bDFF9648",
    topic0_erc20_transfer: "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef",
    topic0_erc20_approve: "0x8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925",
    topic0_anyv4_swapin: "0x05d0634fe981be85c22e2942a880821b70095d84e152c3ea3c17a4e4250d9d61",
    topic0_anyv4_swapout: "0x6b616089d04950dc06c45c6dd787d657980543f89651aec47924752c7d16c888",
    topic0_univ2_swap: "0xd78ad95fa46c994b6551d0da85fc275fe613ce37657fb8d5e3d130840159d822",
    topic0_univ2_pair_created: "0x0d3648bd0f6ba80134a33ba9275ac585d9d315f0ad8355cddefde31afa28d0e9",
    topic0_univ2_mint: "0x4c209b5fc8ad50758f13e2e1088ba56a560dff690a1c6fef26394f4c03821c4f",
    topic0_univ2_burn: "0xdccd412f0b1252819cb1fd330b93224ca42612892bb3f4f789976e6d81936496",
};
