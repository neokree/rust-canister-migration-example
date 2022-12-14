# Rust canister data migration example

This is a test repo which shows the code required to do canister data migration, when upgrading to new canister code. 
It's based on the counter example canister, which comes as default with new rust projects.

## Things to be careful on 

When developing a new canister **ALWAYS** set and use the `pre_upgrade` method.  
Otherwise all canisters data will be lost on the next code migration.

The `pre_upgrade` method is called when an existing canister is being deployed, or the ICP node is starting up and loading data from disk (also during development).

When doing a migration:
- `pre_upgrade` is always called on the code currently deployed on ICP, if you are deploying a new canister, it will call the **OLD** canister code, not the new one you are deploying. 
- `post_upgrade` of the new code (canister) you are deploying will then be triggered, to restore all data stored in the migration. 

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
dfx start --background

# Deploys your canisters to the replica
dfx deploy
```

Once the job completes, your application will be available at `http://localhost:4943?canisterId={canister_id}`.

### If this was helpful to you, consider adding a star, thank you
