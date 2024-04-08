# BTCTimeLock Lock Script

BTCTimeLock is a time lock used when an owner migrates RGB++ assets from Bitcoin to CKB. The assets must remain locked for at least five Bitcoin blocks to safeguard them against potential chain reorgs.

## Operations

### Unlock

The BTC timelock can be unlocked once `btc_txid`'s confirmations surpass `after` (at least five), the `outputs` must contain correspondent cells where each cell's type, data, and capacity are equal to the corresponding input BitcoinTimeLock cell. The lock field must equal `lock_script`.

```yaml
inputs:
  - previous_out_point: (BitcoinTimeLock args: lock_script | after | btc_txid)
outputs:
  - lock: (must be same as lock_script)
    type: (must remain unchanged)
    data: (must remain unchanged)
    capacity: (must remain unchanged)
  ...
```
