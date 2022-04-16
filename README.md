 # Crowdfunding
 What make it different?
 We will implement vesting approach for distribute funding.
 With each interval, every supporter need to vote to release or cancel this funding

## TODO
- Implement DAO for each interval
- admin fns to control this dapp
### new
```sh
near call $ID new '{}' --accountId $ID
```
### new_project
```sh
// Create new job: job_creator.testnet
near call $ID new_project '{}' --accountId job_creator.testnet --amount 1
```
### support_project
```sh
near call $ID support_project '{"project_id": "'$PID'"}' --accountId job_worker.testnet --amount 2
```

### get_claimable_amount
```sh
near view $ID get_claimable_amount '{"project_id": "'$PID'"}'
```

### claim_reward
```sh
near call $ID claim_reward '{"project_id": "'$PID'"}' --accountId job_worker.testnet 
```

//TODO: Force stop cases:
//TODO: Create new project and call force_stop 
### force_stop
```sh
near call $ID force_stop '{"project_id": "'$PID'"}' --accountId job_worker.testnet 

```
