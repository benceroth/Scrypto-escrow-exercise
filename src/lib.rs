use scrypto::prelude::*;

#[blueprint]
#[types(EscrowBadge)]
mod escrow {
    enable_function_auth! {
        instantiate_fungible => rule!(allow_all);
        instantiate_nonfungible => rule!(allow_all);
        instantiate_escrow => rule!(allow_all);
    }
    struct Escrow {
        requested_resource: crate::ResourceSpecifier,
        offered_resource: Vault,
        requested_resource_vault: Vault,
        escrow_nft: ResourceAddress,
    }

    impl Escrow {

        // Provide an option for resim instatiation without Enum for fungible
        pub fn instantiate_fungible(
            requested_address: ResourceAddress,
            requested_amount: Decimal,
            offered_resource: Bucket
        ) -> (Global<Escrow>, NonFungibleBucket) {
            let specifier: crate::ResourceSpecifier = crate::ResourceSpecifier::Fungible{
                    resource_address: requested_address,
                    amount: requested_amount,
                };
            
            Self::instantiate_escrow(specifier, offered_resource)
        }

        // Provide an option for resim instatiation without Enum for nonfungible
        pub fn instantiate_nonfungible(
            requested_address: ResourceAddress, requested_id: NonFungibleLocalId,
            offered_resource: Bucket
        ) -> (Global<Escrow>, NonFungibleBucket) {
            let specifier: crate::ResourceSpecifier = crate::ResourceSpecifier::NonFungible{
                    resource_address: requested_address,
                    non_fungible_local_id: requested_id,
                };
            
            Self::instantiate_escrow(specifier, offered_resource)
        }

        pub fn instantiate_escrow(
            requested_resource: crate::ResourceSpecifier,
            offered_resource: Bucket
        ) -> (Global<Escrow>, NonFungibleBucket) {
            
            // Reserve address to be able to provide burn role only to the component itself
            let (address_reservation, component_address) = Runtime::allocate_component_address(Escrow::blueprint_id());

            // Create badge for the component instantiator so that the initiator can cancel/withdraw.
            let badge: NonFungibleBucket = ResourceBuilder::new_integer_non_fungible::<EscrowBadge>(OwnerRole::None)
            .burn_roles(burn_roles!(
                burner => rule!(require(global_caller(component_address)));
                burner_updater => rule!(deny_all);
                ))
            .mint_initial_supply([
                (IntegerNonFungibleLocalId::new(1),
                    EscrowBadge {
                        offered_resource: offered_resource.resource_address(),
                    },
                ) 
            ])
            ;

            let component = Self {
                offered_resource: Vault::with_bucket(offered_resource),
                requested_resource_vault: Vault::new(requested_resource.get_resource_address()),
                requested_resource: requested_resource,
                escrow_nft: badge.resource_address(),
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::None)
            .with_address(address_reservation)
            .metadata(metadata!(
                init {
                    "name" => "Escrow", locked;
                    "description" => "Escrow challenge.", locked;
                }
            ));

            // Return the globalized component and the badge
            (component.globalize(), badge)
        }

        pub fn exchange(&mut self, bucket_of_resource: Bucket) -> Bucket {
            assert!(
                self.offered_resource.amount() > dec!(0),
                "The exchange is not possible due to cancellation or it has already closed."
            );

            assert_eq!(
                bucket_of_resource.resource_address(), self.requested_resource.get_resource_address(),
                "You have provided a wrong type of resource."
            );

            // Assertions based on the type of the requested resource, for fungible the amount, for NFT the localid
            match &self.requested_resource {
                crate::ResourceSpecifier::Fungible { resource_address: _, amount } => {
                    assert_eq!(
                        bucket_of_resource.amount(), *amount,
                        "You have provided a wrong amount of token."
                    );
                },
                crate::ResourceSpecifier::NonFungible { resource_address: _, non_fungible_local_id } => {
                    let nft_local_id: NonFungibleLocalId = bucket_of_resource.as_non_fungible().non_fungible_local_id();
                    assert_eq!(
                        nft_local_id, *non_fungible_local_id,
                        "You have provided a wrong NFT."
                    );
                },
            };

            // Make the exchange
            self.requested_resource_vault.put(bucket_of_resource);
            self.offered_resource.take_all()
        }

        pub fn withdraw_resource(&mut self, escrow_nft: NonFungibleBucket) -> Bucket {
            assert_eq!(
                escrow_nft.resource_address(), self.escrow_nft,
                "You have provided a wrong badge."
            );

            assert!(
                self.requested_resource_vault.amount() > dec!(0),
                "Withdraw is not possible until requested resource is deposited."
            );

            // Withdraw the requested resource after successful exchange and burn the token
            escrow_nft.burn();
            self.requested_resource_vault.take_all()
        }

        pub fn cancel_escrow(&mut self, escrow_nft: NonFungibleBucket) -> Bucket {
            assert_eq!(
                escrow_nft.resource_address(), self.escrow_nft,
                "You have provided a wrong badge."
            );

            // Cancel the escrow by returning the offered resource and burning the token
            escrow_nft.burn();
            self.offered_resource.take_all()
        }
    }
}

#[derive(ScryptoSbor, Debug)]
pub enum ResourceSpecifier {
    Fungible {
        resource_address: ResourceAddress,
        amount: Decimal
    },
    NonFungible {
        resource_address: ResourceAddress,
        non_fungible_local_id: NonFungibleLocalId
    }
}

impl ResourceSpecifier {

    pub fn get_resource_address(&self) -> ResourceAddress {
        match self {
            Self::Fungible {
                resource_address, ..
            }
            | Self::NonFungible {
                resource_address, ..
            } => *resource_address,
        }
    }
}

#[derive(ScryptoSbor, NonFungibleData)]
pub struct EscrowBadge {
    offered_resource: ResourceAddress
}