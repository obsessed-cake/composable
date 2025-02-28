import { usePicassoAccounts, usePicassoProvider } from "@/defi/polkadot/hooks";
import { useEffect } from "react";
import { useBlockchainProvider } from "bi-lib";
import { fromPerbill } from "shared";
import { DEFAULT_EVM_ID } from "@/defi/polkadot/constants";
import { fetchAssociations } from "./crowdloanRewards";
import { setCrowdloanRewardsState } from "./crowdloanRewards.slice"; // Import static JSON files
// Import static JSON files
// import rewardsAndContributions from "@/defi/polkadot/constants/pica-rewards-contributions.json";
// import rewardsAndContributionsDev from "@/defi/polkadot/constants/pica-rewards-contributions-dev.json";

/**
 * Check for contributions in JSON
 * @param account ethereum or kusama format address
 * @returns string | undefined
 */
export const presentInContributors = async (
  account: string,
  env: "development" | "production" | "test"
): Promise<string | undefined> => {
  const rewardsAndContributions = await import(
    "@/defi/polkadot/constants/pica-rewards-contributions.json"
  );
  const rewardsAndContributionsDev = await import(
    "@/defi/polkadot/constants/pica-rewards-contributions-dev.json"
  );
  const ethAccount = account.startsWith("0x")
    ? account.toLowerCase()
    : undefined;
  const kusamaAccount = account.startsWith("0x") ? undefined : account;
  switch (env) {
    case "production":
      if (ethAccount) {
        return (
          rewardsAndContributions.stablesContributed as Record<string, string>
        )[ethAccount];
      } else if (kusamaAccount) {
        return (
          rewardsAndContributions.ksmContributedWithBoosts as Record<
            string,
            string
          >
        )[kusamaAccount];
      }
      return undefined;
    case "development":
      if (ethAccount) {
        return (
          rewardsAndContributionsDev.stablesContributed as Record<
            string,
            string
          >
        )[ethAccount];
      } else if (kusamaAccount) {
        return (
          rewardsAndContributionsDev.ksmContributedWithBoosts as Record<
            string,
            string
          >
        )[kusamaAccount];
      }
      return undefined;
    default:
      return undefined;
  }
};

/**
 * Check for rewards in JSON
 * @param account ethereum or kusama format address
 * @returns string | undefined
 */
export const presentInRewards = async (
  account: string,
  env: "development" | "production" | "test"
): Promise<string | undefined> => {
  const rewardsAndContributions = await import(
    "@/defi/polkadot/constants/pica-rewards-contributions.json"
  );
  const rewardsAndContributionsDev = await import(
    "@/defi/polkadot/constants/pica-rewards-contributions-dev.json"
  );
  const ethAccount = account.startsWith("0x")
    ? account.toLowerCase()
    : undefined;
  const kusamaAccount = account.startsWith("0x") ? undefined : account;
  switch (env) {
    case "production":
      if (ethAccount) {
        return (
          rewardsAndContributions.rewardedPICAs as Record<string, string>
        )[ethAccount];
      } else if (kusamaAccount) {
        return (
          rewardsAndContributions.rewardedPICAs as Record<string, string>
        )[kusamaAccount];
      }
      return undefined;
    case "development":
      if (ethAccount) {
        return (
          rewardsAndContributionsDev.rewardedPICAs as Record<string, string>
        )[ethAccount];
      } else if (kusamaAccount) {
        return (
          rewardsAndContributionsDev.rewardedPICAs as Record<string, string>
        )[kusamaAccount];
      }
      return undefined;
    default:
      return undefined;
  }
};

const CrowdloanRewardsUpdater = () => {
  const { account } = useBlockchainProvider(DEFAULT_EVM_ID);
  const { parachainApi } = usePicassoProvider();
  const accounts = usePicassoAccounts();
  /**
   * Update initialPayment
   */
  useEffect(() => {
    if (parachainApi) {
      const initialPayment = fromPerbill(
        parachainApi.consts.crowdloanRewards.initialPayment.toString()
      ).div(100);
      setCrowdloanRewardsState({ initialPayment });
    }
  }, [parachainApi]);
  /**
   * Fetch connected accounts' associations
   */
  useEffect(() => {
    if (parachainApi && accounts.length > 0) {
      const addresses = accounts.map((_account) => _account.address);
      fetchAssociations(
        parachainApi,
        addresses.filter((address) => !address.startsWith("0x"))
      )
        .then((onChainAssociations) => {
          setCrowdloanRewardsState({ onChainAssociations });
        })
        .catch(console.error);
    }
  }, [parachainApi, accounts]);

  return null;
};

export default CrowdloanRewardsUpdater;
