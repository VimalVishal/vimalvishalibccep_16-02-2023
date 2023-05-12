// Users can be able to add themselves in the group if the group size is less than 2.
#[weight = 10_000]
fn add_self_to_group(origin) -> DispatchResult {
    let sender = ensure_signed(origin)?;

    ensure!(
        Self::Members().len() < 2,
        "Group is already full"
    );

    let member = GroupMember {
        account: sender.clone(),
        is_sudo: false,
    };
    Self::deposit_event(Event::MemberAdded(sender));
    Self::Members().push(member);
    Ok(())
}

