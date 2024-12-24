# Reliable Agreement Protocol

A simple reliable agreement protocol as described here: [https://eprint.iacr.org/2024/677.pdf](https://eprint.iacr.org/2024/677.pdf)


## Instructions to run

```bash
sudo lsof -ti :7000-7015,5000 | sudo xargs kill -9
```

```bash
./scripts/test.sh testdata/hyb_4/syncer Hi false
```
