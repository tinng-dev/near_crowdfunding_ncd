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
export CUR=date + %s
// Create new job: job_creator.testnet
near call $ID new_project '{"metadata": {"title": "Example", description": "https://abc.com", "target": "1000000000000000000000000000000", "minimum_deposit": "100000000000000000000000", "started_at": 11111111111111111, "ended_at": 4324324234234324234, "funded": "0", .....}}' --accountId job_creator.testnet --amount 1
```
### support_project
```sh
near call $I support_project '{"project_id": $PID}' --accountId job_worker.testnet --amount 2
```
