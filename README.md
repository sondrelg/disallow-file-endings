# Disallow file endings

Pre-commit hook that lets you specify banned file endings.

I keep naming half my `yaml` files `*.yaml` and the other `*.yml`
and it's causing me too much pain.

Could also be used for creating repository standards for other file types.

## How to use it

Just add this to your `.pre-commit-config.yaml` (not `.yml`!)

```yaml
- repo: https://github.com/sondrelg/disallow-file-endings
  rev: ''
  hooks:
    - id: disallow-file-extensions
      args:
        - --extensions=.yml
```
