#![no_std]

multiversx_sc::imports!();

#[multiversx_sc::contract]
pub trait PromisesMemoryAllocation {
    #[init]
    fn init(&self) {}

    #[endpoint]
    fn dummy(&self) {}

    #[endpoint]
    fn execute(&self, address: ManagedAddress<Self::Api>) {
        self.self_proxy(address)
            .dummy()
            .with_gas_limit(60_000_000)
            .async_call_promise()
            .with_callback(self.callbacks().problematic_callback(self.blockchain().get_caller()))
            .with_extra_gas_for_callback(60_000_000u64)
            .register_promise();
    }

    #[promises_callback]
    fn problematic_callback(&self, address: ManagedAddress<Self::Api>) {

    }

    #[proxy]
    fn self_proxy(&self, address: ManagedAddress<Self::Api>) -> crate::Proxy<Self::Api>;
}
