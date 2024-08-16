# Bus Booking Backend

## Overview

The Bus Booking Backend is a decentralized application built on the Internet Computer (IC) platform. It manages a system of administrators, groups, members, contributions, and proposals. This application allows for the creation and management of bus booking-related entities, tracking contributions, handling group memberships, and managing proposals.

## Components

### Canister Types

The system is divided into several key entities:

- **Admin**: Represents an administrator with unique credentials.
- **Group**: Represents a group managed by an administrator.
- **Member**: Represents a member who can contribute to groups and participate in proposals.
- **Contribution**: Represents a contribution made by a member to a group.
- **Proposal**: Represents a proposal made within a group for voting.

### Structs

Each entity is defined as a struct, with the following details:

- **Admin**: 
  - `id`: Unique identifier
  - `name`: Administrator's name
  - `email`: Administrator's email
  - `created_at`: Timestamp of creation

- **Group**:
  - `id`: Unique identifier
  - `name`: Group name
  - `admin_id`: ID of the administrator managing the group
  - `members`: List of member IDs
  - `created_at`: Timestamp of creation

- **Member**:
  - `id`: Unique identifier
  - `name`: Member's name
  - `email`: Member's email
  - `points`: Points accumulated by the member
  - `created_at`: Timestamp of creation

- **Contribution**:
  - `id`: Unique identifier
  - `group_id`: ID of the group receiving the contribution
  - `member_id`: ID of the contributing member
  - `amount`: Amount contributed
  - `created_at`: Timestamp of contribution

- **Proposal**:
  - `id`: Unique identifier
  - `group_id`: ID of the group to which the proposal belongs
  - `proposer_id`: ID of the member making the proposal
  - `description`: Description of the proposal
  - `votes_for`: Number of votes in favor
  - `votes_against`: Number of votes against
  - `created_at`: Timestamp of proposal creation

### Payloads

Payloads are data structures used for creating or updating entities:

- **AdminPayload**: Used to create or update an administrator.
- **GroupPayload**: Used to create a group.
- **MemberPayload**: Used to create a member.
- **ContributionPayload**: Used to add a contribution.
- **ProposalPayload**: Used to create a proposal.
- **VotePayload**: Used to vote on a proposal.
- **AddMemberToGroupPayload**: Used to add a member to a group.

### Storage and State Management

State is managed using the Internet Computer's stable memory:

- **ID_COUNTER**: Generates unique IDs for entities.
- **ADMIN_STORAGE**: Stores administrators.
- **GROUPS_STORAGE**: Stores groups.
- **MEMBERS_STORAGE**: Stores members.
- **CONTRIBUTIONS_STORAGE**: Stores contributions.
- **PROPOSALS_STORAGE**: Stores proposals.

### Functions

Here are the primary functions available:

1. **create_admin**: Creates a new administrator with validation for name and email.
2. **update_admin**: Updates an existing administrator’s details.
3. **create_group**: Creates a new group, ensuring the requesting admin exists.
4. **add_member**: Adds a new member with validation for name and email.
5. **add_member_to_group**: Adds an existing member to a group, validating admin permissions.
6. **add_contribution**: Records a contribution by a member, awarding points.
7. **create_proposal**: Creates a new proposal within a group.
8. **vote_proposal**: Allows a member to vote on a proposal.
9. **get_group_contributions**: Retrieves all contributions for a specific group.
10. **get_leaderboard**: Retrieves all members sorted by points.
11. **get_group_proposals**: Retrieves all proposals for a specific group.

### Error Handling

Errors are handled using the `Error` enum, which includes various types of errors such as `UnAuthorized`, `NotFound`, `EmptyFields`, `NotGroupMember`, `AlreadyExists`, `InvalidEmail`, and `InvalidName`.

### Utility Functions

- **get_current_time**: Retrieves the current time in milliseconds since the Unix epoch.

## Usage

1. **Deploying the Canister**: Build the WASM file using the Internet Computer SDK and deploy the canister.
2. **Interacting with the Canister**: Use the methods exposed by the canister to manage admins, groups, members, contributions, and proposals.

## Building and Running

To build and run this canister:

1. Ensure you have Rust and the Internet Computer SDK installed.
2. Compile the canister:
   ```sh
   cargo build --target wasm32-unknown-unknown --release
   ```
3. Deploy the canister using the Internet Computer SDK.

## Generating the Candid Interface

To generate the Candid interface description file:

```sh
candid-extractor target/wasm32-unknown-unknown/release/bus_booking_backend.wasm > bus_booking_backend.did
```

This command will produce the `bus_booking_backend.did` file, which describes the canister's interface in Candid.

---

# rust_busbooking
