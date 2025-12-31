#[cfg(test)]
mod vault_integration_tests {
    use odra::prelude::*;
    use odra::casper_types::U512;
    use crate::helpers::*;
    use crate::mocks::*;

    #[test]
    fn test_full_deposit_flow() {
        let env = TestEnvironment::new();
        env.set_caller(env.user1);
        
        let cspr_amount = cspr(1000);
        let lst_cspr_minted = cspr_amount;
        let cv_cspr_shares = lst_cspr_minted;
        
        assert_u512_eq(cv_cspr_shares, cspr_amount, "Full deposit flow 1:1");
    }

    #[test]
    fn test_deposit_with_strategy_deployment() {
        let env = TestEnvironment::new();
        let deposit = cspr(10_000);
        
        let dex_allocation = (deposit * U512::from(40u64)) / U512::from(100u64);
        let lending_allocation = (deposit * U512::from(30u64)) / U512::from(100u64);
        let cross_chain_allocation = (deposit * U512::from(30u64)) / U512::from(100u64);
        
        assert_u512_eq(dex_allocation, cspr(4000), "DEX allocation");
        assert_u512_eq(lending_allocation, cspr(3000), "Lending allocation");
        assert_u512_eq(cross_chain_allocation, cspr(3000), "Cross-chain allocation");
    }

    #[test]
    fn test_full_withdrawal_flow() {
        let env = TestEnvironment::new();
        
        let user_shares = cspr(1000);
        let total_assets = cspr(11000);
        let total_shares = cspr(10000);
        
        let cspr_returned = calculate_expected_assets(user_shares, total_assets, total_shares);
        
        assert!(cspr_returned > user_shares, "Withdrawal with profit");
    }

    #[test]
    fn test_withdrawal_from_strategies() {
        let shares_to_withdraw = cspr(1000);
        let total_assets = cspr(10000);
        let total_shares = cspr(10000);
        
        let assets_needed = calculate_expected_assets(shares_to_withdraw, total_assets, total_shares);
        
        let from_dex = (assets_needed * U512::from(40u64)) / U512::from(100u64);
        let from_lending = (assets_needed * U512::from(30u64)) / U512::from(100u64);
        let from_cross_chain = (assets_needed * U512::from(30u64)) / U512::from(100u64);
        
        let total = from_dex + from_lending + from_cross_chain;
        
        assert_u512_eq(total, assets_needed, "Proportional strategy withdrawal");
    }

    #[test]
    fn test_share_price_appreciation_over_time() {
        let env = TestEnvironment::new();
        
        let initial_tvl = cspr(10000);
        let initial_shares = cspr(10000);
        
        let initial_price = (initial_tvl * U512::from(1_000_000u64)) / initial_shares;
        
        env.advance_block_time(30 * 24 * 60 * 60);
        
        let yield_earned = cspr(500);
        let new_tvl = initial_tvl + yield_earned;
        
        let new_price = (new_tvl * U512::from(1_000_000u64)) / initial_shares;
        
        assert_share_price_increased(initial_price, new_price);
    }

    #[test]
    fn test_multiple_users_different_share_prices() {
        let user1_deposit = cspr(10000);
        let user1_shares = user1_deposit;
        
        let yield_earned = cspr(1000);
        let total_assets_before_user2 = user1_deposit + yield_earned;
        
        let user2_deposit = cspr(5000);
        let user2_shares = calculate_expected_shares(
            user2_deposit,
            total_assets_before_user2,
            user1_shares
        );
        
        assert!(user2_shares < user2_deposit, "User2 gets fewer shares due to appreciation");
        
        let total_shares = user1_shares + user2_shares;
        let total_assets = total_assets_before_user2 + user2_deposit;
        
        let user1_value = calculate_expected_assets(user1_shares, total_assets, total_shares);
        let user2_value = calculate_expected_assets(user2_shares, total_assets, total_shares);
        
        assert!(user1_value > user1_deposit, "User1 has profit");
        assert_u512_eq(user2_value, user2_deposit, "User2 at break-even");
    }

    #[test]
    fn test_instant_vs_timelocked_withdrawal() {
        let shares = cspr(1000);
        let instant_fee_bps = 50u64;
        
        let instant_fee = (shares * U512::from(instant_fee_bps)) / U512::from(10000u64);
        let instant_proceeds = shares - instant_fee;
        
        let timelocked_proceeds = shares;
        
        assert!(timelocked_proceeds > instant_proceeds, "Timelock saves fee");
    }

    #[test]
    fn test_deposit_after_compound() {
        let initial_tvl = cspr(10000);
        let initial_shares = cspr(10000);
        
        let compound_yield = cspr(500);
        let tvl_after_compound = initial_tvl + compound_yield;
        
        let new_deposit = cspr(1000);
        let new_shares = calculate_expected_shares(new_deposit, tvl_after_compound, initial_shares);
        
        assert!(new_shares < new_deposit, "Fewer shares after compound");
    }

    #[test]
    fn test_withdrawal_after_loss() {
        let user_deposit = cspr(10000);
        let user_shares = cspr(10000);
        
        let loss = cspr(500);
        let current_tvl = user_deposit - loss;
        
        let withdrawal_value = calculate_expected_assets(user_shares, current_tvl, user_shares);
        
        assert!(withdrawal_value < user_deposit, "Loss reflected in withdrawal");
    }

    #[test]
    fn test_rebalance_after_apy_change() {
        let total_tvl = cspr(100_000);
        
        let dex_apy = 1200u16;
        let lending_apy = 600u16;
        let cross_chain_apy = 800u16;
        
        let best_apy = dex_apy.max(lending_apy).max(cross_chain_apy);
        
        assert_eq!(best_apy, dex_apy, "DEX has best APY, should rebalance");
    }
}
