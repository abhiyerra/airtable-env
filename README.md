# Airtable Env

Use Airtable to host your Environment Variables.

## Usage

```
eval $(airtable-env <api-key> <base-key> <table> <view> <key-field> <value-field>)
```

 - key-field is the field holding the key name
 - value-field is teh field holding the value name

## Why?

Because you are a cheap bastard who doesn't want to pay for AWS Secrets
Manager.