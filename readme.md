# GOAL
write a simple POC to validate if rowan has performance issue, it seems no.

## benchmark
parse the `assets/big.json` 500kb  
```bash
rowan 7.912884ms  
traverse rowan 1.175211ms  
stringify rowan 12.196484ms  
parse lr 7.139681ms  
traverse_lr 110.989µs  
stringify 2.689489ms  
nom 15.36363ms  
```
