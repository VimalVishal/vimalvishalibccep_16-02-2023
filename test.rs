use frame_support::{decl_module, decl_storage, decl_event, dispatch, ensure};
use frame_system::{self as system, ensure_signed};
use sp_std::vec::Vec;

// Define the group struct
#[derive(Default, Clone, PartialEq)]
pub struct Group {
    members: Vec<u64>,
}

// Declare the module
decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        // Initialize the pallet
        fn deposit_event() = default;

        // Add a member to the group
        #[weight = 0]
        pub fn add_member(origin, member: u64) -> dispatch::DispatchResult {
            let who = ensure_signed(origin)?;

            // Only sudo can add members
            ensure!(T::IsSudo::is_sudo(&who), "Only sudo can add members");

            // Add the member to the group
            let mut group = Self::group();
            group.members.push(member);
            <GroupStorage<T>>::put(group);

            Ok(())
        }

        // Remove a member from the group
        #[weight = 0]
        pub fn remove_member(origin, member: u64) -> dispatch::DispatchResult {
            let who = ensure_signed(origin)?;

            // Only sudo can remove members
            ensure!(T::IsSudo::is_sudo(&who), "Only sudo can remove members");

            // Remove the member from the group
            let mut group = Self::group();
            if let Some(index) = group.members.iter().position(|&x| x == member) {
                group.members.remove(index);
                <GroupStorage<T>>::put(group);
            }

            Ok(())
        }

        // Remove the sender from the group
        #[weight = 0]
        pub fn remove_self(origin) -> dispatch::DispatchResult {
            let who = ensure_signed(origin)?;

            // Remove the sender from the group
            let mut group = Self::group();
            if let Some(index) = group.members.iter().position(|&x| x == who) {
                group.members.remove(index);
                <GroupStorage<T>>::put(group);
            }

            Ok(())
        }

        // Add the sender to the group if the group size is less than 2
        #[weight = 0]
        pub fn add_self(origin) -> dispatch::DispatchResult {
            let who = ensure_signed(origin)?;

            // Check if the group size is less than 2
            let group = Self::group();
            ensure!(group.members.len() < 2, "Group is full");

            // Add the sender to the group
            let mut group = group;
            group.members.push(who);
            <GroupStorage<T>>::put(group);

            Ok(())
        }
    }
}

// Declare the storage
decl_storage! {
    trait Store for Module<T: Config> as MyPallet {
        GroupStorage get(fn group): Group;
    }
}

// Declare the events
decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as frame_system::Config>::AccountId,
    {
        MemberAdded(AccountId),
        MemberRemoved(AccountId),
        SelfRemoved(AccountId),
        SelfAdded(AccountId),
    }
);

// Declare the configuration trait
pub trait Config: frame_system::Config {
    // The type to check if a user is sudo
    type IsSudo: IsSudo<Self::AccountId>;
}

// Declare the IsSudo trait
pub trait IsSudo<AccountId> {
    fn is_sudo(account: &AccountId) -> bool;
}

// Implement the IsSudo trait for the root user
impl<T: Config> IsS
