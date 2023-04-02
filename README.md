edit `config.yml` for your purpose;  
  
`key` is the API Key  
`passphrase` is the API Passphrase / Secret  
`pair` is the kraken trading pair -> https://docs.kraken.com/rest/#tag/Market-Data/operation/getTradableAssetPairs   
`volume` is the base asset volume  
`dry_run` if set to true, the order is not executed but rather validated.  


e.g.

```
key: "[APIKEY]"
passphrase: "[SECRET]"
pair: "DOTUSDT"
volume: "10"
dry_run: false
```

(buys 10 DOT for the current market price in USDT)
